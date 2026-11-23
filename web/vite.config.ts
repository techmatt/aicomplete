import { spawn } from 'node:child_process';
import { statSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig, type Plugin, type ViteDevServer } from 'vite';

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

export default defineConfig({
  base: './',
  plugins: [engineWatch()],
  server: {
    // configs/ and content/ live beside web/.
    fs: { allow: [repo] },
  },
});
