// 163's run C: a headless Edge (or Chrome) tab on conductor's worker page, as an extra remote worker on the
// main box, its core slider preset. Writes the browser's pid to PIDFILE and runs until the browser exits (a
// watcher kills it mid-job with `taskkill /F /T /PID`, the tab lost). Logs the page's event log every 20 s.
//   node tab_worker.mjs URL CORES PIDFILE PROFILE_DIR
import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [url, cores, pidFile, profile] = process.argv.slice(2);
if (!profile) throw new Error('usage: node tab_worker.mjs URL CORES PIDFILE PROFILE_DIR');
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const exe = [process.env.AIC_BROWSER, 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe',
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe'].filter(Boolean).find((p) => existsSync(p));
if (!exe) throw new Error('no browser found; set AIC_BROWSER');
mkdirSync(profile, { recursive: true });
const browser = spawn(exe, ['--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
  '--no-first-run', '--no-default-browser-check', 'about:blank'], { stdio: 'ignore' });
writeFileSync(pidFile, String(browser.pid), { encoding: 'utf-8' });
const stamp = () => new Date().toLocaleTimeString('en-US');
console.log(`${stamp()} browser ${exe} pid ${browser.pid}`);
browser.on('exit', (code) => { console.log(`${stamp()} browser exited (${code})`); process.exit(0); });

let port;
for (let i = 0; i < 100 && !port; i++) {
  try {
    const f = join(profile, 'DevToolsActivePort');
    if (existsSync(f)) port = readFileSync(f, 'utf8').split('\n')[0].trim();
  } catch (e) { if (e.code !== 'EBUSY' && e.code !== 'EPERM') throw e; }
  if (!port) await sleep(100);
}
if (!port) throw new Error('browser did not open a DevTools port');
const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
const ws = new WebSocket(pages.find((p) => p.type === 'page').webSocketDebuggerUrl);
await new Promise((r, j) => { ws.onopen = r; ws.onerror = j; });
let nextId = 1;
const waiting = new Map();
ws.onmessage = (m) => {
  const msg = JSON.parse(m.data);
  if (msg.id && waiting.has(msg.id)) { waiting.get(msg.id)(msg); waiting.delete(msg.id); }
};
ws.onclose = () => {};
const send = (method, params = {}) => new Promise((r) => {
  const id = nextId++;
  waiting.set(id, (msg) => r(msg.result ?? msg.error));
  ws.send(JSON.stringify({ id, method, params }));
});
// The page reads its core count from localStorage `prefs` (default all but one): preset it before any script runs.
await send('Page.addScriptToEvaluateOnNewDocument', {
  source: `try { localStorage.setItem('prefs', JSON.stringify({ cores: ${Number(cores)}, paused: false, wake: false })); } catch {}`,
});
await send('Page.enable');
await send('Page.navigate', { url });
console.log(`${stamp()} opened ${url} at ${cores} cores`);
// The page's worker id (a top-level const), beside the pid: the watcher kills the tab once it holds a shard.
for (let i = 0; i < 50; i++) {
  await sleep(200);
  const r = await send('Runtime.evaluate', { expression: 'typeof wid === "string" ? wid : ""', returnByValue: true });
  if (r?.result?.value) {
    writeFileSync(`${pidFile}.wid`, r.result.value, { encoding: 'utf-8' });
    console.log(`${stamp()} worker id ${r.result.value}`);
    break;
  }
}
let seen = null;
for (;;) {
  await sleep(20000);
  // The page's `events` (a top-level const of its classic script): the newest last, the oldest dropped past 300.
  const r = await send('Runtime.evaluate', {
    expression: `events.map((e) => e.at + ' ' + e.text)`, returnByValue: true,
  }).catch(() => null);
  const lines = r?.result?.value ?? [];
  const last = lines.findIndex((l) => l === seen);
  for (const l of lines.slice(last + 1)) console.log(`${stamp()} page: ${l}`);
  if (lines.length) seen = lines[lines.length - 1];
}
