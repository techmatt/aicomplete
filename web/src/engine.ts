// Typed wrapper over aic-wasm (crates/aic-wasm/src/lib.rs). No rules live here.
//
// States go back to the engine as the JSON string it produced; the parsed copy is for
// display. Seeds and RNG positions are decimal strings in that JSON, so nothing is lost.
// Shapes follow crates/aic-core/src/{state,action,event,view,record,map}.rs.

import init, * as wasm from './wasm/aic_wasm.js';

export type Hex = [number, number];

/**
 * An engine action. The client never builds one: it only sends back actions that
 * `legalActions` returned. Known fields are typed for labeling; any other field a
 * future action carries is kept and shown as-is.
 */
export interface Action {
  type: string;
  seat: number;
  card?: string;
  target?: Hex;
  [field: string]: unknown;
}

/** An engine event: `type` plus whatever fields it carries. */
export interface GameEvent {
  type: string;
  [field: string]: unknown;
}

export interface Outcome {
  won: boolean;
  round: number;
  reason: string;
  detail: string;
}

export interface GameState {
  seed: string;
  config: Config;
  round: number;
  phase: number;
  outcome: Outcome | null;
  [part: string]: unknown;
}

export interface Config {
  name?: string;
  countries: number;
  modules: { module: string; variant: string; params: Record<string, number | number[]>; cards?: string[] }[];
  round: string[];
}

// --- the generic view (crates/aic-core/src/view.rs) ---

export interface Value {
  key: string;
  label: string;
  value: number;
  max?: number;
  source: string;
  doc?: string;
}
export interface CardView { id: string; name: string; text?: string; status: string; cost?: number }
export interface CardGroup { key: string; label: string; source: string; cards: CardView[] }
export interface PileView { key: string; label: string; source: string; owner: number | null; count: number; visible: CardView[] }
export interface Token { kind: string; count: number }
export interface HexView {
  index: number;
  hex: Hex;
  country: number;
  piece_pos: [number, number];
  neighbors: number[];
  /** The printed hex number (1-16 on its piece). */
  number: number;
  /** What the hex is: `city`, `base` (a military base), `open` or `ocean`. */
  kind: string;
  /** For a military base: the country across its front. */
  facing?: number;
  /** A printed industrial zone: cities build factories here. */
  industrial?: boolean;
  tokens: Token[];
}
export interface CountryView {
  index: number;
  /** The printed country piece's name (content/map.json). */
  name: string;
  seat: number;
  color: string;
  hexes: number[];
  values: Value[];
  cards: CardGroup[];
}
/** Something printed on a track step; keys: a resource, `tier2`, a tag, `capstone`, `cost`. */
export interface TrackMark { key: string; label: string; doc?: string }
export interface TrackStep { value: number; marks?: TrackMark[] }
/** A printed track: the leftmost `uncovered` steps show, `current` is in effect. */
export interface TrackView { label: string; steps: TrackStep[]; uncovered: number; current: number }
/** A player board, whatever module shows it. */
export interface BoardView {
  key: string;
  source: string;
  id: string;
  name: string;
  text?: string;
  innate?: { name: string; text?: string };
  /** Tags printed on the board: they count for what a card requires (068). */
  tags?: string[];
  tracks: TrackView[];
  slots?: number;
  /** Cards belonging to the board, `status` saying where each is now. */
  cards: CardView[];
}
export interface SeatView { index: number; country: number; passed: boolean; values: Value[]; cards: CardGroup[]; board: BoardView | null }
export interface PhaseInfo { name: string; kind: 'auto' | 'decision' }
export interface PhaseView { index: number; name: string; kind: 'auto' | 'decision'; waiting: number[] }
export interface ChoiceView { seat: number; prompt: string }
export interface StandardView {
  act: string;
  energy: number;
  credits: number;
  chips: number;
  ambassadors: number;
  doc: string;
  /** Chips each seat pays where its cards set the price (the Harden action a card grants). */
  seat_chips?: number[];
}

export interface View {
  round: number;
  phase: PhaseView | null;
  round_order: PhaseInfo[];
  outcome: Outcome | null;
  hexes: HexView[];
  countries: CountryView[];
  seats: SeatView[];
  globals: Value[];
  piles: PileView[];
  /** The standard actions and their costs, as the rules publish them. */
  standard: StandardView[];
  /** The open choice, if the rules are waiting for one. */
  choice: ChoiceView | null;
  /** Where pieces are heading (019: each adversary troop's next target). */
  arrows: ArrowView[];
  /** Each face-up threat's source type, its source hexes and who may pay (070). */
  threat_sources: ThreatSourceView[];
}
/** What feeds a threat: hex indexes of its sources, and the seats that may counter it. */
export interface ThreatSourceView { threat: string; source: string; hexes: number[]; target?: number | null; seats: number[] }
/** A piece's heading, hex index to hex index; `rounds` 1 means it arrives this coming round. */
export interface ArrowView { kind: string; from: number; to: number; count: number; rounds: number; doc: string }

// --- records (crates/aic-core/src/record.rs) ---

export interface GameRecord {
  format: string;
  note?: string;
  config: Config;
  seed: string;
  actions: Action[];
  outcome?: Outcome | null;
  rounds?: number;
  final_fingerprint?: string;
  config_hash?: string;
  content_hash?: string;
  engine_version?: string;
  rules_revision?: number;
  /** Set by `parseRecord` when this engine rejects the record's config. */
  config_error?: string;
}

export interface CardDef { id: string; name: string; text: string; cost?: unknown }
export interface ThreatDef { id: string; name: string; tier: number; text: string; counter: [string, number][] }
export interface AdversaryDef { id: string; name: string; stage: number; text: string; twists: { op: string; amount?: number }[]; escalation?: boolean }
export interface AiBoardDef {
  id: string;
  name: string;
  text: string;
  slots?: number;
  innate: { name: string; text: string };
  /** AI-board powers (021). */
  powers?: { name: string; text?: string; tags: [string, number][]; cost?: [string, number][] }[];
}
export interface Content { cards: CardDef[]; threats: ThreatDef[]; adversary: AdversaryDef[]; ais: AiBoardDef[] }

/** A state as the engine produced it, plus a parsed copy for display. */
export interface Snapshot { json: string; state: GameState }
export interface Step { snap: Snapshot; events: GameEvent[] }

export let content: Content;
export let version = '?';
/** The rules revision the engine plays (`aic_core::RULES_REVISION`). */
export let rulesRevision = 0;

export async function load(): Promise<void> {
  await init();
  content = JSON.parse(wasm.content());
  version = wasm.version();
  rulesRevision = wasm.rules_revision();
}

const snapshot = (json: string): Snapshot => ({ json, state: JSON.parse(json) });

function step(o: wasm.Applied): Step {
  try {
    return { snap: snapshot(o.state), events: JSON.parse(o.events) };
  } finally {
    o.free();
  }
}

/** `{config, hash}`; throws the engine's reason for an invalid config. */
export function resolveConfig(configJson: string): { config: Config; hash: string } {
  return JSON.parse(wasm.resolve_config(configJson));
}

/** A new game with its setup events. `seed` is a decimal u64 string. */
export function start(configJson: string, seed: string): Step {
  return step(wasm.start(configJson, seed));
}

export function legalActions(snap: Snapshot): Action[] {
  return JSON.parse(wasm.legal_actions(snap.json));
}

/** Throws the engine's error string on an illegal action. */
export function apply(snap: Snapshot, action: Action): Step {
  return step(wasm.apply(snap.json, JSON.stringify(action)));
}

export function view(snap: Snapshot): View {
  return JSON.parse(wasm.view(snap.json));
}

/** The record text (engine-formatted) for the game `actions` took to `snap`. */
export function recordText(snap: Snapshot, actions: Action[], note: string): string {
  return wasm.record_text(snap.json, JSON.stringify(actions), note);
}

/** Throws on bad JSON or a different record format. */
export function parseRecord(text: string): GameRecord {
  return JSON.parse(wasm.parse_record(text));
}

/** Replays a record in the engine and compares it with what was saved. */
export function checkRecord(text: string): { ok: boolean; warnings?: string[]; error?: string } {
  return JSON.parse(wasm.check_record(text));
}

export const cardName = (id: string) => content.cards.find((c) => c.id === id)?.name ?? id;
export const threatName = (id: string) => content.threats.find((t) => t.id === id)?.name ?? id;

/** A config parameter of a module, if the config has that module. */
export function param(config: Config, module: string, name: string): number | number[] | undefined {
  return config.modules.find((m) => m.module === module)?.params[name];
}

/** The AI board a seat plays (the `ai` module's `boards` list), if there is one. */
export function aiBoard(config: Config, seat: number): AiBoardDef | undefined {
  const boards = param(config, 'ai', 'boards');
  if (!Array.isArray(boards) || !boards.length) return undefined;
  return content.ais[boards[seat % boards.length]];
}
