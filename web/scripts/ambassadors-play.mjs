// 295: the ambassadors and node powers demonstration. Plays one whole game in the player view, by its own controls,
// to the "no corrupted city" win, and fails unless it saw on the way:
//   (a) an ambassador moved onto a corrupted city in one round, and that seat's Liberate of that city in the next,
//   (b) two node powers of one board used in one round, one of them a costly one: its cost paid, its node placed and
//       its benefit gained, and both shown used in the seat's panel,
//   (c) a tipped ambassador drawn on the map after a move, its hover saying so,
//   (d) no cut piece on any step (cut-pieces.mjs: "Dispatch", the card Dispatch Control Room aside, and "Growth"
//       among them), and the Move action offered as "Move an ambassador".
//
//   node scripts/ambassadors-play.mjs [--seed 7] [--out DIR] [--size 2560x1440] [--override JSON] [--any-end] [--search 1-40]
//
// Needs a served viewer (AIC_URL). Writes OUT/summary.json (what was seen and in which round, the moves, any
// failure), OUT/record.json and screenshots, and ends with a PASS or FAIL line; exit 1 on any failure. The game is
// the baseline with `OVERRIDE` below (`--override` replaces it; the summary records it). The moves come from
// play-policy.mjs's policy; every one is made by clicking the control the player view shows for it, and the
// adversary's turn is stepped through whenever it opens. `--search A-B` plays seeds A to B by the same policy
// straight through the engine, no clicks and no checks, and prints each one's end and the round (a) and (b) first
// held (`moveThenLiberate`, `twoNodePowers`): how the demo's seed was found. `--any-end` takes a whole game that
// ends another way than the win (the plain baseline, `--override {}`: the small policy leaves the adversary's
// research row alone and lost every one of seeds 1-40 to it), with (a) to (d) still required.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, viewerUrl } from './cdp.mjs';
import { installCut } from './cut-pieces.mjs';
import { POLICY } from './play-policy.mjs';

/** The demo's setting, over configs/baseline.json: no further city corrupts (`corrupt_at` 20), so the cities
 *  corrupted at setup are the game; the adversary's research clock, its agents and its troops run as in the baseline.
 *  The small policy buys its ambassadors one at a time: on the plain baseline, at r142's first lever values, it won
 *  none of seeds 1-16 (cities corrupt and re-corrupt faster than it liberates), and 4 of seeds 1-14 with this (7, 11,
 *  12, 13). */
const OVERRIDE = { adversary: { corrupt_at: 20 } };

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const seed = arg('seed', '7');
const out = arg('out', join(web, '..', 'out', 'ambassadors-play'));
const [width, height] = arg('size', '2560x1440').split('x').map(Number);
const maxSteps = Number(arg('steps', '4000'));
const override = JSON.parse(arg('override', JSON.stringify(OVERRIDE)));
const search = arg('search', '');
const anyEnd = process.argv.includes('--any-end');

const config = JSON.parse(readFileSync(join(web, '..', 'configs', 'baseline.json'), 'utf8'));
for (const [name, params] of Object.entries(override)) {
  const m = config.modules.find((x) => x.module === name);
  if (!m) throw new Error(`--override names no module of the baseline: ${name}`);
  Object.assign(m.params, params);
}

/** What a node power's use changed, read off the seat before and after: null when all of it shows, else what is missing. */
function powerProblem(p, before, after) {
  const missing = [];
  const gained = (r) => (after.res[r] ?? 0) - (before.res[r] ?? 0);
  if (after.nodes !== before.nodes + 1) missing.push(`nodes went ${before.nodes} to ${after.nodes}`);
  if (p.pay) {
    const r = [p.pay[2], `${p.pay[2]}s`].find((k) => k in before.res) ?? p.pay[2];
    if (-gained(r) < Number(p.pay[1])) missing.push(`paid ${-gained(r)} ${r} of ${p.pay[1]}`);
  }
  const credits = /gain (\d+) credits?/.exec(p.text);
  if (credits && gained('credits') < Number(credits[1])) missing.push(`gained ${gained('credits')} credits of ${credits[1]}`);
  if (/gain 1 ambassador/.test(p.text) && gained('ambassadors') < 1) missing.push('gained no ambassador');
  if (/research station/.test(p.text) && after.stations !== before.stations + 1) missing.push('built no station');
  const now = after.powers.find((x) => x.name === p.name);
  if (!now?.status.startsWith('used')) missing.push(`its status is "${now?.status}"`);
  const li = after.shown?.find((x) => x.name === p.name);
  if (!li?.used || li.use) missing.push(`its panel row shows ${li ? `"${li.status}"${li.use ? ' with a Use button' : ''}` : 'nothing'}`);
  return missing.length ? missing.join(', ') : null;
}

const page = await openPage({ width, height });
let ok = false;
try {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await page.js(`window.aic.setView('player')`);
  const start = async (s) => {
    await page.js(`window.aic.play(${JSON.stringify(JSON.stringify(config))}, '${s}', 'ambassadors-play')`);
    await page.until(`window.aic.timeline.seed === '${s}' && window.aic.timeline.last === 0`, `the seed-${s} game`, 30000);
    await page.js(POLICY);
  };

  if (search) {
    const [a, b] = search.split('-').map(Number);
    for (let s = a; s <= (b ?? a); s++) {
      await start(String(s));
      const r = await page.js(`window.__tp.fast(${maxSteps})`);
      console.log(`seed ${s}: ${r.end} (${r.actions} actions); first seen in round: ${JSON.stringify(r.seen)}`);
    }
    ok = true;
  } else {
    mkdirSync(out, { recursive: true });
    await start(seed);
    await installCut(page);
    const shots = [];
    const park = () => page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
    const shot = async (name) => {
      writeFileSync(join(out, `${name}.png`), await page.shot());
      shots.push(`${name}.png`);
    };
    const showSeat = (s) => page.js(`(() => { if (aic.focus !== ${s}) [...document.querySelectorAll('#player [data-seat-focus="${s}"]')].find((e) => e.getBoundingClientRect().width > 0)?.click(); return aic.focus; })()`);
    const removed = new Map();
    const lookForRemoved = async (round) => {
      for (const r of await page.js('window.__cut.removed()')) if (!removed.has(r)) removed.set(r, round);
    };
    const seen = { moveThenLiberate: null, nodePowers: null, tipped: null, moveButton: null };
    /** Each seat's last move onto a corrupted city, by `seat:city`; every node power use, by `seat:round`. */
    const movedOn = new Map();
    const uses = new Map();
    const problems = [];
    const clicks = {};
    const moves = [];
    /** The action whose effect is read once its questions are answered. */
    let pending = null;
    const settle = async () => {
      const p = pending;
      pending = null;
      if (p.kind === 'power') {
        await showSeat(p.seat);
        const after = await page.js(`window.__tp.seat(${p.seat})`);
        const problem = powerProblem(p.power, p.before, after);
        const key = `${p.seat}:${p.round}`;
        const had = (x) => ({ ...x.res, nodes: x.nodes });
        uses.set(key, [...(uses.get(key) ?? []), { name: p.power.name, costly: !!p.power.pay, problem, text: p.power.text, before: had(p.before), after: had(after) }]);
        if (problem) problems.push(`round ${p.round}: seat ${p.seat}'s ${p.power.name}: ${problem}`);
        const round = uses.get(key);
        if (!seen.nodePowers && round.length >= 2 && round.some((u) => u.costly) && round.every((u) => !u.problem)) {
          seen.nodePowers = { round: p.round, seat: p.seat, board: await page.js(`aic.view.seats[${p.seat}].board.name`), powers: round.map((u) => `${u.name}${u.costly ? ' (costly)' : ''}`),
            // The costly one's price, node and benefit, as the seat's panel had them before and after it.
            costly: round.filter((u) => u.costly).map((u) => ({ power: `${u.name}: ${u.text}`, before: u.before, after: u.after })),
            shown: after.shown.filter((x) => round.some((u) => u.name === x.name)).map((x) => `${x.name}: ${x.status}`) };
          await park();
          await shot(`b-node-powers-r${p.round}`);
        }
      } else if (p.kind === 'move') {
        const a = await page.js(`window.__tp.ambassadors(${p.seat}, ${p.city})`);
        if (!(a.on > 0 && a.tipped > 0 && a.corrupted && a.kind === 'city')) return problems.push(`round ${p.round}: seat ${p.seat} moved onto ${a.hex}, which now holds ${a.on} of its ambassadors, ${a.tipped} tipped`);
        movedOn.set(`${p.seat}:${p.city}`, { round: p.round, city: a.hex });
        if (!a.tippedDrawn || a.fallback) problems.push(`round ${p.round}: the tipped ambassador on ${a.hex} is not drawn laid over`);
        else if (!seen.tipped) {
          // The tipped stack's hover, as a player reads it.
          await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: a.x, y: a.y });
          await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: a.x + 1, y: a.y });
          const tip = await page.js(`(() => { const t = document.getElementById('tip'); return t && !t.hidden ? t.innerText : ''; })()`);
          seen.tipped = { round: p.round, seat: p.seat, hex: a.hex, hover: tip.split('\n')[0] };
          await lookForRemoved(p.round);
          await shot(`c-tipped-r${p.round}`);
          await park();
        }
      } else if (p.kind === 'liberate') {
        const a = await page.js(`window.__tp.ambassadors(${p.seat}, ${p.city})`);
        const moved = movedOn.get(`${p.seat}:${p.city}`);
        if (a.corrupted) return problems.push(`round ${p.round}: seat ${p.seat}'s Liberate left ${a.hex} corrupted`);
        if (!seen.moveThenLiberate && moved?.round === p.round - 1) {
          seen.moveThenLiberate = { seat: p.seat, city: a.hex, movedOnInRound: moved.round, liberatedInRound: p.round };
          await park();
          await shot(`a-liberated-r${p.round}`);
        }
      }
    };
    for (let i = 0; i < maxSteps; i++) {
      const st = await page.js('window.__tp.state()');
      if (pending && (!st.choice || st.over)) await settle();
      if (st.over) break;
      await lookForRemoved(st.round);
      // The adversary's turn, stepped through as a player reads it.
      const turn = await page.js('window.__tp.turn()');
      if (turn) { clicks[turn] = (clicks[turn] ?? 0) + 1; continue; }
      const d = await page.js('(() => { const d = window.__tp.decide(); return d && { ...d, intent: window.__tp.intent }; })()');
      if (!d) throw new Error(`step ${i}: no legal action and no outcome (round ${st.round}, ${st.phase})`);
      const s = d.action.seat;
      if (d.why === 'node power') {
        await showSeat(s);
        const before = await page.js(`window.__tp.seat(${s})`);
        pending = { kind: 'power', seat: s, round: st.round, before, power: before.powers.find((p) => p.name === d.power) };
      } else if (d.why === 'move onto the city') pending = { kind: 'move', seat: s, round: st.round, city: d.intent.city };
      else if (d.why === 'liberate') pending = { kind: 'liberate', seat: s, round: st.round, city: d.intent.city };
      const what = await page.js(`window.__tp.click(${JSON.stringify(d.action)})`);
      if (what === null) throw new Error(`step ${i}: the player view shows no control for ${JSON.stringify(d.action)} (round ${st.round}, ${st.phase})`);
      if (d.action.type === 'standard' && d.action.act === 'move' && !seen.moveButton) seen.moveButton = what.replace(/^seat \d+, /, '');
      clicks[d.why] = (clicks[d.why] ?? 0) + 1;
      if (!['pass', 'draft'].includes(d.why)) moves.push(`round ${st.round}: seat ${s} ${d.why}: ${what.replace(/\s+/g, ' ').trim()}`);
    }
    await lookForRemoved('end');
    await park();
    await shot('end');
    const end = await page.js(`(() => {
      const o = aic.view.outcome;
      const e = document.querySelector('#phase-bar .outcome');
      return { outcome: o, shown: e ? e.textContent : null, shownAsWin: !!e && e.classList.contains('won'), actions: aic.timeline.last, mode: aic.mode,
        revision: document.querySelector('header')?.innerText.replace(/\\s+/g, ' ').slice(0, 160) ?? '' };
    })()`);
    await page.js(`document.getElementById('record').click()`);
    const record = await page.js(`document.getElementById('record-text').value`);
    await page.js(`document.getElementById('record-close').click()`);
    writeFileSync(join(out, 'record.json'), record.endsWith('\n') ? record : `${record}\n`);

    const removedSeen = [...removed].map(([what, round]) => `round ${round}: ${what}`);
    const failures = [...problems];
    if (!seen.moveThenLiberate) failures.push(`(a) no ambassador moved onto a corrupted city one round and liberated it the next (moves onto a city: ${[...movedOn.values()].map((m) => `${m.city} in round ${m.round}`).join(', ') || 'none'})`);
    if (!seen.nodePowers) failures.push('(b) no seat used two node powers in one round, one of them a costly one, with everything they do showing');
    if (!seen.tipped) failures.push('(c) no tipped ambassador was drawn on the map after a move');
    else if (!/tipped: moved or acted this round/.test(seen.tipped.hover)) failures.push(`(c) the tipped ambassador's hover reads "${seen.tipped.hover}"`);
    if (removedSeen.length) failures.push(`(d) ${removedSeen.length} cut pieces on screen`);
    if (seen.moveButton !== 'Move an ambassador') failures.push(`(d) the Move action's button reads "${seen.moveButton}"`);
    if (!end.outcome) failures.push('the game did not end');
    else if (!end.outcome.won && !anyEnd) failures.push(`the game did not end in a win: ${JSON.stringify(end.outcome)}`);
    else if (!end.outcome.won) { /* --any-end: a whole game, lost */ }
    else if (!end.shownAsWin || !(end.shown ?? '').includes(end.outcome.detail)) failures.push(`the win is shown as "${end.shown}"`);
    if (page.consoleErrors.length) failures.push(`${page.consoleErrors.length} console errors`);
    const summary = {
      how: 'every move clicked in the player view (web/scripts/ambassadors-play.mjs)', config: 'baseline.json', override, seed, size: `${width}x${height}`,
      end: end.shown, outcome: end.outcome, actions: end.actions, view: end.mode, header: end.revision,
      seen, nodePowerUses: Object.fromEntries([...uses].map(([k, v]) => [`seat ${k.replace(':', ', round ')}`, v.map((u) => u.name).join(' + ')])),
      removed: removedSeen.length, removedSamples: removedSeen.slice(0, 40), failures, moves, clicks, consoleErrors: page.consoleErrors, shots,
    };
    writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
    console.log(JSON.stringify(summary, null, 1));
    ok = failures.length === 0;
    const a = seen.moveThenLiberate, b = seen.nodePowers, c = seen.tipped;
    console.log(`${ok ? 'PASS' : 'FAIL'} ambassadors-play seed ${seed}: ${end.shown ?? 'no outcome'}`
      + `; (a) ${a ? `seat ${a.seat} moved onto ${a.city} in round ${a.movedOnInRound}, liberated it in round ${a.liberatedInRound}` : 'not seen'}`
      + `; (b) ${b ? `seat ${b.seat} (${b.board}) used ${b.powers.join(' + ')} in round ${b.round}` : 'not seen'}`
      + `; (c) ${c ? `tipped ambassador drawn on ${c.hex} in round ${c.round}` : 'not seen'}`
      + `; (d) ${removedSeen.length} cut pieces on screen${failures.length ? `; ${failures.length} failures (summary.json)` : ''}`);
  }
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
