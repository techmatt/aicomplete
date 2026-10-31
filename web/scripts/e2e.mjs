// End-to-end check of the viewer in a real browser (Edge or Chrome, headless, driven over
// the DevTools protocol; no extra npm packages). Needs the dev server running.
//   node scripts/e2e.mjs [record.json]
//   AIC_URL     viewer URL (default http://localhost:5173/)
//   AIC_BROWSER browser executable (default: Edge, then Chrome, at their usual paths)
//   AIC_SHOTS   a folder to also save the mid-game and end-of-game screenshots in
//               (viewer-mid.png, viewer-end.png)
// Plays a game to the end through the UI, checks undo, the record dialog, the saved
// session, hover details and the no-scroll layout at 1366x768, 1920x1080 and 2560x1440,
// then loads the record (if given) and steps through it. Screenshots go to web/e2e-out/.

import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { newGame, openPage, sleep } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const url = process.env.AIC_URL ?? 'http://localhost:5173/';
const recordPath = process.argv[2];
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const page = await openPage();
const { send, js, until, size, consoleErrors } = page;
let failures = 0;
const check = (ok, what) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
  if (!ok) failures++;
};

/** Reloads the page and waits for the new document's viewer (not the old one's) and `expr`. */
async function reload(expr, what) {
  await js('(window.__aicOld = true)');
  await send('Page.reload');
  await until(`!window.__aicOld && window.aic && window.aic.view && (${expr})`, what, 30000);
}
/** Saves a screenshot to web/e2e-out/, and under `copy` in AIC_SHOTS too if that is set. */
async function shot(name, copy) {
  const png = await page.shot();
  writeFileSync(join(out, `${name}.png`), png);
  if (copy && process.env.AIC_SHOTS) {
    mkdirSync(process.env.AIC_SHOTS, { recursive: true });
    writeFileSync(join(process.env.AIC_SHOTS, `${copy}.png`), png);
  }
}
const noScroll = () => js(`document.documentElement.scrollHeight <= innerHeight && document.documentElement.scrollWidth <= innerWidth
  && document.body.scrollHeight <= innerHeight && document.body.scrollWidth <= innerWidth`);

try {
  await size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await js('localStorage.clear()');
  await reload('window.aic.timeline.last === 0', 'a fresh game');

  // Board: one data-hex per view hex, in the view's coordinates.
  const board = await js(`(() => {
    const v = window.aic.view;
    const keys = [...document.querySelectorAll('#board .hex')].map((e) => e.dataset.hex);
    return { n: v.hexes.length, keys, want: v.hexes.map((h) => h.hex.join(',')),
             labels: document.querySelectorAll('#board .country-label').length, countries: v.countries.length,
             borders: document.querySelectorAll('#board .border').length,
             kinds: new Set(v.hexes.map((h) => h.kind)).size,
             fills: new Set([...document.querySelectorAll('#board .hex .cell')].map((p) => p.getAttribute('fill'))).size,
             bases: v.hexes.filter((h) => h.kind === 'base' && h.facing !== undefined).length,
             pennants: document.querySelectorAll('#board .hex.k-base .base-mark').length,
             fronts: v.globals.find((g) => g.key === 'board.fronts')?.value ?? -1,
             links: document.querySelectorAll('#board .front-link').length, edges: document.querySelectorAll('#board .front-edge').length };
  })()`);
  check(board.n > 0 && JSON.stringify(board.keys) === JSON.stringify(board.want), `board has ${board.n} hexes with data-hex="q,r"`);
  check(board.labels === board.countries && board.borders > 0, `${board.countries} countries labelled, ${board.borders} border edges`);
  check(board.kinds === 4 && board.fills === board.kinds, `hexes filled by kind (${board.fills} fills for ${board.kinds} kinds: city, base, open, ocean)`);
  check(board.bases > 0 && board.pennants === board.bases, `${board.pennants} military bases marked with the country they face`);
  check(board.fronts > 0 && board.links === board.fronts && board.edges > 0, `${board.links} fronts linked base to base, ${board.edges} front edges`);
  // Tokens: one glyph per stack, owner cubes and presence in their owner's colour.
  const toks = await js(`(() => {
    const v = window.aic.view;
    const stacks = v.hexes.reduce((n, h) => n + h.tokens.length, 0);
    const fill = (k) => [...document.querySelectorAll('#board .token[data-token="' + k + '"] .glyph')].map((e) => e.getAttribute('fill'))[0];
    const kinds = [...new Set(v.hexes.flatMap((h) => h.tokens.map((t) => t.kind)))].sort();
    return { stacks, drawn: document.querySelectorAll('#board .token').length, kinds,
             adv: fill('cube:adversary'), presence0: fill('presence:0'), presence1: fill('presence:1') };
  })()`);
  check(toks.stacks > 0 && toks.drawn === toks.stacks, `${toks.drawn} token stacks drawn (${toks.kinds.join(', ')})`);
  check(!!toks.adv && !!toks.presence0 && !!toks.presence1 && toks.presence0 !== toks.presence1 && toks.adv !== toks.presence0, `owner colours differ: adversary ${toks.adv}, seat 0 ${toks.presence0}, seat 1 ${toks.presence1}`);
  const panels = await js(`({ pipeline: document.querySelectorAll('#pipeline .slot-box').length, alignment: document.querySelectorAll('#status .alignment .tug .bar').length,
                             undecided: !!document.querySelector('#status .alignment .undecided'),
                             next: document.querySelector('#pipeline .slot-box.next .acard .atext')?.textContent ?? '',
                             supply: !!document.querySelector('[data-value="adversary.supply"]'),
                             projects: document.querySelectorAll('#countries .project .cubes').length, countries: window.aic.view.countries.length,
                             threats: document.querySelectorAll('#countries .threat').length })`);
  check(panels.pipeline >= 4 && panels.alignment === 2 && panels.undecided, `adversary row (${panels.pipeline} boxes) and Alignment track (${panels.alignment} buckets, the Undecided) drawn`);
  check(panels.next.length > 0 && panels.supply, `face-up next adversary card shown with its text ("${panels.next}") and the ambassador supply`);
  check(panels.projects === panels.countries, `${panels.projects} megaproject cards shown with their cubes`);
  check(panels.threats > 0, `${panels.threats} threats shown with their countries`);

  // Country names, the AI boards' tracks, and no per-seat bookkeeping among the globals.
  const named = await js(`(() => {
    const v = window.aic.view;
    const label = [...document.querySelectorAll('#board .country-label')].map((g) => g.textContent).join('|');
    const strip = [...document.querySelectorAll('#countries .chead > b')].map((b) => b.textContent);
    const boards = v.seats.map((s) => s.board ? s.board.tracks.map((t) => t.steps.length) : null);
    const rows = [...document.querySelectorAll('.seat .board')].map((b) => ({
      rows: b.querySelectorAll('.track-row').length, steps: b.querySelectorAll('.step').length,
      open: b.querySelectorAll('.step.open').length, current: b.querySelectorAll('.step.current').length,
      t2: b.querySelectorAll('.mark.m-tier2').length }));
    const keys = v.globals.map((x) => x.key);
    return { names: v.countries.map((c) => c.name), label, strip, boards, rows, keys,
             raw: document.querySelectorAll('#vars .value').length };
  })()`);
  check(named.names.every((n) => n && named.label.includes(n.split(' ')[0]) && named.strip.includes(n)),
    `countries named on the board and strip (${named.names.join(', ')})`);
  // 021: a board has two tracks (presence and the second presence track); at the start
  // only the first has an uncovered (and current) step.
  check(named.boards.every((b) => b && b.length >= 1) && named.rows.length === named.boards.length
    && named.rows.every((r, i) => r.rows === named.boards[i].length && r.current >= 1 && r.current <= r.rows && r.open >= 1 && r.steps > r.open && r.t2 > 0),
    `each seat's AI board drawn with its tracks (${named.rows.map((r) => `${r.rows} tracks, ${r.open}/${r.steps} uncovered`).join('; ')})`);
  const bookkeeping = /^(ai\.track\.|threats\.paid\.|threats\.jam\.|cards\.(tier|picked|next_discount|civic_used|unpowered)\.|adversary\.removed_troop\.|board\.(blackout|sabotage|shelter|project|project_done|track)\.)/;
  check(!named.keys.some((k) => bookkeeping.test(k)) && named.raw === 0,
    `no bookkeeping vars among the globals (${named.keys.length} globals)`);

  // Hover a hex and a token: the details card shows.
  const hover = async (sel) => {
    const pt = await js(`(() => { const r = document.querySelector('${sel}')?.getBoundingClientRect(); return r && [r.x + r.width / 2, r.y + r.height / 2]; })()`);
    if (!pt) return '';
    await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: pt[0], y: pt[1] });
    await sleep(50);
    return js(`document.getElementById('tip').hidden ? '' : document.getElementById('tip').textContent`);
  };
  check((await hover('#board .hex polygon')).includes('hex'), 'hovering a hex shows its details');
  check((await hover('#board .token .hit')).includes('×'), 'hovering a token shows its details');
  check((await hover('.value')).length > 0, 'hovering a value shows its details');
  check((await hover('#pipeline .slot-box.next .acard')).length > 0, 'hovering the next adversary card shows its text');
  check((await hover('#board .hex.k-base polygon')).includes('military base'), 'hovering a military base shows it as one');
  check((await hover('.seat .board .step')).includes('step'), 'hovering a track step shows its details');
  check((await hover('.seat .board-name')).length > 0, 'hovering a board name shows the board');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  await shot('play-start-1366');

  // Play the whole game through the UI: random clicks on what the seat panels offer
  // (any action button: growth and draft phases offer no pass). An open choice answered
  // by a hex must highlight its hexes without a click first.
  let clicks = 0;
  let hexChoices = 0;
  let armedAtOnce = 0;
  const shots = new Set();
  const seen = { standard: new Set(), sell: 0, war: 0, warShown: 0 };
  while (clicks < 3000 && !(await js(`!!document.querySelector('#phase-bar .outcome')`))) {
    const st = await js(`(() => {
      const v = window.aic.view;
      const hexPick = !!v.choice && window.aic.legal.some((a) => a.type === 'choose' && a.target);
      const war = v.globals.find((g) => g.key === 'board.war')?.value === 1;
      return { hexPick, targets: document.querySelectorAll('#board .hex.target').length, prompt: document.querySelector('#prompt.choice') ? 1 : 0,
               phase: v.phase?.name ?? '', round: v.round, war,
               warShown: war && !!document.querySelector('#pipeline .war-round') && !!document.querySelector('#board .fronts.war'),
               standard: [...document.querySelectorAll('#seats .actions button.a-standard')].map((b) => JSON.parse(b.dataset.action).act),
               sell: document.querySelectorAll('#seats .actions button.a-sell').length };
    })()`);
    if (st.hexPick) {
      hexChoices++;
      if (st.targets > 0 && st.prompt) armedAtOnce++;
      if (!shots.has('choice')) { shots.add('choice'); await shot('play-choice-1366'); }
    }
    for (const a of st.standard) seen.standard.add(a);
    seen.sell += st.sell ? 1 : 0;
    if (st.war) { seen.war++; seen.warShown += st.warShown ? 1 : 0; }
    if (st.round >= 2 && st.phase.includes('cards.play') && !shots.has('mid')) { shots.add('mid'); await shot('play-mid-1366', 'viewer-mid'); }
    if (st.phase === 'cards.draft' && !shots.has('draft')) { shots.add('draft'); await shot('play-draft-1366'); }
    if (st.phase === 'ai.growth' && !shots.has('growth')) { shots.add('growth'); await shot('play-growth-1366'); }
    const did = await js(`(() => {
      const tgt = document.querySelectorAll('#board .hex.target polygon');
      if (tgt.length) { tgt[Math.floor(Math.random() * tgt.length)].dispatchEvent(new MouseEvent('click', { bubbles: true })); return 'hex'; }
      const bs = [...document.querySelectorAll('#seats .actions button, #seats .pass-all')];
      if (!bs.length) return '';
      // Favour plays over passes so games reach the cards.
      const plays = bs.filter((b) => !b.classList.contains('pass') && !b.classList.contains('pass-all'));
      const pool = plays.length && Math.random() < 0.7 ? plays : bs;
      pool[Math.floor(Math.random() * pool.length)].click();
      return 'button';
    })()`);
    if (!did) break;
    clicks++;
  }
  const end = await js(`({ outcome: window.aic.view.outcome, n: window.aic.timeline.last, legal: window.aic.legal.length,
                          text: document.querySelector('#phase-bar .outcome')?.textContent })`);
  check(!!end.outcome && end.legal === 0, `played to the end in ${clicks} clicks, ${end.n} actions: ${end.text}`);
  const played = await js(`window.aic.timeline.actions.filter((a) => a.type !== 'pass').length`);
  check(played > 0, `${played} non-pass actions taken through the UI`);
  check(armedAtOnce === hexChoices, `hex choices highlight their hexes at once (${armedAtOnce}/${hexChoices})`);
  check(seen.standard.size > 0 && seen.sell > 0, `standard actions offered as buttons (${[...seen.standard].join(', ')}) and selling a card`);
  check(seen.war === 0 || seen.warShown === seen.war, `war rounds marked on the adversary row and fronts (${seen.warShown}/${seen.war} war-round steps)`);
  await shot('play-end-1366', 'viewer-end');

  // Layout: no page scroll at both ends of the supported range.
  for (const [w, h] of [[1366, 768], [1920, 1080], [2560, 1440]]) {
    await size(w, h);
    check(await noScroll(), `no page scroll at ${w}x${h}`);
    await shot(`layout-${w}x${h}`);
  }
  await size(1366, 768);

  // 020: the words on screen (engine hex names and content ids are shown by name), the
  // header's rules revision, and stepping a long game stays fast.
  const words = await js(`(() => {
    // Action lines and the step label for ids; every log line (notes too) for hex names.
    const text = [...document.querySelectorAll('#prompt, #seats button, #tab-log .head, #step')].map((e) => e.textContent).join(' ');
    const lines = [...document.querySelectorAll('#tab-log .head, #tab-log li')].map((e) => e.textContent).join(' ');
    const t = []; for (let i = 0; i < 20; i++) { const s = performance.now(); window.aic.seek(Math.floor(window.aic.timeline.last * i / 20)); t.push(performance.now() - s); }
    window.aic.seek(window.aic.timeline.last);
    return { place: (text + ' ' + lines).match(/\\bC\\d+ #\\d+/)?.[0] ?? null, id: text.match(/\\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\\b/)?.[0] ?? null,
      version: document.getElementById('version').textContent, seek: Math.max(...t) };
  })()`);
  check(!words.place && !words.id, `log, prompt and buttons name hexes and cards in words${words.place || words.id ? ` (saw ${words.place ?? words.id})` : ''}`);
  check(/rules r\d+/.test(words.version), `the header shows the rules revision (${words.version})`);
  check(words.seek < 100, `stepping through ${end.n} actions stays fast (slowest seek ${words.seek.toFixed(0)} ms)`);

  // 022 (D4, D7): Page Up / Page Down move by rounds, the speed menu, and the log's
  // per-phase groups outline their hexes on hover.
  const key = async (k, code) => {
    await send('Input.dispatchKeyEvent', { type: 'keyDown', key: k, code: k, windowsVirtualKeyCode: code });
    await send('Input.dispatchKeyEvent', { type: 'keyUp', key: k, code: k, windowsVirtualKeyCode: code });
  };
  await js(`(() => { document.activeElement?.blur(); window.aic.seek(0); })()`);
  await key('PageDown', 34);
  const r1 = await js(`(() => { const f = window.aic.timeline.frames, c = window.aic.timeline.cursor; return { c, r: f[c].snap.state.round, prev: c > 0 ? f[c - 1].snap.state.round : -1 }; })()`);
  await key('PageDown', 34);
  const r2 = await js(`(() => { const f = window.aic.timeline.frames, c = window.aic.timeline.cursor; return { c, r: f[c].snap.state.round }; })()`);
  await key('PageUp', 33);
  const r3 = await js('window.aic.timeline.cursor');
  check(r1.c > 0 && r1.r === r1.prev + 1 && r2.r === r1.r + 1 && r3 === r1.c,
    `Page Down jumps to the next round (step ${r1.c}, round ${r1.r}; then round ${r2.r}) and Page Up back (step ${r3})`);
  const speeds = await js(`[...document.querySelectorAll('#speed option')].map((o) => o.textContent).join(' ')`);
  check(speeds === '1x 4x 16x', `the autoplay speed menu offers 1x 4x 16x (${speeds})`);
  const groups = await js(`(() => {
    const g = [...document.querySelectorAll('#tab-log details.pgroup[data-hexes]')];
    const d = g[g.length - 1];
    if (!d) return { n: 0 };
    d.querySelector('summary').dispatchEvent(new MouseEvent('mousemove', { bubbles: true }));
    const want = d.dataset.hexes.split(' ');
    const hl = [...document.querySelectorAll('#board .hex.hl')].map((e) => e.dataset.hex);
    return { n: g.length, ok: want.every((h) => hl.includes(h)) && hl.length === want.length };
  })()`);
  check(groups.n > 0 && groups.ok, `the log groups automatic phases (${groups.n} groups with hexes) and a heading's hover outlines its hexes`);
  await js(`(() => { document.body.dispatchEvent(new MouseEvent('mousemove', { bubbles: true })); window.aic.seek(window.aic.timeline.last); })()`);

  // Undo by replay, and the saved session survives a reload.
  await js(`document.getElementById('undo').click()`);
  const afterUndo = await js('window.aic.timeline.last');
  check(afterUndo === end.n - 1 && !(await js('window.aic.view.outcome')), 'undo drops the last action and reopens the game');
  await reload('true', 'reload');
  check((await js('window.aic.timeline.last')) === afterUndo, 'the session is restored from localStorage after a reload');

  // Bug report: the record dialog holds the game in the record format.
  await js(`document.getElementById('record').click()`);
  const rec = await js(`JSON.parse(document.getElementById('record-text').value)`);
  check(rec.format === 'aic-game-record/1' && rec.actions.length === afterUndo && !!rec.config && /^\d+$/.test(rec.seed),
    `record dialog: ${rec.format}, seed ${rec.seed}, ${rec.actions.length} actions`);
  await shot('record-dialog');
  await js(`document.getElementById('record-close').click()`);

  // The viewer's own autoplay: a new baseline game, Random to end, reaches an outcome.
  await newGame(page, 'baseline.json', 5);
  await js(`document.getElementById('random-end').click()`);
  const auto = await js(`({ o: window.aic.view.outcome, n: window.aic.timeline.last, text: document.querySelector('#phase-bar .outcome')?.textContent })`);
  check(!!auto.o && !!auto.text, `Random to end plays baseline seed 5 to an outcome in ${auto.n} actions: ${auto.text}`);

  // The board's size on a fixed game (048): the random game's panels moved it by 60 px.
  // This one reads 350, 547 and 878 px high.
  await newGame(page, 'baseline.json', 11);
  for (let i = 0; i < 150 && !(await js('!!window.aic.view.outcome')); i++) await js('window.aic.act(window.aic.legal[0])');
  for (const [w, h] of [[1366, 768], [1920, 1080], [2560, 1440]]) {
    await size(w, h);
    const svg = await js(`(() => { const r = document.getElementById('board').getBoundingClientRect(); return [r.width, r.height]; })()`);
    check(svg[0] > w * 0.3 && svg[1] > h * 0.44, `board fills its pane at ${w}x${h} (${svg.map(Math.round).join('x')})`);
  }
  await size(1366, 768);

  // Four countries (64 hexes): the default config with `countries: 4`, played a while
  // with random actions, still fits without page scroll and keeps a usable board.
  const four = await js(`(() => {
    const t = window.aic.timeline;
    const cfg = JSON.parse(t.configJson);
    cfg.countries = 4;
    cfg.name = (cfg.name ?? 'config') + ' (4 countries, e2e)';
    localStorage.setItem('aic.session', JSON.stringify({ config: JSON.stringify(cfg), seed: '7', actions: [], cursor: 0, source: { kind: 'play', file: 'e2e-4-countries' } }));
    return cfg.countries;
  })()`);
  await reload(`window.aic.view.hexes.length === ${four * 16}`, 'the 4-country game');
  for (let i = 0; i < 40; i++) await js(`document.getElementById('random-action').click()`);
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
  for (const [w, h] of [[1366, 768], [1920, 1080]]) {
    await size(w, h);
    const svg = await js(`(() => { const r = document.getElementById('board').getBoundingClientRect(); return [r.width, r.height]; })()`);
    check(await noScroll() && svg[1] > h * 0.4, `4 countries: no page scroll, board ${svg.map(Math.round).join('x')} at ${w}x${h}`);
    await shot(`four-countries-${w}x${h}`);
  }
  await size(1366, 768);

  // The shared-row threat variant shows its threats in their own box by the countries.
  if (await js(`[...document.querySelectorAll('#config option')].some((o) => o.value === 'shared_row.json')`)) {
    await newGame(page, 'shared_row.json', 3);
    for (let i = 0; i < 12; i++) await js(`document.getElementById('random-action').click()`);
    const row = await js(`({ pile: !!window.aic.view.piles.find((p) => p.key === 'threats.row'), shown: document.querySelectorAll('#countries .shared .threat').length })`);
    check(!row.pile || row.shown > 0, `shared threat row shown (${row.shown} threats)`);
    await shot('shared-row-1366');
  }

  // Watch a record from the simulator: load, step forward and back, reach its end.
  if (recordPath) {
    const text = readFileSync(recordPath, 'utf8');
    const want = JSON.parse(text);
    await js(`(() => { document.getElementById('record').click(); document.getElementById('record-text').value = ${JSON.stringify(text)};
                        document.getElementById('record-load').click(); })()`);
    await until(`window.aic.timeline.source.kind === 'record'`, 'the record to load');
    const loaded = await js(`({ n: window.aic.timeline.last, cursor: window.aic.timeline.cursor, err: document.getElementById('error').hidden ? '' : document.getElementById('error').textContent })`);
    check(loaded.n === want.actions.length && loaded.cursor === 0 && !loaded.err, `record loaded: ${loaded.n} actions, at step 0 ${loaded.err}`);
    await send('Input.dispatchKeyEvent', { type: 'keyDown', key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39 });
    await send('Input.dispatchKeyEvent', { type: 'keyUp', key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39 });
    await js(`document.getElementById('fwd').click()`);
    check((await js('window.aic.timeline.cursor')) === 2, 'steps forward (key and button)');
    await js(`document.getElementById('back').click()`);
    check((await js('window.aic.timeline.cursor')) === 1, 'steps back');
    await js(`document.getElementById('end').click()`);
    const fin = await js(`({ o: window.aic.view.outcome, cursor: window.aic.timeline.cursor, now: document.querySelector('#tab-log .entry.now')?.dataset.step })`);
    const same = (a, b) => (a && b ? ['won', 'round', 'reason', 'detail'].every((k) => a[k] === b[k]) : a == b);
    check(same(fin.o, want.outcome) && fin.cursor === want.actions.length && Number(fin.now) === fin.cursor,
      `end of the record matches its saved outcome (${fin.o?.reason}) and the log follows the cursor`);
    await js(`document.querySelector('#tab-log .entry[data-step="3"]').click()`);
    check((await js('window.aic.timeline.cursor')) === 3, 'clicking a log entry jumps to that step');
    await shot('watch-record');
    await js(`document.getElementById('first').click()`);
    check((await js('window.aic.timeline.cursor')) === 0, 'back to the start');
  } else {
    console.log('skip watching a record (no record path given)');
  }

  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  failures++;
} finally {
  await page.close();
}
console.log(failures ? `e2e FAILED (${failures})` : 'e2e ok');
process.exit(failures ? 1 : 0);
