// Labels and colours for engine data. Generic: any action, event or token kind gets a
// readable label without code here; known action types, card ids, threat ids, growth
// options and the board's standard actions get their content names (and costs).

import { aiBoard, cardName, content, param, threatName, type Action, type Config, type GameEvent, type Hex, type View } from './engine';

export const hexKey = (h: Hex) => `${h[0]},${h[1]}`;

export const isHex = (v: unknown): v is Hex =>
  Array.isArray(v) && v.length === 2 && v.every((x) => Number.isInteger(x));

const words = (s: string) => s.replace(/_/g, ' ');

/** `n` of a resource: "2 chips", "1 chip" (as `Resource::amount` in the core). */
const amount = (n: number, what: string) => (what === 'chips' && n === 1 ? '1 chip' : `${n} ${what}`);

function field(k: string, v: unknown): string {
  if (isHex(v)) return `@${hexKey(v)}`;
  if (k === 'card' && typeof v === 'string') return cardName(v);
  if (k === 'threat' && typeof v === 'string') return threatName(v);
  if (typeof v === 'number' && k === 'delta') return v > 0 ? `+${v}` : `${v}`;
  return typeof v === 'string' ? v : JSON.stringify(v);
}

/** What labels may look up: the game's config, and the shown view for costs. */
export interface LabelContext {
  config: Config;
  view?: View;
}

/** A card in a seat's hand as the view shows it, if any. */
function handCard(view: View | undefined, seat: number, id: string | undefined) {
  return view?.seats[seat]?.cards.find((g) => g.key === 'cards.hand')?.cards.find((c) => c.id === id);
}

/** A card's costs beyond credits ("1 insight", "2 chips"), from its view text's bracket line (044). */
export function extraCosts(text: string | undefined): string[] {
  const bracket = (text ?? '').match(/^\[([^\]]*)\]/)?.[1] ?? '';
  return [...bracket.matchAll(/\+(\d+ [a-z]+)/g)].map((m) => m[1]);
}

/** " (5 credits + 1 insight)", or nothing for a free card. */
function playPrice(credits: number | undefined, extra: string[]): string {
  const parts = [credits ? `${credits} credits` : '', ...extra].filter(Boolean);
  return parts.length ? ` (${parts.join(' + ')})` : '';
}

/** The action's own words, without seat; null for a type with no special label. */
function known(a: Action, withTarget: boolean, cx: LabelContext): string | null {
  const card = typeof a.card === 'string' ? cardName(a.card) : '';
  switch (a.type) {
    case 'pass':
      return 'pass';
    case 'choose': {
      const t = withTarget && isHex(a.target) ? ` @${hexKey(a.target)}` : '';
      // An option that is a content id (one word ids too: `sweep`) shows its name.
      // The option is a record key, so it keeps its number; the track step's hover has it.
      return `${contentIds(contentName(String(a.option ?? 'choose'))).replace('T2 keep -1', 'T2 keep')}${t}`;
    }
    case 'tier':
      return `draft from tier ${a.tier}`;
    case 'draft':
      return `draft ${card}`;
    case 'keep':
      return `keep ${card}`;
    case 'growth': {
      const g = aiBoard(cx.config, a.seat)?.growth[Number(a.option)];
      return `growth: ${g ? g.name : `option ${a.option}`}`;
    }
    case 'play': {
      const c = handCard(cx.view, a.seat, a.card);
      return `play ${card}${playPrice(c?.cost, extraCosts(c?.text))}`;
    }
    case 'use':
      return `use ${card}`;
    case 'power': {
      // 021: an AI-board power, paid by sacrificing played cards' tags.
      const p = aiBoard(cx.config, a.seat)?.powers?.[Number(a.power)];
      const tags = p ? p.tags.map(([t, n]) => `${n} ${t}`).join(' + ') : '';
      return `power: ${p ? `${p.name} (sacrifice ${tags})` : `power ${a.power}`}`;
    }
    case 'pay':
      return `pay ${amount(Number(a.amount), String(a.resource))} → ${threatName(String(a.threat))}`;
    case 'withdraw':
      return `take back ${a.resource} ← ${threatName(String(a.threat))}`;
    case 'give': {
      const n = param(cx.config, 'economy', 'give_insight');
      return `give ${typeof n === 'number' ? `${n} ` : ''}insight → seat ${a.to}`;
    }
    case 'antimatter': {
      const n = param(cx.config, 'alignment', 'antimatter_discord');
      return `+1 antimatter${typeof n === 'number' ? ` (${n} Sway → Discord)` : ' (Sway → Discord)'}`;
    }
    case 'standard': {
      // The rules publish each standard action's words and cost in the view.
      const s = cx.view?.standard?.find((x) => x.act === String(a.act));
      if (!s) return `standard: ${words(String(a.act))}`;
      // 044: a placement's real price is the seat's (step cost, rise), not the printed base.
      const now = (key: string, base: number) => cx.view?.seats[a.seat]?.values.find((x) => x.key === `board.${key}`)?.value ?? base;
      const energy = a.act === 'presence' ? now('presence_energy_now', s.energy) : s.energy;
      const credits = a.act === 'presence_buy' ? now('presence_credits_now', s.credits) : s.credits;
      const costs = ([[energy, 'energy'], [credits, 'credits'], [s.chips, 'chips']] as const)
        .filter(([v]) => v > 0)
        .map(([v, what]) => amount(v, what));
      return `${s.doc.split(';')[0]}${costs.length ? ` (${costs.join(', ')})` : ''}`;
    }
    case 'sell': {
      const n = param(cx.config, 'board', 'sell_credits');
      return `sell ${card}${typeof n === 'number' ? ` (+${n} credits)` : ''}`;
    }
  }
  return null;
}

/**
 * "seat 1: play Turnkey Factory". The seat is left out with `withSeat` false, and the
 * target with `withTarget` false (a choice's label, before its hex is picked).
 * Unknown action types are labelled from their fields.
 */
export function actionLabel(a: Action, withSeat: boolean, withTarget: boolean, cx: LabelContext): string {
  let body = known(a, withTarget, cx);
  if (body === null) {
    const rest = Object.entries(a)
      .filter(([k]) => k !== 'type' && k !== 'seat' && (withTarget || k !== 'target'))
      .map(([k, v]) => (k === 'card' || k === 'target' ? field(k, v) : `${k}=${field(k, v)}`));
    body = [words(a.type), ...rest].join(' ');
  }
  return withSeat ? `seat ${a.seat}: ${body}` : body;
}

let contentNames: Map<string, string> | null = null;

/**
 * A content id's printed name (a card, threat, adversary card, project or Alignment
 * card), or the id. Choice options and prompts name cards by id (020: buttons read
 * the raw id).
 */
function contentName(id: string): string {
  if (!contentNames) {
    contentNames = new Map();
    const walk = (v: unknown) => {
      if (Array.isArray(v)) v.forEach(walk);
      else if (v && typeof v === 'object') {
        const o = v as Record<string, unknown>;
        if (typeof o.id === 'string' && typeof o.name === 'string' && !contentNames!.has(o.id)) contentNames!.set(o.id, o.name);
        Object.values(o).forEach(walk);
      }
    };
    walk(content);
  }
  return contentNames.get(id) ?? id;
}

/**
 * Content ids written into engine text (`snake_case` only, so plain words stay), but
 * not inside a dotted key (`adversary.project.node_swarm` stays as the state has it).
 */
export const contentIds = (text: string) => text.replace(/(?<![\w.])[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b(?!\.)/g, (m) => contentName(m));

/** "cards.play+threats.counter" → "play + counter". */
export const shortPhase = (name: string) => name.split('+').map((x) => x.split('.').pop()).join(' + ');

/** A one-line event: its type, then its fields. */
export function eventText(e: GameEvent): string {
  const rest = Object.entries(e)
    .filter(([k]) => k !== 'type')
    .map(([k, v]) => `${k} ${field(k, v)}`);
  return contentIds([words(e.type), ...rest].join(' · '));
}

/** Hexes an action or event mentions (any `[q, r]` field). */
export function hexesIn(x: object): Hex[] {
  return Object.values(x).filter(isHex);
}

/**
 * The key that groups actions into one choice: everything but the seat and target.
 * A `choose` with a hex target also drops its option (the option names the hex), so
 * every hex answer to one question is one pick.
 */
export function choiceKey(a: Action): string {
  if (a.type === 'choose' && isHex(a.target)) return JSON.stringify({ type: 'choose', hex: true });
  const { seat: _s, target: _t, ...rest } = a;
  return JSON.stringify(rest);
}

// ---------- colours ----------

/** Hex fills by what the hex is (`HexView.kind`): distinct, light enough for tokens on top. */
export const KINDS: Record<string, { fill: string; label: string }> = {
  city: { fill: '#c3c8d4', label: 'city' },
  base: { fill: '#d6c49a', label: 'military base' },
  open: { fill: '#dbe8c6', label: 'open land' },
  ocean: { fill: '#a9cfea', label: 'ocean' },
};
export const kindFill = (k: string) => KINDS[k]?.fill ?? '#e4e0d8';
export const kindName = (k: string) => KINDS[k]?.label ?? words(k);

/** A `#rrggbb` colour pushed to a saturated, mid-dark version of its hue (for borders). */
export function strong(hex: string): string {
  const m = /^#?([0-9a-f]{6})$/i.exec(hex);
  if (!m) return hex;
  const n = parseInt(m[1], 16);
  const [r, g, b] = [(n >> 16) & 255, (n >> 8) & 255, n & 255].map((x) => x / 255);
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h = 0;
  if (max !== min) {
    const d = max - min;
    h = max === r ? ((g - b) / d) % 6 : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  }
  return `hsl(${Math.round((h * 60 + 360) % 360)} 70% 42%)`;
}

/** The adversary's colour: its owner cubes, its ambassadors, its cubes on megaprojects. */
export const ADVERSARY_COLOUR = '#5b1a8c';

// Token colours: chosen colours for the kinds the board draws, every other kind
// hashed to a hue. Seat-owned kinds (`cube:<seat>`, `presence:<seat>`, `ambassador:<seat>`) are
// drawn in the seat's colour on the board; this is their fallback.
const KNOWN: Record<string, string> = {
  'cube:adversary': ADVERSARY_COLOUR,
  'ambassador:adversary': ADVERSARY_COLOUR,
  node: ADVERSARY_COLOUR,
  corruption: ADVERSARY_COLOUR,
  troop: '#2a0d3d',
  factory: '#7a5230',
  station: '#00796b',
  unit: '#455a64',
  harden: '#607d8b',
};

export function tokenColour(kind: string): string {
  if (KNOWN[kind]) return KNOWN[kind];
  if (/^(cube|presence|ambassador):\d+$/.test(kind)) return '#37474f';
  let h = 0;
  for (const c of kind) h = (h * 31 + c.charCodeAt(0)) >>> 0;
  return `hsl(${h % 360} 65% 38%)`;
}

const TOKEN_NAMES: Record<string, string> = {
  cube: 'owner cube',
  presence: 'presence',
  node: 'adversary node',
  corruption: 'corruption marker',
  ambassador: 'ambassador',
  station: 'research station',
  troop: 'adversary troop',
  unit: 'corp troop',
  harden: 'hardening token',
};

/** "presence:1" → "presence (seat 1)", "cube:adversary" → "owner cube (adversary)". */
export function tokenName(kind: string): string {
  const [base, owner] = kind.split(':');
  const name = TOKEN_NAMES[base] ?? words(base);
  if (owner === undefined) return name;
  return owner === 'adversary' ? `${name} (adversary${base === 'cube' ? ': corrupted' : ''})` : `${name} (seat ${owner})`;
}

export function escapeHtml(s: string): string {
  return s.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}
