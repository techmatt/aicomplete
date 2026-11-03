// `npm run cards`: exports the cards from this checkout and serves the viewer at
// http://localhost:5180/cards.html until stopped. Plain Node: no npm install, no cargo.
//   --port N    another port (the fixed one is what conductor's button opens)
//   --root DIR  read content and keep notes in another tree (scripts/cards-check.mjs)
// Notes live in <root>/review/card-notes.json, git-ignored; review/card-seen.json is the
// ledger behind the "new" badge. See web/README.md, Card viewer.

import { closeSync, copyFileSync, existsSync, fsyncSync, mkdirSync, openSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeSync } from 'node:fs';
import { createServer } from 'node:http';
import { dirname, extname, join, resolve, sep } from 'node:path';

import { originalsDir } from '../card/originals.mjs';
import { REPO, exportCards } from './cards-export.mjs';

const arg = (name) => {
  const i = process.argv.indexOf(name);
  return i > 0 ? process.argv[i + 1] : undefined;
};
const PORT = Number(arg('--port') ?? 5180);
const ROOT = resolve(arg('--root') ?? REPO);
const WEB = join(REPO, 'web');
const NOTES = join(ROOT, 'review', 'card-notes.json');
const SEEN = join(ROOT, 'review', 'card-seen.json');
const FLAGS = ['ok', 'too cheap', 'too dear', 'rework', 'unclear text'];
/** Scene batch folders (`b<n>`) under the originals, newest first: a redrawn id takes the latest draw. */
function sceneRounds(base) {
  const dir = join(base, 'scenes');
  if (!existsSync(dir)) return [];
  return readdirSync(dir)
    .filter((d) => /^b\d+$/.test(d))
    .sort((a, b) => Number(b.slice(1)) - Number(a.slice(1)));
}
const MIME = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8', '.json': 'application/json', '.png': 'image/png', '.jpg': 'image/jpeg', '.woff2': 'font/woff2', '.ttf': 'font/ttf' };

let data;
let exported = '';

/** A broken file must stop a save, never be overwritten by one. */
function readLedger(file) {
  if (!existsSync(file)) return {};
  return JSON.parse(readFileSync(file, 'utf8').replace(/^\uFEFF/, ''));
}

/** Windows refuses a rename while anything (a sync client, a scanner) has the target open. */
function renameRetrying(from, to) {
  for (let tries = 0; ; tries++) {
    try {
      return renameSync(from, to);
    } catch (e) {
      if (tries >= 20 || !['EPERM', 'EBUSY', 'EACCES'].includes(e.code)) {
        rmSync(from, { force: true });
        throw e;
      }
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 50);
    }
  }
}

/** On disk before it replaces the old file, and the old file kept as .bak: nothing regenerates notes. */
function writeLedger(file, value) {
  mkdirSync(dirname(file), { recursive: true });
  const tmp = `${file}.tmp`;
  const fd = openSync(tmp, 'w');
  try {
    writeSync(fd, `${JSON.stringify(value, null, 2)}\n`);
    fsyncSync(fd);
  } finally {
    closeSync(fd);
  }
  if (existsSync(file)) copyFileSync(file, `${file}.bak`);
  renameRetrying(tmp, file);
}

function rebuild() {
  const t0 = performance.now();
  const next = exportCards(ROOT);
  // The first run is the baseline: a whole set of "new" says nothing.
  if (!existsSync(SEEN)) writeLedger(SEEN, Object.fromEntries(next.cards.map((c) => [c.id, c.hash])));
  data = next;
  exported = new Date().toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' }).toLowerCase().replace(' ', '');
  return Math.round(performance.now() - t0);
}

/** The page still opens on a broken ledger, and says so; a save stays refused. */
function state() {
  const warnings = [...data.warnings];
  const read = (file) => {
    try {
      return readLedger(file);
    } catch (e) {
      warnings.push(`${file} is broken, so it is ignored and saving is refused until it is fixed (${e.message})`);
      return {};
    }
  };
  return { ...data, exported, flags: FLAGS, notes: read(NOTES), seen: read(SEEN), warnings };
}

function saveNote({ id, flag = '', comment = '' }) {
  const card = data.cards.find((c) => c.id === id);
  if (!card) throw Object.assign(new Error(`no card ${id}`), { status: 404 });
  if (flag && !FLAGS.includes(flag)) throw Object.assign(new Error(`no flag ${flag}`), { status: 400 });
  if (typeof comment !== 'string') throw Object.assign(new Error('comment is text'), { status: 400 });
  const notes = readLedger(NOTES);
  if (!flag && !comment.trim()) delete notes[id];
  else notes[id] = { name: card.name, kind: card.kind, hash: card.hash, flag, comment: comment.trim(), text: card.text, at: new Date().toISOString() };
  writeLedger(NOTES, notes);
  try {
    markSeen([id]);
  } catch (e) {
    console.warn(`cards: ${e.message}`);
  }
  return notes[id] ?? null;
}

function markSeen(ids) {
  const seen = readLedger(SEEN);
  for (const c of data.cards) if (ids.includes(c.id)) seen[c.id] = c.hash;
  writeLedger(SEEN, seen);
}

function sceneFile(id) {
  if (!/^[a-z0-9_-]+$/.test(id)) return null;
  const base = originalsDir(REPO);
  for (const round of sceneRounds(base)) {
    const f = join(base, 'scenes', round, `${id}.png`);
    if (existsSync(f)) return f;
  }
  return null;
}

/** The URL path's file, inside the folders the page may read. */
function staticFile(path) {
  const within = (base, rest) => {
    const f = resolve(base, rest);
    return f.startsWith(base + sep) ? f : null;
  };
  if (path === '/cards.html') return join(WEB, 'cards.html');
  if (path.startsWith('/cards/')) return within(join(WEB, 'cards'), path.slice(7));
  if (path.startsWith('/card/')) return within(join(WEB, 'card'), path.slice(6));
  if (path.startsWith('/art/')) return within(join(REPO, 'art'), path.slice(5));
  if (path.startsWith('/scene/')) return sceneFile(path.slice(7).replace(/\.png$/, ''));
  return null;
}

const send = (res, status, body, type = 'application/json') => {
  res.writeHead(status, { 'content-type': type, 'cache-control': 'no-store' });
  res.end(typeof body === 'string' || Buffer.isBuffer(body) ? body : JSON.stringify(body));
};

const readBody = (req) =>
  new Promise((ok, fail) => {
    let body = '';
    req.setEncoding('utf8');
    req.on('data', (d) => {
      body += d;
      if (body.length > 1e6) fail(Object.assign(new Error('too long'), { status: 413 }));
    });
    req.on('end', () => {
      try {
        ok(body ? JSON.parse(body) : {});
      } catch {
        fail(Object.assign(new Error('not JSON'), { status: 400 }));
      }
    });
    req.on('error', fail);
  });

/** Another site's page must not be able to write notes through the visitor's browser. */
function local(req) {
  const host = (req.headers.host ?? '').replace(/:\d+$/, '');
  const origin = req.headers.origin;
  if (!['localhost', '127.0.0.1', '[::1]'].includes(host)) return false;
  return !origin || origin === `http://${req.headers.host}`;
}

async function handle(req, res) {
  const url = new URL(req.url, 'http://localhost');
  const path = decodeURIComponent(url.pathname);
  if (req.method === 'POST') {
    if (!local(req)) return send(res, 403, { error: 'local pages only' });
    if (!(req.headers['content-type'] ?? '').startsWith('application/json')) return send(res, 415, { error: 'send JSON' });
    if (path === '/api/notes') return send(res, 200, { note: saveNote(await readBody(req)), ...state() });
    if (path === '/api/seen') {
      const { ids, all } = await readBody(req);
      markSeen(all ? data.cards.map((c) => c.id) : (ids ?? []));
      return send(res, 200, { seen: readLedger(SEEN) });
    }
    if (path === '/api/rebuild') return send(res, 200, { ms: rebuild(), ...state() });
    return send(res, 404, { error: 'no such call' });
  }
  if (req.method !== 'GET' && req.method !== 'HEAD') return send(res, 405, { error: 'GET or POST' });
  if (path === '/') {
    res.writeHead(302, { location: '/cards.html' });
    return res.end();
  }
  if (path === '/api/cards') return send(res, 200, state());
  const file = staticFile(path);
  if (!file || !existsSync(file) || !statSync(file).isFile()) return send(res, 404, 'not found', 'text/plain');
  // Scenes are megabytes each and never change under a name; the rest follows the checkout.
  const body = readFileSync(file);
  res.writeHead(200, { 'content-type': MIME[extname(file)] ?? 'application/octet-stream', 'cache-control': path.startsWith('/scene/') ? 'max-age=86400' : 'no-store' });
  res.end(body);
}

const onRequest = (req, res) => {
  handle(req, res).catch((e) => {
    if (res.headersSent) res.destroy();
    else send(res, e.status ?? 500, { error: e.message });
  });
};

const taken = () => {
  console.error(`cards: port ${PORT} is taken. If that is this viewer, it is already serving: open http://localhost:${PORT}/cards.html`);
  process.exit(1);
};

const ms = rebuild();
for (const w of data.warnings) console.warn(`warning: ${w}`);

// `localhost` resolves to ::1 first here, and listen() binds only one address: serve both
// families, so a browser and a script reach the same server. A machine may have no IPv6.
const v4 = createServer(onRequest);
v4.on('error', (e) => {
  if (e.code === 'EADDRINUSE') taken();
  throw e;
});
v4.listen(PORT, '127.0.0.1', () => {
  const v6 = createServer(onRequest);
  const ready = () => {
    console.log(`cards: ${data.cards.length} cards exported in ${ms} ms from ${ROOT}`);
    console.log(`cards: http://localhost:${PORT}/cards.html (Ctrl+C stops it)`);
  };
  v6.on('error', (e) => {
    if (e.code === 'EADDRINUSE') taken();
    if (['EADDRNOTAVAIL', 'EAFNOSUPPORT'].includes(e.code)) ready();
    else throw e;
  });
  v6.listen(PORT, '::1', ready);
});
