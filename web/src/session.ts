// A game as a timeline: config + seed + the actions taken, with every state on the way
// (each one the replay of the actions before it). Config + seed + actions is the whole
// game, so it is also the save (localStorage), the bug report and the sim's record.
// Undo drops the last action; stepping back moves a cursor; acting from an earlier
// step drops the later actions first.

import { apply, parseRecord, rulesRevision, start, type Action, type GameEvent, type GameRecord, type Snapshot } from './engine';

const SAVE_KEY = 'aic.session';
const U64_MAX = 0xffff_ffff_ffff_ffffn;

/** localStorage may throw (blocked, sandboxed, full): saving is a convenience only. */
export function storageGet(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

export function storageSet(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // See above.
  }
}

/** Frame 0 is the start of the game (setup); frame i > 0 follows action i - 1. */
export interface Frame {
  action: Action | null;
  snap: Snapshot;
  events: GameEvent[];
}

/** Where the game came from: played here, or loaded from a record. */
export type Source =
  | { kind: 'play'; file: string }
  | { kind: 'record'; file: string; note: string };

export class Timeline {
  readonly frames: Frame[] = [];
  cursor = 0;
  /** Set when a loaded record stops replaying: the frames hold what did replay. */
  replayError: string | null = null;

  private constructor(
    readonly configJson: string,
    readonly seed: string,
    public source: Source,
  ) {}

  /** Throws the engine's error for a bad config or seed. */
  static start(configJson: string, seed: string, source: Source): Timeline {
    const t = new Timeline(configJson, seed, source);
    const s = start(configJson, seed);
    t.frames.push({ action: null, snap: s.snap, events: s.events });
    return t;
  }

  /** Replays `actions`; stops at the first one the engine rejects (see `replayError`). */
  static replay(configJson: string, seed: string, actions: Action[], source: Source): Timeline {
    const t = Timeline.start(configJson, seed, source);
    for (const [i, a] of actions.entries()) {
      try {
        t.push(a);
      } catch (e) {
        t.replayError = `action ${i} ${JSON.stringify(a)} rejected: ${e}`;
        break;
      }
    }
    return t;
  }

  static fromRecord(record: GameRecord, file: string): Timeline {
    const t = Timeline.replay(JSON.stringify(record.config), record.seed, record.actions, {
      kind: 'record',
      file,
      note: record.note ?? '',
    });
    t.cursor = 0;
    // 118: kept, so a reload still says it is another revision's game (0: not recorded).
    if (record.rules_revision) t.savedRules = record.rules_revision;
    return t;
  }

  get last(): number {
    return this.frames.length - 1;
  }
  get frame(): Frame {
    return this.frames[this.cursor];
  }
  get actions(): Action[] {
    return this.frames.slice(1).map((f) => f.action as Action);
  }
  /** Actions up to the cursor: the game as it stands at the shown step. */
  get actionsToCursor(): Action[] {
    return this.frames.slice(1, this.cursor + 1).map((f) => f.action as Action);
  }
  get atEnd(): boolean {
    return this.cursor === this.last;
  }

  private push(a: Action) {
    const s = apply(this.frames[this.last].snap, a);
    this.frames.push({ action: a, snap: s.snap, events: s.events });
    this.cursor = this.last;
  }

  /** Applies `a` at the cursor, dropping later actions first. Throws if rejected. */
  act(a: Action) {
    const later = this.frames.splice(this.cursor + 1);
    try {
      this.push(a);
    } catch (e) {
      this.frames.push(...later);
      throw e;
    }
    this.changed(this.cursor - 1);
    // 099: acting on a game that stopped replaying makes it this game: it may save now.
    this.replayError = null;
  }

  /** Drops the last action. */
  undo() {
    if (this.last === 0) return;
    this.frames.pop();
    this.cursor = Math.min(this.cursor, this.last);
    this.changed(this.last);
    this.replayError = null;
  }

  /** A loaded record that has been changed is no longer that record. */
  private changed(step: number) {
    if (this.source.kind === 'record') {
      this.source = { kind: 'play', file: `${this.source.file} (changed at step ${step})` };
      // 118: the record's warnings (its fingerprint, its hashes) were about the record.
      this.warning = null;
    }
  }

  seek(i: number) {
    this.cursor = Math.max(0, Math.min(this.last, i));
  }

  save() {
    // 099: a game that stopped replaying isn't saved over the session (seeking or
    // autoplay wrote its replayed prefix, and the next load showed it as whole);
    // the first action or undo on it clears `replayError` and saves.
    if (this.replayError) return;
    // 099: with the rules revision, so a session from other rules can say so. 118: the
    // revision the game came from, not this engine's: the first save stamped an older
    // record or session as this revision, so after a reload it showed as clean.
    storageSet(SAVE_KEY, JSON.stringify({
      config: this.configJson, seed: this.seed, actions: this.actions, cursor: this.cursor, source: this.source,
      rules: this.savedRules ?? rulesRevision, warning: this.warning,
    }));
  }

  /**
   * The rules revision this game came from, when it isn't this engine's: a loaded
   * record's (118) or a restored session's (undefined before 099, or for a game begun here).
   */
  savedRules?: number;
  /** 118: a loaded record's 'loaded with warnings' line, kept so a reload still shows it. */
  warning: string | null = null;

  /** The saved session, replayed; null if there is none. Throws if its config no longer loads. */
  static restore(): Timeline | null {
    let saved: { config: string; seed: string; actions: Action[]; cursor: number; source: Source & { note?: string }; rules?: number; warning?: string | null };
    const raw = storageGet(SAVE_KEY);
    if (!raw) return null;
    try {
      saved = JSON.parse(raw);
    } catch {
      return null;
    }
    const source: Source = saved.source?.kind === 'record'
      ? { kind: 'record', file: saved.source.file, note: saved.source.note ?? '' }
      : { kind: 'play', file: saved.source?.file ?? '?' };
    let t: Timeline;
    try {
      t = Timeline.replay(saved.config, saved.seed, saved.actions, source);
    } catch (e) {
      // 118: the replacement game's first save overwrote this session at once, so its
      // actions were lost for good: keep it under its own key (see `keptSession`).
      storageSet(KEPT_KEY, raw);
      throw e;
    }
    t.seek(saved.cursor ?? t.last);
    t.savedRules = saved.rules;
    t.warning = saved.warning ?? null;
    return t;
  }
}

/** 118: a saved session that could not be restored, kept for `keptSession`. */
const KEPT_KEY = 'aic.session.unloadable';

/**
 * The kept session as record text (an older aic-sim may replay it), or null. Its config
 * no longer loads here, so it can't be opened in this viewer.
 */
export function keptSession(): string | null {
  const raw = storageGet(KEPT_KEY);
  if (!raw) return null;
  try {
    const s = JSON.parse(raw);
    const note = s.source?.kind === 'record' ? s.source.note : `kept session (${s.source?.file ?? '?'})`;
    return JSON.stringify({ note, config: JSON.parse(s.config), seed: s.seed, actions: s.actions, ...(s.rules ? { rules_revision: s.rules } : {}) }, null, 1);
  } catch {
    return raw;
  }
}

/** A seed as a decimal u64 string, or null. */
export function parseSeed(text: string): string | null {
  const t = text.trim();
  if (!/^\d+$/.test(t)) return null;
  const n = BigInt(t);
  return n <= U64_MAX ? n.toString() : null;
}

export function randomSeed(): string {
  return String(Math.floor(Math.random() * 2 ** 32));
}

/**
 * The lead-in for a record from another rules revision, or nothing. 0 is "not recorded"
 * (the field defaults to it).
 */
export function revisionNote(r: GameRecord): string {
  return r.rules_revision && r.rules_revision !== rulesRevision
    ? `this record is from rules revision ${r.rules_revision} and this engine plays revision ${rulesRevision}: `
    : '';
}

/** A pasted or opened record; throws with the reason it can't be used. */
export function readRecord(text: string): GameRecord {
  const r = parseRecord(text);
  // Say first that it is an older game (020: only a long parameter list said so).
  if (r.config_error) throw new Error(`${revisionNote(r)}the record's config is invalid for this engine: ${r.config_error}`);
  return r;
}
