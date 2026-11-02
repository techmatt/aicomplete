// Shared by the card-art scripts: paths, the one browser, Codex's image tool.
import { spawn } from 'node:child_process';
import { appendFile, copyFile, readdir, readFile, rename, stat, mkdir } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { chromium } from 'playwright-core';

import { fitCards, frameOf } from '../../web/card/card.js';
import { originalsDir } from '../../web/card/originals.mjs';

export const TOOL = path.dirname(fileURLToPath(import.meta.url));
export const REPO = path.resolve(TOOL, '..', '..');
export const CARD = path.join(REPO, 'web', 'card');
export const ART = path.join(REPO, 'art');
export const OUT = path.join(REPO, 'out', 'cards');
export const FRAMES = path.join(ART, 'frames', 'r3');
export const ORIGINALS = originalsDir(REPO);

const CODEX_HOME = process.env.CODEX_HOME ?? path.join(os.homedir(), '.codex');
const TIMEOUT_MS = 6 * 60_000;
// How long pipes may stay open after the process has gone: a stray grandchild can hold them.
const GRACE_MS = 5_000;

/**
 * The originals folder for a picker, or stops: its manifest links ../art-originals, which
 * only the junction serves. AIC_ART_ORIGINALS stands in for the junction, with a warning.
 */
export function pickerOriginals(who) {
  const junction = path.join(REPO, 'art-originals');
  if (process.env.AIC_ART_ORIGINALS !== undefined) console.warn(`${who}: images in ${ORIGINALS}: the manifest's ../art-originals links show them only if ${junction} points there`);
  else if (!existsSync(junction)) throw new Error(`${who}: ${junction} is not reachable: no picker written`);
  return ORIGINALS;
}

/** The n for a kept <id>.try<n>.png: above every n in use, so a gap can't overwrite a try. */
async function nextTry(dir, id) {
  const used = (await readdir(dir)).filter((f) => f.startsWith(`${id}.try`)).map((f) => parseInt(f.slice(id.length + 4), 10));
  return Math.max(0, ...used.filter(Number.isInteger)) + 1;
}

/** Ends a process and all it started: on Windows `child` is the cmd.exe before codex's shim. */
function killTree(child) {
  if (process.platform !== 'win32') return child.kill();
  spawn('taskkill', ['/pid', String(child.pid), '/t', '/f'], { stdio: 'ignore' }).on('error', () => child.kill());
}

export const readJson = async (f) => JSON.parse(await readFile(f, 'utf8'));
export const layout = () => readJson(path.join(CARD, 'layout.json'));

/**
 * Player cards from content: `cards` by id, and `faced(id)`, a card with its generated face
 * (web/card/faces.json) merged in.
 */
export async function loadCards() {
  const cards = Object.fromEntries((await readJson(path.join(REPO, 'content', 'cards.json'))).map((c) => [c.id, c]));
  const faces = await readJson(path.join(CARD, 'faces.json'));
  return { cards, faced: (id) => ({ ...cards[id], ...faces[id] }) };
}

/** The largest box of `shape` (width over height) centered in an image of this size. */
export function centerBox({ width, height }, shape) {
  const w = Math.min(width, Math.round(height * shape));
  const h = Math.min(height, Math.round(width / shape));
  return { left: Math.floor((width - w) / 2), top: Math.floor((height - h) / 2), width: w, height: h };
}

/** One headless browser (system Chrome), one page, for every render. */
export async function openPage(width, height) {
  const browser = await chromium.launch({ channel: 'chrome' });
  const page = await browser.newPage({ viewport: { width, height } });
  return { page, close: () => browser.close() };
}

export const MM = 96 / 25.4;
export { frameOf };
export const frameUrl = (v) => pathToFileURL(path.join(FRAMES, `${v}.jpg`)).href;
/** Cards at true size, 9 to an A4 page, bleeds touching: wrap each page in `.sheet-page`. */
export const SHEET_CSS = '@page{size:210mm 297mm;margin:0}.sheet-page{width:207mm;height:282mm;padding:7.500mm 1.500mm;display:grid;grid-template-columns:repeat(3,69mm);grid-auto-rows:94mm;break-after:page}';
/** Card HTML laid out in `.sheet-page`s, for SHEET_CSS: `{ body, pages }`, pages being how many. */
export function sheetPages(faces) {
  const pages = [];
  for (let i = 0; i < faces.length; i += 9) pages.push(`<section class="sheet-page">${faces.slice(i, i + 9).join('')}</section>`);
  return { body: pages.join(''), pages: pages.length };
}

/**
 * A page at print scale for layout `l`. Returns `{ p, show, clip, close }`: `show(body,
 * extra)` sets the page to `body` under card.css plus `extra`, waits for the fonts and fits
 * every card in it (fitCards, whose rows it returns); `clip` is one card with its bleed.
 */
export async function cardPage(l, width, height) {
  const browser = await chromium.launch({ channel: 'chrome' });
  const ctx = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: l.print_px[0] / (l.bleed_mm[0] * MM) });
  const p = await ctx.newPage();
  // A file: page so the relative font and container urls in card.css resolve.
  const stub = path.join(CARD, 'card.css');
  await p.goto(pathToFileURL(stub).href.replace('card.css', 'layout.json'));
  const css = await readFile(stub, 'utf8');
  const base = pathToFileURL(CARD + path.sep).href;
  const show = async (body, extra = '') => {
    await p.setContent(`<!doctype html><html><head><base href="${base}"><style>body{margin:0}${css}${extra}</style></head><body>${body}</body></html>`, { waitUntil: 'load' });
    await p.evaluate(() => document.fonts.ready);
    return p.evaluate(fitCards);
  };
  const clip = { x: 0, y: 0, width: l.bleed_mm[0] * MM, height: l.bleed_mm[1] * MM };
  return { p, show, clip, close: () => browser.close() };
}

/** The opening every image prompt shares; the prompt's own lines follow it. */
export const IMAGE_PREAMBLE = [
  'Use your built-in image_gen tool ($imagegen) exactly once to generate the image below.',
  'Do not read any files, run any commands, or try to save the image anywhere: just generate it, then stop.',
];

/** `--pool=N`, default 3: how many draws run at once. `who` prefixes the message. */
export function parsePool(argv, who) {
  const pool = Number(argv.find((a) => a.startsWith('--pool='))?.slice(7) ?? 3);
  if (!Number.isInteger(pool) || pool < 1) throw new Error(`${who}: --pool must be a whole number, 1 or more`);
  return pool;
}

/**
 * Draws `ids`, `pool` at a time, stopping at the first usage-limit message. `draw(id)`
 * returns `{ run, ...fields }`; the fields go into log.jsonl in `dir`. A new image goes to
 * dir/<id>.png, any earlier one to a try, then `kept(id, dest, run, fields)` runs. Sets
 * exit code 1 if a draw failed or the limit left some undrawn.
 */
export async function drawAll(ids, pool, who, { dir, draw, kept }) {
  let stop = false;
  let failed = 0;
  async function one(id) {
    const { run, ...fields } = await draw(id);
    await appendFile(path.join(dir, 'log.jsonl'), `${JSON.stringify({ id, at: new Date().toISOString(), ...fields, seconds: run.seconds, thread: run.thread, limit: run.limit, usage: run.usage, message: run.image ? undefined : run.message })}\n`);
    if (run.limit) {
      stop = true;
      console.error(`${who}: usage limit: ${run.limit}`);
    }
    if (!run.image) {
      failed++;
      console.error(`${who}: ${id}: no image after ${run.seconds}s. ${run.message ?? ''}`);
      return;
    }
    const dest = path.join(dir, `${id}.png`);
    if (existsSync(dest)) await rename(dest, path.join(dir, `${id}.try${await nextTry(dir, id)}.png`));
    await copyFile(run.image, dest);
    await kept(id, dest, run, fields);
  }
  const queue = [...ids];
  await Promise.all(
    Array.from({ length: pool }, async () => {
      while (queue.length && !stop) await one(queue.shift());
    }),
  );
  if (failed || queue.length) {
    process.exitCode = 1;
    console.error(`${who}: ${ids.length - failed - queue.length} of ${ids.length} drawn, ${failed} failed, ${queue.length} not started`);
  }
}

/** Draw one image with Codex's built-in image tool; same route as wordwarriors' art:gen. */
export async function runCodex(prompt, image) {
  const workdir = path.join(os.tmpdir(), 'aic-codex');
  await mkdir(workdir, { recursive: true });
  const started = Date.now();
  // codex is an npm .cmd shim, which Node only spawns through a shell. The prompt goes on stdin.
  const child = spawn(`codex exec ${image ? `-i "${image}" ` : ''}--json --skip-git-repo-check -s read-only -C "${workdir}" -`, { shell: true, stdio: ['pipe', 'pipe', 'pipe'] });
  child.stdin.end(prompt);
  let out = '';
  let err = '';
  child.stdout.on('data', (d) => (out += d.toString('utf8')));
  child.stderr.on('data', (d) => (err += d.toString('utf8')));
  let timedOut = false;
  const timer = setTimeout(() => {
    timedOut = true;
    killTree(child);
  }, TIMEOUT_MS);
  let grace;
  // 'close' waits for the pipes, so 'exit' plus a grace settles it if something still holds them.
  const code = await new Promise((resolve) => {
    child.on('close', resolve);
    child.on('exit', (c) => (grace = setTimeout(() => resolve(c), GRACE_MS)));
  });
  clearTimeout(timer);
  clearTimeout(grace);
  child.stdout.destroy();
  child.stderr.destroy();
  const run = { seconds: Math.round((Date.now() - started) / 100) / 10 };
  for (const line of out.split('\n')) {
    let ev;
    try {
      ev = JSON.parse(line);
    } catch {
      continue;
    }
    if (ev.type === 'thread.started') run.thread = ev.thread_id;
    if (ev.item?.type === 'agent_message' && ev.item.text) run.message = ev.item.text.trim();
    const msg = ev.message ?? ev.error?.message;
    if ((ev.type === 'error' || ev.type === 'turn.failed') && msg) run.message = msg;
  }
  const limit = `${run.message ?? ''}\n${err}`.match(/[^\n]*(usage limit|rate limit|limit reached|try again (at|in))[^\n]*/i);
  if (limit) run.limit = limit[0].trim();
  if (run.thread) {
    const dir = path.join(CODEX_HOME, 'generated_images', run.thread);
    if (existsSync(dir)) {
      const pngs = (await readdir(dir)).filter((f) => f.endsWith('.png'));
      const timed = await Promise.all(pngs.map(async (f) => ({ f, t: (await stat(path.join(dir, f))).mtimeMs })));
      const newest = timed.sort((a, b) => b.t - a.t)[0];
      if (newest) run.image = path.join(dir, newest.f);
    }
    run.usage = await rateLimits(run.thread);
  }
  if (!run.image && timedOut) run.message = `codex stopped after ${TIMEOUT_MS / 1000}s`;
  if (!run.image && code !== 0 && !run.message) run.message = `codex exited ${code}: ${err.trim().split('\n').slice(-3).join(' | ')}`;
  return run;
}

/** The allowance Codex logged for this thread (the last `rate_limits` in its session file). Nothing else is read. */
async function rateLimits(thread) {
  const find = async (dir) => {
    for (const e of await readdir(dir, { withFileTypes: true }).catch(() => [])) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) {
        const hit = await find(p);
        if (hit) return hit;
      } else if (e.name.endsWith(`${thread}.jsonl`)) return p;
    }
    return undefined;
  };
  const file = await find(path.join(CODEX_HOME, 'sessions'));
  if (!file) return undefined;
  let last;
  for (const line of (await readFile(file, 'utf8')).split('\n')) {
    if (!line.includes('"rate_limits"')) continue;
    try {
      const rl = JSON.parse(line)?.payload?.rate_limits;
      if (rl) last = { primary: rl.primary?.used_percent, secondary: rl.secondary?.used_percent };
    } catch {
      /* partial line */
    }
  }
  return last;
}
