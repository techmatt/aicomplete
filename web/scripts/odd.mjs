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
    // 118: undo at an earlier step asks first, as acting there does: say yes.
    await page.js('(window.confirm = () => true, true)');
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

  // 261: the player view: a record opened there plays from its last step (it showed setup, where every control
  // dropped the record); Space after a mouse click on a header button does nothing (it pressed the button again);
  // pinning another seat while one answers a question keeps that question's answers in the prompt bar.
  await scenario('player-view-record-space-pin', async () => {
    await page.js("(() => { for (let i = 0; i < 40 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[(i * 7) % window.aic.legal.length]); })()");
    const text = await record();
    await page.js("(document.getElementById('record-close').click(), document.getElementById('view-toggle').click(), true)");
    await page.js(`(window.aic.open(${JSON.stringify(text)}, 'odd'), true)`);
    const s = await state();
    if (s.cursor !== s.last) fails.push(`player-view-record-space-pin: a record opened in the player view shows step ${s.cursor} of ${s.last}`);
    // 325: the player view's header button is New game… (the header's form is the debug view's).
    const kept = await page.js(`(() => { const b = document.getElementById('new-open'); b.focus();
      b.dispatchEvent(new PointerEvent('pointerdown', { bubbles: true })); return ${key(' ')}; })()`);
    if (kept) fails.push('player-view-record-space-pin: Space after clicking a header button was left to the button');
    if (!(await toChoice())) return void fails.push('player-view-record-space-pin: no question came up to pin past');
    const answers = () => page.js("document.querySelectorAll('#prompt [data-action]').length");
    const before = await answers();
    if (!before) return void fails.push('player-view-record-space-pin: the open question shows no answers');
    const clicked = await page.js(`(() => { const v = window.aic.view; const other = v.seats.find((x) => x.index !== v.choice.seat);
      const el = other && document.querySelector('[data-seat-focus="' + other.index + '"]'); if (el) el.click(); return !!el; })()`);
    if (!clicked) return void fails.push('player-view-record-space-pin: no other seat to pin');
    const after = await answers();
    if (!after) fails.push(`player-view-record-space-pin: pinning another seat emptied the prompt's ${before} answers`);
  });

  // 265 (D116): a record that stops replaying is saved whole, so a reload shows the same stop, not the game before.
  await scenario('stopped-record-reload', async () => {
    await page.js("(() => { for (let i = 0; i < 12 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    const text = await record();
    await page.js("document.querySelector('dialog[open]')?.close()");
    const r = JSON.parse(text);
    const stopAt = Math.min(5, r.actions.length);
    r.actions = [...r.actions.slice(0, stopAt), { type: 'pass', seat: 9 }, ...r.actions.slice(stopAt)];
    await page.js(`(window.aic.open(${JSON.stringify(JSON.stringify(r))}, 'stopped.json'), true)`);
    const a = await state();
    if (!a.bar.includes('rejected')) return void fails.push(`stopped-record-reload: the stop said nothing: ${a.bar.slice(0, 120)}`);
    await load();
    await sleep(100);
    const b = await state();
    console.log(`     after reload: ${(b.bar || b.source).slice(0, 140)}`);
    if (!b.bar.includes(`action ${stopAt} `)) fails.push(`stopped-record-reload: a reload lost the stop (bar: ${b.bar.slice(0, 120)})`);
    if (!b.source.includes('stopped.json')) fails.push(`stopped-record-reload: a reload shows another game (${b.source.slice(0, 80)})`);
    if (b.last !== a.last) fails.push(`stopped-record-reload: ${a.last} steps replayed, ${b.last} after a reload`);
  });

  // 265 (D109, D112, D115): the player view's bar names standard actions (name and cost, the rule in the hover), the
  // legend names a threat's dotted outline, the turn panel shows the rules' notes only under "Since your last move";
  // the debug step bar names a power and its cost, its rule in the hover.
  await scenario('player-labels-legend-turn', async () => {
    await page.js("(document.getElementById('view-toggle').click(), true)");
    const RAW = /resource changed|card moved|cards moved|tokens changed|var changed| · seat \d| · delta |seat \d [+-]\d/;
    let bar = false, turn = false, power = false, conv = false;
    const cubedSeen = new Set();
    for (let i = 0; i < 1500 && !(bar && turn && power && conv && cubedSeen.size); i++) {
      const r = await page.js(`(() => {
        const out = {};
        const std = [...document.querySelectorAll('#p-act .a-standard')];
        if (std.length) out.std = std.map((b) => [b.textContent, b.title]);
        // The cube-priced ones (Lift Fallout, Turn a cube into insight, Make antimatter) name their cubes.
        const cubed = std.filter((b) => ['lift', 'insight', 'antimatter'].includes(JSON.parse(b.dataset.action).act));
        if (cubed.length) out.cubed = cubed.map((b) => b.textContent);
        const t = document.getElementById('p-turn');
        if (!t.hidden && t.querySelector('.pturn-head')) out.turn = [t.querySelector('.pturn-head b').textContent, t.querySelector('.pturn-ev').textContent + ' ' + t.querySelector('ol').textContent];
        else if (!t.hidden && t.querySelector('[data-turn=open]')) t.querySelector('[data-turn=open]').click();
        const conv = [...document.querySelectorAll('#p-board-body .converter.unusable')].map((c) => c.textContent);
        if (conv.length) out.conv = conv;
        const f = window.aic.timeline.frame.action;
        if (f && f.type === 'power') out.power = [document.getElementById('step').textContent, document.getElementById('step').title];
        const v = window.aic.view;
        if (v.outcome) return { over: true, ...out };
        const l = window.aic.legal; const np = l.filter((a) => a.type !== 'pass' || Math.random() < 0.2);
        const p = np.length ? np : l;
        window.aic.act(p[Math.floor(Math.random() * p.length)]);
        return out;
      })()`);
      if (r.std && !bar) {
        bar = true;
        for (const [text, title] of r.std) {
          if (/Target:|;/.test(text)) fails.push(`player-labels-legend-turn: a standard button reads its rule: ${text}`);
          if (!title || title.length <= text.length) fails.push(`player-labels-legend-turn: "${text}" has no rule in its hover`);
        }
        console.log(`     bar: ${r.std.map(([t]) => t).join(' | ').slice(0, 200)}`);
      }
      if (r.turn && !turn) {
        turn = true;
        if (r.turn[0] !== 'Since your last move') fails.push(`player-labels-legend-turn: the turn panel is headed "${r.turn[0]}"`);
        if (RAW.test(r.turn[1])) fails.push(`player-labels-legend-turn: the turn panel lists a raw event: ${r.turn[1].slice(0, 160)}`);
      }
      if (r.power && !power) {
        power = true;
        // Read as the power's step was shown (the step bar renders in either view), before the next action.
        const step = r.power;
        console.log(`     step bar: ${step[0].slice(0, 120)}`);
        if (/\): .|: place|: gain/.test(step[0].replace(/^step [^·]*· seat \d+: power: /, ''))) fails.push(`player-labels-legend-turn: the step bar prints the power's rule: ${step[0].slice(0, 160)}`);
        if (step[1].length <= step[0].length) fails.push('player-labels-legend-turn: the step bar\'s hover has no rule');
      }
      for (const c of r.cubed ?? []) {
        cubedSeen.add(c);
        if (!/research cube/.test(c)) fails.push(`player-labels-legend-turn: a cube-priced standard action shows no price: ${c}`);
      }
      if (r.conv && !conv) {
        conv = true;
        for (const c of r.conv) if (!c.includes('→')) fails.push(`player-labels-legend-turn: an unusable converter doesn't say what it trades: ${c}`);
      }
      if (r.over) break;
    }
    if (!conv) console.log('     (no unusable converter came up: unchecked)');
    console.log(`     cube-priced: ${[...cubedSeen].join(' | ') || '(none came up)'}`);
    if (!bar) fails.push('player-labels-legend-turn: no standard action came up');
    if (!turn) fails.push('player-labels-legend-turn: no turn panel came up');
    if (!power) console.log('     (no power used in this game: step bar unchecked)');
    const legend = await page.js("document.getElementById('legend').textContent");
    if (!/threat's source/.test(legend)) fails.push('player-labels-legend-turn: the legend has no threat source outline');
  });

  // 285: keys did nothing behind the open record dialog only while the focus was inside it: a click on its text
  // left the focus on the body, and R, Space, the arrows, V and Ctrl+Z moved the game unseen behind it.
  await scenario('keys-behind-record-dialog', async () => {
    await page.js("(() => { for (let i = 0; i < 12 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    await page.js('(window.confirm = () => true, window.aic.seek(6), true)');
    const look = () => page.js(`({ cursor: window.aic.timeline.cursor, last: window.aic.timeline.last,
      mode: document.getElementById('view-toggle').textContent, play: document.getElementById('play').textContent })`);
    await page.js("(document.getElementById('record').click(), true)");
    const a = await look();
    await page.js("(document.querySelector('#record-dialog h2, #record-dialog h3, #record-dialog p')?.click(), document.activeElement?.blur(), true)");
    const focus = await page.js("document.activeElement === document.body && document.getElementById('record-dialog').open");
    if (!focus) return void fails.push('keys-behind-record-dialog: could not put the focus on the body under the open dialog');
    for (const [k, extra] of [['r', ''], [' ', ''], ['ArrowRight', ''], ['v', ''], ['z', ', ctrlKey: true']]) {
      await page.js(`(${key(k, extra)}, true)`);
      const b = await look();
      if (JSON.stringify(a) !== JSON.stringify(b)) fails.push(`keys-behind-record-dialog: "${k}" behind the dialog moved ${JSON.stringify(a)} to ${JSON.stringify(b)}`);
    }
    await page.js("(/pause/i.test(document.getElementById('play').textContent) && document.getElementById('play').click(), document.querySelector('dialog[open]')?.close(), true)");
  });

  // 285: a record's load warning stayed in the error bar after Undo made it a played game (acting cleared it).
  await scenario('undo-clears-record-warning', async () => {
    await page.js("(() => { for (let i = 0; i < 12 && !window.aic.view.outcome; i++) window.aic.act(window.aic.legal[0]); })()");
    const r = JSON.parse(await record());
    await page.js("document.querySelector('dialog[open]')?.close()");
    // Another revision's stamp: the record replays (same build) but loads with a warning.
    r.rules_revision -= 1;
    await page.js(`(window.aic.open(${JSON.stringify(JSON.stringify(r))}, 'odd-old.json'), true)`);
    const a = await state();
    if (!a.bar) return void fails.push('undo-clears-record-warning: an older revision\'s record loaded with no warning');
    await page.js("(window.confirm = () => true, document.getElementById('undo').click(), true)");
    const b = await state();
    if (b.last >= a.last) return void fails.push(`undo-clears-record-warning: undo dropped nothing (${a.last} → ${b.last})`);
    if (b.bar) fails.push(`undo-clears-record-warning: the warning stayed after undo: ${b.bar.slice(0, 120)}`);
  });

  // 285: the player view's "Since your last move" names the adversary's lines (no Infiltrate note before), and a
  // step that moved pieces (a -1 here, +1 there nets to 0) read "bookkeeping only (tracks)".
  await scenario('player-view-turn-panel-names-lines', async () => {
    await page.js("(localStorage.setItem('aic.startView', 'player'), true)");
    await load();
    let infiltrate = '', panels = 0, netZero = 0;
    for (let i = 0; i < 1500; i++) {
      const r = await page.js(`(() => {
        const out = {};
        const t = document.getElementById('p-turn');
        if (!t.hidden && t.querySelector('.pturn-head')) {
          // turnOf's steps: the frame's events split at each phase mark, rounds and empty phases dropped.
          const gs = [{ round: false, ev: [] }];
          for (const e of window.aic.timeline.frame.events) {
            if (e.type === 'phase_started') gs.push({ round: false, ev: [] });
            else if (e.type === 'round_started') gs.push({ round: true, ev: [] });
            else gs[gs.length - 1].ev.push(e);
          }
          const steps = gs.filter((g) => !g.round && g.ev.length).map((g) => g.ev);
          const rows = [...t.querySelectorAll('li[data-turn-step]')].map((li) => li.textContent);
          out.panel = { rows, steps: steps.map((ev) => {
            const net = new Map();
            for (const e of ev) if (e.type === 'tokens_changed') net.set(e.token, (net.get(e.token) ?? 0) + Number(e.delta));
            return { tokens: ev.some((e) => e.type === 'tokens_changed'), notes: ev.some((e) => e.type === 'note'), zero: net.size > 0 && [...net.values()].every((d) => !d) };
          }) };
        }
        const v = window.aic.view;
        if (v.outcome || v.round > 2) return { done: true, ...out };
        const l = window.aic.legal; const np = l.filter((a) => a.type !== 'pass' || Math.random() < 0.2);
        const p = np.length ? np : l;
        window.aic.act(p[Math.floor(Math.random() * p.length)]);
        return out;
      })()`);
      if (r.panel) {
        panels++;
        const { rows, steps } = r.panel;
        if (rows.length !== steps.length) fails.push(`player-view-turn-panel-names-lines: ${rows.length} rows for ${steps.length} steps`);
        rows.forEach((row, j) => {
          if (!infiltrate && row.includes('Infiltrate (')) infiltrate = row;
          const s = steps[j];
          if (s?.tokens && !s.notes && s.zero) netZero++;
          if (s?.tokens && /bookkeeping only/.test(row)) fails.push(`player-view-turn-panel-names-lines: a step that moved pieces reads ${row.slice(0, 120)}`);
        });
      }
      if (r.done) break;
    }
    console.log(`     ${panels} panels; ${netZero} note-less net-zero piece steps; ${infiltrate.slice(0, 120) || '(no Infiltrate line)'}`);
    if (!panels) fails.push('player-view-turn-panel-names-lines: no turn panel came up by round 2');
    else if (!infiltrate) fails.push('player-view-turn-panel-names-lines: no step names an "Infiltrate (" line');
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
