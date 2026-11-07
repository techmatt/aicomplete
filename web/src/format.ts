// Labels and colors for engine data. Generic: any action, event or token kind gets a
// readable label without code here; known action types, card ids, threat ids, power
// options and the board's standard actions get their content names (and costs).

import { aiBoard, cardName, content, param, threatName, type Action, type Config, type GameEvent, type Hex, type View } from './engine';

export const hexKey = (h: Hex) => `${h[0]},${h[1]}`;

/** "Horizon", or `C<i>` if the view has no name. */
export const countryName = (v: View, i: number) => v.countries[i]?.name || `C${i}`;

export const isHex = (v: unknown): v is Hex =>
  Array.isArray(v) && v.length === 2 && v.every((x) => Number.isInteger(x));

const words = (s: string) => s.replace(/_/g, ' ');

/** `n` of a resource: "2 chips", "1 chip" (as `Resource::amount` in the core). */
const SINGULAR: Record<string, string> = { chips: 'chip', credits: 'credit', ambassadors: 'ambassador', research_cubes: 'research cube' };
// 075: `research_cubes` reads as words.
const amount = (n: number, what: string) => `${n} ${(n === 1 ? (SINGULAR[what] ?? what) : what).replace(/_/g, ' ')}`;

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
  /** Country names by index, for labels made with no view (the log, the step line; 102, D30). */
  countries?: string[];
}

/** A card in a seat's hand as the view shows it, if any. */
function handCard(view: View | undefined, seat: number, id: string | undefined) {
  return view?.seats[seat]?.cards.find((g) => g.key === 'cards.hand')?.cards.find((c) => c.id === id);
}

/** A card's costs beyond credits ("1 insight", "2 chips"), from its view text's bracket line (044). */
export function extraCosts(text: string | undefined): string[] {
  const bracket = (text ?? '').match(/^\[([^\]]*)\]/)?.[1] ?? '';
  // 099: by part, so a cost of more than one word stays whole ("+4 research cubes" read "4 research").
  return bracket.split(', ').filter((p) => /^\+\d+ /.test(p)).map((p) => p.slice(1));
}

/** A built card of the seat's as the view shows it, if any. */
function builtCard(view: View | undefined, seat: number, id: string | undefined) {
  return view?.seats[seat]?.cards.find((g) => g.key === 'cards.built')?.cards.find((c) => c.id === id);
}

/** The "Action: ..." sentence of a card's text, without its period (099: the use button named no cost). */
function actionClause(text: string | undefined): string {
  return (text ?? '').match(/Action: ([^.]*)\./)?.[1] ?? '';
}

/** " (5 credits + 1 insight)", or nothing for a free card. */
function playPrice(credits: number | undefined, extra: string[]): string {
  const parts = [credits ? amount(credits, 'credits') : '', ...extra].filter(Boolean);
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
      // 099: an Action's second half is named by its op (`CashOut`, `GainPerTokens`): in words.
      const o = String(a.option ?? 'choose');
      // Two humps at least, so a one-word name (Horizon, a country) keeps its capital.
      const opName = /^[A-Z][a-z]+(?:[A-Z][a-z]+)+$/.test(o) ? o.replace(/(?!^)([A-Z])/g, ' $1').toLowerCase() : o;
      return `${contentIds(contentName(opName)).replace('T2 keep -1', 'T2 keep')}${t}`;
    }
    case 'tier':
      return `draft from tier ${a.tier}`;
    case 'draft':
      return `draft ${card}`;
    case 'keep':
      return `keep ${card}`;
    case 'play': {
      const c = handCard(cx.view, a.seat, a.card);
      return `play ${card}${playPrice(c?.cost, extraCosts(c?.text))}`;
    }
    case 'use': {
      const what = actionClause(builtCard(cx.view, a.seat, a.card)?.text);
      return `use ${card}${what ? `: ${what}` : ''}`;
    }
    case 'power': {
      // 021: an AI-board power, paid in resources or by sacrificing played cards' tags.
      const p = aiBoard(cx.config, a.seat)?.powers?.[Number(a.power)];
      if (!p) return `power: power ${a.power}`;
      const paid = (p.cost ?? []).map(([r, n]) => amount(n, r));
      const tags = p.tags.map(([t, n]) => `${n} ${t}`).join(' + ');
      const cost = [...paid, ...(tags ? [`sacrifice ${tags}`] : [])].join(', ');
      // 118: Synthesis has no price: no empty "()".
      return `power: ${p.name}${cost ? ` (${cost})` : ''}: ${p.text ?? ''}`;
    }
    case 'pay':
      return `pay ${amount(Number(a.amount), String(a.resource))} → ${threatName(String(a.threat))}`;
    case 'withdraw':
      // 145: "research_cubes" read raw (the sweep's raw-id).
      return `take back ${words(String(a.resource))} ← ${threatName(String(a.threat))}`;
    case 'counter_with': {
      // 145: a whole menu option (tags, or tags with resources), named by its priced words
      // as the threat's card shows them (its status: the options joined by " or ").
      const t = String(a.threat);
      const shown = cx.view?.countries.flatMap((c) => c.cards.flatMap((g) => g.cards)).find((c) => c.id === t);
      const option = shown?.status.split(' or ')[Number(a.option)];
      return `counter ${threatName(t)}${option ? `: ${option}` : ''}`;
    }
    case 'antimatter': {
      const n = param(cx.config, 'alignment', 'antimatter_discord');
      return `+1 antimatter${typeof n === 'number' ? ` (${n} Sway → Discord)` : ' (Sway → Discord)'}`;
    }
    case 'standard': {
      // The rules publish each standard action's words and cost in the view.
      const s = cx.view?.standard.find((x) => x.act === String(a.act));
      if (!s) return `standard: ${words(String(a.act))}`;
      // 099: a card-set price (the Harden action a card grants) is the seat's own.
      const chips = s.seat_chips?.[Number(a.seat)] ?? s.chips;
      const costs = ([[s.energy, 'energy'], [s.credits, 'credits'], [chips, 'chips'], [s.ambassadors, 'ambassadors']] as const)
        .filter(([v]) => v > 0)
        .map(([v, what]) => amount(v, what));
      return `${s.doc.split(';')[0]}${costs.length ? ` (${costs.join(', ')})` : ''}`;
    }
    case 'convert': {
      const x = cx.view?.seats[Number(a.seat)]?.values.find((y) => y.key === `board.converter.${a.country}`);
      // The log and the step line have no view: the country's name, handed in, names it.
      if (!x) return `the converter of ${cx.countries?.[Number(a.country)] ?? `country ${a.country}`}`;
      return `${x.label}: ${(x.doc ?? '').split(':')[0]}`;
    }
    case 'sell': {
      const n = param(cx.config, 'board', 'sell_credits');
      return `sell ${card}${typeof n === 'number' ? ` (+${amount(n, 'credits')})` : ''}`;
    }
  }
  return null;
}

/**
 * "seat 1: play Prefab Plant Kit". The seat is left out with `withSeat` false, and the
 * target with `withTarget` false (a choice's label, before its hex is picked).
 * Unknown action types are labeled from their fields.
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
 * not inside a dotted key (`adversary.project.botnet_swarm` stays as the state has it).
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

// ---------- colors ----------

/** Hex fills by what the hex is (`HexView.kind`): distinct, light enough for tokens on top. */
export const KINDS: Record<string, { fill: string; label: string }> = {
  city: { fill: '#c3c8d4', label: 'city' },
  base: { fill: '#d6c49a', label: 'military base' },
  open: { fill: '#dbe8c6', label: 'open land' },
  ocean: { fill: '#a9cfea', label: 'ocean' },
};
export const kindFill = (k: string) => KINDS[k]?.fill ?? '#e4e0d8';
export const kindName = (k: string) => KINDS[k]?.label ?? words(k);

/** A `#rrggbb` color pushed to a saturated, mid-dark version of its hue (for borders). */
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

/** The adversary's color: its owner cubes, its ambassadors, its cubes on megaprojects. */
/** Neutral stations' cubes on megaprojects (074). */
export const NEUTRAL_COLOR = '#9a9a9a';
export const ADVERSARY_COLOR = '#5b1a8c';

// Token colors: chosen colors for the kinds the board draws, every other kind
// hashed to a hue. Seat-owned kinds (`cube:<seat>`, `node:<seat>`, `ambassador:<seat>`) are
// drawn in the seat's color on the board; this is their fallback.
const KNOWN: Record<string, string> = {
  'cube:adversary': ADVERSARY_COLOR,
  'ambassador:adversary': ADVERSARY_COLOR,
  botnet: ADVERSARY_COLOR,
  corruption: ADVERSARY_COLOR,
  land_corruption: ADVERSARY_COLOR,
  troop: '#2a0d3d',
  factory: '#7a5230',
  station: '#00796b',
  unit: '#455a64',
  harden: '#607d8b',
};

export function tokenColor(kind: string): string {
  if (KNOWN[kind]) return KNOWN[kind];
  if (/^(cube|node|ambassador):\d+$/.test(kind)) return '#37474f';
  let h = 0;
  for (const c of kind) h = (h * 31 + c.charCodeAt(0)) >>> 0;
  return `hsl(${h % 360} 65% 38%)`;
}

const TOKEN_NAMES: Record<string, string> = {
  cube: 'owner cube',
  node: 'node',
  botnet: 'botnet',
  corruption: 'corruption marker',
  land_corruption: 'land corruption (no node may be placed here)',
  ambassador: 'ambassador',
  station: 'research station',
  troop: 'adversary troop',
  unit: 'corp troop',
  harden: 'hardening token',
};

/** "node:1" → "node (seat 1)", "cube:adversary" → "owner cube (adversary)". */
export function tokenName(kind: string): string {
  const [base, owner] = kind.split(':');
  const name = TOKEN_NAMES[base] ?? words(base);
  if (owner === undefined) return name;
  return owner === 'adversary' ? `${name} (adversary${base === 'cube' ? ': corrupted' : ''})` : `${name} (seat ${owner})`;
}

export function escapeHtml(s: string): string {
  return s.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}
