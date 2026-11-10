// The viewer sweep: plays and watches many games across configs, variants and seeds at
// test speed, screenshots only the moments that matter (a new round, a phase change, a
// new choice, a project or Alignment tip, the end) and checks the page at
// each of them. See web/README.md, *The sweep*.
//   node scripts/sweep.mjs [--quick] [--case NAME]...
//   AIC_URL       viewer URL (default http://localhost:5173/); serve a snapshot
//                 (vite preview), not the dev server, while the engine is edited
//   AIC_SIM       aic-sim binary (default ../target/release/aic-sim[.exe])
//   AIC_SWEEP_OUT output folder (default web/sweep-out/, git-ignored)
//   AIC_SWEEP_SEEDS games per case and mode (default 3; --quick 1)
// For each case, aic-sim plays games (greedy, or the case's agent) and saves them as records; the sweep then
// watches each record step by step (realistic games: wins, purges, late rounds) and
// plays as many fresh games of the same resolved config by picking legal actions at
// random through the page's own `act` (choices, hex picks, passes, undo). Exit code 1
// if any hard check failed. Writes issues.json, index.html and contact sheets
// (sheet-N.png: thumbnails of every capture with its moment and issues).

import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { openPage, viewerUrl as url } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const repo = resolve(web, '..');
const args = process.argv.slice(2);
const quick = args.includes('--quick');
const only = args.flatMap((a, i) => (args[i - 1] === '--case' ? [a] : []));
const out = resolve(process.env.AIC_SWEEP_OUT ?? join(web, 'sweep-out'));
const games = Number(process.env.AIC_SWEEP_SEEDS ?? (quick ? 1 : 3));
/** First seed of each case's simulated games (099: a hunt names its seed range). */
const firstSeed = Number(process.env.AIC_SWEEP_SEED ?? 100);
const sim = process.env.AIC_SIM ?? join(repo, 'target', 'release', process.platform === 'win32' ? 'aic-sim.exe' : 'aic-sim');

/** Cases: a config file and `--set` overrides. The quick run keeps the first four. */
const CASES = [
  { name: 'c2-d0', config: 'baseline.json', set: [] },
  { name: 'c4-d0', config: 'baseline.json', set: ['countries=4'] },
  { name: 'c1-d4', config: 'baseline.json', set: ['countries=1', 'adversary.difficulty=4'] },
  { name: 'shared-row', config: 'shared_row.json', set: ['countries=3'] },
  { name: 'c3-d2', config: 'baseline.json', set: ['countries=3', 'adversary.difficulty=2'] },
  { name: 'c2-d4', config: 'baseline.json', set: ['adversary.difficulty=4'] },
  { name: 'threats-off', config: 'baseline.json', set: ['threats.variant=off'] },
  { name: 'alignment-off', config: 'baseline.json', set: ['alignment.variant=off'] },
  { name: 'alignment-no-hold-off', config: 'baseline.json', set: ['alignment.hold_off=0', 'alignment.momentum_every=2'] },
  { name: 'cards-off', config: 'baseline.json', set: ['cards.variant=off'] },
  // 044: the round cap at each difficulty's deck, the engine's fuller games (markers,
  // shields, Clear, the rich action, delays), a record that overlays content.
  { name: 'c2-d1-engine', config: 'baseline.json', set: ['adversary.difficulty=1'], agent: 'engine' },
  { name: 'c2-d2-engine', config: 'baseline.json', set: ['adversary.difficulty=2'], agent: 'engine' },
  { name: 'c3-d3-engine', config: 'baseline.json', set: ['countries=3', 'adversary.difficulty=3'], agent: 'engine' },
  { name: 'overlay', config: 'baseline.json', set: ['cards.telemetry_archive.cost=9', 'threats.*.counters=[[{"resource":"credits","amount":1}]]'] },
  // 099: r35-r38's table: stations to free, research cubes and converters, liberations and
  // their twists, dark cities, the gift buildings and the one-shot Actions (engine games
  // play the most cards), and the three boards' slots at 3 and 4 countries.
  { name: 'c2-d0-engine', config: 'baseline.json', set: [], agent: 'engine' },
  { name: 'c3-d0-engine', config: 'baseline.json', set: ['countries=3'], agent: 'engine' },
  { name: 'c4-d2-engine', config: 'baseline.json', set: ['countries=4', 'adversary.difficulty=2'], agent: 'engine' },
  { name: 'cubes-3', config: 'baseline.json', set: ['countries=3', 'board.station_cubes=3'], agent: 'engine' },
  // 118: r41-r46's engines (triggers, At N, reuse, discounts) as the strategist plays them, and
  // the 1-player game's "another player means you" with the engine agent.
  { name: 'c2-d2-strategist', config: 'baseline.json', set: ['adversary.difficulty=2'], agent: 'engine:strategy=1' },
  { name: 'c1-d2-engine', config: 'baseline.json', set: ['countries=1', 'adversary.difficulty=2'], agent: 'engine' },
  // 145: r55-r59's pieces: claims, liberations and Envoy, Corrupt X on land at 4 countries,
  // war every round, d3's dearer menus and harsher harms, as the strategist and engine play them.
  { name: 'c2-d3-strategist', config: 'baseline.json', set: ['adversary.difficulty=3'], agent: 'engine:strategy=1' },
  { name: 'c4-d3-engine', config: 'baseline.json', set: ['countries=4', 'adversary.difficulty=3'], agent: 'engine' },
  // 169: r70-r72's moving pieces (gather, push, Patrol, standing liberations) and fear cards
  // (prohibitions, remove_where): the strategist at 3 countries seats the Protector beside
  // the Researcher, and d1's long games tip the most Unity cards.
  { name: 'c3-d1-strategist', config: 'baseline.json', set: ['countries=3', 'adversary.difficulty=1'], agent: 'engine:strategy=1' },
  { name: 'c2-d1-strategist', config: 'baseline.json', set: ['adversary.difficulty=1'], agent: 'engine:strategy=1' },
];
const unknown = only.filter((n) => !CASES.some((c) => c.name === n));
if (unknown.length) throw new Error(`no case ${unknown.join(', ')}; cases: ${CASES.map((c) => c.name).join(', ')}`);
const cases = CASES.filter((c, i) => (only.length ? only.includes(c.name) : !quick || i < 4));

/** Viewport sizes: the first is used for play, the others are checked at every capture. */
const SIZES = [[1366, 768], [1920, 1080], [2560, 1440]];
/** Checks that fail the run; the rest are reported for a look. */
const HARD = new Set(['console', 'stuck', 'error-banner', 'page-scroll', 'placeholder', 'offscreen', 'hidden-target', 'case', 'clipped', 'token-unknown', 'stale-word']);
/** One act or seek slower than this (ms, render included) is a frame-time spike. */
const SPIKE_MS = 150;
/** Captures per game, so a long game can't flood the sheets. */
const MAX_SHOTS = 14;

if (!existsSync(sim)) throw new Error(`no aic-sim at ${sim}: cargo build --release -p aic-sim, or set AIC_SIM`);
rmSync(out, { recursive: true, force: true });
mkdirSync(join(out, 'shots'), { recursive: true });

// ---------- in the page ----------

/** Installed once per page load: moment detection and the layout checks. */
const PAGE_LIB = String.raw`(() => {
  const aic = window.aic;
  const rnd = (s) => () => ((s = (s * 1103515245 + 12345) >>> 0) / 4294967296);
  const sig = (v) => ({ round: v.round, phase: v.phase ? v.phase.index : -1, choice: v.choice ? v.choice.prompt.replace(/\d+/g, '#') : null,
    outcome: v.outcome ? v.outcome.reason : null });
  /** What an engine event means for the table, if it is a key one (events are generic:
   *  tokens_changed, var_changed, card_moved, note; 020: matching on type never fired). */
  function keyEvent(e) {
    const up = (e.delta ?? 0) > 0;
    if (e.type === 'var_changed' && up && /^alignment\.(unity|discord)_tips$/.test(e.key)) return 'tip: ' + e.key.split('.')[1].replace('_tips', '');
    if (e.type === 'card_moved' && e.to === 'adversary.projects_done') return 'project resolved';
    if (e.type === 'card_moved' && /^alignment\.resolved/.test(e.to)) return 'alignment card';
    if (e.type === 'tokens_changed' && e.token === 'botnet') return up ? 'botnet made' : 'botnet removed';
    if (e.type === 'tokens_changed' && e.token === 'cube:adversary' && up) return 'corruption';
    if (e.type === 'tokens_changed' && e.token === 'troop' && up) return 'troop';
    // 044: r19 and r20's table: markers, shields on nodes, delays, the rich action.
    if (e.type === 'tokens_changed' && e.token === 'corruption') return up ? 'marker placed' : 'marker cleared';
    if (e.type === 'tokens_changed' && e.token === 'harden' && up) return 'hardened';
    if (e.type === 'var_changed' && /^threats\.late\./.test(e.key)) return 'threat waits';
    if (e.type === 'var_changed' && e.key === 'adversary.stage' && up) return 'stage ' + e.total;
    if (e.type === 'note' && / destroyed$/.test(e.text ?? '')) return 'destruction';
    return null;
  }
  /** 099: r35-r38's rules show in the engine's counters, not in its events: a counter
   *  that rose in the step names a key moment (a station freed, a liberation and its
   *  twist, a dark city, a converter, a one-shot Action discarded, troops removed). */
  const COUNTER_EVENTS = /^(board\.(liberated_[a-z]+|city_dark(_aligned)?|cubes_converted)|adversary\.station_flip_held|cards\.(once_discarded|free_built|remove_troops\.\d+))$/;
  // 118: r41-r46's engine wording and insight economy: a trigger, an At N cash-in, a reuse, a
  // discard from hand, a discount, insight bought, given or paid for a counter.
  const COUNTER_EVENTS_R41 = /^(cards\.(triggers|thresholds|reused|hand_discarded|discount_credits|insight_bought)|threats\.counter_paid\.insight)$/;
  // 145: r55-r59's sites, war, Corrupt X and menus: a claim or liberation by site, Envoy, a
  // hold-off, a node lost by cause, war's destroys, a corruption by site, a created site, a
  // top-tier counter by currency (tier 3 until 146, tier 2 since). (144's blackout and sabotage flags and the "is freed" note are gone.)
  const COUNTER_EVENTS_R55 = /^(board\.(claimed|liberated)\.(city|station|factory)|board\.(std\.envoy|cities_freed|war_destroyed)|board\.node_lost\.[a-z0-9]+|adversary\.(corruptions\.(city|station|factory|land)|sites_created\.(station|factory)|factories_seized|stations_flipped)|alignment\.(unity|discord)_held_off|alignment\.unity_pulled|threats\.(countered_by\.t2\.[a-z_]+|dealt_at_node))$/;
  // 169: r70-r71's moving pieces and fear cards: a gather, a push, an arrival that cancels, a
  // gather-then-strike, a liberation spending standing ambassadors, a prohibition made or fired,
  // a remove_where sweep and a cleanse.
  const COUNTER_EVENTS_R70 = /^(effects\.(gathered|pushed|moved_canceled|struck|cleansed|firewalls|remove_where\..+)|board\.liberate_standing|prohibit\.[a-z]+\.[a-z_]+\.(made|fired))$/;
  const counters = () => { const c = aic.timeline.frame.snap.state.stats?.counters ?? {}; return Array.isArray(c) ? Object.fromEntries(c) : c; };
  function counterEvents(c0, c1) {
    return Object.keys(c1).filter((k) => (COUNTER_EVENTS.test(k) || COUNTER_EVENTS_R41.test(k) || COUNTER_EVENTS_R55.test(k) || COUNTER_EVENTS_R70.test(k)) && (c1[k] ?? 0) > (c0[k] ?? 0)).map((k) => 'count: ' + k);
  }
  /** Why the step from a to b is worth a capture, or null. */
  function moment(a, b, events, extra = []) {
    // 044: every new key event of the step is seen and named, whatever else the step is.
    const fresh = [...new Set([...events.map(keyEvent), ...extra].filter((k) => k && !seenEvents.has(k)))];
    fresh.forEach((k) => seenEvents.add(k));
    const ev = fresh.length ? ' event: ' + fresh.join(', ') : '';
    // 099: with what was new in the last step (it was marked seen and left off).
    if (b.outcome && !a.outcome) return 'end: ' + b.outcome + ev;
    // 044: the adversary's whole turn lands in that one step: say what was new in it.
    if (b.round !== a.round) return 'round ' + b.round + ev;
    if (b.choice && b.choice !== a.choice && !seenChoices.has(b.choice)) { seenChoices.add(b.choice); return 'choice: ' + b.choice.slice(0, 60) + ev; }
    if (ev) return ev.trim();
    if (b.phase !== a.phase && !seenPhases.has(b.round + ':' + b.phase)) { seenPhases.add(b.round + ':' + b.phase); return b.round <= 1 ? 'phase ' + b.phase : null; }
    return null;
  }
  let seenChoices = new Set(), seenEvents = new Set(), seenPhases = new Set();
  const reset = () => { seenChoices = new Set(); seenEvents = new Set(); seenPhases = new Set(); };
  function timed(f) { const t = performance.now(); f(); return performance.now() - t; }

  /** Plays random legal actions until a moment, the end, or maxSteps; returns what happened. */
  function playUntilMoment(seed, maxSteps) {
    const r = window.__aicRnd ??= rnd(seed);
    const times = [];
    for (let i = 0; i < maxSteps; i++) {
      const before = sig(aic.view);
      if (aic.view.outcome) return { done: true, times };
      const legal = aic.legal;
      if (!legal.length) return { stuck: true, times };
      // Mostly act; pass only now and then, so games reach the late rounds.
      const nonPass = legal.filter((a) => a.type !== 'pass');
      const pool = nonPass.length && r() < 0.85 ? nonPass : legal;
      const a = pool[Math.floor(r() * pool.length)];
      const c0 = counters();
      times.push(timed(() => aic.act(a)));
      if (aic.error) return { error: aic.error, action: a, times };
      // Now and then: undo and act again (the undo path), never at the end.
      if (r() < 0.01 && !aic.view.outcome) { times.push(timed(() => document.getElementById('undo').click())); times.push(timed(() => aic.act(a))); }
      const after = sig(aic.view);
      const m = moment(before, after, aic.timeline.frame.events, counterEvents(c0, counters()));
      if (m) return { moment: m, times };
    }
    return { times };
  }
  /** Steps a loaded record forward until a moment or its end. */
  function watchUntilMoment(maxSteps) {
    const times = [];
    for (let i = 0; i < maxSteps; i++) {
      const tl = aic.timeline;
      if (tl.atEnd) return { done: true, times };
      const before = sig(aic.view);
      const c0 = counters();
      times.push(timed(() => aic.seek(tl.cursor + 1)));
      const m = moment(before, sig(aic.view), aic.timeline.frame.events, counterEvents(c0, counters()));
      if (m) return { moment: m, times };
    }
    return { times };
  }

  // ---------- checks ----------
  const vis = (el) => { const s = getComputedStyle(el); const r = el.getBoundingClientRect();
    return s.visibility !== 'hidden' && s.display !== 'none' && r.width > 0 && r.height > 0; };
  const label = (el) => { const id = el.id ? '#' + el.id : ''; const c = typeof el.className === 'string' && el.className ? '.' + el.className.trim().split(/\s+/).join('.') : '';
    return el.tagName.toLowerCase() + id + c + ' "' + (el.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 50) + '"'; };
  const scrollParent = (el) => { for (let p = el.parentElement; p; p = p.parentElement) { const o = getComputedStyle(p); if (/(auto|scroll)/.test(o.overflow + o.overflowX + o.overflowY)) return p; } return null; };
  const inside = (r, b) => r.left >= b.left - 1 && r.right <= b.right + 1 && r.top >= b.top - 1 && r.bottom <= b.bottom + 1;
  /** Words on screen that are code, not text: unfilled values and raw ids. */
  const PLACEHOLDER = /\b(undefined|NaN|null|\[object Object\]|TODO|FIXME)\b|\?\?/;
  const RAW_ID = /\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b/;
  // The words docs/spelling.rs refuses in source (STALE there), plus the engine's labels it can't read.
  const STALE = /adversary(?:'s)? ambassadors?|enemy ambassadors?|adv\. ambassadors?|agents token|Matt to confirm|Security agents?\b|on board: \d+ ambassadors?/i;
  const TOKENS = new Set(['cube', 'station', 'factory', 'node', 'harden', 'unit', 'ambassador', 'corruption', 'land_corruption', 'botnet', 'troop', 'firewall']);

  function checks() {
    const issues = [];
    const add = (kind, what) => issues.push({ kind, what });
    const de = document.documentElement;
    if (de.scrollHeight > innerHeight || de.scrollWidth > innerWidth || document.body.scrollHeight > innerHeight || document.body.scrollWidth > innerWidth)
      add('page-scroll', 'page scrolls: ' + de.scrollWidth + 'x' + de.scrollHeight + ' in ' + innerWidth + 'x' + innerHeight);
    if (aic.error) add('error-banner', aic.error.slice(0, 200));
    if (!aic.view.outcome && !aic.legal.length) add('stuck', 'no legal action and no outcome, round ' + aic.view.round + ' phase ' + (aic.view.phase && aic.view.phase.name));
    const ui = document.querySelectorAll('#top, #source, #board-pane *, #seats *, #piles *, #timeline *');
    const view = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    for (const el of ui) {
      if (!(el instanceof HTMLElement) || !vis(el)) continue;
      const r = el.getBoundingClientRect();
      const sp = scrollParent(el);
      // Off the screen with nothing to scroll it into view.
      if (!sp && !inside(r, view) && el.closest('#data') === null) add('offscreen', label(el) + ' at ' + Math.round(r.left) + ',' + Math.round(r.top) + '-' + Math.round(r.right) + ',' + Math.round(r.bottom));
      const s = getComputedStyle(el);
      const own = [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim());
      if (/(hidden|clip)/.test(s.overflow + s.overflowX + s.overflowY) && (el.scrollWidth > el.clientWidth + 2 || el.scrollHeight > el.clientHeight + 2)) {
        const ell = s.textOverflow === 'ellipsis' || s.webkitLineClamp !== 'none';
        const hover = !!el.closest('[title],[data-card],[data-value],[data-pile],[data-track],[data-board-seat],[data-action]');
        add(ell ? (hover ? 'truncated' : 'truncated-no-hover') : 'clipped', label(el) + ' ' + el.scrollWidth + 'x' + el.scrollHeight + ' in ' + el.clientWidth + 'x' + el.clientHeight);
      }
      if (own) {
        const t = [...el.childNodes].filter((n) => n.nodeType === 3).map((n) => n.textContent).join(' ');
        if (PLACEHOLDER.test(t)) add('placeholder', label(el));
        // #source names the config file and the config's own name (shared_row): not a raw id.
        // (No backticks in here: this code sits inside a template string.)
        if (!el.closest('code, pre, #data, #source') && RAW_ID.test(t)) add('raw-id', label(el) + ' (' + t.match(RAW_ID)[0] + ')');
        // The engine's hex names (C1 #13) are shown with the country's name (020).
        if (!el.closest('code, pre, #data') && /\bC\d+ #\d+/.test(t)) add('raw-id', label(el) + ' (engine hex name)');
        // 169: the house words (r70, amendment 7657): the adversary's pieces are agents, the
        // Security token Operatives; an old name or an open naming note is a stale word.
        if (!el.closest('code, pre, #data') && STALE.test(t)) add('stale-word', label(el) + ' (' + t.match(STALE)[0] + ')');
      }
    }
    // Buttons: each one on screen must be the element under its own center (not covered).
    const buttons = [...document.querySelectorAll('button')].filter((b) => vis(b) && !b.closest('dialog'));
    for (const b of buttons) {
      const r = b.getBoundingClientRect();
      const sp = scrollParent(b);
      // A scroll pane's client area (no border, no scrollbar): a button half scrolled out
      // at its edge is not covered, just scrolled (the 020 full sweep's two false alarms).
      const pr = sp && sp.getBoundingClientRect();
      const box = sp ? { left: pr.left + sp.clientLeft, top: pr.top + sp.clientTop, right: pr.left + sp.clientLeft + sp.clientWidth, bottom: pr.top + sp.clientTop + sp.clientHeight } : view;
      const cx = (r.left + r.right) / 2, cy = (r.top + r.bottom) / 2;
      if (cx < box.left + 2 || cx > box.right - 2 || cy < box.top + 2 || cy > box.bottom - 2 || cx > innerWidth || cy > innerHeight) continue; // scrolled away
      const hit = document.elementFromPoint(cx, cy);
      if (!hit || !(hit === b || b.contains(hit))) add('hidden-target', label(b) + ' under ' + (hit ? label(hit) : 'nothing'));
    }
    // 118: each button as it shows, clipped to its scroll pane: one scrolled out of the seat
    // pane sat "over" the toolbar above it (925 false overlaps once a choice scrolled the pane).
    const shown = (b) => {
      const r = b.getBoundingClientRect();
      const sp = scrollParent(b);
      if (!sp) return r;
      const pr = sp.getBoundingClientRect();
      const left = Math.max(r.left, pr.left + sp.clientLeft), top = Math.max(r.top, pr.top + sp.clientTop);
      const right = Math.min(r.right, pr.left + sp.clientLeft + sp.clientWidth), bottom = Math.min(r.bottom, pr.top + sp.clientTop + sp.clientHeight);
      return { left, top, right: Math.max(left, right), bottom: Math.max(top, bottom) };
    };
    for (let i = 0; i < buttons.length; i++) for (let j = i + 1; j < buttons.length; j++) {
      const a = shown(buttons[i]), c = shown(buttons[j]);
      const w = Math.min(a.right, c.right) - Math.max(a.left, c.left), h = Math.min(a.bottom, c.bottom) - Math.max(a.top, c.top);
      if (w > 2 && h > 2) add('overlap', label(buttons[i]) + ' and ' + label(buttons[j]));
    }
    // The board: hex numbers and token counts must not sit on each other (the big
    // country names are drawn faint across the hexes on purpose, so they are left out).
    const texts = [...document.querySelectorAll('#board text.num, #board text.count')].filter(vis).map((t) => ({ t, r: t.getBoundingClientRect() }));
    for (let i = 0; i < texts.length; i++) for (let j = i + 1; j < texts.length; j++) {
      const a = texts[i].r, c = texts[j].r;
      const w = Math.min(a.right, c.right) - Math.max(a.left, c.left), h = Math.min(a.bottom, c.bottom) - Math.max(a.top, c.top);
      if (w > 1.5 && h > 1.5) add('board-text-overlap', label(texts[i].t) + ' and ' + label(texts[j].t));
    }
    // Every token in the view is drawn.
    for (const h of aic.view.hexes) for (const tok of h.tokens) {
      if (tok.count > 0 && !document.querySelector('#board [data-hex="' + h.hex.join(',') + '"][data-token="' + tok.kind + '"]'))
        add('token-missing', tok.kind + ' x' + tok.count + ' on hex ' + h.hex.join(','));
    }
    // 044: every token has its own glyph and name (r19's marker fell to a hashed-color disc).
    for (const h of aic.view.hexes) for (const tok of h.tokens)
      if (!TOKENS.has(tok.kind.split(':')[0])) add('token-unknown', tok.kind + ' has no glyph');
    // 099: input is accepted: every legal action has a control on screen (a button with
    // its action, or a hex pick naming its hex), so no stretch of play has a move the
    // page can't take.
    const controls = [...document.querySelectorAll('[data-action]')].filter(vis);
    const exact = new Set(controls.map((el) => el.dataset.action));
    const hexes = new Set(controls.flatMap((el) => (el.dataset.hexes ?? '').split(' ').filter(Boolean)));
    const isHex = (t) => Array.isArray(t) && t.length === 2 && t.every(Number.isInteger);
    for (const a of aic.legal) {
      if (exact.has(JSON.stringify(a)) || (isHex(a.target) && hexes.has(a.target.join(',')))) continue;
      add('no-control', 'no control for ' + JSON.stringify(a).slice(0, 160));
    }
    // Arrows (019) point between hexes that exist.
    for (const a of aic.view.arrows ?? []) if (!aic.view.hexes[a.from] || !aic.view.hexes[a.to]) add('arrow', JSON.stringify(a));
    return issues;
  }
  window.__aicSweep = { playUntilMoment, watchUntilMoment, checks, reset };
  return true;
})()`;

// ---------- in node ----------

const issues = [];
const captures = [];
const spikes = [];
const heap = [];
let shotNo = 0;

function simRecords(c) {
  const dir = join(out, 'records', c.name);
  const r = spawnSync(sim, ['run', join(repo, 'configs', c.config), '--games', String(games), '--seed', String(firstSeed),'--agent', c.agent ?? 'greedy',
    ...c.set.flatMap((s) => ['--set', s]), '--save', dir, '--save-when', 'all', '--threads', '4'], { encoding: 'utf8' });
  if (r.status !== 0) throw new Error(`aic-sim failed for ${c.name}: ${r.stderr || r.stdout}`);
  return readdirSync(dir).filter((f) => f.endsWith('.json')).sort().map((f) => readFileSync(join(dir, f), 'utf8'));
}

const verbose = !!process.env.AIC_SWEEP_VERBOSE;

async function capture(page, game, what) {
  const tc = Date.now();
  const found = await page.js('window.__aicSweep.checks()');
  if (verbose) console.log(`  ${game.name} ${what}: checks ${Date.now() - tc} ms`);
  // The other layouts: checked, not shot.
  for (const [w, h] of SIZES.slice(1)) {
    await page.size(w, h);
    for (const i of await page.js('window.__aicSweep.checks()')) if (/page-scroll|offscreen|hidden-target|overlap/.test(i.kind)) found.push({ ...i, what: `${w}x${h}: ${i.what}` });
  }
  await page.size(...SIZES[0]);
  const errs = page.consoleErrors.splice(0);
  for (const e of errs) found.push({ kind: 'console', what: e.slice(0, 300) });
  const step = await page.js('window.aic.timeline.cursor');
  const file = `shots/${String(++shotNo).padStart(4, '0')}.png`;
  // The end, a stuck game, an error or a failing check is always shot, past the cap
  // (020: the cap used to eat every game's end screen).
  // 099: and the first rise of an r35-r38 counter: they come late, after the cap, so a
  // 20-game sweep shot no liberation, freed station or converted cube.
  const must = /^(end|stuck|error)|event: .*(marker|threat waits|count: )/.test(what) || found.some((i) => HARD.has(i.kind));
  const shoot = must || game.shots < MAX_SHOTS;
  if (shoot) {
    game.shots++;
    writeFileSync(join(out, file), await page.shot());
  }
  if (verbose) console.log(`  capture done in ${Date.now() - tc} ms`);
  const cap = { game: game.name, step, what, file: shoot ? file : null, issues: found };
  captures.push(cap);
  for (const i of found) issues.push({ ...i, game: game.name, step, moment: what });
}

async function drive(page, game, kind, seed) {
  await page.js('window.__aicSweep.reset()');
  await capture(page, game, 'start');
  for (let guard = 0; guard < 400; guard++) {
    const r = kind === 'play'
      ? await page.js(`window.__aicSweep.playUntilMoment(${seed}, 3000)`)
      : await page.js('window.__aicSweep.watchUntilMoment(3000)');
    if (verbose) console.log(`  ${game.name}: ${r.times.length} steps, ${r.times.reduce((a, b) => a + b, 0).toFixed(0)} ms -> ${r.moment ?? (r.done ? 'end' : r.stuck ? 'stuck' : r.error ? 'error' : '?')}`);
    for (const t of r.times) if (t > SPIKE_MS) spikes.push({ game: game.name, ms: Math.round(t) });
    game.times.push(...r.times);
    if (r.error) { issues.push({ kind: 'error-banner', what: `${r.error} after ${JSON.stringify(r.action)}`, game: game.name }); await capture(page, game, 'error'); return; }
    if (r.stuck) { await capture(page, game, 'stuck'); return; }
    if (r.done) { await capture(page, game, 'end'); return; }
    if (r.moment) await capture(page, game, r.moment);
    if (r.moment?.startsWith('end')) return; // shot once, not again as 'end'
  }
}

const page = await openPage({ width: SIZES[0][0], height: SIZES[0][1] });
const t0 = Date.now();
try {
  await page.size(...SIZES[0]);
  await page.send('Page.navigate', { url });
  await page.until('window.aic && window.aic.view && window.aic.play', 'the viewer to start', 30000);
  await page.js('localStorage.clear()');
  await page.js(PAGE_LIB);
  for (const c of cases) {
    let records;
    try {
      records = simRecords(c);
    } catch (e) {
      issues.push({ kind: 'case', what: String(e).slice(0, 300), game: c.name });
      continue;
    }
    for (const [i, text] of records.entries()) {
      const rec = JSON.parse(text);
      const watch = { name: `${c.name}/watch-${rec.seed}`, shots: 0, times: [] };
      await page.js(`window.aic.open(${JSON.stringify(text)}, ${JSON.stringify(`${c.name}-${rec.seed}.json`)})`);
      await drive(page, watch, 'watch');
      const seed = 1000 + i;
      const play = { name: `${c.name}/play-${seed}`, shots: 0, times: [] };
      await page.js(`(window.__aicRnd = null, window.aic.play(${JSON.stringify(JSON.stringify(rec.config))}, '${seed}', '${c.name}'))`);
      await drive(page, play, 'play', seed);
      for (const g of [watch, play]) {
        const sorted = [...g.times].sort((a, b) => a - b);
        g.p50 = sorted[Math.floor(sorted.length / 2)] ?? 0;
        g.max = sorted[sorted.length - 1] ?? 0;
        const caps = captures.filter((c) => c.game === g.name).length;
        console.log(`${g.name}: ${g.times.length} steps, ${caps} captures (${g.shots} shots), step p50 ${g.p50.toFixed(1)} ms, max ${g.max.toFixed(0)} ms`);
      }
      await page.send('HeapProfiler.collectGarbage');
      const h = await page.send('Runtime.getHeapUsage');
      heap.push({ after: play.name, mb: +(h.usedSize / 1e6).toFixed(1) });
    }
  }
} finally {
  await page.close();
}

// ---------- report ----------

// Memory first, so it reaches the summary (020: it was added after the summary).
const growth = heap.length > 2 ? heap[heap.length - 1].mb - heap[1].mb : 0;
if (growth > 50) issues.push({ kind: 'memory', what: `heap grew ${growth.toFixed(0)} MB across ${heap.length} game pairs` });
const byKind = {};
for (const i of issues) (byKind[i.kind] ??= []).push(i);
writeFileSync(join(out, 'issues.json'), JSON.stringify({ issues, spikes, heap }, null, 1));

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
const cards = captures.filter((c) => c.file).map((c) => `<figure class="${c.issues.some((i) => HARD.has(i.kind)) ? 'bad' : ''}"><img src="${c.file}"><figcaption><b>${esc(c.game)}</b> #${c.step} ${esc(c.what)}${c.issues.length ? `<br><i>${esc([...new Set(c.issues.map((i) => i.kind))].join(', '))}</i>` : ''}</figcaption></figure>`);
const css = 'body{margin:0;font:12px sans-serif;background:#fff}main{display:grid;grid-template-columns:repeat(4,400px);gap:6px;padding:6px}figure{margin:0;border:2px solid #ddd}figure.bad{border-color:#d33}img{width:100%;display:block}figcaption{padding:2px 4px;height:3.2em;overflow:hidden}';
const PER_SHEET = 24;
for (let s = 0; s * PER_SHEET < cards.length; s++) {
  writeFileSync(join(out, `sheet-${s + 1}.html`), `<!doctype html><style>${css}</style><main>${cards.slice(s * PER_SHEET, (s + 1) * PER_SHEET).join('')}</main>`);
}
writeFileSync(join(out, 'index.html'), `<!doctype html><style>${css}</style><h1>Viewer sweep</h1><pre>${esc(JSON.stringify(Object.fromEntries(Object.entries(byKind).map(([k, v]) => [k, v.length])), null, 1))}</pre><main>${cards.join('')}</main>`);
// Contact sheets as images, rendered by the same browser.
const sheetPage = await openPage({ width: 1630, height: 1200 });
try {
  for (let s = 0; s * PER_SHEET < cards.length; s++) {
    await sheetPage.send('Page.navigate', { url: pathToFileURL(join(out, `sheet-${s + 1}.html`)).href });
    await sheetPage.until('[...document.images].every((i) => i.complete)', 'the sheet images', 20000);
    const h = await sheetPage.js('document.documentElement.scrollHeight');
    await sheetPage.size(1630, h);
    writeFileSync(join(out, `sheet-${s + 1}.png`), await sheetPage.shot());
  }
} finally {
  await sheetPage.close();
}

console.log(`\n${captures.length} captures, ${issues.length} issues, ${spikes.length} spikes over ${SPIKE_MS} ms, ${((Date.now() - t0) / 1000).toFixed(0)} s; heap ${heap.map((h) => h.mb).join(' ')} MB`);
for (const [k, v] of Object.entries(byKind).sort()) {
  console.log(`${HARD.has(k) ? 'FAIL' : 'look'} ${k} x${v.length}`);
  for (const i of [...new Map(v.map((x) => [x.what, x])).values()].slice(0, 6)) console.log(`       ${i.game ?? ''} #${i.step ?? ''} ${i.what}`);
}
console.log(`\nwrote ${out} (index.html, sheet-*.png, issues.json)`);
process.exit(issues.some((i) => HARD.has(i.kind)) ? 1 : 0);
