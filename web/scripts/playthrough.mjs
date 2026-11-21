// 305: a whole plain game played through the player view, as a person playing every seat would: the bar for Matt's
// playthrough of the overhauled game. Unlike troops-play.mjs and ambassadors-play.mjs it plays a config as it ships
// (no override): `baseline.json` (two seats, d0) or `three_seat.json`.
//
//   node scripts/playthrough.mjs [--config baseline.json] [--seed 1] [--out DIR] [--size 1366x768] [--steps 4000]
//                                [--search A-B]
//
// Needs a served viewer (AIC_URL). The moves come from play-policy.mjs's small policy (it reads the engine's view to
// choose); every one is made by clicking the control the player view shows for it, and the adversary's turn is
// stepped through ("Since your last move") whenever it opens. The debug view is never switched on, and no action
// goes through `aic.act`. On the way it:
//   - shoots each turn phase the first time it shows (round 2 on, so the board has moved): each round-bar phase
//     the seats act in, each prompt kind, and each step of the adversary's turn panel (OUT/phase-*.png);
//   - checks every step for what a player could not read: page scroll, an element off screen with no scroll, text
//     clipped with no ellipsis, a Draft / Keep button or the turn panel below its fold, a placeholder word, a cut
//     piece (cut-pieces.mjs), a console error;
//   - ticks the overhaul's rules as the player view shows them (`seen` in the summary, `RULES` below): a rule the
//     game never reached is listed as unreached, not failed (a plain d0 game may never see a troop).
// Fails (exit 1) on no outcome, any check above, or a rule shown wrongly. Writes OUT/summary.json and OUT/record.json.
// `--search A-B` plays seeds A to B by the policy straight through the engine (no clicks) and prints each end and the
// rounds the troop rules first showed: how a seed that reaches them is found.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, viewerUrl } from './cdp.mjs';
import { installCut } from './cut-pieces.mjs';
import { POLICY } from './play-policy.mjs';

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const file = arg('config', 'baseline.json');
const seed = arg('seed', '1');
const out = arg('out', join(web, '..', 'out', 'playthrough', file.replace(/\.json$/, '')));
const [width, height] = arg('size', '1366x768').split('x').map(Number);
const maxSteps = Number(arg('steps', '4000'));
const search = arg('search', '');
const config = readFileSync(join(web, '..', 'configs', file), 'utf8');

/** In the page: the per-step screen checks, and the overhaul's rules as the player view shows them. */
const CHECKS = `(() => {
  const vis = (el) => { const s = getComputedStyle(el); const r = el.getBoundingClientRect(); return s.visibility !== 'hidden' && s.display !== 'none' && r.width > 0 && r.height > 0; };
  const label = (el) => el.tagName.toLowerCase() + (el.id ? '#' + el.id : '') + (typeof el.className === 'string' && el.className ? '.' + el.className.trim().split(/\\s+/).join('.') : '') + ' "' + (el.textContent || '').trim().replace(/\\s+/g, ' ').slice(0, 50) + '"';
  const scrollParent = (el) => { for (let p = el.parentElement; p; p = p.parentElement) { const o = getComputedStyle(p); if (/(auto|scroll)/.test(o.overflow + o.overflowX + o.overflowY)) return p; } return null; };
  const inside = (r, b) => r.left >= b.left - 1 && r.right <= b.right + 1 && r.top >= b.top - 1 && r.bottom <= b.bottom + 1;
  const PLACEHOLDER = /\\b(undefined|NaN|null|\\[object Object\\])\\b/;
  const count = (h, kind) => h.tokens.find((t) => t.kind === kind)?.count ?? 0;
  const glob = (k) => aic.view.globals.find((x) => x.key === k);
  const any = (sel) => [...document.querySelectorAll(sel)].find(vis);
  window.__pt = {
    screen() {
      const issues = [];
      const add = (kind, what) => issues.push(kind + ': ' + what);
      const de = document.documentElement;
      if (de.scrollHeight > innerHeight || de.scrollWidth > innerWidth) add('page-scroll', de.scrollWidth + 'x' + de.scrollHeight);
      if (aic.mode !== 'player') add('debug-view', 'the debug view is on');
      if (aic.error) add('error-banner', aic.error.slice(0, 160));
      const view = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
      for (const el of document.querySelectorAll('#top, #top *, #player, #player *')) {
        if (!(el instanceof HTMLElement) || !vis(el)) continue;
        const r = el.getBoundingClientRect();
        if (!scrollParent(el) && !inside(r, view)) add('offscreen', label(el) + ' at ' + Math.round(r.left) + ',' + Math.round(r.top) + '-' + Math.round(r.right) + ',' + Math.round(r.bottom));
        const s = getComputedStyle(el);
        if (/(hidden|clip)/.test(s.overflow + s.overflowX + s.overflowY) && (el.scrollWidth > el.clientWidth + 2 || el.scrollHeight > el.clientHeight + 2)
          && s.textOverflow !== 'ellipsis' && s.webkitLineClamp === 'none') add('clipped', label(el) + ' ' + el.scrollWidth + 'x' + el.scrollHeight + ' in ' + el.clientWidth + 'x' + el.clientHeight);
        const own = [...el.childNodes].filter((n) => n.nodeType === 3).map((n) => n.textContent).join(' ');
        if (PLACEHOLDER.test(own)) add('placeholder', label(el));
      }
      for (const el of [...document.querySelectorAll('#p-seat .pdcard .pact, #p-turn')].filter((e) => !e.hidden && vis(e))) {
        const r = el.getBoundingClientRect();
        const sp = scrollParent(el);
        const pr = sp && sp.getBoundingClientRect();
        const box = sp ? { left: pr.left + sp.clientLeft, top: pr.top + sp.clientTop, right: Math.min(innerWidth, pr.left + sp.clientLeft + sp.clientWidth), bottom: Math.min(innerHeight, pr.top + sp.clientTop + sp.clientHeight) } : view;
        if (!inside(r, box)) add('below-fold', label(el) + ' ends at ' + Math.round(r.bottom) + ', its pane shows to ' + Math.round(box.bottom));
      }
      return [...new Set(issues)];
    },
    /** The overhaul's rules on screen now: each key true when the player view shows it, a string when it shows it wrongly. */
    rules() {
      const v = aic.view, r = {};
      const tipped = !document.querySelector('#p-turn.phase-board:not([hidden])') && v.hexes.some((h) => h.tokens.some((t) => t.kind.startsWith('tipped:') && t.count > 0));
      if (tipped) r.tippedAmbassador = any('#board .token[data-token^="tipped:"] .tipped') ? true : 'a tipped ambassador in the view is not drawn tipped';
      if (aic.legal.some((a) => a.type === 'standard' && a.act === 'liberate' && a.seat === aic.focus)) r.liberateOffered = any('#p-bar button.a-standard[data-action*="liberate"]') ? true : 'Liberate is legal with no button';
      const bill = any('#player .plib');
      // The bill as the rules print it (3 at any site: Liberate at 3), a site's own line or the rule's (308: one price
      // for every site, "3 ambassadors at any site").
      if (bill) r.liberateBill = /3 ambassadors, \\d+ here \\+ \\d+ from supply|3 ambassadors (at any site|a city)/.test(bill.textContent) ? true : 'the Liberate line reads "' + bill.textContent + '"';
      const blocked = any('#player .plib .blocked');
      if (blocked) r.liberateBlocked = true;
      const powers = [...document.querySelectorAll('#player .ppowers.node li')].filter(vis);
      if (powers.length) r.nodePowers = powers.length === 3 && any('#player .pnode-head') ? true : powers.length + ' node powers on the board';
      if (powers.some((li) => li.classList.contains('used'))) r.nodePowerUsed = true;
      const turn = document.querySelector('#p-turn .pturn-head span');
      if (turn && vis(turn) && /card beat/i.test(turn.textContent)) r.cardBeat = true;
      if (turn && vis(turn) && /income beat/i.test(turn.textContent)) r.incomeBeat = true;
      if (document.querySelector('#p-turn.phase-board')) r.phaseBoard = true;
      if (glob('board.strike_ahead')?.value === 1) r.strikeAhead = any('#player .slot-box.next .ahead.strike') ? true : 'a Strike ahead with no notice';
      if (glob('board.redeploy_ahead')?.value === 1) r.redeployAhead = any('#player .slot-box.next .ahead.redeploy') ? true : 'a Redeploy ahead with no notice';
      // While the turn panel shows an earlier step the map draws that step (#p-turn.phase-board), not the view after.
      const after = !document.querySelector('#p-turn.phase-board:not([hidden])');
      const troops = after ? v.hexes.reduce((n, h) => n + count(h, 'troop'), 0) : 0;
      if (troops) r.troops = any('#board .token[data-token="troop"]') ? true : 'troops in the view, none drawn';
      if (after && v.arrows.some((a) => a.kind === 'guard')) r.garrison = any('#board .arrows .arrow.guard path.shield') ? true : 'a guard link with no shield';
      const row = any('#player .research-row');
      if (row) r.researchRow = true;
      const k = any('#player .research-row .rk');
      if (k) r.researchK = /\\d/.test(k.textContent) ? true : 'K reads "' + k.textContent + '"';
      const cap = any('#player .round-cap');
      if (cap) r.roundCap = /\\d/.test(cap.textContent) ? true : 'the round cap reads "' + cap.textContent + '"';
      const deck = any('#player .deck-left');
      if (deck) r.deckLeft = /\\d/.test(deck.textContent) ? true : 'the deck reads "' + deck.textContent + '"';
      if (any('#player [data-track="alignment.discord"], #player [data-value="alignment.discord"]')) r.discord = true;
      if (any('#player [data-track="alignment.unity"], #player [data-value="alignment.unity"]')) r.unity = true;
      return r;
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
    await page.js(`window.aic.play(${JSON.stringify(config)}, '${s}', ${JSON.stringify(file)})`);
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
    await page.js(CHECKS);
    const shots = [];
    const shot = async (name) => {
      await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
      writeFileSync(join(out, `${name}.png`), await page.shot());
      shots.push(`${name}.png`);
    };
    const issues = new Map();
    const note = (round, list) => { for (const x of list) if (!issues.has(x)) issues.set(x, round); };
    const seen = {};
    const wrong = new Map();
    const look = async (round) => {
      note(round, await page.js('window.__pt.screen()'));
      note(round, (await page.js('window.__cut.removed()')).map((x) => 'cut: ' + x));
      for (const [k, x] of Object.entries(await page.js('window.__pt.rules()'))) {
        if (x === true) seen[k] ??= round;
        else if (!wrong.has(k)) wrong.set(k, `round ${round}: ${x}`);
      }
    };
    const shotSeen = new Set();
    const clicks = {};
    const moves = [];
    for (let i = 0; i < maxSteps; i++) {
      const st = await page.js(`({ ...window.__tp.state(), prompt: aic.view.choice ? aic.view.choice.prompt.replace(/^Seat \\d+: */, '').replace(/[0-9#]+/g, 'N').slice(0, 40) : null,
        turn: (() => { const e = document.querySelector('#p-turn .pturn-head span'); return e ? e.textContent : null; })() })`);
      if (st.over) break;
      await look(st.round);
      // Each phase the first time it shows from round 2: the turn panel's steps, then the phase the seats act in.
      const moment = st.turn ? `turn ${st.turn.replace(/^step \d+ of \d+: /, '')}` : st.prompt ? `prompt ${st.prompt}` : st.phase ? `phase ${st.phase}` : null;
      if (moment && st.round >= 2 && !shotSeen.has(moment.replace(/ \(.*$/, ''))) {
        shotSeen.add(moment.replace(/ \(.*$/, ''));
        await shot(`phase-${String(shots.length + 1).padStart(2, '0')}-r${st.round}-${moment.replace(/[^A-Za-z0-9]+/g, '-').replace(/-+$/, '').slice(0, 48)}`);
      }
      // The troops' moments, the first time the turn panel's notes show each: a Strike, a Redeploy, a garrison.
      if (st.turn) {
        const notes = await page.js(`document.querySelector('#p-turn .pturn-ev')?.innerText ?? ''`);
        for (const [k, re] of [['strike', / strikes /], ['redeploy', / redeploys? from /], ['garrison', / garrisons? /]]) {
          if (!shotSeen.has(`moment ${k}`) && re.test(notes)) { shotSeen.add(`moment ${k}`); await shot(`moment-${k}-r${st.round}`); }
        }
      }
      const turn = await page.js('window.__tp.turn()');
      if (turn) { clicks[turn] = (clicks[turn] ?? 0) + 1; continue; }
      const d = await page.js('window.__tp.decide()');
      if (!d) throw new Error(`step ${i}: no legal action and no outcome (round ${st.round}, ${st.phase})`);
      const what = await page.js(`window.__tp.click(${JSON.stringify(d.action)})`);
      if (what === null) throw new Error(`step ${i}: the player view shows no control for ${JSON.stringify(d.action)} (round ${st.round}, ${st.phase})`);
      clicks[d.why] = (clicks[d.why] ?? 0) + 1;
      if (!['pass', 'draft'].includes(d.why)) moves.push(`round ${st.round}: seat ${d.action.seat} ${d.why}: ${what.replace(/\s+/g, ' ').trim()}`);
      if (d.why === 'liberate' && !shotSeen.has('moment liberated')) { shotSeen.add('moment liberated'); await shot(`moment-liberated-r${st.round}`); }
    }
    await look('end');
    await shot('end');
    // D52: the timeline keeps a state a step; the heap the whole game holds, after a collection.
    await page.send('HeapProfiler.collectGarbage').catch(() => {});
    const heap = await page.send('Runtime.getHeapUsage').catch(() => null);
    const end = await page.js(`(() => {
      const e = document.querySelector('#phase-bar .outcome');
      return { outcome: aic.view.outcome, shown: e ? e.textContent : null, actions: aic.timeline.last, view: aic.mode, round: aic.view.round,
        header: document.querySelector('header')?.innerText.replace(/\\s+/g, ' ').slice(0, 160) ?? '' };
    })()`);
    await page.js(`document.getElementById('record').click()`);
    const record = await page.js(`document.getElementById('record-text').value`);
    await page.js(`document.getElementById('record-close').click()`);
    writeFileSync(join(out, 'record.json'), record.endsWith('\n') ? record : `${record}\n`);
    const RULES = ['tippedAmbassador', 'liberateOffered', 'liberateBill', 'liberateBlocked', 'nodePowers', 'nodePowerUsed', 'cardBeat', 'incomeBeat', 'phaseBoard',
      'strikeAhead', 'redeployAhead', 'troops', 'garrison', 'researchRow', 'researchK', 'roundCap', 'deckLeft', 'discord', 'unity'];
    const failures = [...[...issues].map(([x, r]) => `round ${r}: ${x}`), ...wrong.values()];
    if (!end.outcome) failures.push('no outcome');
    if (page.consoleErrors.length) failures.push(`${page.consoleErrors.length} console errors`);
    const summary = {
      how: 'every move clicked in the player view (web/scripts/playthrough.mjs)', config: file, seed, size: `${width}x${height}`,
      end: end.shown, outcome: end.outcome, rounds: end.round, actions: end.actions, view: end.view, header: end.header,
      heapMB: heap ? Math.round(heap.usedSize / 1e6) : null,
      seen, unreached: RULES.filter((k) => seen[k] === undefined && !wrong.has(k)), failures: failures.slice(0, 60), failureCount: failures.length,
      moves, clicks, consoleErrors: page.consoleErrors, shots,
    };
    writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
    const { moves: _m, ...brief } = summary;
    console.log(JSON.stringify(brief, null, 1));
    console.log(failures.length ? `FAIL: ${failures.length} problems` : 'PASS');
    ok = failures.length === 0;
  }
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
