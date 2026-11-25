import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig, type Plugin, type ViteDevServer } from 'vite';
import { originalsDir } from './card/originals.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '..');

/** The page served in place of the viewer while the server's first wasm build runs. */
const waitPage = (failed: boolean) => `<!doctype html>
<meta charset="utf-8"><meta http-equiv="refresh" content="${failed ? 5 : 1}">
<title>AI Complete: ${failed ? 'engine build failed' : 'building the engine'}</title>
<body style="font: 16px system-ui; margin: 3em">
<p>${
  failed
    ? 'The wasm build failed and there is no earlier engine to serve: the error is in the dev server&#39;s output. An edit under crates/ or content/ rebuilds it; this page retries every 5 s.'
    : 'Building the engine (the wasm, from crates/): a minute or two after a crate change, seconds otherwise. This page opens the viewer when it is ready.'
}</p>`;

/**
 * Dev only: builds the wasm package when the server starts (build-wasm.mjs skips it when
 * its inputs are unchanged) and again when engine sources or content change, then
 * reloads the page. Until the first build ends the viewer's URL answers with a waiting
 * page, so a launcher's ready check passes during a cold build. Configs are read through
 * import.meta.glob, so editing one reloads too, and a new file shows up after this
 * plugin's reload.
 */
function engineWatch(): Plugin {
  const wasmTime = () => {
    try {
      return statSync(resolve(here, 'src/wasm/aic_wasm_bg.wasm')).mtimeMs;
    } catch {
      return 0;
    }
  };
  let running = false;
  let again = false;
  let starting = true; // the first build hasn't ended
  let noEngine = false; // it failed and web/src/wasm holds no earlier build
  const rebuild = (server: ViteDevServer, why: string) => {
    if (running) {
      again = true;
      return;
    }
    running = true;
    server.config.logger.info(`${why}: building wasm…`, { timestamp: true });
    const before = wasmTime();
    const p = spawn(process.execPath, [resolve(here, 'scripts/build-wasm.mjs')], { stdio: 'inherit' });
    p.on('exit', (code) => {
      running = false;
      if (code === 0) noEngine = false;
      else noEngine = wasmTime() === 0;
      starting = false;
      if (again) {
        again = false;
        rebuild(server, 'engine changed');
      } else if (code === 0) {
        // A build its inputs' hash skipped (an aic-sim edit, a test) leaves the page be.
        if (wasmTime() !== before) server.ws.send({ type: 'full-reload' });
      } else {
        server.config.logger.error('wasm build failed; the page keeps the last good engine');
      }
    });
  };
  return {
    name: 'aic-engine-watch',
    apply: 'serve',
    configureServer(server) {
      const watched = [resolve(repo, 'crates'), resolve(repo, 'content'), resolve(repo, 'configs')];
      server.watcher.add(watched);
      const onChange = (file: string) => {
        const f = resolve(file);
        if (f.startsWith(resolve(repo, 'configs'))) server.ws.send({ type: 'full-reload' });
        else if (/[\\/](crates|content)[\\/]/.test(f) && !/[\\/](pkg|target)[\\/]/.test(f) && /\.(rs|json|toml)$/.test(f))
          rebuild(server, 'engine changed');
      };
      server.watcher.on('change', onChange);
      server.watcher.on('add', onChange);
      server.middlewares.use((req, res, next) => {
        const path = (req.url ?? '/').split('?')[0];
        if (!(starting || noEngine) || req.method !== 'GET' || !(path === '/' || path === '/index.html')) return next();
        res.statusCode = 200;
        res.setHeader('Content-Type', 'text/html; charset=utf-8');
        res.setHeader('Cache-Control', 'no-store');
        res.end(waitPage(!starting));
      });
      rebuild(server, 'server start');
    },
  };
}

/**
 * 325 (Matt, Nov 25): the player view's printed cards show their scenes. Dev only: `/__scenes.json` maps each card id
 * to its scene's URL (tools/card-art/scenes.json's `card`, for a scene drawn in art-originals/, the latest batch first,
 * as the card viewer picks it), and `/__scene/<id>.jpg` serves that scene downsized (scripts/scene-thumb.py, Python and
 * Pillow), cached in out/scene-cache/ (git-ignored). Nothing is committed; a built site, or a card with no scene drawn,
 * prints its frame alone.
 */
function sceneArt(): Plugin {
  const WIDTH = 480;
  const cacheDir = resolve(repo, 'out', 'scene-cache');
  let files: Map<string, string> | null = null;
  let cards: Record<string, string> = {};
  /** Scene id -> its newest original, and card id -> scene id; read once (a new batch: restart the server). */
  const index = () => {
    if (files) return;
    files = new Map();
    const base = originalsDir(repo);
    const dir = join(base, 'scenes');
    const batches = existsSync(dir) ? readdirSync(dir).filter((d) => /^b\d+$/.test(d)).sort((a, b) => Number(b.slice(1)) - Number(a.slice(1))) : [];
    for (const b of batches) for (const f of readdirSync(join(dir, b))) if (f.endsWith('.png') && !files.has(f.slice(0, -4))) files.set(f.slice(0, -4), join(dir, b, f));
    try {
      const pool = JSON.parse(readFileSync(resolve(repo, 'tools/card-art/scenes.json'), 'utf8')).scenes ?? {};
      for (const group of Object.values(pool) as Record<string, { card?: string }>[])
        for (const [id, s] of Object.entries(group)) if (s.card && files.has(id)) cards[s.card] = `./__scene/${id}.jpg`;
    } catch {
      cards = {};
    }
  };
  const pending = new Map<string, Promise<boolean>>();
  // At most 3 downsizes at once (a cold cache asks for a draft's and a hand's scenes together), each given 30 s (an
  // original still syncing from Drive can stall the read): a refused or slow one prints the frame alone.
  let running = 0;
  const queue: (() => void)[] = [];
  const slot = () => new Promise<void>((go) => (running < 3 ? (running++, go()) : queue.push(() => (running++, go()))));
  const free = () => {
    running--;
    queue.shift()?.();
  };
  const thumb = (src: string, dst: string) => {
    let p = pending.get(dst);
    if (!p) {
      p = slot().then(() => new Promise<boolean>((done) => {
        mkdirSync(cacheDir, { recursive: true });
        const py = spawn(process.platform === 'win32' ? 'python' : 'python3', [resolve(here, 'scripts/scene-thumb.py'), src, dst, String(WIDTH)], { stdio: 'ignore' });
        const timer = setTimeout(() => py.kill(), 30_000);
        py.on('error', () => done(false));
        py.on('exit', (code) => {
          clearTimeout(timer);
          done(code === 0 && existsSync(dst));
        });
      })).finally(() => {
        free();
        pending.delete(dst);
      });
      pending.set(dst, p);
    }
    return p;
  };
  return {
    name: 'aic-scene-art',
    apply: 'serve',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const path = (req.url ?? '/').split('?')[0];
        if (path === '/__scenes.json') {
          index();
          res.setHeader('Content-Type', 'application/json');
          res.setHeader('Cache-Control', 'no-store');
          return res.end(JSON.stringify(cards));
        }
        const m = /^\/__scene\/([a-z0-9_-]+)\.jpg$/.exec(path);
        if (!m) return next();
        index();
        const src = files!.get(m[1]);
        if (!src) {
          res.statusCode = 404;
          return res.end();
        }
        const dst = join(cacheDir, `${m[1]}-${Math.round(statSync(src).mtimeMs)}-${WIDTH}.jpg`);
        const send = () => {
          res.setHeader('Content-Type', 'image/jpeg');
          res.setHeader('Cache-Control', 'max-age=86400');
          res.end(readFileSync(dst));
        };
        if (existsSync(dst)) return send();
        void thumb(src, dst).then((ok) => {
          if (ok) return send();
          server.config.logger.warn(`scene ${m[1]}: could not downsize ${src} (Python with Pillow on PATH?)`);
          res.statusCode = 404;
          res.end();
        });
      });
    },
  };
}

export default defineConfig({
  base: './',
  plugins: [engineWatch(), sceneArt()],
  server: {
    // configs/ and content/ live beside web/.
    fs: { allow: [repo] },
  },
});
