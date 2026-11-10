// Plays a farm shard on the wasm bundle, in Node or a headless browser, as a remote worker would,
// and times each game: 160's wasm measurements and the bundle's smoke test.
//   node scripts/farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N] [--fingerprint]
// BUNDLE: a folder scripts/farm/build.mjs wrote. Writes the shard's lines to OUT, one run() a seed
// (each timed; the lines are the same as one run() over the range), and prints each game's ms to
// stderr and a summary line to stdout. --fingerprint prints the bundle's fingerprint() first.
// --browser plays in headless Edge or Chrome (web/scripts/cdp.mjs; AIC_BROWSER to pick one),
// the bundle served from 127.0.0.1:PORT (--port, default 0: any free port).
import { createServer } from 'node:http';
import { readFileSync, writeFileSync } from 'node:fs';
import { extname, join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const args = process.argv.slice(2);
// Takes `name` (and its value, if `valued`) out of args: the value, true, or undefined.
const take = (name, valued = false) => {
  const i = args.indexOf(name);
  if (i < 0) return undefined;
  const v = valued ? args[i + 1] : true;
  args.splice(i, valued ? 2 : 1);
  return v;
};
const browser = take('--browser') === true;
const port = Number(take('--port', true) ?? 0);
const wantPrint = take('--fingerprint') === true;
if (args.length !== 5) {
  console.error('usage: node scripts/farm/run.mjs BUNDLE SPEC FIRST COUNT OUT [--browser] [--port N] [--fingerprint]');
  process.exit(2);
}
const [bundleArg, spec, firstArg, countArg, out] = args;
const bundle = resolve(bundleArg);
const first = Number(firstArg);
const count = Number(countArg);

// One game: { line text, ms }. In Node the bundle is imported here; in a browser, in the page.
let play;
let fingerprint;
let close = async () => {};
let setup = async () => {};
if (!browser) {
  const farm = await import(pathToFileURL(join(bundle, 'farm.mjs')).href);
  await farm.default();
  play = async (seed) => {
    const t = performance.now();
    const text = farm.run(seed, 1, spec);
    return { text, ms: performance.now() - t };
  };
  fingerprint = async () => farm.fingerprint();
} else {
  const { openPage } = await import(pathToFileURL(resolve(import.meta.dirname, '..', '..', 'web', 'scripts', 'cdp.mjs')).href);
  const types = { '.mjs': 'text/javascript', '.js': 'text/javascript', '.wasm': 'application/wasm', '.html': 'text/html' };
  const server = createServer((req, res) => {
    const name = req.url === '/' ? 'index.html' : req.url.slice(1).split('?')[0];
    if (name === 'index.html') {
      res.writeHead(200, { 'content-type': 'text/html' });
      res.end('<!doctype html><title>farm</title>');
      return;
    }
    try {
      const body = readFileSync(join(bundle, name));
      res.writeHead(200, { 'content-type': types[extname(name)] ?? 'application/octet-stream' });
      res.end(body);
    } catch {
      res.writeHead(404);
      res.end();
    }
  });
  await new Promise((r) => server.listen(port, '127.0.0.1', r));
  const url = `http://127.0.0.1:${server.address().port}/`;
  let page;
  try {
    page = await openPage();
  } catch (e) {
    server.close();
    throw e;
  }
  // From here the browser and its profile go with close(), whatever fails (below, in try/finally).
  close = async () => {
    await page.close();
    server.close();
  };
  setup = async () => {
    await page.send('Page.navigate', { url });
    await page.until('document.title === "farm"', 'the farm page', 30000);
    await page.js(`import('/farm.mjs').then(async (m) => { window.farm = m; await m.default(); return true; })`);
  };
  play = async (seed) => page.js(`(() => { const t = performance.now(); const text = window.farm.run(${seed}, 1, ${JSON.stringify(spec)});
    return { text, ms: performance.now() - t }; })()`);
  fingerprint = async () => page.js('window.farm.fingerprint()');
}

const engine = browser ? 'wasm-browser' : 'wasm-node';
try {
  await setup();
  if (wantPrint) {
    const t = performance.now();
    console.log(`${await fingerprint()}  (${engine}, ${((performance.now() - t) / 1000).toFixed(1)} s)`);
  }
  let text = '';
  let total = 0;
  let games = 0;
  for (let seed = first; seed < first + count; seed++) {
    const g = await play(seed);
    text += g.text;
    const n = g.text.split('\n').filter(Boolean).length;
    games += n;
    total += g.ms;
    console.error(`farm: seed ${seed} ${n} games ${g.ms.toFixed(1)} ms (${engine})`);
  }
  writeFileSync(out, text, { encoding: 'utf-8' });
  console.log(`farm: ${spec} ${games} games, seeds ${first}..${first + count - 1}, ${(total / Math.max(games, 1)).toFixed(1)} ms a game (${engine})`);
} finally {
  await close();
}
