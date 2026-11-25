// 305: a whole plain game played through the player view, as a person playing every seat would: the bar for Matt's
// playthrough of the overhauled game. Unlike troops-play.mjs and ambassadors-play.mjs it plays a config as it ships
// (no override): `baseline.json` (two seats, d0), `three_seat.json` or `solo.json` (323: one player, started through
// the header's Config and Board selects and New game, as a player starts it; `--board` 0-2, default the Builder).
//
//   node scripts/playthrough.mjs [--config baseline.json] [--seed 1] [--out DIR] [--size 1366x768] [--steps 4000]
//                                [--search A-B] [--board N]
//
// Needs a served viewer (AIC_URL). The moves come from play-policy.mjs's small policy (it reads the engine's view to
// choose); every one is made by clicking the control the player view shows for it, and the adversary's turn is
// stepped through ("Since your last move") whenever it opens. The debug view is never switched on, and no action
// goes through `aic.act`. On the way it:
//   - shoots each turn phase the first time it shows (round 2 on, so the board has moved): each round-bar phase
//     the seats act in, each prompt kind, and each step of the adversary's turn panel (OUT/phase-*.png);
//   - checks every step for what a player could not read, with sweep.mjs's screen checks (screen-checks.mjs): page
//     scroll, an element off screen with no scroll, text clipped with no ellipsis, a Draft / Keep button or the turn
//     panel below its fold, a placeholder or stale word; and a cut piece (cut-pieces.mjs), a console error (truncated
//     text and raw ids are listed as `looks`, not failed);
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
import { SCREEN_HARD, SCREEN_LIB } from './screen-checks.mjs';

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
// 323: a one-country config is solo: it starts through the New game form, on `--board` (0 Builder, 1 Protector,
// 2 Researcher; default the config's own).
const solo = JSON.parse(config).countries === 1;
const board = arg('board', String(JSON.parse(config).modules.find((m) => m.module === 'ai').params.boards[0]));

/** In the page: the per-step screen checks, and the overhaul's rules as the player view shows them. */
// 312: the screen checks are screen-checks.mjs's, sweep.mjs's own: a hard kind fails the run, the rest (truncated
// text, a raw id) are listed in the summary's `looks`.
const CHECKS = `(() => {
  ${SCREEN_LIB};
  const { vis } = window.__aicScreen;
  const HARD = new Set(${JSON.stringify(SCREEN_HARD)});
  const count = (h, kind) => h.tokens.find((t) => t.kind === kind)?.count ?? 0;
  // 320: the board the page draws (the turn panel's earlier step while it shows one: map, tracks and rows alike).
  const glob = (k) => aic.mapView.globals.find((x) => x.key === k);
  const any = (sel) => [...document.querySelectorAll(sel)].find(vis);
  window.__pt = {
    /** { hard, soft }: each a list of "kind: what" lines. */
    screen() {
      const issues = window.__aicScreen.screen('#top, #top *, #player, #player *');
      if (aic.mode !== 'player') issues.push({ kind: 'debug-view', what: 'the debug view is on' });
      const lines = (hard) => [...new Set(issues.filter((i) => (i.kind === 'debug-view' || HARD.has(i.kind)) === hard).map((i) => i.kind + ': ' + i.what))];
      return { hard: lines(true), soft: lines(false) };
    },
    /** The overhaul's rules on screen now: each key true when the player view shows it, a string when it shows it wrongly. */
    rules() {
      const v = aic.mapView, r = {};
      const tipped = v.hexes.some((h) => h.tokens.some((t) => t.kind.startsWith('tipped:') && t.count > 0));
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
      // 320: an earlier step of the turn shows that step's tracks and rows, not the board after the turn: each seat
      // row's resources and the research row's slot read the drawn board's, and once they differ from the board now
      // the rollback is seen.
      if (document.querySelector('#p-turn.phase-board:not([hidden])') && v !== aic.view) {
        const res = (view, s, k) => view.seats[s].values.find((x) => x.key === 'resource.' + k)?.value ?? 0;
        const bad = [];
        let differs = false;
        for (const s of v.seats) {
          for (const el of document.querySelectorAll('#p-seats .pseat[data-seat-focus="' + s.index + '"] .sres[data-value^="resource."]')) {
            const k = el.dataset.value.slice('resource.'.length);
            const shown = Number(el.querySelector('b')?.textContent);
            if (shown !== res(v, s.index, k)) bad.push('seat ' + s.index + ' ' + k + ' reads ' + shown + ', the step has ' + res(v, s.index, k));
            if (res(v, s.index, k) !== res(aic.view, s.index, k)) differs = true;
          }
        }
        const slot = document.querySelector('#player .research-row')?.dataset.researchSlot;
        const want = glob('adversary.research_slot')?.value;
        if (slot !== undefined && want !== undefined && Number(slot) !== want) bad.push('the research row reads slot ' + slot + ', the step has ' + want);
        if (want !== aic.view.globals.find((x) => x.key === 'adversary.research_slot')?.value) differs = true;
        if (bad.length) r.pastRows = bad.join('; ');
        else if (differs) r.pastRows = true;
      }
      if (glob('board.strike_ahead')?.value === 1) r.strikeAhead = any('#player .slot-box.next .ahead.strike') ? true : 'a Strike ahead with no notice';
      if (glob('board.redeploy_ahead')?.value === 1) r.redeployAhead = any('#player .slot-box.next .ahead.redeploy') ? true : 'a Redeploy ahead with no notice';
      // v is the board drawn: an earlier step's while the turn panel shows one (#p-turn.phase-board).
      const troops = v.hexes.reduce((n, h) => n + count(h, 'troop'), 0);
      if (troops) r.troops = any('#board .token[data-token="troop"]') ? true : 'troops in the view, none drawn';
      if (v.arrows.some((a) => a.kind === 'guard')) r.garrison = any('#board .arrows .arrow.guard path.shield') ? true : 'a guard link with no shield';
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
    // 323: a solo config starts as a player starts it: the header's Config and Board selects, the seed, New game.
    if (solo) {
      const bad = await page.js(`(() => {
        const cfg = document.getElementById('config'), pick = document.getElementById('board-pick'), b = document.getElementById('board-choice');
        cfg.value = ${JSON.stringify(file)}; cfg.dispatchEvent(new Event('change'));
        if (cfg.value !== ${JSON.stringify(file)}) return 'no ' + ${JSON.stringify(file)} + ' in the Config select';
        if (pick.hidden) return 'no Board select for a solo config';
        b.value = ${JSON.stringify(board)};
        if (b.value !== ${JSON.stringify(board)}) return 'no board ' + ${JSON.stringify(board)} + ' in the Board select';
        document.getElementById('seed').value = '${s}';
        document.querySelector('#new-form button[type=submit]').click();
        return null;
      })()`);
      if (bad) throw new Error(`new game: ${bad}`);
      await page.until(`window.aic.timeline.seed === '${s}' && window.aic.timeline.last === 0 && JSON.parse(window.aic.timeline.configJson).modules.find((m) => m.module === 'ai').params.boards[0] === ${Number(board)}`, `the seed-${s} solo game on board ${board}`, 30000);
      await page.js(POLICY);
      return;
    }
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
    const looks = new Map();
    const look = async (round) => {
      const { hard, soft } = await page.js('window.__pt.screen()');
      note(round, hard);
      for (const x of soft) if (!looks.has(x)) looks.set(x, round);
      note(round, (await page.js('window.__cut.removed()')).map((x) => 'cut: ' + x));
      for (const [k, x] of Object.entries(await page.js('window.__pt.rules()'))) {
        if (x === true) seen[k] ??= round;
        else if (!wrong.has(k)) wrong.set(k, `round ${round}: ${x}`);
      }
    };
    const shotSeen = new Set();
    const hovered = new Set();
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
      // 321: a player-view control hovers its words, never the action's raw JSON (it did over every button): one
      // real mouse move over the first control at each new moment.
      if (moment && !hovered.has(moment)) {
        hovered.add(moment);
        const at = await page.js(`(() => { const b = [...document.querySelectorAll('#player button[data-action]')].find((e) => { const r = e.getBoundingClientRect(); return r.width > 0 && r.top >= 0 && r.bottom <= innerHeight; });
          if (!b) return null; const r = b.getBoundingClientRect(); return { x: r.left + r.width / 2, y: r.top + r.height / 2, what: b.textContent.trim() }; })()`);
        if (at) {
          await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: at.x, y: at.y });
          const tip = await page.js(`(() => { const t = document.getElementById('tip'); return t && !t.hidden ? t.innerText : ''; })()`);
          if (/\{"type":/.test(tip)) note(st.round, [`raw-hover: "${at.what}" hovers its action's JSON`]);
          await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
        }
      }
      // 321: ◀ ▶ Done hold their place from step to step (the step's label wrapped the row and moved ▶ under the mouse).
      const navAt = `(() => { const b = document.querySelector('#p-turn [data-turn="1"]'); if (!b) return null; const r = b.getBoundingClientRect(); return Math.round(r.left) + ',' + Math.round(r.top); })()`;
      const nav = await page.js(navAt);
      const turn = await page.js('window.__tp.turn()');
      if (turn) {
        clicks[turn] = (clicks[turn] ?? 0) + 1;
        const after = turn === 'turn step' ? await page.js(navAt) : null;
        if (nav && after && after !== nav) note(st.round, [`turn-nav-moved: the turn panel's ▶ moved from ${nav} to ${after} between steps`]);
        continue;
      }
      const d = await page.js('window.__tp.decide()');
      if (!d) throw new Error(`step ${i}: no legal action and no outcome (round ${st.round}, ${st.phase})`);
      const before = await page.js(`aic.view.round + ':' + (aic.view.phase?.name ?? '')`);
      const what = await page.js(`window.__tp.click(${JSON.stringify(d.action)})`);
      if (what === null) throw new Error(`step ${i}: the player view shows no control for ${JSON.stringify(d.action)} (round ${st.round}, ${st.phase})`);
      // 321: one person plays every seat: after a seat's move the view stays on it while it has a move in the same
      // phase (it went back to seat 0, whose buttons then stood where the acting seat's had been). A draft pick hands
      // the next pick round to the first seat.
      const left = d.action.type === 'draft' ? null : await page.js(`(() => { const s = ${d.action.seat}; if (aic.view.choice || aic.view.outcome || aic.view.round + ':' + (aic.view.phase?.name ?? '') !== ${JSON.stringify(before)}) return null;
        return aic.legal.some((a) => a.seat === s) && aic.focus !== s ? 'seat ' + s + ' still has a move, the view shows seat ' + aic.focus : null; })()`);
      if (left) note(st.round, [`focus-left-seat: ${left}`]);
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
    const RULES = ['tippedAmbassador', 'liberateOffered', 'liberateBill', 'liberateBlocked', 'nodePowers', 'nodePowerUsed', 'cardBeat', 'incomeBeat', 'phaseBoard', 'pastRows',
      'strikeAhead', 'redeployAhead', 'troops', 'garrison', 'researchRow', 'researchK', 'roundCap', 'deckLeft', 'discord', 'unity'];
    const failures = [...[...issues].map(([x, r]) => `round ${r}: ${x}`), ...wrong.values()];
    if (!end.outcome) failures.push('no outcome');
    // 320: the bar for Matt's playthrough: a turn step shown before the board after it shows that step's rows.
    if (seen.pastRows === undefined && !wrong.has('pastRows')) failures.push('no earlier turn step showed rows that differ from the board after the turn');
    if (page.consoleErrors.length) failures.push(`${page.consoleErrors.length} console errors`);
    const summary = {
      how: 'every move clicked in the player view (web/scripts/playthrough.mjs)', config: file, seed, size: `${width}x${height}`,
      end: end.shown, outcome: end.outcome, rounds: end.round, actions: end.actions, view: end.view, header: end.header,
      heapMB: heap ? Math.round(heap.usedSize / 1e6) : null,
      seen, unreached: RULES.filter((k) => seen[k] === undefined && !wrong.has(k)), failures: failures.slice(0, 60), failureCount: failures.length,
      looks: [...looks].slice(0, 60).map(([x, r]) => `round ${r}: ${x}`),
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
