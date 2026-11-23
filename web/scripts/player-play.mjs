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
//
// 292: every step, and at the end, it also looks for the pieces cut from the game (`removed` in the summary; any
// fails the run): the list, and what 292, 293 and 294 each cut, is cut-pieces.mjs.
//
// 293: and for the adversary's spine (`spine` in the summary; any problem fails the run): the research row (its slot
// boxes, the reached ones, the cubes) and the zones' printed arrows match the engine's view every step, the next
// card says what the next Card beat escalates (320: the engine's escalate_ahead), and over the game the row moved.
//
// 294: the spine check counts the troops' strike arrows (solid only with a Strike shown ahead) and their guard links.
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { openPage, viewerUrl } from './cdp.mjs';
import { installCut } from './cut-pieces.mjs';

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

await installCut(page);
// In the page: the controls the player view shows for the focused seat, and the no-control count over each seat to act.
await page.js(`(() => {
  const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden'; };
  const controls = () => {
    const cs = [...document.querySelectorAll('#player [data-action]')].filter(vis);
    return { exact: new Set(cs.map((e) => e.dataset.action)), hexes: new Set(cs.flatMap((e) => (e.dataset.hexes ?? '').split(' ').filter(Boolean))) };
  };
  const isHex = (t) => Array.isArray(t) && t.length === 2;
  window.__pp = {
    /** 292: the cut pieces on screen now (cut-pieces.mjs). */
    removed: () => window.__cut.removed(),
    /** 293: the adversary's spine as the player view shows it now, against the engine's view: the research row, the
     *  zones' printed arrows on the map, last round's card, and what the next Card beat escalates. */
    spine() {
      const problems = [];
      // 320: the board the page draws (an earlier step's while the turn panel shows one: map, tracks and rows).
      const v = aic.mapView ?? aic.view;
      const g = (k) => v.globals.find((x) => x.key === k);
      const slot = g('adversary.research_slot'), cubes = g('adversary.research');
      const row = [...document.querySelectorAll('#player .research-row')].find(vis);
      if (!slot || !cubes) problems.push('the view has no research track');
      else if (!row) problems.push('no research row on screen');
      else {
        const boxes = [...row.querySelectorAll('.rslot')];
        if (boxes.length !== slot.max || !boxes.every(vis)) problems.push('research row: ' + boxes.length + ' slot boxes shown, the track has ' + slot.max);
        if (!/Loss/.test(boxes[boxes.length - 1]?.textContent ?? '')) problems.push('research row: its last box is not the loss');
        const reached = row.querySelectorAll('.rslot.reached').length;
        if (reached !== slot.value) problems.push('research row: ' + reached + ' slots marked reached, the marker is on ' + slot.value);
        if (boxes.some((b) => !b.classList.contains('loss') && !/: \\S/.test(b.title))) problems.push('research row: a slot with no icon text in its hover');
        const shown = row.querySelector('.rcubes')?.textContent.replace(/\\s+/g, ' ') ?? '';
        if (!shown.includes(cubes.value + ' / ' + cubes.max)) problems.push('research row: cubes read "' + shown + '", the view has ' + cubes.value + ' / ' + cubes.max);
      }
      const zones = v.arrows.filter((a) => a.kind === 'zone');
      const drawn = [...document.querySelectorAll('#board .arrows .arrow.zone')];
      if (drawn.length !== zones.length) problems.push(drawn.length + ' zone arrows drawn, the view has ' + zones.length);
      const live = zones.filter((a) => a.count > 0 || a.rounds === 1).length;
      const drawnLive = drawn.filter((e) => e.classList.contains('live')).length;
      if (drawnLive !== live) problems.push(drawnLive + ' zone arrows drawn live, the view has ' + live);
      if (drawn.some((e) => e.classList.contains('soon'))) problems.push('a zone arrow drawn as a troop about to strike');
      const troops = v.arrows.filter((a) => a.kind === 'troop').length;
      const drawnTroops = document.querySelectorAll('#board .arrows .arrow.troop').length;
      if (drawnTroops !== troops) problems.push(drawnTroops + ' troop arrows drawn, the view has ' + troops);
      // 294: each guard link (a troop guards a corrupted site) is drawn with its shield, and none as a strike.
      const guards = v.arrows.filter((a) => a.kind === 'guard').length;
      const drawnGuards = [...document.querySelectorAll('#board .arrows .arrow.guard')];
      if (drawnGuards.length !== guards) problems.push(drawnGuards.length + ' guard links drawn, the view has ' + guards);
      if (drawnGuards.some((e) => !e.querySelector('path.shield') || e.classList.contains('soon'))) problems.push('a guard link drawn with no shield, or as a strike');
      const soon = v.arrows.filter((a) => a.kind === 'troop' && a.rounds <= 1).length;
      const drawnSoon = document.querySelectorAll('#board .arrows .arrow.troop.soon').length;
      if (drawnSoon !== soon) problems.push(drawnSoon + ' strike arrows drawn solid, the view has ' + soon + ' with a Strike shown');
      const escalates = v.arrows.filter((a) => a.kind === 'escalate').length;
      const badges = [...document.querySelectorAll('#board .arrows .arrow.escalate')];
      if (badges.length !== escalates) problems.push(badges.length + ' escalate badges drawn, the view has ' + escalates);
      if (badges.some((e) => !e.querySelector('text') || e.querySelector('line, path'))) problems.push('an escalate badge drawn as a ring or an arrow');
      const last = v.piles.find((p) => p.key === 'adversary.last');
      const box = [...document.querySelectorAll('#player .slot-box.last')].find(vis);
      let escalating = null;
      if (!last) problems.push('the view has no last-round pile');
      else if (!box) problems.push('no last-round box on screen');
      else if (last.visible.length) {
        const card = last.visible[0];
        if (!box.textContent.includes(card.name)) problems.push('the last-round box does not name ' + card.name);
      }
      // 320 (D139): what the next Card beat escalates sits under the next card, from the engine's
      // adversary.escalate_ahead; nothing when no Card beat is ahead (the cap's last round).
      const ahead = v.globals.find((x) => x.key === 'adversary.escalate_ahead');
      const nextBox = [...document.querySelectorAll('#player .slot-box.next')].find(vis);
      if (ahead && nextBox && !(v.phase?.name === 'adversary.build' && v.choice)) {
        escalating = nextBox.querySelector('.escalates')?.dataset.escalating ?? null;
        if (ahead.value > 0 && escalating === null) problems.push('the next-card box does not say what escalates');
        else if (ahead.value > 0 && !ahead.doc.includes(escalating)) problems.push('escalating reads "' + escalating + '", the view says "' + ahead.doc + '"');
        else if (!ahead.value && escalating !== null) problems.push('an Escalate notice with none ahead');
      }
      return { problems, slot: slot?.value ?? 0, cubes: cubes?.value ?? 0, zones: drawn.length, live: drawnLive, badges: badges.length, escalating, rowInView: !!row && (() => { const r = row.getBoundingClientRect(); return row.contains(document.elementFromPoint(r.left + 4, r.bottom - 4)); })() };
    },
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
// 292: each cut piece seen, with the round it first showed in.
const removed = new Map();
const lookForRemoved = async (round) => {
  for (const r of await page.js('window.__pp.removed()')) if (!removed.has(r)) removed.set(r, round);
};
// 293: the adversary's spine over the game: each problem with the round it first showed in, the research marker's
// slots and cube counts seen, the zone arrows drawn (and the most drawn live), and what was shown escalating.
const spine = { problems: new Map(), slots: new Set(), cubes: new Set(), zones: 0, live: 0, badges: 0, escalating: new Set() };
// 293: one mid-game shot with the research row, a zone arrow and an escalate badge together.
let spineShot = null;
const lookAtSpine = async (round) => {
  const s = await page.js('window.__pp.spine()');
  for (const p of s.problems) if (!spine.problems.has(p)) spine.problems.set(p, round);
  spine.slots.add(s.slot);
  spine.cubes.add(s.cubes);
  spine.zones = Math.max(spine.zones, s.zones);
  spine.live = Math.max(spine.live, s.live);
  spine.badges = Math.max(spine.badges, s.badges);
  if (!spineShot && s.badges && s.zones && s.slot >= 2 && s.rowInView && round !== 'end') { spineShot = `r${round}-spine`; await shot(spineShot); }
  if (s.escalating !== null) spine.escalating.add(s.escalating);
};
for (let i = 0; i < maxSteps; i++) {
  const st = await page.js(`({ over: !!aic.view.outcome, phase: aic.view.phase && aic.view.phase.name, round: aic.view.round, turn: !!document.querySelector('#p-turn ol') })`);
  if (st.over) break;
  await lookAtSpine(st.round);
  const missing = await page.js('window.__pp.noControl()');
  noControl += missing.length;
  if (missing.length && noControlSamples.length < 10) noControlSamples.push(...missing.slice(0, 3));
  await lookForRemoved(st.round);
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
await lookForRemoved('end');
await lookAtSpine('end');
// 293: over a whole game the research track moved, the map showed its zones' arrows and a last-round card escalated.
const spineProblems = [...spine.problems].map(([what, round]) => `round ${round}: ${what}`);
if (spine.slots.size + spine.cubes.size < 3) spineProblems.push('the research row never moved');
if (!spine.zones) spineProblems.push('no zone arrow was ever drawn');
if (!spine.badges) spineProblems.push('no escalate badge was ever drawn');
if (!spine.escalating.size) spineProblems.push('no last-round card ever showed what escalates');
const spineSeen = { slots: [...spine.slots], cubes: [...spine.cubes], zoneArrows: spine.zones, liveZoneArrows: spine.live, escalateBadges: spine.badges, escalating: [...spine.escalating], problems: spineProblems };
// The record, through the Record dialog.
await page.js(`document.getElementById('record').click()`);
const record = await page.js(`document.getElementById('record-text').value`);
await page.js(`document.getElementById('record-close').click()`);
writeFileSync(join(out, 'record.json'), record.endsWith('\n') ? record : `${record}\n`);
const end = await page.js(`({ outcome: aic.view.outcome, actions: aic.timeline.last, seats: aic.view.seats.length, mode: aic.mode })`);
const removedSeen = [...removed].map(([what, round]) => `round ${round}: ${what}`);
const summary = { config, seed, size: `${width}x${height}`, ...end, noControl, noControlSamples, removed: removedSeen.length, removedSamples: removedSeen.slice(0, 40), spine: spineSeen, turnSteps, clicks, consoleErrors: page.consoleErrors, shots };
writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
console.log(JSON.stringify(summary, null, 1));
ok = !!end.outcome && noControl === 0 && removedSeen.length === 0 && spineProblems.length === 0 && !page.consoleErrors.length;
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
