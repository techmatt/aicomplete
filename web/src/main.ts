// The AI Complete viewer: play a game (any seat, any order) or step through a saved
// one. Renders the engine's generic view and offers exactly the engine's
// `legal_actions`. No game rules live here.

import { renderBoard } from './board';
import {
  checkRecord, legalActions, load, recordText, resolveConfig, rulesRevision, version, view as engineView,
  type Action, type CardView, type GameEvent, type View,
} from './engine';
import {
  actionLabel, actionRule, choiceKey, contentIds, countryName, escapeHtml, hexKey, hexesIn, isHex, PLAYER_STEPS, playerStep,
  seatNames, seatText, setPlayerSeats, shortPhase, strong, type LabelContext,
} from './format';
import { details } from './hover';
import { logEntry } from './log';
import {
  countriesHtml, legendHtml, pilesHtml, pipelineHtml, seatCardsHtml, seatValuesHtml,
  statusHtml, varsHtml,
} from './panels';
import { Timeline, keptSession, parseSeed, randomSeed, readRecord, revisionNote, storageGet, storageSet } from './session';
import { loadScenes, printedCard } from './player/printed';
import {
  focusSeat, isAdversaryPhase, isAdversaryTurn, pin, renderPlayer, replayTurn, stayWith, stepPhaseView, toggleDraft, turnHexes, turnOpen,
  turnStep, turnStepFrame,
  turnView, type UndoRoom,
} from './player/view';
import { TUTORIAL, initTutorial, startTutorial, stopTutorial, tutorialActive, tutorialState, updateTutorial } from './tutorial';
import './style.css';

const $ = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;

const ui = {
  version: $('version'),
  newForm: $<HTMLFormElement>('new-form'),
  config: $<HTMLSelectElement>('config'),
  boardPick: $('board-pick'),
  boardChoice: $<HTMLSelectElement>('board-choice'),
  seed: $<HTMLInputElement>('seed'),
  error: $('error'),
  source: $('source'),
  phaseBar: $('phase-bar'),
  board: document.getElementById('board') as unknown as SVGSVGElement,
  prompt: $('prompt'),
  status: $('status'),
  pipeline: $('pipeline'),
  countries: $('countries'),
  seats: $('seats'),
  piles: $('piles'),
  log: $('tab-log'),
  state: $('tab-state'),
  view: $('tab-view'),
  configTab: $('tab-config'),
  scrub: $<HTMLInputElement>('scrub'),
  step: $('step'),
  play: $<HTMLButtonElement>('play'),
  speed: $<HTMLSelectElement>('speed'),
  tip: $('tip'),
  recordDialog: $<HTMLDialogElement>('record-dialog'),
  recordText: $<HTMLTextAreaElement>('record-text'),
  recordNote: $<HTMLInputElement>('record-note'),
  recordMsg: $('record-msg'),
  recordFile: $<HTMLInputElement>('record-file'),
  newDialog: $<HTMLDialogElement>('new-dialog'),
  ndBoard: $<HTMLSelectElement>('nd-board'),
  ndConfig: $<HTMLSelectElement>('nd-config'),
  ndSeed: $<HTMLInputElement>('nd-seed'),
};

/** Every config in configs/ (a file with `modules`; not variants.json's override matrix), read
 *  at build time (the dev server reloads on changes). */
const CONFIGS: Record<string, string> = Object.fromEntries(
  Object.entries(import.meta.glob('../../configs/*.json', { query: '?raw', import: 'default', eager: true }))
    .map(([path, text]) => [path.split('/').pop() as string, text as string])
    .filter(([, text]) => isConfig(text))
    .sort(([a], [b]) => a.localeCompare(b)),
);

function isConfig(text: string): boolean {
  try {
    return 'modules' in JSON.parse(text);
  } catch {
    return false;
  }
}

/** 323: the AI boards a solo player picks from (content/ais.json's order, `ai.boards`' indices). */
const BOARD_NAMES: string[] = (JSON.parse(
  Object.values(import.meta.glob('../../content/ais.json', { query: '?raw', import: 'default', eager: true }))[0] as string,
) as { name: string }[]).map((b) => b.name);

/** 323: a one-country config (solo) plays the board the Board select names: its `ai.boards` set to it. */
function soloConfig(text: string): boolean {
  try {
    return JSON.parse(text).countries === 1;
  } catch {
    return false;
  }
}

/** The config text a new game plays: a solo config with the chosen board, else the file as it is. */
function withBoard(text: string, board: string): string {
  if (!soloConfig(text) || board === '') return text;
  const cfg = JSON.parse(text);
  const ai = (cfg.modules as { module: string; params: Record<string, unknown> }[]).find((m) => m.module === 'ai');
  if (!ai) return text;
  ai.params.boards = [Number(board)];
  return JSON.stringify(cfg, null, 2) + '\n';
}

/** The Board select shows for a solo config only, on the config's own board until the player picks another. */
function syncBoardPick(fromConfig: boolean) {
  const text = CONFIGS[ui.config.value];
  const solo = text !== undefined && soloConfig(text);
  ui.boardPick.hidden = !solo;
  if (solo && fromConfig) {
    const ai = (JSON.parse(text).modules as { module: string; params: { boards?: number[] } }[]).find((m) => m.module === 'ai');
    ui.boardChoice.value = String(ai?.params.boards?.[0] ?? 0);
  }
}

let tl: Timeline;
let shown: View;
/** 305: the view the map was drawn from: `shown`, or the turn panel's earlier step (`turnView`). */
let mapView: View;
let legal: Action[] = [];
/** The choice (actions differing only in target hex) waiting for a hex click; set by `render`. */
let armed: { seat: number; key: string } | null = null;
let autoplay: number | null = null;
let activeTab = 'log';
/** Whether the raw-vars section under the piles is unfolded. */
let varsOpen = false;

// ---------- the two views (242) ----------

/** The player view (play through the objects) or the debug view (everything, the timeline and the raw data). */
type Mode = 'player' | 'debug';
// 326 (Matt, Nov 25): the player view is the default: a fresh or returning browser opens it, and the toggle isn't
// remembered (a saved debug setting no longer wins). `aic.startView` in localStorage is the scripts' own start
// (scripts/cdp.mjs keeps theirs on the debug view), as `aic.firstConfig` is.
let mode: Mode = storageGet('aic.startView') === 'debug' ? 'debug' : 'player';

const playerUi = {
  root: $('player'),
  main: $('p-main'),
  adv: $('p-adv'),
  seat: $('p-seat'),
  act: $('p-act'),
  turn: $('p-turn'),
  draft: $('p-draft'),
  boardDialog: $<HTMLDialogElement>('p-board-dialog'),
  boardBody: $('p-board-body'),
};

/** Moves the shared panels (map, phase bar, prompt, status, adversary row, countries) into the chosen view. */
function setMode(m: Mode) {
  mode = m;
  document.body.classList.toggle('player-mode', m === 'player');
  playerUi.root.hidden = m !== 'player';
  $('app').hidden = m === 'player';
  $('view-toggle').textContent = m === 'player' ? 'Debug view' : 'Player view';
  const wrap = $('board-wrap');
  if (m === 'player') {
    // It plays the live game: no autoplay, no earlier step (it has no timeline to show either).
    stopAutoplay();
    if (tl && tl.cursor < tl.last) {
      tl.seek(tl.last);
      tl.save();
    }
    playerUi.main.append(ui.phaseBar, ui.prompt, wrap, playerUi.draft);
    playerUi.adv.append(ui.status, ui.pipeline, ui.countries);
  } else {
    $('board-pane').append(ui.phaseBar, ui.prompt, ui.status, ui.pipeline, wrap, ui.countries);
  }
  if (tl) render();
}

// ---------- errors ----------

function showError(msg: string | null) {
  ui.error.hidden = !msg;
  ui.error.textContent = msg ?? '';
}

function guard(f: () => void) {
  try {
    f();
  } catch (e) {
    showError(e instanceof Error ? e.message : String(e));
  }
}

// ---------- sessions ----------

/** Saves the session and draws it. */
function commit() {
  tl.save();
  render();
}

/** Asked before acting at an earlier step; false if the player backs out. */
function confirmDropLater(): boolean {
  const later = tl.last - tl.cursor;
  // 285: a game that stopped replaying also drops the actions that didn't replay (it asked about the replayed only).
  const lost = tl.unreplayed.length;
  if (later <= 0 && lost === 0) return true;
  const parts = [later > 0 ? `the ${later} later action(s)` : '', lost > 0 ? `the ${lost} action(s) that didn't replay` : ''].filter(Boolean);
  return confirm(`Acting here drops ${parts.join(' and ')}. Go on?`);
}

function setTimeline(t: Timeline) {
  stopAutoplay();
  pin(null);
  stayWith(null);
  // 325: a game started any way (a script's too) closes the new-game screen.
  if (ui.newDialog.open) ui.newDialog.close();
  tl = t;
  // 261: the player view plays the live game, as `setMode` keeps it: a record opened, dropped or loaded there (or a
  // session restored) showed its setup step, with no timeline to move on and every control dropping the record.
  if (mode === 'player' && tl.cursor < tl.last) tl.seek(tl.last);
  showError(t.replayError);
  // 265: a game that stopped replaying saves whole, its unreplayed actions too (`Timeline.save`).
  commit();
}

function newGame(file: string, seed: string, tutorial = false) {
  const text = CONFIGS[file];
  if (text === undefined) throw new Error(`no config ${file}`);
  // 325: any other new game leaves the tutorial.
  if (!tutorial) stopTutorial();
  setTimeline(Timeline.start(withBoard(text, ui.boardChoice.value), seed, { kind: 'play', file }));
}

function loadRecordText(text: string, file: string) {
  const rec = readRecord(text);
  const t = Timeline.fromRecord(rec, file);
  const check = checkRecord(text);
  const warn = [...(check.warnings ?? []), ...(check.error ? [check.error] : [])];
  // 118: on the timeline before its first save, so a reload still shows it (it was gone).
  if (warn.length && !t.replayError) t.warning = `record loaded with warnings: ${warn.join('; ')}`;
  setTimeline(t);
  // 118: the seed box showed the last new game's seed while a record played.
  ui.seed.value = String(rec.seed);
  // A record dropped or opened under the dialog: the dialog's old text would be stale.
  if (ui.recordDialog.open) ui.recordDialog.close();
  if (t.warning) showError(t.warning);
  // A record that stops replaying: say first if it is from another rules revision (020:
  // only the rejected action showed).
  else if (t.replayError && revisionNote(rec)) showError(`${revisionNote(rec)}${t.replayError}`);
}

function act(a: Action) {
  if (!confirmDropLater()) return;
  // 242: after an action the player view follows whoever must act again. 321: the seat that acted first, while it
  // still has a move.
  pin(null);
  // A draft pick hands the next pick round back to the first seat (the last to pick went first again).
  stayWith(a.type === 'draft' ? null : a.seat, shown);
  // Acting from a focused action button (the keyboard): keep focus in that seat's
  // buttons across the re-render (020: it fell back to the page body every time).
  // (235: only when the keyboard put it there: after a click the refocused button took the next Space.)
  const fromButton = !pointerFocus && !!(document.activeElement as Element | null)?.closest?.('#seats .actions');
  guard(() => {
    tl.act(a);
    showError(null);
  });
  commit();
  if (fromButton) {
    const next = (ui.seats.querySelector(`.seat[data-seat="${a.seat}"] .actions button`) ?? ui.seats.querySelector('.actions button')) as HTMLElement | null;
    next?.focus();
  }
}

function seek(i: number) {
  if (!tl) return; // still loading
  tl.seek(i);
  commit();
}

// ---------- rendering ----------

function render() {
  const f = tl.frame;
  shown = engineView(f.snap);
  legal = legalActions(f.snap);
  // 326: the player view names seats by their boards (or "you"), its pieces' hovers and legend included.
  setPlayerSeats(mode === 'player' ? seatNames(shown) : null);
  // An open choice answered by picking a hex arms that pick at once.
  const pick = legal.find((a) => a.seat === shown.choice?.seat && a.type === 'choose' && isHex(a.target));
  armed = pick ? { seat: pick.seat, key: choiceKey(pick) } : null;

  const src = tl.source;
  const cfgName = f.snap.state.config.name || 'config';
  ui.source.innerHTML = src.kind === 'record'
    ? `<b>Watching</b> ${escapeHtml(src.file)} · ${escapeHtml(cfgName)} · seed ${tl.seed}${src.note ? ` · <i>${escapeHtml(src.note)}</i>` : ''}`
    : `<b>Playing</b> ${escapeHtml(src.file)} · ${escapeHtml(cfgName)} · seed ${tl.seed}`;
  // A long record note is cut off with an ellipsis: the hover has it whole.
  ui.source.title = ui.source.textContent ?? '';

  renderPhase();
  const targets = new Set<string>();
  if (armed) {
    for (const a of legal) if (a.seat === armed.seat && choiceKey(a) === armed.key && isHex(a.target)) targets.add(hexKey(a.target));
  }
  const changed = new Set<string>();
  // Setup touches every hex: only mark what an action changed.
  if (f.action) {
    for (const e of f.events) for (const h of hexesIn(e)) changed.add(hexKey(h));
    for (const h of hexesIn(f.action)) changed.add(hexKey(h));
  }
  // 242: the player view steps through the adversary's turn: only the shown step's hexes are outlined.
  const turn = mode === 'player' ? turnHexes(f, stepPhase(), !!shown.outcome) : null;
  if (turn) {
    changed.clear();
    for (const h of turn) changed.add(h);
  }
  // 289 (D129): during a hex answer only the hexes to pick are marked (about 23 changed-hex outlines hid the 2).
  if (mode === 'player' && targets.size) changed.clear();
  const sources = new Set(shown.threat_sources.flatMap((s) => s.hexes.map((i) => hexKey(shown.hexes[i].hex))));
  // 305: while the turn panel shows an earlier step, the map shows the board as it stood at its end. 320: the phase
  // bar, tracks, adversary row, threat rows and seat rows too (only the map went back, and they showed the board
  // after the turn); the question, the shown seat's hand and powers and every control stay the board now.
  // 324 (D142): the turn shown may be an earlier frame's (its Replay button): the state before that frame.
  const tf = turn ? turnStepFrame() : null;
  const ti = tf ? tl.frames.indexOf(tf) : -1;
  const before = ti > 0 ? ti - 1 : tl.cursor - 1;
  mapView = (turn && turnView((fr) => tl.frames[tl.frames.indexOf(fr) - 1]?.snap)) || shown;
  if (mapView !== shown) {
    const ps = tl.frames[before]?.snap.state;
    renderPhase(stepPhaseView(mapView, ps ? ps.config.round[ps.phase] : undefined));
  }
  renderBoard(ui.board, mapView, { targets, changed, sources });
  renderPrompt(targets.size);
  ui.status.innerHTML = statusHtml(mapView, f.snap.state.config);
  ui.pipeline.innerHTML = pipelineHtml(mapView);
  ui.countries.innerHTML = countriesHtml(mapView);
  if (mode === 'player') {
    const adv = lastAdversaryTurn();
    renderPlayer({ ...playerUi, countries: ui.countries, prompt: ui.prompt, pipeline: ui.pipeline }, {
      view: shown, legal, config: f.snap.state.config, cx: labels(), frame: f, before: stepPhase(), board: mapView,
      undo: undoRoom(), replay: adv >= 0 ? tl.frames[adv].snap.state.round : null,
    });
  } else {
    renderSeats();
    renderPiles();
    renderLog();
    renderTab();
  }
  renderTimeline();
  updateTutorial();
  // 118: the hover was redone only when the mouse moved, so autoplay, the keys or an
  // action left it showing the old step's values (or a removed button's) under a still mouse.
  if (mouse) hover(document.elementFromPoint(mouse.x, mouse.y), mouse.x, mouse.y);
  else ui.tip.hidden = true;
}

/** What a phase asks of the seats, where its key alone doesn't say (the buttons are at a panel's foot). */
// 235: keyed by the config's phase name (the bar's names are the rulebook's now).
const PHASE_HINTS: Record<string, string> = {
  'cards.tier': 'pick the tier deck to draft from, at the foot of the panel',
  'cards.draft': 'draft a card, at the foot of the panel',
  'cards.keep': 'choose which drafted cards to keep, each at its keep price, at the foot of the panel',
};

/** Who the game waits for, and the open choice's question, above the board. */
function renderPrompt(targets: number) {
  const v = shown;
  // 326: the player view names a seat by its board, solo's as "you" (no "Seat 0").
  const names = mode === 'player' ? seatNames(v) : null;
  const seatName = (s: number) => {
    if (names) return escapeHtml(names[s] ?? '');
    const ai = v.seats[s]?.board?.name;
    return `Seat ${s}${ai ? ` (${escapeHtml(ai)})` : ''}`;
  };
  ui.prompt.className = '';
  if (v.outcome) {
    ui.prompt.innerHTML = '';
    return;
  }
  if (v.choice) {
    ui.prompt.className = 'choice';
    const how = targets === 1 ? `click the highlighted hex` : targets ? `click one of the ${targets} highlighted hexes` : mode === 'player' ? 'answer here' : `answer in ${seatName(v.choice.seat)}'s panel`;
    // 305 (D85): a piece's owner in the words ("seat 1's node") named as the seat rows name it.
    const stripped = contentIds(v.choice.prompt.replace(/^Seat \d+:\s*/, '').replace(/^([^:]+): seat \d+:\s*/, '$1: '));
    // 326: the player view's seat words, solo's in grammar ("your node", "you pay").
    const words = names ? seatText(stripped)
      : stripped.replace(/\bseat (\d+)'s\b/gi, (_, s: string) => `Seat ${s}${v.seats[Number(s)]?.board ? ` (${v.seats[Number(s)].board!.name})` : ''}'s`);
    const who = names?.length === 1 ? 'You decide' : `${seatName(v.choice.seat)} decides`;
    ui.prompt.innerHTML = `<b>${who}:</b> ${escapeHtml(words)} <span class="how">— ${how}</span>`;
    return;
  }
  // 326: the player view's right panel says whose move it is ("Your move"): no engine phase line over the map.
  if (mode === 'player') {
    ui.prompt.innerHTML = '';
    return;
  }
  const acting = [...new Set(legal.map((a) => a.seat))];
  // 184: a draft phase named only its key ("tier: Seat 0 to act"); say what the panel's buttons are for.
  const short = v.phase ? shortPhase(v.phase.name) : '';
  // 242: the player view's draft layout says it itself.
  const what = v.phase && mode === 'debug' ? PHASE_HINTS[v.phase.name] : undefined;
  ui.prompt.innerHTML = v.phase && acting.length
    ? `<b>${escapeHtml(short)}:</b> ${acting.map(seatName).join(', ')} to act${what ? ` <span class="how">— ${what}</span>` : ''}`
    : '';
}

/** The phase the shown step's action was taken in (the log's first group); `i`, another step's. */
function stepPhase(i = tl.cursor): string {
  const ps = tl.frames[i - 1]?.snap.state;
  return ps ? shortPhase(ps.config.round[ps.phase] ?? 'setup') : 'setup';
}

/** 324 (D142): the last step up to the one shown that ran the adversary's turn, or -1. Found once a step. */
let advFound: { frame: unknown; at: number } | null = null;
function lastAdversaryTurn(): number {
  if (advFound?.frame === tl.frame) return advFound.at;
  let at = -1;
  for (let i = tl.cursor; i > 0; i--) {
    if (isAdversaryTurn(tl.frames[i], stepPhase(i))) {
      at = i;
      break;
    }
  }
  advFound = { frame: tl.frame, at };
  return at;
}

/** 324: a deck's card (a draw, a deal, a reveal) or a reshuffle: what a printed game can't take back. */
const revealed = (events: GameEvent[]) => events.some((e) => (e.type === 'card_moved' && /(^|\.)(deck|project_deck)(\.|$)/.test(String(e.from)))
  || (e.type === 'note' && /reshuffl/i.test(String(e.text))));

/**
 * 324: what the player view's Undo may take back: the actions since the shown action phase began (every seat's, in a
 * game of several), stopping after one that drew or revealed a card from a deck (undoing it would show what comes
 * next). Nothing outside an action phase, nor once the game is over.
 */
function undoRoom(): UndoRoom {
  const cur = tl.frames[tl.last].snap.state;
  const name = cur.config.round[cur.phase] ?? '';
  if (cur.outcome || !/board\.act/.test(name) || tl.source.kind !== 'play') return { back: 0, why: '' };
  let i = tl.last;
  while (i > 0) {
    const ps = tl.frames[i - 1].snap.state;
    if (ps.round !== cur.round || ps.phase !== cur.phase) break;
    if (revealed(tl.frames[i].events)) {
      const a = tl.frames[i].action;
      const what = a ? actionLabel(a, false, false, labels()) : 'a step';
      return { back: tl.last - i, why: `Undo stops after ${what}: it drew a card from a deck, and taking it back would show what comes next` };
    }
    i--;
  }
  return { back: tl.last - i, why: '' };
}

/** 324: Undo in the player view, `n` actions or back to the phase's start, inside `undoRoom` only. */
function undoPlayer(n: number | 'start') {
  const room = undoRoom();
  const k = Math.min(room.back, n === 'start' ? room.back : n);
  if (!k) return;
  for (let j = 0; j < k; j++) tl.undo();
  showError(null);
  pin(null);
  stayWith(null);
  commit();
}

function labels(): LabelContext {
  return { config: tl.frame.snap.state.config, view: shown };
}

/** The countries' names, fixed for a game: the log's and step line's labels have no view (102, D30). */
function countryNames(): string[] | undefined {
  return shown?.countries.map((c) => c.name);
}

/** 326: what each of the player view's five steps is, on its hover. */
const STEP_HINTS = [
  "Sway (each Fallout token adds a Discord), then the adversary's turn: its Card beat (threats, its lines) and its Income beat (what its corrupted sites make, compute onto its project)",
  'Build a hand from the tier-1 and tier-2 decks, then keep what you pay insight for',
  'Play cards, use powers, building Actions and standard actions, in any order, until you pass',
  'Your factories, cities, stations and income pay',
  'Every threat nobody paid off takes effect; then cleanup stands your ambassadors back up',
];

/** The round and phase bar, of `v` (320: the turn panel's earlier step while it shows one). */
function renderPhase(v: View = shown) {
  // 326 (Matt, Nov 25): the player view shows a round's five steps, the current one lit; the engine's phases (its
  // sub-steps) are the debug view's.
  const now = v.phase ? playerStep(v.phase.name) : -1;
  const order = mode === 'player'
    ? PLAYER_STEPS.map((name, i) => `<span class="phase step${now === i ? ' now' : ''}" title="${escapeHtml(STEP_HINTS[i])}">${name}</span>`).join('<span class="arrow">›</span>')
    : v.round_order
      .map((p, i) => `<span class="phase ${p.kind}${v.phase?.index === i ? ' now' : ''}" title="${escapeHtml(p.name)} (${p.kind})">${escapeHtml(shortPhase(p.name))}</span>`)
      .join('<span class="arrow">›</span>');
  let status: string;
  if (v.outcome) {
    const o = v.outcome;
    // The sentence; the reason code (what `--save-when` takes) in its hover (235: "on the board node" read as a typo).
    // 308: a deck-out ends the game as the next round's Card beat finds no card: the cap's last round was the one before.
    const when = o.reason === 'deck_empty' ? `after round ${o.round - 1}, the round cap` : `in round ${o.round}`;
    // 326: the player view names a seat as it does everywhere ("Builder has no nodes left", "you have ...").
    const detail = mode === 'player' ? seatText(o.detail) : o.detail;
    status = `<span class="outcome ${o.won ? 'won' : 'lost'}" title="reason code: ${escapeHtml(o.reason)}">${o.won ? 'WON' : 'LOST'} ${when}: ${escapeHtml(detail)}</span>`;
  } else status = ''; // who is to act is in #prompt
  // 305: the round cap beside the round: the adversary's cards left are the rounds left after this one.
  // The count holds the face-up next card too: before this round's Card beat turns it into this round's card, it is
  // this round's, not a later one's.
  const deck = v.outcome ? undefined : v.globals.find((g) => g.key === 'adversary.deck');
  const cardBeat = v.round_order.findIndex((p) => p.name === 'adversary.event');
  const later = deck ? deck.value - (v.phase && cardBeat >= 0 && v.phase.index < cardBeat ? 1 : 0) : 0;
  const cap = deck ? `<small class="round-cap" title="${escapeHtml(`${deck.value} adversary card${deck.value === 1 ? '' : 's'} left: ${deck.doc ?? ''}`)}"> of ${v.round + later} at most · <span class="deck-left">${deck.value} adversary card${deck.value === 1 ? '' : 's'} left</span></small>` : '';
  ui.phaseBar.innerHTML = `<b>Round ${v.round}</b>${cap} <span class="order">${order}</span> ${status}`;
}

/** The choice the seat panels last showed, so a new one scrolls into view once. */
let lastAsked = '';

function renderSeats() {
  const v = shown;
  ui.seats.innerHTML = '';
  for (const seat of v.seats) {
    const country = v.countries[seat.country];
    const mine = legal.filter((a) => a.seat === seat.index);
    const waiting = v.phase?.waiting.includes(seat.index);
    const choosing = v.choice?.seat === seat.index;
    const box = document.createElement('div');
    box.className = `seat${seat.passed && !choosing ? ' passed' : ''}${mine.length ? ' active' : ''}`;
    box.dataset.seat = String(seat.index);
    box.style.setProperty('--tint', strong(country?.color ?? '#cccccc'));
    const status = v.outcome ? '' : choosing ? 'deciding' : mine.length ? 'to act' : seat.passed ? 'passed' : waiting ? 'waiting' : '';
    const ai = seat.board;
    box.innerHTML = `
      <h3><span class="swatch"></span>Seat ${seat.index}${ai ? ` · <span class="board-name" data-board-seat="${seat.index}">${escapeHtml(ai.name)}</span>` : ''} <small title="country ${seat.country}">${escapeHtml(countryName(v, seat.country))}</small><span class="status">${status}</span></h3>
      ${ai?.innate ? `<div class="innate" title="${escapeHtml(ai.text ?? '')}"><b>${escapeHtml(ai.innate.name)}:</b> ${escapeHtml(ai.innate.text ?? '')}${ai.tags?.length ? ` <span class="board-tags" title="printed tags: they count like the tags on your cards, but can't be sacrificed">${ai.tags.map(escapeHtml).join(', ')} tag</span>` : ''}</div>` : ''}
      ${powersHtml(ai?.cards ?? [])}
      ${seatValuesHtml(v, seat.index)}
      ${seatCardsHtml(v, seat.index)}`;
    if (mine.length) box.append(actionButtons(mine));
    ui.seats.append(box);
  }
  // 118: a choice is answered in its seat's panel: bring its buttons into view when it
  // opens (they sat below the fold of another seat's panel, "answer in Seat 1's panel").
  const asked = v.choice ? `${v.round}:${v.choice.seat}:${v.choice.prompt}` : '';
  // Only at the live end of the timeline: stepping back to an old choice leaves the pane where the user put it.
  if (asked && asked !== lastAsked && tl.cursor === tl.last) {
    ui.seats.querySelector(`.seat[data-seat="${v.choice!.seat}"] .actions`)?.scrollIntoView({ block: 'nearest' });
  }
  lastAsked = asked;
  const passers = (v.phase?.waiting ?? []).filter((s) => legal.some((a) => a.seat === s && a.type === 'pass'));
  if (!v.outcome && !v.choice && passers.length > 1) {
    const all = document.createElement('button');
    all.type = 'button';
    all.className = 'pass-all';
    all.textContent = `Pass every seat still to act (${passers.join(', ')})`;
    all.onclick = () => passAll();
    ui.seats.append(all);
  }
}

/** A board's powers, always shown: a button appears only when one can be used. */
function powersHtml(cards: CardView[]): string {
  const rows = cards.filter((c) => c.id.includes('.power.')).map((c) =>
    `<li class="${c.status.startsWith('ready') ? 'ready' : ''}"><b>${escapeHtml(c.name)}:</b> ${escapeHtml(c.text ?? '')} <small>${escapeHtml(c.status)}</small></li>`);
  return rows.length ? `<ul class="powers">${rows.join('')}</ul>` : '';
}

/**
 * One button per choice. Only `choose` actions have a target hex, and `render` arms
 * such a choice at once: it shows as a count here and is answered on the board.
 */
function actionButtons(mine: Action[]): HTMLElement {
  const row = document.createElement('div');
  row.className = 'actions';
  const groups = new Map<string, Action[]>();
  const sorted = [...mine].sort((a, b) => typeRank(a.type) - typeRank(b.type));
  for (const a of sorted) {
    const k = choiceKey(a);
    groups.set(k, [...(groups.get(k) ?? []), a]);
  }
  const cx = labels();
  for (const acts of groups.values()) {
    if (acts.every((a) => isHex(a.target))) {
      const count = document.createElement('span');
      count.className = 'hex-pick';
      // 169: hexes, not answers (a Gather or Push offers two batch sizes on one hex), as the prompt counts them.
      count.textContent = `pick a hex ▸ (${new Set(acts.map((a) => hexKey(a.target!))).size})`;
      count.dataset.hexes = acts.map((a) => hexKey(a.target!)).join(' ');
      count.dataset.action = JSON.stringify(acts[0]);
      row.append(count);
      // Two answers on one hex (a sacrifice: the ambassador or the node there) can't
      // both be a click on it: each gets its own button (020: only the first was reachable).
      const perHex = new Map<string, Action[]>();
      for (const a of acts) perHex.set(hexKey(a.target!), [...(perHex.get(hexKey(a.target!)) ?? []), a]);
      for (const same of perHex.values()) {
        if (same.length < 2) continue;
        for (const a of same) {
          const one = document.createElement('button');
          one.type = 'button';
          one.textContent = actionLabel(a, false, false, cx);
          one.className = `a-${a.type}`;
          one.dataset.action = JSON.stringify(a);
          one.dataset.hexes = hexKey(a.target!);
          one.onclick = () => act(a);
          row.append(one);
        }
      }
      continue;
    }
    // Anything else is one button per action, labeled in full.
    for (const a of acts) {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = actionLabel(a, false, true, cx);
      // 265 (D109): a standard action's rule text, which its label leaves out.
      if (a.type === 'standard') b.title = actionRule(a, cx);
      b.className = `a-${a.type}${a.type === 'pass' ? ' pass' : ''}`;
      b.dataset.action = JSON.stringify(a);
      b.onclick = () => act(a);
      row.append(b);
    }
  }
  return row;
}

/** Button order: answers and decisions first, standard actions, threat counters and gifts next, selling a card and pass last. */
const TYPE_ORDER = ['choose', 'tier', 'draft', 'keep', 'play', 'use', 'power', 'convert', 'standard', 'pay', 'withdraw', 'sell'];
const typeRank = (t: string) => (t === 'pass' ? 99 : TYPE_ORDER.indexOf(t) < 0 ? 50 : TYPE_ORDER.indexOf(t));

function passAll() {
  if (!confirmDropLater()) return;
  pin(null);
  guard(() => {
    for (const s of shown.phase?.waiting ?? []) {
      const p = legalActions(tl.frame.snap).find((a) => a.seat === s && a.type === 'pass');
      if (p) tl.act(p);
    }
  });
  commit();
}

function renderPiles() {
  ui.piles.innerHTML = pilesHtml(shown) + varsHtml(shown, varsOpen);
  ui.piles.querySelector('#vars')?.addEventListener('toggle', (e) => {
    varsOpen = (e.target as HTMLDetailsElement).open;
  });
}

/** The timeline and frames whose entries the log shows (see `renderLog`). */
let logBuilt: { tl: Timeline; frames: unknown[] } | null = null;

/**
 * The log keeps the entries of the frames it already shows and only adds or drops
 * the ones that changed (020: rebuilding a long game's log on every step or action
 * cost ~130 ms a step at 400 actions, ~30 ms an action at 1,000).
 */
function renderLog() {
  if (logBuilt?.tl !== tl) {
    ui.log.innerHTML = '';
    logBuilt = { tl, frames: [] };
  }
  const had = logBuilt.frames;
  let keep = 0;
  while (keep < had.length && keep < tl.frames.length && had[keep] === tl.frames[keep]) keep++;
  while (ui.log.children.length > keep) ui.log.lastElementChild!.remove();
  if (tl.frames.length > keep) {
    const names = countryNames();
    ui.log.insertAdjacentHTML('beforeend', tl.frames.slice(keep).map((f, j) => logEntry(f, keep + j, tl.frames[keep + j - 1], names)).join(''));
  }
  logBuilt.frames = [...tl.frames];
  const entries = ui.log.children;
  for (let i = 0; i < entries.length; i++) {
    entries[i].classList.toggle('now', i === tl.cursor);
    entries[i].classList.toggle('future', i > tl.cursor);
  }
  entries[tl.cursor]?.scrollIntoView({ block: 'nearest' });
}

function renderTimeline() {
  ui.scrub.max = String(tl.last);
  ui.scrub.value = String(tl.cursor);
  const a = tl.frame.action;
  // 265 (D112): a power by its name and cost (a node route's rule was cut off mid-sentence); the hover has it whole.
  const scx = { config: tl.frame.snap.state.config, countries: countryNames() };
  ui.step.textContent = `step ${tl.cursor} / ${tl.last}${a ? ` · ${actionLabel(a, true, true, { ...scx, brief: true })}` : ' · setup'}`;
  ui.step.title = `step ${tl.cursor} / ${tl.last}${a ? ` · ${actionLabel(a, true, true, scx)}` : ' · setup'}`;
  ui.play.textContent = autoplay !== null ? '⏸ pause' : '▶ play';
}

function renderTab() {
  for (const t of ['log', 'state', 'view', 'config']) {
    $(`tab-${t}`).hidden = t !== activeTab;
    document.querySelector(`#tabs [data-tab="${t}"]`)?.classList.toggle('on', t === activeTab);
  }
  if (!tl) return; // still loading (020: Raw state and Config threw)
  if (activeTab === 'state') ui.state.textContent = JSON.stringify(tl.frame.snap.state, null, 1);
  if (activeTab === 'view') ui.view.textContent = JSON.stringify(shown, null, 1);
  if (activeTab === 'config') {
    let hash = '?';
    try {
      hash = resolveConfig(tl.configJson).hash;
    } catch (e) {
      hash = String(e);
    }
    ui.configTab.textContent = `config hash ${hash}\n${JSON.stringify(tl.frame.snap.state.config, null, 1)}`;
  }
}

// ---------- hover details ----------

function highlight(hexes: string[]) {
  ui.board.querySelectorAll('.hex.hl').forEach((e) => e.classList.remove('hl'));
  for (const h of hexes) ui.board.querySelector(`.hex[data-hex="${h}"]`)?.classList.add('hl');
}

/** Where the mouse last was over the page, so `render` can redo the hover (118). */
let mouse: { x: number; y: number } | null = null;
/** The printed card (id and status) the tip shows, so a still card isn't redrawn on every mouse move (242). */
let tipPrinted = '';

function hover(el: Element | null, cx: number, cy: number) {
  const target = el?.closest?.('[data-token],[data-hex][data-index],[data-value],[data-card],[data-pile],[data-track],[data-board-seat],[data-action]') as HTMLElement | null;
  // 242: a player card in the player view opens as it prints, its status under it.
  const printed = mode === 'player' ? (el?.closest?.('[data-printed]') as HTMLElement | null) : null;
  // 324: why a shaded card, power or building Action can't be used now, atop the tip.
  const whyEl = mode === 'player' ? (el?.closest?.('[data-why]') as HTMLElement | null) : null;
  const why = whyEl?.dataset.why ? `<div class="tip-why">${escapeHtml(whyEl.dataset.why)}</div>` : '';
  if (printed && target) {
    const more = why + (details(target, shown, legal, labels(), shown, true) ?? '');
    const key = `${printed.dataset.printed}|${more}`;
    // Built once per card and status: the mouse moving over it only moves the tip.
    const host = key === tipPrinted && !ui.tip.hidden ? null : printedCard(printed.dataset.printed!, 1.25);
    if (host || key === tipPrinted) {
      if (host) {
        ui.tip.replaceChildren(host);
        if (more) ui.tip.insertAdjacentHTML('beforeend', `<div class="tip-under">${more}</div>`);
        tipPrinted = key;
      }
      ui.tip.hidden = false;
      place(cx, cy);
      return;
    }
  }
  tipPrinted = '';
  const found = target ? details(target, shown, legal, labels(), mapView ?? shown, mode === 'player') : null;
  const html = why || found ? why + (found ?? '') : null;
  const hl = el?.closest?.('[data-hexes]') as HTMLElement | null;
  highlight(hl?.dataset.hexes ? hl.dataset.hexes.split(' ').filter(Boolean) : []);
  if (!html) {
    ui.tip.hidden = true;
    return;
  }
  ui.tip.innerHTML = html;
  ui.tip.hidden = false;
  place(cx, cy);
}

/** Puts the tip beside the mouse, inside the window. */
function place(cx: number, cy: number) {
  const pad = 14;
  const w = ui.tip.offsetWidth;
  const h = ui.tip.offsetHeight;
  const x = cx + pad + w > innerWidth ? cx - pad - w : cx + pad;
  const y = cy + pad + h > innerHeight ? Math.max(4, cy - pad - h) : cy + pad;
  ui.tip.style.left = `${x}px`;
  ui.tip.style.top = `${y}px`;
}

document.addEventListener('mousemove', (e) => {
  mouse = { x: e.clientX, y: e.clientY };
  hover(e.target as Element, e.clientX, e.clientY);
});
// 118: off the page there is nothing to redo (184: and the tip and outlines go, or they stayed while paused).
document.addEventListener('mouseout', (e) => {
  if (!e.relatedTarget) {
    mouse = null;
    ui.tip.hidden = true;
    highlight([]);
  }
});

// ---------- input ----------

ui.board.addEventListener('click', (e) => {
  const g = (e.target as Element).closest('[data-hex]') as SVGElement | null;
  if (!g || !armed) return;
  const key = g.dataset.hex;
  // Two answers on one hex: the click takes the first, and each has its own button.
  const a = legal.find((x) => x.seat === armed!.seat && choiceKey(x) === armed!.key && isHex(x.target) && hexKey(x.target) === key);
  if (a) act(a);
});

// 242: the player view's controls: an action button, a seat to show, the adversary's turn's steps.
playerUi.root.addEventListener('click', (e) => {
  const el = e.target as Element;
  const b = el.closest('button[data-action]') as HTMLElement | null;
  if (b) {
    const a = JSON.parse(b.dataset.action!) as Action;
    if (legal.some((x) => JSON.stringify(x) === b.dataset.action)) act(a);
    return;
  }
  if (el.closest('[data-pass-all]')) return passAll();
  // 324: Undo inside the action phase; the board's whole face; the draft folded away to see the map.
  const u = el.closest('button[data-undo]') as HTMLElement | null;
  if (u) return undoPlayer(u.dataset.undo === 'start' ? 'start' : 1);
  const bo = el.closest('[data-board-open]') as HTMLElement | null;
  if (bo) {
    if (Number(bo.dataset.boardOpen) !== focusSeat(shown, legal)) pin(Number(bo.dataset.boardOpen));
    render();
    playerUi.boardDialog.showModal();
    // Its Close button takes the focus, which scrolled the face to its foot.
    playerUi.boardDialog.scrollTop = 0;
    return;
  }
  if (el.closest('[data-draft-toggle]')) {
    toggleDraft();
    return render();
  }
  if (el.closest('[data-turn="replay"]')) {
    const at = lastAdversaryTurn();
    if (at > 0) {
      // 324 (diff review): a question asked inside the turn (a Strike's target, a node's protection) splits it into
      // the last pass's frame and each answer's: the replay runs them all, from the pass.
      let from = at;
      while (from > 1 && tl.frames[from].action?.type === 'choose' && isAdversaryPhase(stepPhase(from))) from--;
      let to = at;
      while (to < tl.cursor && tl.frames[to + 1].action?.type === 'choose' && isAdversaryPhase(stepPhase(to + 1))) to++;
      const parts = [];
      for (let i = from; i <= to; i++) parts.push({ frame: tl.frames[i], before: stepPhase(i) });
      replayTurn(parts, tl.frame);
    }
    return render();
  }
  const seat = el.closest('[data-seat-focus]') as HTMLElement | null;
  if (seat) {
    pin(Number(seat.dataset.seatFocus));
    return render();
  }
  const t = el.closest('[data-turn],[data-turn-step]') as HTMLElement | null;
  if (t) {
    if (t.dataset.turnStep !== undefined) turnStep(Number(t.dataset.turnStep) - Number(document.querySelector('#p-turn li.now')?.getAttribute('data-turn-step') ?? 0));
    else if (t.dataset.turn === 'open') turnStep('open');
    else turnStep(t.dataset.turn === 'close' ? 'close' : Number(t.dataset.turn));
    render();
  }
});

$('view-toggle').onclick = () => setMode(mode === 'player' ? 'debug' : 'player');

ui.log.addEventListener('click', (e) => {
  // A phase heading folds or unfolds its group; it doesn't move the timeline.
  if ((e.target as Element).closest('summary')) return;
  const entry = (e.target as Element).closest('[data-step]') as HTMLElement | null;
  if (entry) seek(Number(entry.dataset.step));
});

document.getElementById('tabs')!.addEventListener('click', (e) => {
  const b = (e.target as Element).closest('[data-tab]') as HTMLElement | null;
  if (!b) return;
  activeTab = b.dataset.tab!;
  renderTab();
});

ui.newForm.addEventListener('submit', (e) => {
  e.preventDefault();
  // 118: before the engine loads (or after it failed to) this replaced the reason with another error.
  if (!tl) return;
  const seed = parseSeed(ui.seed.value);
  if (seed === null) return showError('the seed must be a whole number from 0 to 18446744073709551615');
  guard(() => newGame(ui.config.value, seed));
});

// ---------- 325: the new-game screen and the tutorial ----------

/** A new game of `file` (a solo config on `board`) at `seed`, the header's selects following it. */
function startGame(file: string, seed: string, board: string | null, tutorial = false) {
  if (!tl) return;
  ui.config.value = file;
  syncBoardPick(true);
  if (board !== null) ui.boardChoice.value = board;
  ui.seed.value = seed;
  guard(() => newGame(file, seed, tutorial));
}

/** The solo game's board, from its config (null for a multi-seat config). */
function soloBoard(configJson: string): number | null {
  if (!soloConfig(configJson)) return null;
  const ai = (JSON.parse(configJson).modules as { module: string; params: { boards?: number[] } }[]).find((m) => m.module === 'ai');
  return ai?.params.boards?.[0] ?? 0;
}

let boardOf: { json: string; board: number | null } | null = null;
initTutorial({
  view: () => shown,
  legal: () => legal,
  config: () => tl.frame.snap.state.config,
  playerMode: () => mode === 'player',
  game: () => {
    if (boardOf?.json !== tl.configJson) boardOf = { json: tl.configJson, board: soloBoard(tl.configJson) };
    return { seed: tl.seed, file: tl.source.kind === 'play' ? tl.source.file : '', board: boardOf.board };
  },
  restart: () => startGame(TUTORIAL.file, TUTORIAL.seed, TUTORIAL.board, true),
});

function openNewGame() {
  if (!tl || ui.newDialog.open) return;
  ui.ndConfig.value = ui.config.value;
  ui.newDialog.showModal();
  $('nd-tutorial').focus();
}

$('new-open').onclick = openNewGame;
$('nd-close').onclick = () => ui.newDialog.close();
$('nd-tutorial').onclick = () => {
  if (CONFIGS[TUTORIAL.file] === undefined) return showError(`the tutorial needs configs/${TUTORIAL.file}`);
  startTutorial();
};
$('nd-solo').onclick = () => {
  const solo = Object.keys(CONFIGS).find((f) => f === TUTORIAL.file) ?? Object.keys(CONFIGS).find((f) => soloConfig(CONFIGS[f]));
  if (solo) startGame(solo, randomSeed(), ui.ndBoard.value);
};
$('nd-other').onclick = () => {
  const seed = ui.ndSeed.value.trim() === '' ? randomSeed() : parseSeed(ui.ndSeed.value);
  if (seed === null) return showError('the seed must be a whole number from 0 to 18446744073709551615');
  startGame(ui.ndConfig.value, seed, null);
};

$('random-seed').onclick = () => {
  if (!tl) return; // 118: as New game
  ui.seed.value = randomSeed();
  guard(() => newGame(ui.config.value, ui.seed.value));
};

$('undo').onclick = () => {
  if (!tl) return;
  // 118: undo drops the game's last action, not the shown step's: at an earlier step it
  // dropped one the player couldn't see, unasked (acting there asks, `confirmDropLater`).
  if (tl.cursor < tl.last && !confirm(`Undo drops the game's last action (step ${tl.last}), not the step shown (${tl.cursor}). Go on?`)) return;
  const [before, stopped] = [tl.last, !!tl.replayError];
  tl.undo();
  // 285: a dropped action (or a stopped game's unreplayed tail) makes the record a played game, as acting does: its
  // old warning no longer applies.
  if (tl.last < before || (stopped && !tl.replayError)) showError(null);
  pin(null);
  commit();
};

/** One uniformly random legal action at the last step; false once the game is over. */
function randomAction(): boolean {
  pin(null);
  stayWith(null);
  tl.seek(tl.last);
  const moves = legalActions(tl.frame.snap);
  if (!moves.length) return false;
  tl.act(moves[Math.floor(Math.random() * moves.length)]);
  return true;
}

function randomly(times: number) {
  if (!tl) return;
  guard(() => {
    for (let i = 0; i < times && randomAction(); i++);
  });
  commit();
}

$('random-action').onclick = () => randomly(1);
$('random-end').onclick = () => randomly(100_000);

function stopAutoplay() {
  if (autoplay !== null) clearInterval(autoplay);
  autoplay = null;
}

$('first').onclick = () => seek(0);
$('back').onclick = () => tl && seek(tl.cursor - 1);
$('fwd').onclick = () => tl && seek(tl.cursor + 1);
$('end').onclick = () => tl && seek(tl.last);
ui.scrub.oninput = () => seek(Number(ui.scrub.value));

/** The step where the round shown at `i` began (its `round_started` frame, or 0). */
function roundStart(i: number): number {
  const r = tl.frames[i].snap.state.round;
  while (i > 0 && tl.frames[i - 1].snap.state.round === r) i--;
  return i;
}

/**
 * Page Up / Page Down (022, D4): back to the start of the shown round, or of the round
 * before when already there; on to the start of the next round, or the last step.
 */
function jumpRound(dir: -1 | 1) {
  if (dir < 0) {
    const s = roundStart(tl.cursor);
    seek(s < tl.cursor || s === 0 ? s : roundStart(s - 1));
  } else {
    const r = tl.frame.snap.state.round;
    let i = tl.cursor;
    while (i < tl.last && tl.frames[i].snap.state.round === r) i++;
    seek(i);
  }
}

/** Autoplay speed (022, D4): 1x is 4 steps a second; 16x moves 4 steps a tick. */
const SPEED_KEY = 'aic.speed';
let speed = Number(storageGet(SPEED_KEY)) || 1;
if (![1, 4, 16].includes(speed)) speed = 1;
ui.speed.value = String(speed);

function startAutoplay() {
  const perTick = Math.max(1, speed / 4);
  autoplay = window.setInterval(() => {
    if (tl.atEnd) stopAutoplay();
    else {
      tl.seek(Math.min(tl.last, tl.cursor + perTick));
      tl.save();
    }
    render();
  }, 250 / Math.min(speed, 4));
}

ui.speed.onchange = () => {
  speed = Number(ui.speed.value) || 1;
  storageSet(SPEED_KEY, String(speed));
  if (autoplay !== null) {
    stopAutoplay();
    startAutoplay();
  }
  // Give the keys (Space, arrows, Page Up/Down) back to the timeline.
  ui.speed.blur();
};

ui.play.onclick = () => {
  if (!tl) return;
  if (autoplay !== null) stopAutoplay();
  else {
    if (tl.atEnd) tl.seek(0);
    startAutoplay();
  }
  render();
};

// 235: how focus last moved: a pointer click focuses its button unseen (:focus-visible turns on at the next keydown in
// Chromium, so it can't tell); Tab or an arrow key moves focus on purpose.
let pointerFocus = false;
document.addEventListener('pointerdown', () => { pointerFocus = true; }, true);

// 289 (D127): the player view sizes the draft's printed cards to the window: a resize redraws it.
let resizeQueued = false;
window.addEventListener('resize', () => {
  if (mode !== 'player' || resizeQueued || !tl) return;
  resizeQueued = true;
  requestAnimationFrame(() => { resizeQueued = false; render(); });
});

document.addEventListener('keydown', (e) => {
  if (e.key === 'Tab' || e.key.startsWith('Arrow')) pointerFocus = false;
  // Keys and the toolbar do nothing until the engine has loaded (020: they threw).
  if (!tl) return;
  // 285: nor behind the open record dialog (a click on its text leaves the focus on the body, outside it).
  if (ui.recordDialog.open) return;
  const target = e.target as Element;
  // The slider keeps focus after a drag: it moves itself with the arrows, Home and End,
  // and the other keys still work (020: Space, R and Ctrl+Z stopped after using it).
  if (target.closest('input[type=range]') && ['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(e.key)) return;
  if (target.closest('input:not([type=range]), textarea, select, dialog')) return;
  // Space on a focused button presses that button (020: it started autoplay instead), when the focus came from the
  // keyboard (235: a mouse click leaves its button focused, unseen, and Space then took, undid or replaced a game).
  if (e.key === ' ' && target.closest('button') && !pointerFocus) return;
  // 242: the player view has no timeline: its keys (and R) would move the game unseen.
  // 261: and Space there must not press the header button a click left focused (New game, Undo, the view toggle):
  // a keyboard-focused button got its Space above.
  if (mode === 'player' && ['ArrowLeft', 'ArrowRight', 'Home', 'End', 'PageUp', 'PageDown', ' ', 'r'].includes(e.key)) {
    if (e.key === ' ' && target.closest('button')) e.preventDefault();
    // 321: ← → step the open "Since your last move" panel, as its ◀ ▶ do (the game itself never moves by a key here).
    if ((e.key === 'ArrowLeft' || e.key === 'ArrowRight') && turnOpen() && !e.ctrlKey && !e.metaKey && !e.altKey) {
      e.preventDefault();
      turnStep(e.key === 'ArrowLeft' ? -1 : 1);
      render();
    }
    return;
  }
  const keys: Record<string, () => void> = {
    ArrowLeft: () => seek(tl.cursor - 1),
    ArrowRight: () => seek(tl.cursor + 1),
    Home: () => seek(0),
    End: () => seek(tl.last),
    PageUp: () => jumpRound(-1),
    PageDown: () => jumpRound(1),
    ' ': () => ui.play.click(),
    r: () => $('random-action').click(),
    v: () => $('view-toggle').click(),
  };
  if (e.key === 'z' && (e.ctrlKey || e.metaKey)) {
    e.preventDefault();
    // 324: the player view's Undo stays inside the action phase (`undoRoom`).
    if (mode === 'player') undoPlayer(1);
    else $('undo').click();
    return;
  }
  // Leave browser shortcuts (Ctrl+R, Alt+←, ...) alone.
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  const f = keys[e.key];
  if (f) {
    e.preventDefault();
    f();
  }
});

// ---------- records ----------

function currentRecord(toCursor: boolean): string {
  const actions = toCursor ? tl.actionsToCursor : tl.actions;
  const snap = toCursor ? tl.frame.snap : tl.frames[tl.last].snap;
  return recordText(snap, actions, ui.recordNote.value.trim());
}

async function copy(text: string) {
  ui.recordText.value = text;
  try {
    await navigator.clipboard.writeText(text);
    ui.recordMsg.textContent = `Copied (${text.length} characters).`;
  } catch {
    ui.recordText.select();
    ui.recordMsg.textContent = 'Clipboard unavailable: the record is selected below, copy it with Ctrl+C.';
  }
}

$('record').onclick = () => {
  if (!tl) return;
  // 099: autoplay kept moving the shown step behind the dialog.
  stopAutoplay();
  render();
  ui.recordNote.value = tl.source.kind === 'record' ? tl.source.note : '';
  ui.recordText.value = currentRecord(false);
  // 099: a game that stopped replaying copies only the actions that replayed.
  // 184: and still says it is stamped as this engine's revision.
  ui.recordMsg.textContent = [
    tl.replayError ? `This game stopped replaying (the error bar names the action): the copy holds only the ${tl.last} actions that replayed.` : '',
    restamped(),
  ].filter(Boolean).join(' ');
  ui.recordDialog.showModal();
};

/**
 * 118: a copy is stamped with this engine's revision and its own replay's fingerprint,
 * so an older or diverged record copied here checked clean, unsaid.
 */
function restamped(): string {
  const old = tl.savedRules !== undefined && tl.savedRules !== rulesRevision;
  return old || tl.warning
    ? `This game was ${old ? `recorded under rules revision ${tl.savedRules}` : 'loaded with warnings'}: a copy is stamped as this engine's revision ${rulesRevision} replay, so it may not be the game recorded.`
    : '';
}

/** 118: the caveat stays beside 'Copied' (it was replaced). */
const copyWithCaveat = async (text: string, toCursor = false) => {
  await copy(text);
  // A copy of a game that stopped replaying holds only the actions that replayed: say so too
  // (up to the shown step, that many).
  const cut = tl.replayError ? `The copy holds only the ${toCursor ? tl.cursor : tl.last} actions that replayed.` : '';
  const c = [cut, restamped()].filter(Boolean).join(' ');
  if (c) ui.recordMsg.textContent += ` ${c}`;
};
$('record-copy').onclick = () => void copyWithCaveat(currentRecord(false));
$('record-copy-cursor').onclick = () => void copyWithCaveat(currentRecord(true), true);
$('record-close').onclick = () => ui.recordDialog.close();
$('record-load').onclick = () => {
  try {
    loadRecordText(ui.recordText.value, 'pasted record');
  } catch (e) {
    ui.recordMsg.textContent = e instanceof Error ? e.message : String(e);
  }
};

/** Loads a chosen or dropped file. An unreadable one (a folder, a locked file) says so (020: nothing happened). */
function openFile(f: File) {
  if (!tl) return; // 118: as New game (an unloaded wasm threw a raw TypeError)
  void f.text().then((t) => guard(() => loadRecordText(t, f.name)), (e) => showError(`could not read ${f.name}: ${e}`));
}

$('open-record').onclick = () => ui.recordFile.click();
ui.recordFile.onchange = () => {
  const f = ui.recordFile.files?.[0];
  if (f) openFile(f);
  ui.recordFile.value = '';
};
document.addEventListener('dragover', (e) => e.preventDefault());
document.addEventListener('drop', (e) => {
  e.preventDefault();
  const f = e.dataTransfer?.files?.[0];
  if (f) openFile(f);
});

// ---------- start ----------

/** Whether two JSON texts hold the same value (whitespace and key order aside). */
function sameJson(a: string, b: string): boolean {
  const norm = (v: unknown): unknown =>
    Array.isArray(v) ? v.map(norm)
    : v && typeof v === 'object' ? Object.fromEntries(Object.entries(v).sort(([x], [y]) => x.localeCompare(y)).map(([k, x]) => [k, norm(x)]))
    : v;
  try {
    return JSON.stringify(norm(JSON.parse(a))) === JSON.stringify(norm(JSON.parse(b)));
  } catch {
    return a === b;
  }
}

async function main() {
  // 325: which cards have a scene (the dev server's), before the first card prints.
  await Promise.all([load(), loadScenes()]);
  setMode(mode);
  ui.version.textContent = `${version} · rules r${rulesRevision}`;
  $('legend').innerHTML = legendHtml();
  ui.config.innerHTML = Object.keys(CONFIGS).map((f) => `<option>${escapeHtml(f)}</option>`).join('');
  // 323: solo's board, the Builder suggested for a first game.
  ui.boardChoice.innerHTML = BOARD_NAMES.map((n, i) => `<option value="${i}">${escapeHtml(n)}${i === 0 ? ' (suggested)' : ''}</option>`).join('');
  ui.config.addEventListener('change', () => syncBoardPick(true));
  ui.ndBoard.innerHTML = ui.boardChoice.innerHTML;
  ui.ndConfig.innerHTML = ui.config.innerHTML;
  let restored: Timeline | null = null;
  let restoreError: string | null = null;
  try {
    restored = Timeline.restore();
  } catch (e) {
    // 118: the session is kept (`Timeline.restore`), not lost under the new game: say how to get it.
    restoreError = `could not restore the saved session (a config from an older engine?): ${e}. `
      + 'Its actions are kept in this browser: run copy(aic.keptSession()) in the console to copy them as a record';
  }
  if (restored) {
    const file = restored.source.file;
    if (CONFIGS[file] !== undefined) ui.config.value = file;
    syncBoardPick(true);
    // 323: a solo session keeps the board it was started on.
    if (soloConfig(restored.configJson)) {
      const ai = (JSON.parse(restored.configJson).modules as { module: string; params: { boards?: number[] } }[]).find((m) => m.module === 'ai');
      if (ai?.params.boards?.length) ui.boardChoice.value = String(ai.params.boards[0]);
    }
    ui.seed.value = restored.seed;
    setTimeline(restored);
    // 099: a session saved under other rules says so, replayed or not.
    const other = restored.savedRules !== undefined && restored.savedRules !== rulesRevision
      // 261: worded to follow "saved session: " (it read "saved session: saved session from ...").
      ? `from rules revision ${restored.savedRules} (this engine plays ${rulesRevision}): `
      : '';
    // 184: a session that stopped replaying is kept whole (`Timeline.restore`): say how to get it.
    if (restored.replayError && restored.savedStopped) showError(`saved session: ${other}${restored.replayError} (it stopped here when loaded; an action or Undo makes the replayed part this game)`);
    else if (restored.replayError) showError(`saved session: ${other}${restored.replayError}. Its full actions are kept in this browser: run copy(aic.keptSession()) in the console`);
    // 118: a watched record's warnings come back on a reload (184: with the revision note, as they were written then).
    else if (restored.warning) showError(`saved session: ${other}${restored.warning}`);
    else if (other) showError(`saved session ${other}it replayed, but may not be the game it was`);
    // The session keeps the config it started with: say so when the file has changed
    // since (a retuned baseline), or the game shown is not what the file now plays.
    else if (restored.source.kind === 'play' && CONFIGS[file] !== undefined && !sameJson(withBoard(CONFIGS[file], ui.boardChoice.value), restored.configJson))
      showError(`saved session: this game was started with an older copy of ${file}; New game plays the current one`);
    else if (restored.source.kind === 'play' && CONFIGS[file] === undefined && file.endsWith('.json'))
      showError(`saved session: ${file} is no longer in configs/; this game keeps the copy it started with`);
  } else {
    // 325 (Matt, Nov 25): a fresh browser plays solo, the new-game screen over it offering the tutorial first.
    // `aic.firstConfig` in localStorage: the scripts' own start (scripts/cdp.mjs keeps theirs on baseline.json).
    const first = [storageGet('aic.firstConfig'), TUTORIAL.file, 'baseline.json'].find((f) => f && CONFIGS[f] !== undefined);
    ui.config.value = first ?? Object.keys(CONFIGS)[0];
    syncBoardPick(true);
    ui.seed.value = '1';
    newGame(ui.config.value, '1');
    // After the new game, which clears the error bar (020: the reason was wiped at once).
    if (restoreError) showError(restoreError);
    // (Not over a restore error: the dialog would hide the bar saying how to get the session back.)
    if (mode === 'player' && !restoreError) openNewGame();
  }
}

main().catch((e) => showError(`failed to start: ${e instanceof Error ? e.stack : e}`));

// For the scripts in scripts/ (e2e, sweep, odd, shots) and the console. The helpers go through the
// same paths as the buttons (render included); `play` takes a config's text, so a script
// can start a game on any override of a config file.
Object.assign(window, {
  aic: {
    get timeline() { return tl; },
    get view() { return shown; },
    get legal() { return legal; },
    /** 305: the view the map shows (an earlier step's while the player view's turn panel shows one). */
    get mapView() { return mapView; },
    play: (configJson: string, seed: string, file = 'script') => setTimeline(Timeline.start(configJson, seed, { kind: 'play', file })),
    open: (text: string, file = 'script') => loadRecordText(text, file),
    act,
    seek,
    /** 242: 'player' or 'debug'; `setView` switches, as the toggle does (326: for this page only). */
    get mode() { return mode; },
    setView: (m: Mode) => setMode(m),
    /** 242: the seat the player view shows; `showSeat(null)` follows whoever must act again. */
    get focus() { return focusSeat(shown, legal); },
    showSeat: (s: number | null) => { pin(s); render(); },
    keptSession,
    get error() { return ui.error.hidden ? null : ui.error.textContent; },
    /** 325: the tutorial (its state: the step shown, every step's id), and the new-game screen. */
    tutorial: { start: startTutorial, stop: stopTutorial, get state() { return tutorialState(); }, get active() { return tutorialActive(); } },
    newGameScreen: openNewGame,
  },
});
