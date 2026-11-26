// 325: the tutorial: a scripted first game of solo (configs/solo.json, the Builder, one fixed seed) that teaches the
// game inside the player view, in docs/teach.md's order and words where they fit. A step is a short prompt pinned to
// what it names on screen (outlined); an info step waits for *Next*, a "do" step for the board to show it done (a node
// placed, an ambassador moved in, a city liberated), and *Skip step* always moves on. It suggests, never forces: every
// legal move stays open. After the first Liberate it hands over and the game plays on. No rules live here: every
// number it quotes is read from the view, the page the view drew, or the game's config (`param`).

import type { Action, Config, View } from './engine';
import { escapeHtml as esc, hexKey } from './format';
import { storageGet, storageSet } from './session';

/** The tutorial's game: solo.json on the Builder (board 0) at this seed (picked in 325: a threat up in round 1, the
 *  corrupted cities near the first node, the game not lost before round 11 on the sim policy's line). */
export const TUTORIAL = { file: 'solo.json', board: '0', seed: '24' };

const KEY = 'aic.tutorial';

interface Saved {
  seed: string;
  step: number;
  /** The round the first Liberate came in (the hand-off waits for the round after it to start). */
  freedRound?: number;
  /** Your nodes on the map when it started (the node step waits for one more). */
  nodes?: number;
  /** The corrupted cities (hex indexes) when it started: the Liberate step waits for one to be free. */
  cities?: number[];
  done?: boolean;
}

export interface TutorialHost {
  view(): View;
  legal(): Action[];
  /** The game's config, resolved (its params are the numbers the prompts quote). */
  config(): Config;
  /** Whether the player view is showing (the tutorial hides in the debug view). */
  playerMode(): boolean;
  /** The seed of the game on screen, and whether it is a play of solo.json on the Builder. */
  game(): { seed: string; file: string; board: number | null };
  /** Starts the tutorial's game afresh. */
  restart(): void;
}

// ---------- reading the view ----------

const count = (v: View, i: number, kind: string) => v.hexes[i].tokens.find((t) => t.kind === kind)?.count ?? 0;
const val = (v: View, k: string) => v.seats[0]?.values.find((x) => x.key === k)?.value ?? 0;
const phase = (v: View) => v.phase?.name ?? '';
/** The cards phases before the action phase: build (tier), draft (none in solo since 326), keep. */
const drafting = (v: View) => /^cards\.(tier|draft|keep)$/.test(phase(v));
const acting = (v: View) => /board\.act/.test(phase(v)) && !v.outcome;
const corrupted = (v: View) => v.hexes.filter((h) => count(v, h.index, 'cube:adversary') > 0);
const corruptedCities = (v: View) => corrupted(v).filter((h) => h.kind === 'city');
const nodes = (v: View) => v.hexes.filter((h) => count(v, h.index, 'node:0') > 0);
const ambassadors = (v: View) => v.hexes.filter((h) => count(v, h.index, 'ambassador:0') > 0);
const hexSel = (_v: View, hs: { hex: [number, number] }[]) => hs.map((h) => `#board .hex[data-hex="${hexKey(h.hex)}"]`);
/** A city as the map prints it: its hex number. */
const cityName = (_v: View, h: { number: number }) => `#${h.number}`;
const liberateBill = (v: View) => v.standard.find((s) => s.act === 'liberate')?.sites?.find((s) => s.kind === 'city')?.ambassadors;
const roundCap = () => /of (\d+)/.exec(document.querySelector('#phase-bar')?.textContent ?? '')?.[1];
const pileCount = (v: View, k: string) => v.piles.find((p) => p.key === k)?.count;
/** A module's param in the game's (resolved) config: a number, a list's first; undefined when it has none. */
let config: Config | null = null;
const param = (mod: string, key: string): number | undefined => {
  const p = config?.modules.find((m) => m.module === mod)?.params[key];
  return Array.isArray(p) ? p[0] : p;
};
const freed = (v: View, s: Saved) => (s.cities ?? []).some((i) => count(v, i, 'cube:adversary') === 0);
/** The Move standard action's button, and Liberate's. */
const stdButton = (act: string) => `#p-act button[data-action*='"act":"${act}"']`;

// ---------- the steps ----------

interface Step {
  id: string;
  title: string;
  /** The prompt's HTML. */
  body(v: View): string;
  /** What it points at: selectors, outlined (each that is on screen). */
  at(v: View): string[];
  /** When the step can show; until then a small line says what comes (`wait`). */
  when?(v: View, s: Saved): boolean;
  wait?: string;
  /** A "do" step: done when this holds (the tutorial moves on by itself). */
  done?(v: View, s: Saved): boolean;
  /** The button of an info step (default *Next*). */
  next?: string;
}

const list = (items: string[]) => `<ul>${items.map((x) => `<li>${x}</li>`).join('')}</ul>`;

const STEPS: Step[] = [
  {
    id: 'welcome',
    title: 'What you are',
    body: () => `<p>You are a good AI, working against one evil AI: <b>the adversary</b>. It sends <b>agents</b> onto the
      world's cities, research stations and factories. When enough agents crowd onto one, it is <b>corrupted</b>: the
      adversary's.</p><p>You spread <b>nodes</b> over the map, send <b>ambassadors</b> onto corrupted places to take them
      back, and build a tableau of cards that pays for it all. This is a solo game: one board (the Builder) and one
      country, the map in the middle.</p>`,
    at: () => ['#board-wrap'],
  },
  {
    id: 'win',
    title: 'How you win',
    body: (v) => {
      const cs = corruptedCities(v);
      return `<p><b>You win the moment no city on the map is corrupted.</b> Your country starts with ${param('board', 'start_corrupted') ?? 'its'} corrupted cities${cs.length ? `, outlined: ${cs.map((h) => cityName(v, h)).join(' and ')}` : ''} (the adversary's cube on them). Take every
        corrupted city back and you win, in any round. Nothing else counts.</p>`;
    },
    at: (v) => hexSel(v, corruptedCities(v)),
  },
  {
    id: 'lose',
    title: 'How you lose',
    body: (v) => {
      const discord = pileCount(v, 'alignment.deck.discord');
      const cap = roundCap();
      return `<p>You lose if any one of four clocks runs out:</p>${list([
        `the Discord deck: ${discord !== undefined ? `${discord} cards` : 'its cards'}, never reshuffled; when Discord tips and finds it empty, you lose;`,
        'your nodes: if you have no node left on the map, you lose;',
        `the adversary's event deck: one card a round${cap ? `, ${cap} rounds at most` : ''};`,
        "the adversary's research row: when its marker reaches its last slot (Loss), you lose.",
      ])}<p>The Alignment and the clocks are here, the research row atop this column.</p>`;
    },
    at: () => ['#status .alignment'],
  },
  {
    id: 'round',
    title: 'A round, in order',
    body: () => `<p>Five steps, the bar above the map lighting the one under way:</p>${list([
      "<b>Adversary.</b> Each Fallout token on the map adds a Discord (Sway); then the adversary's turn, two beats: its <b>Card</b> beat, then its <b>Income</b> beat.",
      '<b>Cards.</b> Build a hand, then keep.',
      '<b>Actions.</b> In any order, until you pass.',
      '<b>Income.</b> Your factories, cities, stations and income pay.',
      '<b>Threats.</b> Every threat nobody paid off takes effect; then cleanup stands your ambassadors back up.',
    ])}<p>Notice the adversary acts before you do, and its next card is already face up: you always see its next move
      coming. Round 1's adversary turn has run already.</p>`,
    at: () => ['#phase-bar'],
  },
  {
    id: 'card',
    title: "The adversary's next card",
    body: () => `<p>This is the adversary's card for next round, face up. In its <b>Card</b> beat the card:</p>${list([
      'deals <b>threats</b> into your country;',
      '<b>Escalate</b>: every site of the kind last round\'s card named, and every factory, that still holds an agent takes one more;',
      "<b>Infiltrate</b>: one agent onto every site of the kind it names that isn't the adversary's yet;",
      'a <b>Strike</b> or <b>Redeploy</b> icon, if it shows one, for the troops; and at most one other line.',
    ])}<p>Agents corrupt a site <b>the moment they land</b> and reach its number. ${param('adversary', 'corrupt_node_at') ?? 'Enough'} agents landing on your node's hex
      destroy the node instead.</p>`,
    at: () => ['#pipeline .slot-box.next'],
  },
  {
    id: 'income',
    title: "The adversary's Income beat",
    body: () => `<p>Then <b>each corrupted site makes one thing</b>: a city makes a Discord; a station puts a cube on the
      research row; a factory pushes its agents along its arrow onto its city, then, if empty, makes a new one. Your ambassador
      standing on any of them stops it.</p><p>All the compute goes onto the leftmost <b>project</b>; when one fills, it
      fires. The research row advances a slot for every few cubes, and each slot's icon happens once. Stop its cubes by
      standing on, or liberating, corrupted stations.</p>`,
    at: () => ['#pipeline .projects-row', '#pipeline .research-row'],
  },
  {
    id: 'build',
    title: 'Cards: build your hand',
    // 326 (Matt, Nov 25): solo's draft hand is 4 cards, the hand you build: nothing is taken from it or passed.
    body: () => `<p>Each round you build a hand of <b>${param('cards', 'deal') ?? 'your'} cards</b>, one at a time, from either deck: a tier-1
        card costs ${param('cards', 'keep_tier1') ?? 'less'} insight to keep, a tier-2 card ${param('cards', 'keep_tier2') ?? 'more'} and is stronger.
        Each card arrives face up, so you see it before choosing the next.</p><p>Pick a deck for each card now.</p>`,
    at: () => ['#p-draft .ptiers'],
    when: (v) => phase(v) === 'cards.tier',
    wait: 'the cards phase',
    done: (v) => phase(v) !== 'cards.tier',
  },
  {
    id: 'keep',
    title: 'Keep what you can pay for',
    body: (v) => `<p>Keep what you built for <b>insight</b>: ${param('cards', 'keep_tier1') ?? 'less'} for a tier-1 card, ${param('cards', 'keep_tier2') ?? 'more'} for a tier-2 (each Keep button shows its price). You have
      <b>${val(v, 'resource.insight')}</b> insight, and what you don't spend is gone; the cards you don't keep are discarded.</p>
      <p><b>Programs</b> stay in front of you and are where lasting income comes from. <b>Buildings</b> go in your slots
      (the Builder has ${val(v, 'cards.slots') || 'five'}): an Action once a round, or an Effect that pays when its trigger happens. <b>Events</b> are
      used once. Every card shows tags: they unlock other cards and pay for your board's sacrifice powers. Hover a card
      to read it. Keep the cards you want, then <i>Done keeping</i>.</p>`,
    at: () => ['#p-draft', '#p-draft .pass'],
    when: (v) => phase(v) === 'cards.keep',
    wait: 'the keep',
    done: (v) => !drafting(v),
  },
  {
    id: 'board',
    title: 'Your board',
    body: () => `<p>Your board, as a strip of numbers (click <i>Builder</i> for its whole face). Your nodes sit on two
      tracks: place one and the space you uncover is yours for good (income, more insight, a tag). Your <b>insight</b> is the
      number on the rightmost card-play space you've uncovered.</p><p>Energy comes from cities: <b>every free city with
      your node on or next to it pays you ${param('board', 'corp_energy') ?? 'its'} energy a round</b>. One agent on a city shuts it off.</p>`,
    at: () => ['#p-seats'],
    when: acting,
    wait: 'your action phase',
  },
  {
    id: 'node',
    title: 'Place a node',
    body: (v) => `<p>Nodes come from your three <b>node powers</b> (and from cards): one cheap power that's free, and two
      costly ones with a bonus. Each works once a round, and every placement pays the energy printed on the space you
      uncover.</p><p><b>Use a node power now</b> and put the node <b>next to a corrupted city</b>${corruptedCities(v).length ? ` (${corruptedCities(v).map((h) => cityName(v, h)).join(' or ')})` : ''}: anything you target must be in your reach, on or next to a node, and an ambassador can arrive there.</p>`,
    at: (v) => v.choice ? ['#prompt', ...hexSel(v, corruptedCities(v))] : ['#p-act .ppowers.node'],
    when: acting,
    wait: 'your action phase',
    done: (v, s) => nodes(v).length > (s.nodes ?? 1),
  },
  {
    id: 'arrive',
    title: 'Bring an ambassador in',
    body: () => `<p><b>Each of your ambassadors on the map does one thing a round</b>, then tips over until cleanup: it
      <b>moves</b> to a hex next to it or to any of your nodes, or it <b>acts</b> where it stands. From your supply they come
      onto your nodes, free, and arrive standing, ready to go.</p><p><b>Move</b> one from your supply onto your node next to
      the corrupted city.</p>`,
    at: (v) => v.choice ? ['#prompt'] : [stdButton('move')],
    when: acting,
    wait: 'your action phase',
    done: (v) => ambassadors(v).length > 0,
  },
  {
    id: 'movein',
    title: 'Move in',
    body: (v) => `<p>Taking a site back is <b>move in, then act</b>. <b>Move</b> your ambassador onto the corrupted city
      next to it. While it stands there, the site makes nothing for the adversary; next round it stands up and acts.</p>
      <p>When an agent lands on your ambassador, or your ambassador walks onto an agent, both are removed: one for one.${corruptedCities(v).some((h) => count(v, h.index, 'ambassador:adversary') > 0) ? ' A city with an agent on it must be cleared before it can be liberated.' : ''}</p>`,
    at: (v) => v.choice ? ['#prompt', ...hexSel(v, corruptedCities(v))] : [stdButton('move'), ...hexSel(v, corruptedCities(v))],
    when: acting,
    wait: 'your action phase',
    done: (v) => corrupted(v).some((h) => count(v, h.index, 'ambassador:0') > 0),
  },
  {
    id: 'threats',
    title: 'Threats',
    body: () => `<p>Each threat sits on one or two sites in your country. If your node or ambassador is on or next to one of
      its tokens, you're <b>near</b> it and may pay it off: one way in full, or sacrificing cards with its tag (the
      <i>Counter threats</i> buttons). Unpaid at the end of the round, it does what it prints to you, and lands one more agent
      on its spot.</p>`,
    at: () => ['#countries'],
    when: acting,
    wait: 'your action phase',
  },
  {
    id: 'pass',
    title: 'Done for the round',
    body: () => `<p>Use anything else you like (hover a card or power to read it), then <b>Done (pass)</b>. Then production
      pays, threats nobody paid take effect, cleanup stands your ambassadors back up, and round 2 starts with the
      adversary's turn.</p>`,
    at: () => ['#p-act .pa-foot button[data-action]'],
    when: acting,
    wait: 'your action phase',
    done: (v) => v.round > 1,
  },
  {
    id: 'replay',
    title: "The adversary's turn",
    body: () => `<p>The adversary's turn ran before your cards. <b>Since your last move</b> (or <i>Replay the adversary's
      turn</i>) steps through it, ◀ ▶, the map as it stood at each step.</p><p><b>Read the face-up card every round.</b> It
      tells you which sites take agents and whether troops strike.</p>`,
    at: () => ['#p-turn'],
    when: (v) => v.round > 1,
    wait: 'round 2',
  },
  {
    id: 'cards2',
    title: 'Cards again',
    body: () => `<p>Build and keep, as in round 1. Next you liberate the city.</p>`,
    at: () => ['#p-draft'],
    when: (v) => v.round > 1 && drafting(v),
    wait: "round 2's cards",
    // Round 2's action phase reached (not merely "not drafting": a question in the adversary's turn comes first).
    done: (v) => v.round > 1 && acting(v),
  },
  {
    id: 'liberate',
    title: 'Liberate',
    body: (v) => {
      const bill = liberateBill(v);
      const have = val(v, 'resource.ambassadors');
      return `<p>Your ambassador stands on the city, ready: it acts, <b>Liberate</b>. Pay in ambassadors: ${bill ?? 'the bill'} for a
        city, the ones on the site first, then your supply (you have ${have} there). The adversary's cube comes off, the
        site is neutral again, and you gain Unity. You need no reach: your ambassador carries it.</p>
        <p>Short of ambassadors? <b>Gain a player ambassador</b> (a standard action) buys one. Two things stop a Liberate: <b>an agent on the
        site</b>, and <b>a troop on or next to it</b>; the Liberate line under the standard actions says what blocks it.</p>`;
    },
    at: () => ['#p-act .plib', stdButton('liberate'), stdButton('ambassador')],
    when: (v) => v.round > 1 && acting(v),
    wait: "round 2's action phase",
    // A city corrupted at the start is free (another may have fallen meanwhile: the count can stay the same).
    done: (v, s) => freed(v, s),
  },
  {
    id: 'unity',
    title: 'Unity and Discord',
    body: () => `<p>Two separate tracks. Liberating sways <b>Unity</b>; corrupted cities, Fallout, threats and projects sway
      <b>Discord</b>. When a track fills it <b>tips</b>: draw that side's card. Unity cards help; Discord cards hurt, and
      their deck is a loss clock. <b>Unity never cancels Discord.</b></p>`,
    at: () => ['#status .alignment'],
  },
  {
    id: 'handoff',
    title: "You're on your own",
    body: (v) => {
      const left = corruptedCities(v);
      return `<p>That's the game. ${left.length ? `Take ${left.map((h) => cityName(v, h)).join(' and ')} back and you win.` : ''} Three tips:</p>${list([
        '<b>Read the face-up card every round.</b> Clear the agent that would escalate; move pieces out of a Strike\'s reach.',
        '<b>Keep a second node on the map</b>, and never let one sit alone next to agents.',
        '<b>Liberate early.</b> Stand on corrupted factories to stop their agents, and deal with a troop before it garrisons next to the cities you need.',
      ])}<p>Hover anything for its rules. <i>New game…</i> at the top starts another, or this tutorial again.</p>`;
    },
    at: () => [],
    // After the round of the first Liberate has run its threats (the next round's first decision).
    when: (v, s) => s.freedRound === undefined || v.round > s.freedRound || !!v.outcome,
    wait: "the next round (finish this one: Done (pass) when you're through)",
    next: 'Play on',
  },
];

// ---------- the controller ----------

let host: TutorialHost | null = null;
let saved: Saved | null = null;
let box: HTMLElement;
let rings: HTMLElement;
/** The step (and question state) whose targets were last scrolled into view. */
let scrolledFor = '';

function load(): Saved | null {
  try {
    const s = JSON.parse(storageGet(KEY) ?? 'null');
    return s && typeof s.step === 'number' ? s : null;
  } catch {
    return null;
  }
}
const save = () => storageSet(KEY, JSON.stringify(saved));

/** Whether the game on screen is the tutorial's. */
function onTutorialGame(): boolean {
  if (!host || !saved) return false;
  const g = host.game();
  return g.seed === saved.seed && g.file === TUTORIAL.file && g.board === Number(TUTORIAL.board);
}

export function initTutorial(h: TutorialHost) {
  host = h;
  saved = load();
  box = document.createElement('aside');
  box.id = 'tutorial';
  box.hidden = true;
  rings = document.createElement('div');
  rings.id = 'tutorial-rings';
  document.body.append(rings, box);
  box.addEventListener('click', (e) => {
    const b = (e.target as Element).closest('[data-tut]') as HTMLElement | null;
    if (!b || !saved) return;
    const what = b.dataset.tut;
    if (what === 'next' || what === 'skip') go(saved.step + 1);
    else if (what === 'back') go(Math.max(0, saved.step - 1), true);
    else if (what === 'exit') stopTutorial();
    else if (what === 'restart') startTutorial();
  });
  addEventListener('resize', () => updateTutorial());
}

/** Starts (or restarts) the tutorial: its game afresh, at step 1. */
export function startTutorial() {
  saved = { seed: TUTORIAL.seed, step: 0 };
  save();
  host?.restart();
  updateTutorial();
}

/** Ends it: the game plays on, the prompts are gone. */
export function stopTutorial() {
  if (saved) {
    saved.done = true;
    save();
  }
  updateTutorial();
}

/** Whether a tutorial is running on the game shown. */
export const tutorialActive = () => !!saved && !saved.done && onTutorialGame();

/** For the scripts: the step shown, and every step's id. */
export function tutorialState() {
  const v = host?.view();
  const s = saved && STEPS[saved.step];
  return {
    active: tutorialActive(), step: saved?.step ?? null, id: s?.id ?? null, of: STEPS.length, ids: STEPS.map((x) => x.id),
    showing: !box.hidden && box.dataset.state === 'step', waiting: !box.hidden && box.dataset.state === 'wait',
    kind: s ? (s.done ? 'do' : 'info') : null, round: v?.round ?? null,
  };
}

function go(i: number, back = false) {
  if (!saved || !host) return;
  const v = host.view();
  // Back passes over a "do" step already done (it would move on again at once).
  while (back && i > 0 && STEPS[i].done?.(v, saved)) i--;
  saved.step = i;
  if (i >= STEPS.length) saved.done = true;
  save();
  updateTutorial();
}

/** Redraws the prompt and outlines; after every render of the page. */
export function updateTutorial() {
  if (!host || !box) return;
  rings.innerHTML = '';
  if (!tutorialActive() || !host.playerMode()) {
    box.hidden = true;
    return;
  }
  const v = host.view();
  config = host.config();
  if (saved!.nodes === undefined || saved!.cities === undefined) {
    saved!.nodes = nodes(v).length;
    saved!.cities = corruptedCities(v).map((h) => h.index);
    save();
  }
  // A "do" step whose goal the board shows is done (the player did it, early or now).
  let moved = false;
  while (saved!.step < STEPS.length) {
    const s = STEPS[saved!.step];
    if (!(s.done && s.done(v, saved!))) break;
    if (s.id === 'liberate') saved!.freedRound = v.round;
    saved!.step++;
    moved = true;
  }
  if (moved) save();
  if (saved!.step >= STEPS.length) {
    saved!.done = true;
    save();
    box.hidden = true;
    return;
  }
  const s = STEPS[saved!.step];
  const n = saved!.step + 1;
  const foot = `<div class="tut-foot"><button type="button" data-tut="restart" title="Start the tutorial's game again, from step 1">Restart tutorial</button><button type="button" data-tut="exit" title="Leave the tutorial; the game plays on">Skip tutorial</button></div>`;
  if (s.when && !s.when(v, saved!)) {
    // Between steps (the player plays on): one line, out of the way.
    box.dataset.state = 'wait';
    box.className = 'tut-wait';
    box.innerHTML = `<div class="tut-head"><b>Tutorial</b> <small>${n} of ${STEPS.length}</small></div><p>Play on: the next tip comes with ${esc(s.wait ?? 'the next phase')}.${v.outcome ? ' The game is over.' : ''}</p><div class="tut-btns"><button type="button" data-tut="skip" title="Move on to the next tip now">Skip step</button></div>${foot}`;
    box.hidden = false;
    place(null);
    return;
  }
  box.dataset.state = 'step';
  box.className = s.done ? 'tut-do' : 'tut-info';
  const buttons = s.done
    ? `<button type="button" data-tut="skip" title="Move on without doing it">Skip step</button>`
    : `<button type="button" class="primary" data-tut="next">${esc(s.next ?? 'Next')}</button>`;
  box.innerHTML = `<div class="tut-head"><b>${esc(s.title)}</b> <small>Tutorial ${n} of ${STEPS.length}</small></div>
    <div class="tut-body">${s.body(v)}</div>
    <div class="tut-btns">${saved!.step > 0 ? '<button type="button" data-tut="back">◀ Back</button>' : ''}${s.done ? '<small class="tut-todo">waiting for you to do it</small>' : ''}${buttons}</div>${foot}`;
  box.hidden = false;
  const els = s.at(v).flatMap((sel) => [...document.querySelectorAll<Element>(sel)]).filter((e) => {
    const r = e.getBoundingClientRect();
    return r.width > 0 && r.height > 0;
  });
  // A target in a scrolled column (the adversary's, the actions') is brought into view, the first last (it wins).
  // Once a step (and on a question's answer, which redraws the targets): a later render leaves the player's scroll be.
  const key = `${s.id}:${!!v.choice}`;
  if (key !== scrolledFor) {
    scrolledFor = key;
    for (const e of [...els].reverse()) e.scrollIntoView({ block: 'nearest' });
  }
  for (const e of els) {
    const r = e.getBoundingClientRect();
    const d = document.createElement('div');
    d.className = 'tut-ring';
    Object.assign(d.style, { left: `${r.left - 3}px`, top: `${r.top - 3}px`, width: `${r.width + 6}px`, height: `${r.height + 6}px` });
    rings.append(d);
  }
  place(els.length ? els.map((e) => e.getBoundingClientRect()).reduce((a, r) => new DOMRect(
    Math.min(a.left, r.left), Math.min(a.top, r.top),
    Math.max(a.right, r.right) - Math.min(a.left, r.left), Math.max(a.bottom, r.bottom) - Math.min(a.top, r.top))) : null);
}

/** Puts the prompt beside `r` (right, left, below, above: the first side it fits), else over the map's foot. */
function place(r: DOMRect | null) {
  const w = box.offsetWidth, h = box.offsetHeight, pad = 10;
  const map = document.querySelector('#board-wrap')?.getBoundingClientRect();
  let x: number, y: number;
  const fits = (px: number, py: number) => px >= 4 && py >= 4 && px + w <= innerWidth - 4 && py + h <= innerHeight - 4;
  const sides = r ? [
    [r.right + pad, r.top], [r.left - pad - w, r.top], [r.left, r.bottom + pad], [r.left, r.top - pad - h],
  ] : [];
  const side = sides.find(([px, py]) => fits(px, Math.min(py, innerHeight - h - 4)));
  if (side) {
    x = side[0];
    y = Math.max(4, Math.min(side[1], innerHeight - h - 4));
  } else if (map) {
    x = map.right - w - 8;
    y = map.bottom - h - 8;
  } else {
    x = innerWidth - w - 8;
    y = innerHeight - h - 8;
  }
  box.style.left = `${Math.max(4, x)}px`;
  box.style.top = `${Math.max(4, y)}px`;
}
