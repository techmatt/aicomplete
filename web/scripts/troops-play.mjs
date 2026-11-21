// 294: the troops demonstration. Plays one whole game in the player view, by its own controls, to the "no corrupted
// city" win, and checks on the way that the view showed the garrison rules:
//   (a) a troop on the map with its guard link (the shield on the corrupted site it guards),
//   (b) the Strike-ahead notice by the adversary's face-up next card,
//   (c) the "Liberate refused" note in a seat's bar while a troop guarded a corrupted site in its reach, the site
//       marked on the map and its hover saying why,
//   (d) the end shown as a win reading "no city is corrupted", and no cut piece (cut-pieces.mjs) on any step.
//
//   node scripts/troops-play.mjs [--seed 30] [--out DIR] [--size 2560x1440] [--override JSON] [--search 1-40]
//
// Needs a served viewer (AIC_URL). Writes OUT/summary.json (what was seen and in which round, the moves, any
// failure), OUT/record.json and screenshots; exit 1 on any failure. At the plain setting troops arrive late
// (research slot 4), so the game runs on the baseline with `OVERRIDE` below (`--override` replaces it; the summary
// records it). A Strike is shown ahead by a stage-II card, from round 5: the seed is one whose game lasts that long
// and shows one (305, r148, seeds 13-60: 44 wins, (a)-(c) all in 14, 25, 28, 30, 41, 47-51 and more; 295's 17 no
// longer shows a guard; 312, r151, seeds 13-36: (a)-(c) seen in 15, 25, 30, 36, not 14; 30 passes every check, 15 and 25 miss the guarded site's hover). The moves come from a small
// policy (`decide`, play-policy.mjs) that reads the engine's view and legal actions; every one is then made by clicking the
// control the player view shows for it (a button, or a highlighted hex on the map), the seat's row first when
// another seat is shown, and the adversary's turn is stepped through whenever it opens. `--search A-B` plays seeds
// A to B by the same policy straight through the engine, no clicks and no checks, and prints each one's end and
// the rounds (a)-(c) could be seen: how the demo's seed was found.
//
// The policy is play-policy.mjs (295: shared with ambassadors-play.mjs, and r142's: node powers each round, an
// ambassador that moves onto the city one round and liberates it the next).
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, viewerUrl } from './cdp.mjs';
import { installCut } from './cut-pieces.mjs';
import { POLICY } from './play-policy.mjs';

/** The demo's setting, over configs/baseline.json. The research row starts at 8 cubes a country, so its slots 1-4
 *  (the fourth a Garrison) fire in round 1's Income beat: a troop garrisons each country's corrupted city. So that
 *  this small policy can win from there: no further city corrupts (`corrupt_at` 20: the two cities corrupted at setup
 *  are the game) and stations make no cubes (`station_cubes` 0: the row moves by threats alone, where the corrupted
 *  stations would run it from slot 4 to its loss in about three rounds). Without the last two the policy wins
 *  1 seed in 8 or none (294). */
const OVERRIDE = { adversary: { research_start: 8, corrupt_at: 20 }, board: { station_cubes: 0 } };

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const seed = arg('seed', '30');
const out = arg('out', join(web, '..', 'out', 'troops-play'));
const [width, height] = arg('size', '2560x1440').split('x').map(Number);
const maxSteps = Number(arg('steps', '4000'));
const override = JSON.parse(arg('override', JSON.stringify(OVERRIDE)));
const search = arg('search', '');

const config = JSON.parse(readFileSync(join(web, '..', 'configs', 'baseline.json'), 'utf8'));
for (const [name, params] of Object.entries(override)) {
  const m = config.modules.find((x) => x.module === name);
  if (!m) throw new Error(`--override names no module of the baseline: ${name}`);
  Object.assign(m.params, params);
}

const page = await openPage({ width, height });
let ok = false;
try {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await page.js(`window.aic.setView('player')`);
  const start = async (s) => {
    await page.js(`window.aic.play(${JSON.stringify(JSON.stringify(config))}, '${s}', 'troops-play')`);
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
    const shot = async (name, park = true) => {
      if (park) await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
      writeFileSync(join(out, `${name}.png`), await page.shot());
      shots.push(`${name}.png`);
    };
    const removed = new Map();
    const lookForRemoved = async (round) => {
      for (const r of await page.js('window.__cut.removed()')) if (!removed.has(r)) removed.set(r, round);
    };
    const seen = { garrison: null, strikeAhead: null, liberateRefused: null };
    const clicks = {};
    const moves = [];
    let liberated = 0;
    for (let i = 0; i < maxSteps; i++) {
      const st = await page.js('window.__tp.state()');
      if (st.over) break;
      await lookForRemoved(st.round);
      // The adversary's turn, stepped through as a player reads it.
      const turn = await page.js('window.__tp.turn()');
      if (turn) { clicks[turn] = (clicks[turn] ?? 0) + 1; continue; }
      if (st.acting) {
        if (!seen.garrison) {
          const g = await page.js('window.__tp.garrison()');
          if (g) { seen.garrison = { round: st.round, ...g }; await shot(`a-garrison-r${st.round}`); }
        }
        if (!seen.liberateRefused && st.refused.some((n) => n > 0)) {
          // Show the refused seat by its row, as a player would, then read its bar and hover the guarded site.
          const s = st.refused.findIndex((n) => n > 0);
          await page.js(`[...document.querySelectorAll('#player [data-seat-focus="${s}"]')].find((e) => e.getBoundingClientRect().width > 0)?.click()`);
          const c = await page.js('window.__tp.refusedNote()');
          if (c) {
            await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: c.x, y: c.y });
            await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: c.x + 1, y: c.y });
            const tip = await page.js(`(() => { const t = document.getElementById('tip'); return t && !t.hidden ? t.innerText : ''; })()`);
            const { x: _x, y: _y, ...note } = c;
            seen.liberateRefused = { round: st.round, ...note, siteHover: tip.split('\n').find((l) => /can't be liberated/.test(l)) ?? null };
            await lookForRemoved(st.round);
            await shot(`c-liberate-refused-r${st.round}`, false);
            await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
          }
        }
        if (!seen.strikeAhead && st.strikeAhead === 1) {
          const b = await page.js('window.__tp.strikeNotice()');
          if (b) { seen.strikeAhead = { round: st.round, troopsOnBoard: st.troops, ...b }; await shot(`b-strike-ahead-r${st.round}`); }
        }
      }
      const d = await page.js('window.__tp.decide()');
      if (!d) throw new Error(`step ${i}: no legal action and no outcome (round ${st.round}, ${st.phase})`);
      const what = await page.js(`window.__tp.click(${JSON.stringify(d.action)})`);
      if (what === null) throw new Error(`step ${i}: the player view shows no control for ${JSON.stringify(d.action)} (round ${st.round}, ${st.phase})`);
      clicks[d.why] = (clicks[d.why] ?? 0) + 1;
      if (!['pass', 'draft'].includes(d.why)) moves.push(`round ${st.round}: seat ${d.action.seat} ${d.why}: ${what.replace(/\s+/g, ' ').trim()}`);
      const after = await page.js('window.__tp.state()');
      if (after.corrupted < st.corrupted && !after.over) { liberated++; await shot(`liberated-${liberated}-r${after.round}`); }
    }
    await lookForRemoved('end');
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
    const failures = [];
    if (!seen.garrison) failures.push('(a) no troop with a drawn guard link was seen');
    if (!seen.strikeAhead) failures.push('(b) the Strike-ahead notice was never on screen');
    else if (!/^Strike ahead/.test(seen.strikeAhead.text)) failures.push(`(b) the notice reads "${seen.strikeAhead.text}"`);
    if (!seen.liberateRefused) failures.push('(c) the Liberate-refused note was never on screen beside a guarded site');
    else {
      if (!/^Liberate refused: \d+ corrupted sites? in reach guarded by a troop$/.test(seen.liberateRefused.text)) failures.push(`(c) the note reads "${seen.liberateRefused.text}"`);
      if (!seen.liberateRefused.siteHover) failures.push('(c) the guarded site\'s hover does not say it can\'t be liberated');
    }
    if (!end.outcome?.won) failures.push(`(d) the game did not end in a win: ${JSON.stringify(end.outcome)}`);
    else if (!/no city is corrupted/.test(end.outcome.detail) || !end.shownAsWin || !(end.shown ?? '').includes(end.outcome.detail)) failures.push(`(d) the win is shown as "${end.shown}"`);
    if (removedSeen.length) failures.push(`(d) ${removedSeen.length} cut pieces on screen`);
    if (page.consoleErrors.length) failures.push(`${page.consoleErrors.length} console errors`);
    const summary = {
      how: 'every move clicked in the player view (web/scripts/troops-play.mjs)', config: 'baseline.json', override, seed, size: `${width}x${height}`,
      end: end.shown, outcome: end.outcome, actions: end.actions, view: end.mode, header: end.revision,
      seen, removed: removedSeen.length, removedSamples: removedSeen.slice(0, 40), failures, moves, clicks, consoleErrors: page.consoleErrors, shots,
    };
    writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
    console.log(JSON.stringify(summary, null, 1));
    ok = failures.length === 0;
  }
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
