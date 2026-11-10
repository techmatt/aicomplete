// A headless Edge or Chrome page driven over the DevTools protocol, with no extra npm
// packages. Shared by every browser script here: e2e, e2e-sources, e2e-corps, e2e-researcher,
// sweep, odd, cards-check, cards-fit and the shots-*.
//   AIC_BROWSER browser executable (default: Edge, then Chrome, at their usual paths)
//   AIC_URL     the viewer the scripts drive (default http://localhost:5173/)

import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

export const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

export const viewerUrl = process.env.AIC_URL ?? 'http://localhost:5173/';

/** `check(ok, what)` prints a line and counts a failure; `failed()` says how many so far. */
export function checker() {
  let failures = 0;
  const check = (ok, what) => {
    console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
    if (!ok) failures++;
  };
  return { check, failed: () => failures };
}

/** Starts a game through the viewer's New game form and waits for it. */
export async function newGame(page, file, seed) {
  await page.js(`(() => { document.getElementById('config').value = '${file}'; document.getElementById('seed').value = '${seed}';
                          document.getElementById('new-form').requestSubmit(); })()`);
  await page.until(`window.aic.timeline.last === 0 && window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`, 30000);
}

/** The action phase's menu is up (seat 0 may pass or a standard action is legal). */
export const ACTING = `window.aic.legal.some((a) => a.type === 'standard' || a.type === 'pass' && a.seat === 0)`;

/** Opens the viewer and plays `config` (a parsed config object) at `seed`, up to its first decision. */
export async function playConfig(page, config, seed = '1') {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await page.js(`window.aic.play(${JSON.stringify(JSON.stringify(config))}, '${seed}')`);
  await page.until(`window.aic.timeline.last === 0 || ${ACTING}`, 'the game to start', 10000);
}

/** Takes the last legal action (a pass where there is one) until round `n`'s action phase,
 * at most `limit` actions; false if the game ends or the limit runs out first. */
export async function toRound(page, n, limit = 900) {
  for (let i = 0; i < limit; i++) {
    const at = await page.js(`window.aic.view.round >= ${n} && window.aic.legal.some((a) => a.type === 'standard')`);
    if (at) return true;
    if (await page.js('window.aic.legal.length === 0')) return false;
    await page.js('window.aic.act(window.aic.legal[window.aic.legal.length - 1])');
  }
  return false;
}

/** Saves the viewport as `dir/name.png`, the mouse moved off anything with a hover first. */
export async function shotTo(page, dir, name) {
  await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  writeFileSync(join(dir, `${name}.png`), await page.shot());
}

/** A shots script's page: `shot(name)` saves into the AIC_SHOTS folder. */
export async function shotsPage() {
  const dir = process.env.AIC_SHOTS;
  if (!dir) throw new Error('set AIC_SHOTS to the folder for the screenshots');
  mkdirSync(dir, { recursive: true });
  const page = await openPage();
  return { page, shot: (name) => shotTo(page, dir, name) };
}

function browserPath() {
  const candidates = [
    process.env.AIC_BROWSER,
    'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe',
    'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
    'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
    '/usr/bin/google-chrome',
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  ].filter(Boolean);
  const exe = candidates.find((p) => existsSync(p));
  if (!exe) throw new Error('no browser found; set AIC_BROWSER');
  return exe;
}

/**
 * Opens a headless browser with one page. Returns `{ send, js, until, size, shot,
 * consoleErrors, close }`; `js(expr)` evaluates an (awaited) expression in the page
 * and returns its JSON value.
 */
export async function openPage({ width = 1366, height = 768 } = {}) {
  const profile = mkdtempSync(join(tmpdir(), 'aic-cdp-'));
  const browser = spawn(browserPath(), [
    '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
    '--no-first-run', '--no-default-browser-check', `--window-size=${width},${height}`,
    '--enable-precise-memory-info', 'about:blank',
  ], { stdio: 'ignore' });

  let port;
  for (let i = 0; i < 100 && !port; i++) {
    const file = join(profile, 'DevToolsActivePort');
    // 160: the browser may still hold the file as it writes it (EBUSY on Windows): try again.
    try {
      if (existsSync(file)) port = readFileSync(file, 'utf8').split('\n')[0].trim();
    } catch (e) {
      if (e.code !== 'EBUSY' && e.code !== 'EPERM') throw e;
    }
    if (!port) await sleep(100);
  }
  if (!port) throw new Error('browser did not open a DevTools port');
  const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  const ws = new WebSocket(pages.find((p) => p.type === 'page').webSocketDebuggerUrl);
  await new Promise((r, j) => { ws.onopen = r; ws.onerror = j; });

  let nextId = 1;
  const waiting = new Map();
  const consoleErrors = [];
  ws.onmessage = (m) => {
    const msg = JSON.parse(m.data);
    if (msg.id && waiting.has(msg.id)) {
      waiting.get(msg.id)(msg);
      waiting.delete(msg.id);
    } else if (msg.method === 'Runtime.exceptionThrown') {
      consoleErrors.push(msg.params.exceptionDetails.exception?.description ?? msg.params.exceptionDetails.text);
    } else if (msg.method === 'Runtime.consoleAPICalled' && msg.params.type === 'error') {
      consoleErrors.push(msg.params.args.map((a) => a.value ?? a.description).join(' '));
    } else if (msg.method === 'Page.javascriptDialogOpening') {
      // A confirm() or alert() would block every later evaluate: accept it, and say so.
      consoleErrors.push(`dialog (${msg.params.type}): ${msg.params.message}`);
      send('Page.handleJavaScriptDialog', { accept: true }).catch(() => {});
    } else if (msg.method === 'Log.entryAdded' && msg.params.entry.level === 'error') {
      consoleErrors.push(`${msg.params.entry.text} ${msg.params.entry.url ?? ''}`.trim());
    }
  };
  const send = (method, params = {}) => new Promise((r, j) => {
    const id = nextId++;
    waiting.set(id, (msg) => (msg.error ? j(new Error(`${method}: ${msg.error.message}`)) : r(msg.result)));
    ws.send(JSON.stringify({ id, method, params }));
  });
  async function js(expr) {
    const r = await send('Runtime.evaluate', { expression: `(async () => (${expr}))()`, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) throw new Error(`${expr.slice(0, 80)}: ${r.exceptionDetails.exception?.description ?? r.exceptionDetails.text}`);
    return r.result.value;
  }
  async function until(expr, what, ms = 10000) {
    for (let t = 0; t < ms; t += 100) {
      if (await js(expr).catch(() => false)) return true;
      await sleep(100);
    }
    // 144: a stale wasm or a refused config shows only in the page's error bar: say what it says.
    const shown = await js(`document.getElementById('error')?.textContent ?? ''`).catch(() => '');
    throw new Error(`timed out waiting for ${what}${shown ? ` (the page says: ${String(shown).trim().slice(0, 300)})` : ''}`);
  }
  async function size(w, h) {
    await send('Emulation.setDeviceMetricsOverride', { width: w, height: h, deviceScaleFactor: 1, mobile: false });
    await sleep(250);
  }
  /** A PNG of the viewport (or of `clip`), as a Buffer. */
  async function shot(clip) {
    const r = await send('Page.captureScreenshot', { format: 'png', ...(clip ? { clip: { ...clip, scale: 1 }, captureBeyondViewport: true } : {}) });
    return Buffer.from(r.data, 'base64');
  }
  /** Closes the browser and deletes its profile (020: killing only the parent left the
   *  children holding the profile, so every run leaked a 20–500 MB folder in %TEMP%). */
  async function close() {
    const exited = new Promise((r) => (browser.exitCode !== null ? r() : browser.once('exit', r)));
    await Promise.race([send('Browser.close').catch(() => {}), sleep(2000)]);
    ws.close();
    await Promise.race([exited, sleep(5000)]);
    if (browser.exitCode === null) browser.kill();
    try {
      rmSync(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 300 });
    } catch (e) {
      console.warn(`could not delete the browser profile ${profile}: ${e.message}`);
    }
  }
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Log.enable');
  return { send, js, until, size, shot, consoleErrors, close };
}
