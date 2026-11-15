// 242: plays a whole game in the player view through its own controls only, and checks that every legal action
// has a control there.
//
//   node scripts/player-play.mjs [--config three_seat.json] [--seed 1] [--out DIR] [--size 2560x1440] [--steps 4000]
//
// Needs a served viewer (AIC_URL). Each step it looks only at what the player view shows: the seat it focuses,
// the visible buttons carrying a legal action and, for a hex answer, the highlighted hexes on the map (clicked
// there). When the focused seat has nothing to do it clicks another seat's "to act" button. A move is picked at
// random, passing less often than anything else, so games run their length. Every step, with each seat to act
// focused in turn, it counts legal actions with no visible control (`no-control`; the sweep's check), then
// steps through the adversary's turn whenever one is open. At the end it copies the record through the Record
// dialog into OUT/record.json and writes OUT/summary.json and a few screenshots.
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { openPage, viewerUrl } from './cdp.mjs';

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const config = arg('config', 'three_seat.json');
const seed = arg('seed', '1');
const out = arg('out', join('..', 'out', 'player-play'));
const [width, height] = arg('size', '2560x1440').split('x').map(Number);
const maxSteps = Number(arg('steps', '4000'));
mkdirSync(out, { recursive: true });

const page = await openPage({ width, height });
let ok = false;
try {
await page.send('Page.navigate', { url: viewerUrl });
await page.until('window.aic && window.aic.view', 'the viewer to start', 30000);
await page.js(`window.aic.setView('player')`);
await page.js(`(() => { document.getElementById('config').value = '${config}'; document.getElementById('seed').value = '${seed}';
  document.getElementById('new-form').requestSubmit(); })()`);
await page.until(`window.aic.timeline.seed === '${seed}' && window.aic.timeline.last === 0`, 'the new game', 30000);

// In the page: the controls the player view shows for the focused seat, and the no-control count over each seat to act.
await page.js(`(() => {
  const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden'; };
  const controls = () => {
    const cs = [...document.querySelectorAll('#player [data-action]')].filter(vis);
    return { exact: new Set(cs.map((e) => e.dataset.action)), hexes: new Set(cs.flatMap((e) => (e.dataset.hexes ?? '').split(' ').filter(Boolean))) };
  };
  const isHex = (t) => Array.isArray(t) && t.length === 2;
  window.__pp = {
    noControl() {
      const missing = [];
      const seats = [...new Set(aic.legal.map((a) => a.seat))];
      for (const s of seats) {
        aic.showSeat(s);
        const c = controls();
        for (const a of aic.legal.filter((x) => x.seat === s))
          if (!c.exact.has(JSON.stringify(a)) && !(isHex(a.target) && c.hexes.has(a.target.join(',')))) missing.push(JSON.stringify(a));
      }
      aic.showSeat(null);
      return missing;
    },
    /** One move through the view; returns what it clicked. */
    move(switched = false) {
      const turn = document.querySelector('#p-turn [data-turn="1"]:not([disabled])');
      if (turn) { turn.click(); return 'turn step'; }
      const done = document.querySelector('#p-turn [data-turn="close"]');
      if (done) { done.click(); return 'turn done'; }
      const buttons = [...document.querySelectorAll('#player button[data-action]')].filter(vis).filter((b) => aic.legal.some((a) => JSON.stringify(a) === b.dataset.action));
      const picks = [...document.querySelectorAll('#player .hex-pick[data-hexes]')].filter(vis).flatMap((e) => e.dataset.hexes.split(' '));
      const opts = [...buttons.map((b) => ({ b, w: /a-pass|a-sell/.test(b.className) ? 1 : 6 })), ...picks.map((h) => ({ h, w: 6 }))];
      if (!opts.length) {
        // Show another seat to act and move there in the same call (the next step's count unpins it).
        const other = switched ? null : document.querySelector('#p-bar [data-seat-focus]');
        if (other) { other.click(); const m = this.move(true); return m && 'switch seat, ' + m; }
        return null;
      }
      let r = Math.random() * opts.reduce((n, o) => n + o.w, 0);
      const o = opts.find((x) => (r -= x.w) < 0) ?? opts[opts.length - 1];
      if (o.b) { const t = o.b.textContent; o.b.click(); return t; }
      const hex = document.querySelector('#board .hex[data-hex="' + o.h + '"]');
      hex.dispatchEvent(new MouseEvent('click', { bubbles: true }));
      return 'hex ' + o.h;
    },
  };
  return true;
})()`);

const shots = [];
const shot = async (name) => {
  await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  writeFileSync(join(out, `${name}.png`), await page.shot());
  shots.push(name);
};
const seen = new Set();
let noControl = 0;
const noControlSamples = [];
const clicks = {};
let turnSteps = 0;
for (let i = 0; i < maxSteps; i++) {
  const st = await page.js(`({ over: !!aic.view.outcome, phase: aic.view.phase && aic.view.phase.name, round: aic.view.round, turn: !!document.querySelector('#p-turn ol') })`);
  if (st.over) break;
  const missing = await page.js('window.__pp.noControl()');
  noControl += missing.length;
  if (missing.length && noControlSamples.length < 10) noControlSamples.push(...missing.slice(0, 3));
  // A screenshot at the first draft, keep, action phase and adversary's turn.
  const moment = st.turn ? 'adversary-turn' : st.phase ? st.phase.split('.').pop().split('+')[0] : null;
  if (moment && !seen.has(moment) && st.round >= 2) {
    seen.add(moment);
    await shot(`r${st.round}-${moment}`);
  }
  const what = await page.js('window.__pp.move()');
  if (what === null) throw new Error(`step ${i}: no control to click (round ${st.round}, ${st.phase})`);
  if (what === 'turn step' || what === 'turn done') turnSteps++;
  const kind = /^hex /.test(what) ? 'hex' : what.split(/[ (:]/)[0];
  clicks[kind] = (clicks[kind] ?? 0) + 1;
}
await shot('end');
// The record, through the Record dialog.
await page.js(`document.getElementById('record').click()`);
const record = await page.js(`document.getElementById('record-text').value`);
await page.js(`document.getElementById('record-close').click()`);
writeFileSync(join(out, 'record.json'), record.endsWith('\n') ? record : `${record}\n`);
const end = await page.js(`({ outcome: aic.view.outcome, actions: aic.timeline.last, seats: aic.view.seats.length, mode: aic.mode })`);
const summary = { config, seed, size: `${width}x${height}`, ...end, noControl, noControlSamples, turnSteps, clicks, consoleErrors: page.consoleErrors, shots };
writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
console.log(JSON.stringify(summary, null, 1));
ok = !!end.outcome && noControl === 0 && !page.consoleErrors.length;
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
