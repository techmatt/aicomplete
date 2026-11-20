// 294: the troops demonstration. Plays one whole game in the player view, by its own controls, to the "no corrupted
// city" win, and checks on the way that the view showed the garrison rules:
//   (a) a troop on the map with its guard link (the shield on the corrupted site it guards),
//   (b) the Strike-ahead notice by the adversary's face-up next card,
//   (c) the "Liberate refused" note in a seat's bar while a troop guarded a corrupted site in its reach, the site
//       marked on the map and its hover saying why,
//   (d) the end shown as a win reading "no city is corrupted", and no cut piece (cut-pieces.mjs) on any step.
//
//   node scripts/troops-play.mjs [--seed 17] [--out DIR] [--size 2560x1440] [--override JSON] [--search 1-40]
//
// Needs a served viewer (AIC_URL). Writes OUT/summary.json (what was seen and in which round, the moves, any
// failure), OUT/record.json and screenshots; exit 1 on any failure. At the plain setting troops arrive late
// (research slot 4), so the game runs on the baseline with `OVERRIDE` below (`--override` replaces it; the summary
// records it). A Strike is shown ahead by a stage-II card, from round 5: the seed is one whose game lasts that long
// and shows one (294, seeds 13-90: 75 wins, 9 games that show it, 8 of them wins: 17, 23, 29, 54, 64, 65, 72, 86). The moves come from a small
// policy (`decide` below) that reads the engine's view and legal actions; every one is then made by clicking the
// control the player view shows for it (a button, or a highlighted hex on the map), the seat's row first when
// another seat is shown, and the adversary's turn is stepped through whenever it opens. `--search A-B` plays seeds
// A to B by the same policy straight through the engine, no clicks and no checks, and prints each one's end and
// the rounds (a)-(c) could be seen: how the demo's seed was found.
//
// The policy: draft from the first tier deck and keep nothing; each round use the board's free Growth power (or one
// that gains an ambassador), placing the node toward the seat's corrupted city; buy ambassadors toward the Liberate
// bill; with the bill in hand, or when a Strike is shown ahead against the seat's node, Remove the troop guarding
// the city; clear agents off it; Liberate.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, viewerUrl } from './cdp.mjs';
import { installCut } from './cut-pieces.mjs';

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
const seed = arg('seed', '17');
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

/** In the page: the policy (`decide`), the click that makes its move (`click`) and what the view shows (`look`). */
const POLICY = `(() => {
  const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden'; };
  const isHex = (t) => Array.isArray(t) && t.length === 2;
  const count = (h, kind) => h.tokens.find((t) => t.kind === kind)?.count ?? 0;
  const val = (s, k) => aic.view.seats[s].values.find((x) => x.key === k)?.value ?? 0;
  const glob = (k) => aic.view.globals.find((x) => x.key === k)?.value ?? 0;
  const reach = (s, h) => [h.index, ...h.neighbors].some((i) => count(aic.view.hexes[i], 'node:' + s) > 0);
  const guardsOf = (h) => aic.view.arrows.filter((a) => a.kind === 'guard' && a.to === h.index).map((a) => a.from);
  /** Steps from hex index \`from\` to every hex, over the map's neighbors. */
  const steps = (from) => {
    const d = new Map([[from, 0]]);
    for (const q = [from]; q.length;) { const i = q.shift(); for (const n of aic.view.hexes[i].neighbors) if (!d.has(n)) { d.set(n, d.get(i) + 1); q.push(n); } }
    return d;
  };
  /** Each seat's corrupted city: the pairs taken nearest first, a seat and a city once each while both last. */
  const targets = () => {
    const v = aic.view;
    const cities = v.hexes.filter((h) => h.kind === 'city' && count(h, 'cube:adversary') > 0);
    const pairs = [];
    for (const c of cities) {
      const d = steps(c.index);
      for (const s of v.seats) {
        const nodes = v.hexes.filter((h) => count(h, 'node:' + s.index) > 0).map((h) => d.get(h.index) ?? 99);
        pairs.push({ seat: s.index, city: c, d: Math.min(99, ...nodes) });
      }
    }
    pairs.sort((a, b) => a.d - b.d || a.city.index - b.city.index || a.seat - b.seat);
    const out = new Map(), taken = new Set();
    for (const p of pairs) if (!out.has(p.seat) && !taken.has(p.city.index)) { out.set(p.seat, p.city); taken.add(p.city.index); }
    for (const p of pairs) if (!out.has(p.seat)) out.set(p.seat, p.city);
    return out;
  };
  const hexIndex = (t) => aic.view.hexes.findIndex((h) => h.hex[0] === t[0] && h.hex[1] === t[1]);
  window.__tp = {
    /** What the last action taken was for: it picks the answer to the question that follows. */
    intent: null,
    /** What one seat wants to do now, or null (it passes). */
    want(s, mine) {
      const v = aic.view;
      const first = mine.find((a) => a.type === 'tier') ?? mine.find((a) => a.type === 'draft');
      if (first) return { action: first, why: 'draft' };
      if (!mine.some((a) => a.type === 'pass')) return { action: mine[0], why: 'the only kind of move' };
      if (!/board\\.act/.test(v.phase?.name ?? '')) return null;
      const std = (act) => mine.find((a) => a.type === 'standard' && a.act === act);
      const city = targets().get(s) ?? null;
      const amb = val(s, 'resource.ambassadors'), credits = val(s, 'resource.credits');
      const guards = city ? guardsOf(city) : [];
      const agents = city ? count(city, 'ambassador:adversary') : 0;
      const bill = 5 + 2 * Math.ceil(agents / 3);
      if (city && reach(s, city)) {
        if (!guards.length && !agents && amb >= 5 && std('liberate')) return { action: std('liberate'), why: 'liberate', intent: { kind: 'site', city: city.index } };
        if (!guards.length && agents && amb >= bill && std('remove')) return { action: std('remove'), why: 'clear the agents', intent: { kind: 'site', city: city.index } };
        const struck = v.arrows.some((a) => a.kind === 'troop' && a.rounds === 1 && guards.includes(a.from) && count(v.hexes[a.to], 'node:' + s) > 0);
        if (guards.length && std('troop') && (amb >= bill || struck)) return { action: std('troop'), why: 'remove the guard', intent: { kind: 'troop', city: city.index } };
      }
      // One Growth option a round: the one that gains an ambassador, else the free one.
      const powers = v.seats[s].board?.cards ?? [];
      const growth = mine.filter((a) => a.type === 'power').map((a) => ({ a, text: powers.find((c) => c.id.endsWith('.power.' + a.power))?.text ?? '' })).filter((p) => /Growth/.test(p.text));
      const grow = growth.find((p) => /gain 1 ambassador/.test(p.text)) ?? growth.find((p) => !/^Pay /.test(p.text));
      if (grow) return { action: grow.a, why: 'grow', intent: { kind: 'node', city: city?.index ?? null } };
      if (city && amb < bill && std('ambassador') && credits >= 16 + (guards.length ? 7 : 0)) return { action: std('ambassador'), why: 'gain an ambassador' };
      return null;
    },
    /** The next move: { action, why }, or null when no action is legal. */
    decide() {
      const v = aic.view, legal = aic.legal;
      if (!legal.length) return null;
      if (v.choice) {
        const answers = legal.filter((a) => a.type === 'choose');
        const hexes = answers.filter((a) => isHex(a.target));
        const it = this.intent;
        let best = hexes[0] ?? answers[0] ?? legal[0];
        if (hexes.length && it?.kind === 'node' && it.city !== null) {
          const d = steps(it.city);
          best = hexes.reduce((b, a) => ((d.get(hexIndex(a.target)) ?? 99) < (d.get(hexIndex(b.target)) ?? 99) ? a : b));
        } else if (hexes.length && it?.kind === 'troop') {
          const from = (a) => v.arrows.filter((x) => x.kind === 'guard' && x.from === hexIndex(a.target));
          best = hexes.find((a) => from(a).some((x) => x.to === it.city)) ?? hexes.find((a) => from(a).length) ?? best;
        } else if (hexes.length && it?.kind === 'site') {
          best = hexes.find((a) => hexIndex(a.target) === it.city) ?? best;
        }
        return { action: best, why: 'answer' + (it ? ' (' + it.kind + ')' : '') };
      }
      this.intent = null;
      for (const s of [...new Set(legal.map((a) => a.seat))].sort()) {
        const w = this.want(s, legal.filter((a) => a.seat === s));
        if (w) { this.intent = w.intent ?? null; return w; }
      }
      return { action: legal.find((a) => a.type === 'pass') ?? legal[0], why: 'pass' };
    },
    /** Makes \`a\` by the player view's own control for it; returns what it clicked, or null when there is none. */
    click(a) {
      const key = JSON.stringify(a);
      const button = () => [...document.querySelectorAll('#player button[data-action]')].filter(vis).find((b) => b.dataset.action === key);
      const hex = () => isHex(a.target) && [...document.querySelectorAll('#player .hex-pick[data-hexes]')].filter(vis).some((e) => e.dataset.hexes.split(' ').includes(a.target.join(',')));
      let switched = '';
      if (!button() && !hex() && aic.focus !== a.seat) {
        const row = [...document.querySelectorAll('#player [data-seat-focus="' + a.seat + '"]')].find(vis);
        if (row) { row.click(); switched = 'seat ' + a.seat + ', '; }
      }
      const b = button();
      if (b) { const t = b.textContent; b.click(); return switched + t; }
      if (hex()) {
        document.querySelector('#board .hex[data-hex="' + a.target.join(',') + '"]').dispatchEvent(new MouseEvent('click', { bubbles: true }));
        return switched + 'hex ' + a.target.join(',');
      }
      return null;
    },
    /** One click of the open "Since your last move" panel (the adversary's turn), or null when it is closed. */
    turn() {
      const next = document.querySelector('#p-turn [data-turn="1"]:not([disabled])');
      if (next) { next.click(); return 'turn step'; }
      const done = document.querySelector('#p-turn [data-turn="close"]');
      if (done) { done.click(); return 'turn done'; }
      return null;
    },
    /** What the engine's view holds of the garrison rules now. */
    state() {
      const v = aic.view;
      const guard = v.arrows.filter((a) => a.kind === 'guard');
      const refused = v.seats.map((s) => val(s.index, 'board.liberate_refused'));
      return {
        over: !!v.outcome, round: v.round, phase: v.phase?.name ?? null, choice: !!v.choice, acting: /board\\.act/.test(v.phase?.name ?? '') && !v.choice,
        troops: v.hexes.reduce((n, h) => n + count(h, 'troop'), 0), guards: guard.length, strikeArrows: v.arrows.filter((a) => a.kind === 'troop').length,
        strikeAhead: glob('board.strike_ahead'), redeployAhead: glob('board.redeploy_ahead'), corrupted: glob('alignment.corrupted_cities'), refused,
      };
    },
    /** (a) on screen: a troop glyph on a hex whose guard link is drawn with its shield. */
    garrison() {
      for (const a of aic.view.arrows.filter((x) => x.kind === 'guard')) {
        const from = aic.view.hexes[a.from], to = aic.view.hexes[a.to];
        const troop = [...document.querySelectorAll('#board .token[data-token="troop"][data-hex="' + from.hex.join(',') + '"]')].find(vis);
        const link = document.querySelector('#board .arrows .arrow.guard[data-from="' + a.from + '"][data-to="' + a.to + '"]');
        if (troop && link?.querySelector('path.shield') && vis(link)) return { troop: aic.view.countries[from.country].name + ' #' + from.number, guards: aic.view.countries[to.country].name + ' #' + to.number, doc: a.doc };
      }
      return null;
    },
    /** (b) on screen: the Strike-ahead notice in the adversary's next-round box. */
    strikeNotice() {
      const e = [...document.querySelectorAll('#player .slot-box.next .ahead.strike')].find(vis);
      const next = aic.view.piles.find((p) => p.key === 'adversary.next')?.visible[0];
      return e ? { text: e.textContent, hover: e.title, next: next?.name ?? null, solidArrows: document.querySelectorAll('#board .arrows .arrow.troop.soon').length } : null;
    },
    /** (c) on screen for the shown seat: the Liberate-refused note, and a corrupted site in its reach wearing the
     *  guard's shield; the hex to hover for the reason. */
    refusedNote() {
      const s = aic.focus;
      const e = [...document.querySelectorAll('#p-bar .prefused')].find(vis);
      if (!e || !val(s, 'board.liberate_refused')) return null;
      const a = aic.view.arrows.find((x) => x.kind === 'guard' && reach(s, aic.view.hexes[x.to]) && count(aic.view.hexes[x.to], 'ambassador:adversary') === 0);
      const mark = a && document.querySelector('#board .arrows .arrow.guard[data-to="' + a.to + '"] path.shield');
      if (!a || !mark) return null;
      const to = aic.view.hexes[a.to];
      const r = document.querySelector('#board .hex[data-hex="' + to.hex.join(',') + '"] .cell').getBoundingClientRect();
      return { seat: s, text: e.textContent, hover: e.title, site: aic.view.countries[to.country].name + ' #' + to.number, x: r.left + r.width / 2, y: r.top + r.height * 0.75 };
    },
    /** \`--search\`: the whole game by the policy, straight through the engine. */
    fast(max) {
      const seen = {};
      const mark = (k, ok) => { if (ok && seen[k] === undefined) seen[k] = aic.view.round; };
      let i = 0;
      for (; i < max && !aic.view.outcome; i++) {
        const st = this.state();
        if (st.acting) { mark('a', st.guards > 0); mark('b', st.strikeAhead === 1); mark('c', st.refused.some((n) => n > 0)); mark('strikeArrow', st.strikeArrows > 0); mark('redeploy', st.redeployAhead === 1); }
        const d = this.decide();
        if (!d) break;
        aic.act(d.action);
      }
      const o = aic.view.outcome;
      return { seen, actions: i, end: o ? (o.won ? 'WON' : 'LOST') + ' in round ' + o.round + ': ' + o.detail : 'no outcome', won: !!o?.won };
    },
  };
  return true;
})()`;

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
