// `npm run odd` (044): odd input the sweep doesn't try, one scenario at a time against a
// served viewer (AIC_URL): mashing keys and buttons, reloading mid-choice, resizing and
// switching tabs during autoplay, freezing the page, full storage, records and sessions
// that are old or broken. Fails on a console error, a stuck game, a page scroll or a
// scenario's own check. Screenshots of failures in web/sweep-out/odd/ (AIC_SWEEP_OUT).
//   node scripts/odd.mjs [--seeds N] [--only name]

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, sleep, viewerUrl as url } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const args = process.argv.slice(2);
const opt = (n) => args.flatMap((a, i) => (args[i - 1] === n ? [a] : []))[0];
const seeds = Number(opt('--seeds') ?? 2);
const only = opt('--only');
const out = join(resolve(process.env.AIC_SWEEP_OUT ?? join(web, 'sweep-out')), 'odd');
mkdirSync(out, { recursive: true });

const page = await openPage({ width: 1366, height: 768 });
const fails = [];
const passed = [];

const ready = async () => {
  await page.until('window.aic && window.aic.view && window.aic.play', 'the viewer to start', 30000);
};
const load = async () => {
  await page.send('Page.navigate', { url });
  await sleep(200);
  await ready();
};
const TABS = ['state', 'view', 'config', 'log'];

/** The page's state in one object, for a scenario's checks. */
const state = () => page.js(`({ error: window.aic.error ?? null, round: window.aic.view.round, over: !!window.aic.view.outcome,
  legal: window.aic.legal.length, cursor: window.aic.timeline.cursor, last: window.aic.timeline.last,
  choice: window.aic.view.choice ? window.aic.view.choice.prompt : null,
  scroll: document.documentElement.scrollHeight > innerHeight || document.documentElement.scrollWidth > innerWidth,
  bar: document.getElementById('error').hidden ? '' : document.getElementById('error').textContent,
  source: document.getElementById('source').textContent })`);

async function general(name) {
  const s = await state();
  const errs = page.consoleErrors.splice(0);
  for (const e of errs) fails.push(`${name}: console: ${e.slice(0, 240)}`);
  if (s.scroll) fails.push(`${name}: the page scrolls`);
  if (!s.over && !s.legal) fails.push(`${name}: no legal action and no outcome`);
  return s;
}

const scenario = async (name, f) => {
  if (only && only !== name) return;
  const before = fails.length;
  const t = Date.now();
  try {
    await load();
    await page.js('localStorage.clear()');
    await load();
    await f();
    await general(name);
  } catch (e) {
    fails.push(`${name}: threw: ${String(e).slice(0, 300)}`);
  }
  const ok = fails.length === before;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${name} (${((Date.now() - t) / 1000).toFixed(1)} s)`);
  if (ok) passed.push(name);
  else writeFileSync(join(out, `${name}.png`), await page.shot().catch(() => Buffer.alloc(0)));
};

const key = (k, extra = '') => `document.activeElement.dispatchEvent(new KeyboardEvent('keydown', { key: ${JSON.stringify(k)}, bubbles: true, cancelable: true ${extra} }))`;
/** Plays random actions until a choice is up, or `max` of them. */
const toChoice = (max = 400) => page.js(`(() => { for (let i = 0; i < ${max}; i++) { const v = window.aic.view; if (v.outcome) return false; if (v.choice && window.aic.legal.length > 1) return true; const l = window.aic.legal; const np = l.filter((a) => a.type !== 'pass'); const p = np.length ? np : l; window.aic.act(p[(i * 7919) % p.length]); } return false; })()`);
const record = () => page.js(`(document.getElementById('record').click(), new Promise((r) => setTimeout(() => { r(document.getElementById('record-text').value); }, 50)))`);

try {
  await scenario('mash-keys', async () => {
    const keys = ['ArrowLeft', 'ArrowRight', 'Home', 'End', 'PageUp', 'PageDown', ' ', 'r', 'Tab', 'Enter', 'Escape', 'z'];
    for (let s = 0; s < seeds; s++) {
      for (let i = 0; i < 400; i++) {
        const k = keys[(i * 2654435761 + s * 31) % keys.length];
        await page.js(`(${key(k, k === 'z' ? ', ctrlKey: true' : '')}, true)`);
      }
      // Space may have left autoplay running: stop it before checking.
      await page.js("(/pause/i.test(document.getElementById('play').textContent) && document.getElementById('play').click(), true)");
      await general('mash-keys');
    }
  });

  await scenario('mash-buttons', async () => {
    const ids = ['undo', 'random-action', 'first', 'back', 'fwd', 'end', 'play', 'random-seed'];
    for (let i = 0; i < 300; i++) await page.js(`(document.getElementById('${ids[(i * 5) % ids.length]}').click(), true)`);
    await sleep(100);
  });

  await scenario('random-end-twice', async () => {
    await page.js("(document.getElementById('random-end').click(), document.getElementById('random-end').click(), true)");
    await page.until('window.aic.view.outcome', 'the game to end', 60000);
    await page.js("(document.getElementById('undo').click(), document.getElementById('undo').click(), true)");
    const s = await state();
    if (s.over) fails.push('random-end-twice: undo did not reopen the game');
  });

  await scenario('reload-mid-choice', async () => {
    if (!(await toChoice())) return;
    const a = await state();
    await load();
    const b = await state();
    if (a.choice !== b.choice || a.cursor !== b.cursor) fails.push(`reload-mid-choice: ${a.cursor} "${a.choice}" came back as ${b.cursor} "${b.choice}"`);
  });

  await scenario('seek-back-then-act', async () => {
    await page.js("(() => { for (let i = 0; i < 60 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    const a = await state();
    await page.js(`window.aic.seek(${Math.floor(a.cursor / 2)})`);
    await page.js('(window.confirm = () => true, window.aic.act(window.aic.legal[window.aic.legal.length - 1]))');
    const b = await state();
    if (b.error) fails.push(`seek-back-then-act: ${b.error}`);
    if (b.last !== Math.floor(a.cursor / 2) + 1) fails.push(`seek-back-then-act: the timeline runs to ${b.last}, expected a branch at ${Math.floor(a.cursor / 2) + 1}`);
  });

  await scenario('resize-and-tabs-during-autoplay', async () => {
    await page.js("(() => { for (let i = 0; i < 200 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[(i * 31) % window.aic.legal.length]); })()");
    await page.js("window.aic.seek(0)");
    await page.js("(document.getElementById('speed').value = document.getElementById('speed').options[2].value, document.getElementById('speed').dispatchEvent(new Event('change')), document.getElementById('play').click(), true)");
    for (const [w, h] of [[800, 600], [1920, 1080], [1366, 768], [2560, 1440], [1366, 768]]) {
      await page.size(w, h);
      for (const b of TABS) await page.js(`(document.querySelector('#tabs [data-tab="${b}"]')?.click(), true)`);
      await sleep(150);
    }
    await page.js("(document.getElementById('play').click(), true)");
  });

  await scenario('freeze-mid-autoplay', async () => {
    await page.js("(() => { for (let i = 0; i < 150 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[(i * 31) % window.aic.legal.length]); })()");
    await page.js("window.aic.seek(0)");
    await page.js("(document.getElementById('play').click(), true)");
    await sleep(300);
    await page.send('Page.setWebLifecycleState', { state: 'frozen' });
    await sleep(600);
    await page.send('Page.setWebLifecycleState', { state: 'active' });
    await sleep(300);
    await page.js("(document.getElementById('play').click(), true)");
  });

  await scenario('full-storage', async () => {
    await page.js("(Storage.prototype.setItem = () => { throw new DOMException('full', 'QuotaExceededError'); }, true)");
    await page.js("(() => { for (let i = 0; i < 30 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    const s = await state();
    if (s.error) fails.push(`full-storage: ${s.error}`);
  });

  await scenario('broken-records', async () => {
    const good = await record();
    if (!good) throw new Error('no record text');
    await page.js("document.querySelector('dialog[open]')?.close()");
    const r = JSON.parse(good);
    const cases = {
      junk: 'not json at all',
      truncated: good.slice(0, Math.floor(good.length / 2)),
      'empty object': '{}',
      'no actions': JSON.stringify({ config: r.config, seed: r.seed }),
      'bad seed': JSON.stringify({ ...r, seed: 'abc' }),
      'illegal action': JSON.stringify({ ...r, actions: [{ type: 'pass', seat: 9 }] }),
      'old revision': JSON.stringify({ ...r, rules_revision: 1 }),
    };
    for (const [name, text] of Object.entries(cases)) {
      // Through the dialog, as a user pastes one (the `open` hook has no error guard).
      await page.js(`(() => { document.getElementById('record').click(); document.getElementById('record-text').value = ${JSON.stringify(text)}; document.getElementById('record-load').click(); return true; })()`);
      const msg = await page.js("document.getElementById('record-msg').textContent");
      await page.js("document.querySelector('dialog[open]')?.close()");
      const s = await general(`broken-records ${name}`);
      if (name !== 'no actions' && name !== 'old revision' && !s.error && !s.bar && !msg) fails.push(`broken-records: ${name} loaded with no word`);
      console.log(`     ${name}: ${(msg || s.bar || s.error || s.source).slice(0, 120)}`);
    }
  });

  await scenario('old-session', async () => {
    await page.js("(() => { for (let i = 0; i < 20 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    const saved = await page.js("localStorage.getItem('aic.session')");
    if (!saved) throw new Error('nothing saved');
    const j = JSON.parse(saved);
    const c = JSON.parse(j.config);
    c.modules[0].params.no_such_param = 3;
    j.config = JSON.stringify(c);
    await page.js(`(localStorage.setItem('aic.session', ${JSON.stringify(JSON.stringify(j))}), true)`);
    await load();
    await sleep(100);
    const s = await state();
    console.log(`     ${(s.bar || s.source).slice(0, 140)}`);
    if (!s.bar && !/older|revision/.test(s.source)) fails.push('old-session: a session that no longer loads said nothing');
  });

  await scenario('junk-session', async () => {
    await page.js("(localStorage.setItem('aic.session', '{nonsense'), true)");
    await load();
  });

  await scenario('bad-seed-field', async () => {
    for (const v of ['', 'abc', '-1', '1.5', '99999999999999999999999', '0x10']) {
      await page.js(`(document.getElementById('seed').value = ${JSON.stringify(v)}, document.getElementById('new-form').requestSubmit(), true)`);
      await sleep(30);
      const s = await general(`bad-seed-field ${v}`);
      console.log(`     seed "${v}": ${(s.bar || s.source).slice(0, 100)}`);
    }
  });
} finally {
  await page.close();
}

if (only && !passed.length && !fails.length) fails.push(`--only ${only}: no such scenario`);
console.log(`\n${passed.length} scenarios passed, ${fails.length} failures`);
for (const f of fails) console.log(`FAIL ${f}`);
process.exit(fails.length ? 1 : 0);
