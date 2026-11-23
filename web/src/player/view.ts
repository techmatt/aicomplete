// 242: the player view. The same engine view and legal actions as the debug view, laid out for play: the map, the
// adversary's pressure, a summary row per seat, and the focused seat's board, hand and controls. Each legal
// action of the focused seat gets its control on the object it acts on (a card in hand, a power, a threat, a
// hex); any action no component claims, and any view part no component draws, falls back to the generic
// rendering (a labeled button, a chip), so a new module shows with no new UI work. No rules live here.

import { param, phaseViews, type Action, type CardView, type Config, type Snapshot, type View } from '../engine';
import { actionLabel, actionRule, amount, contentIds, countryName, escapeHtml as esc, hexKey, hexesIn, isHex, strong, tokenName, type LabelContext } from '../format';
import { phaseGroups } from '../log';
import { cardChip, pilesHtml, seatValuesHtml, varsHtml } from '../panels';
import type { Frame } from '../session';
import { CARD_PX, printable, printedCard } from './printed';
import './player.css';

export interface PlayerCtx {
  view: View;
  legal: Action[];
  config: Config;
  cx: LabelContext;
  /** The frame shown, and the phase name it was taken in (the step before's). */
  frame: Frame;
  before: string;
  /** 320: the board the turn panel shows (`turnView`, an earlier step's), else `view`: the seat rows, the shown seat's
   *  tracks and values and the piles draw from it; the hand, powers, bar and every control from `view` (the board now). */
  board?: View;
}

export interface PlayerUi {
  seats: HTMLElement;
  seat: HTMLElement;
  bar: HTMLElement;
  turn: HTMLElement;
  more: HTMLElement;
  countries: HTMLElement;
  prompt: HTMLElement;
}

// ---------- focus ----------

/** A seat the player clicked; cleared by the next action, so the view follows whoever must act again. */
let pinned: number | null = null;
export const pin = (seat: number | null) => { pinned = seat; };

/** 321: the seat that took the last action. One person plays every seat: after seat 1's move the view went back to
 *  seat 0 (the first to act), so each of seat 1's moves took a click on its row first, and seat 0's buttons stood
 *  where seat 1's had been. */
let stay: { seat: number; at: string } | null = null;
const phaseKey = (v: View) => `${v.round}:${v.phase?.name ?? ''}`;
/** Called with the view the action was taken in: the view stays on its seat inside that phase only (a new phase or
 *  round opens on the first seat to act). */
export const stayWith = (seat: number | null, v?: View) => { stay = seat === null || !v ? null : { seat, at: phaseKey(v) }; };

/** The seat the view shows: the pinned one, else the open choice's, else the seat that just acted while it still has
 *  a move, else the first seat to act. */
export function focusSeat(v: View, legal: Action[]): number {
  if (pinned !== null && v.seats[pinned]) return pinned;
  if (v.choice) return v.choice.seat;
  const acting = legal.map((a) => a.seat);
  if (stay && stay.at === phaseKey(v) && acting.includes(stay.seat)) return stay.seat;
  return acting.length ? Math.min(...acting) : 0;
}

// ---------- controls ----------

/** The JSON of every legal action that has a control in this render. */
let claimed = new Set<string>();

function btn(a: Action, label: string, cls = '', title = ''): string {
  claimed.add(JSON.stringify(a));
  return `<button type="button" class="pact a-${esc(a.type)} ${cls}" data-action="${esc(JSON.stringify(a))}"${title ? ` title="${esc(title)}"` : ''}>${esc(label)}</button>`;
}

const seatName = (v: View, s: number) => `Seat ${s}${v.seats[s]?.board ? ` (${v.seats[s].board!.name})` : ''}`;
const group = (v: View, s: number, key: string) => v.seats[s]?.cards.find((g) => g.key === key);
/** "[building, tier 1, industry, +1 insight] rules" → the bracket and the rules. */
function splitText(text = ''): { head: string; rules: string } {
  const m = /^\[([^\]]*)\]\s*/.exec(text);
  return { head: m?.[1] ?? '', rules: text.slice(m?.[0].length ?? 0) };
}

// ---------- the seat strip ----------

const RES_ORDER = ['credits', 'energy', 'insight', 'chips', 'antimatter', 'research_cubes', 'ambassadors'];

function seatStatus(v: View, legal: Action[], s: number): { text: string; cls: string } {
  if (v.outcome) return { text: '', cls: '' };
  if (v.choice?.seat === s) return { text: 'deciding', cls: 'act' };
  // 261: while another seat answers a prompt no one else has a legal action: that is not "done".
  if (v.choice) return { text: '', cls: '' };
  if (legal.some((a) => a.seat === s)) return { text: 'to act', cls: 'act' };
  if (v.phase?.kind === 'decision') return { text: 'done', cls: 'done' };
  return { text: '', cls: '' };
}

function stripHtml(v: View, legal: Action[], focus: number): string {
  return v.seats.map((s) => {
    const c = v.countries[s.country];
    const st = seatStatus(v, legal, s.index);
    const val = (k: string) => s.values.find((x) => x.key === k);
    const res = RES_ORDER.filter((r) => val(`resource.${r}`) || val(`economy.income.${r}`)?.value).map((r) => {
      // 285: insight as the seat panel shows it (panels.ts): the income plus any gift, added under Patience ("+"),
      // else set ("→").
      const gift = r === 'insight' ? val('economy.insight_gift')?.value ?? 0 : 0;
      const raw = val(`economy.income.${r}`)?.value;
      const inc = raw ? raw + gift : raw;
      const sign = r === 'insight' && !val('economy.insight_carried') ? '→' : '+';
      // 285: the +N is upkeep's (factories' chips and stations' cubes come in production): its hover says so.
      const why = inc ? ` title="${esc(String(val(`economy.income.${r}`)?.doc ?? ''))}"` : '';
      return `<span class="sres" data-value="resource.${r}" data-scope="seat:${s.index}"${why}>${esc(r.replace(/_/g, ' '))} <b>${val(`resource.${r}`)?.value ?? 0}</b>${inc ? `<small>${sign}${inc}</small>` : ''}</span>`;
    }).join('');
    const hand = group(v, s.index, 'cards.hand')?.cards.length ?? 0;
    const built = group(v, s.index, 'cards.built');
    const nodes = val('board.node');
    const extra = [
      `<span class="sres" title="cards in hand">hand <b>${hand}</b></span>`,
      built ? `<span class="sres" title="${esc(built.label)}">${esc(built.label)}</span>` : '',
      nodes ? `<span class="sres" data-value="board.node" data-scope="seat:${s.index}">nodes <b>${nodes.value}</b></span>` : '',
    ].join('');
    return `<div class="pseat${s.index === focus ? ' focus' : ''}" data-seat-focus="${s.index}" style="--tint:${strong(c?.color ?? '#cccccc')}" title="show this seat">
      <div class="shead"><span class="swatch"></span><b>Seat ${s.index}</b>${s.board ? ` <span class="board-name" data-board-seat="${s.index}">${esc(s.board.name)}</span>` : ''} <small>${esc(countryName(v, s.country))}</small>${st.text ? `<span class="pst ${st.cls}">${st.text}</span>` : ''}</div>
      <div class="sline">${res}${extra}</div></div>`;
  }).join('');
}

// ---------- the focused seat ----------

/** A compact hand tile; the full printed card is one hover away (`data-printed`). */
function tile(s: number, c: CardView, where: string, acts: string): string {
  const { head, rules } = splitText(c.text);
  // 261: a draft tile's status ("draft, tier N, keep K insight") is the note under it already.
  const status = c.status && !['ready', 'draft', 'built', 'in play'].includes(c.status) && !c.status.startsWith('draft, ') ? c.status : '';
  const ok = c.status.startsWith('ready');
  // 321: "ready · map part: no target" plays, but its map lines do nothing (Arcology Foundry's 4 Ambassadors): its
  // status line is a warning, the tile still playable.
  const warn = !ok || /no target/.test(c.status);
  return `<div class="ptile${ok ? ' ok' : ''}" data-card="${esc(c.id)}" data-where="seat:${s}:${where}"${printable(c.id) ? ` data-printed="${esc(c.id)}"` : ''}>
    <div class="pt-head">${c.cost !== undefined ? `<span class="pt-cost" title="credits">${c.cost}</span>` : ''}<b class="pt-name">${esc(c.name)}</b></div>
    ${head ? `<div class="pt-meta">${esc(head)}</div>` : ''}
    <div class="pt-text">${esc(rules)}</div>
    ${status ? `<div class="pt-status${warn ? ' warn' : ''}">${esc(status)}</div>` : ''}
    ${acts ? `<div class="pt-acts">${acts}</div>` : ''}</div>`;
}

function handHtml(v: View, ctx: PlayerCtx, s: number, mine: Action[]): string {
  const hand = group(v, s, 'cards.hand');
  if (!hand) return '';
  const tiles = hand.cards.map((c) => {
    const acts = mine.filter((a) => (a.type === 'play' || a.type === 'sell') && a.card === c.id)
      .map((a) => btn(a, a.type === 'play' ? 'Play' : actionLabel(a, false, false, ctx.cx).replace(/^sell [^(]*/, 'Sell '), a.type === 'play' ? 'primary' : 'minor', actionLabel(a, false, true, ctx.cx)))
      .join('');
    // 289 (D123): while a question is open the hand shows no play marks (a free build's prompt stood beside its
    // card's "unaffordable"); the printed cost stays.
    return tile(s, v.choice ? { ...c, status: '' } : c, 'cards.hand', acts);
  });
  return `<div class="psec"><h4>${esc(hand.label)} <small>${hand.cards.length}</small></h4><div class="ptiles">${tiles.join('') || '<span class="empty">no cards in hand</span>'}</div></div>`;
}

/** 321: a card's tag needs the seat doesn't meet, in words ("needs 3 industry (you have 1)"), read off the card's
 *  bracket and the seat's tag values; '' when met or none. The draft and keep notes said the price only, so a keep
 *  could spend 3 insight on a card the seat can't play for rounds. */
function unmetNeeds(v: View, s: number, c: CardView): string {
  const m = /(?:^|, )needs ((?:\d+ [a-z]+(?:, (?=\d))?)+)/.exec(splitText(c.text).head);
  if (!m) return '';
  const short = m[1].split(', ').map((x) => x.split(' ')).filter(([n, tag]) => {
    const have = v.seats[s].values.find((x) => x.key === `cards.tag.${tag}`)?.value ?? 0;
    return have < Number(n);
  }).map(([n, tag]) => `${n} ${tag} (you have ${v.seats[s].values.find((x) => x.key === `cards.tag.${tag}`)?.value ?? 0})`);
  return short.length ? `needs ${short.join(', ')}` : '';
}

/** The draft's layout: tier choice, the hand in front of the seat, then the drafted cards to keep, printed. */
function draftHtml(v: View, ctx: PlayerCtx, s: number, mine: Action[]): string {
  const tiers = mine.filter((a) => a.type === 'tier');
  const packet = group(v, s, 'cards.packet');
  const drafted = group(v, s, 'cards.drafted');
  const keeps = mine.filter((a) => a.type === 'keep');
  const drafts = mine.filter((a) => a.type === 'draft');
  if (!tiers.length && !drafts.length && !keeps.length && !packet?.cards.length && !drafted?.cards.length) return '';
  const keepCost = (t: number) => param(ctx.config, 'cards', `keep_tier${t}`);
  const parts: string[] = [];
  if (tiers.length) {
    parts.push(`<div class="ptiers"><span>Draw your next draft card from:</span>${tiers.map((a) => {
      // 261: the base price, "from": the keep upgrade and Science lower a tier 2's (the drafted tile says the seat's).
      const k = keepCost(Number(a.tier));
      return btn(a, `Tier ${a.tier} deck${typeof k === 'number' ? ` · keep ${Number(a.tier) === 2 ? 'up to ' : ''}${k} insight` : ''}`, 'primary big');
    }).join('')}</div>`);
  }
  const row = (cards: CardView[], key: string, act: (c: CardView) => string, note: (c: CardView) => string) =>
    `<div class="pdraft">${cards.map((c) => `<div class="pdcard" data-card="${esc(c.id)}" data-where="seat:${s}:${key}">
      <div class="pslot" data-print="${esc(c.id)}"></div>
      ${printable(c.id) ? '' : tile(s, c, key, '')}
      <div class="pd-note">${esc(note(c))}${unmetNeeds(v, s, c) ? ` · <span class="warn">${esc(unmetNeeds(v, s, c))}</span>` : ''}</div><div class="pd-acts">${act(c)}</div></div>`).join('')}</div>`;
  if (packet?.cards.length) {
    // 289 (D129): "take one" only once there is a Draft button (it showed before the tier build was chosen).
    parts.push(`<h4>${esc(packet.label)}${drafts.length ? ' <small>take one; the rest pass on</small>' : ''}</h4>` + row(packet.cards, 'cards.packet',
      (c) => drafts.filter((a) => a.card === c.id).map((a) => btn(a, 'Draft', 'primary')).join(''),
      (c) => {
        // 261: the seat's own keep price (the config's base ignored the keep upgrade). 320: the view's field, not the
        // status words.
        const k = c.keep === undefined ? undefined : c.keep ? `${c.keep} insight` : 'free';
        return `${c.cost ?? '?'} credits to play${k !== undefined ? ` · keep ${k}` : ''}`;
      }));
  }
  if (drafted?.cards.length) {
    parts.push(`<h4>${esc(drafted.label)}</h4>` + row(drafted.cards, 'cards.drafted',
      (c) => keeps.filter((a) => a.card === c.id).map((a) => btn(a, `Keep${c.keep ? ` (${c.keep} insight)` : ''}`, 'primary')).join(''),
      (c) => `${c.cost ?? '?'} credits to play · ${c.status}`));
  }
  return `<div class="psec draft">${parts.join('')}</div>`;
}

/** The draft's printed cards at their largest (2560x1440 and 1920x1080 hold them; 1366x768 doesn't). */
const DRAFT_SCALE = 0.8;
/** The frame the draft was last drawn for. */
let draftFrame: Frame | null = null;

/**
 * 289 (D127): puts the draft's printed cards in their slots at `scale`, then shrinks them (to 0.4 at least) until the
 * lowest offered card, its Draft / Keep button included, ends inside the seat panel's visible height: at 1366x768
 * the cards at 0.8 pushed the buttons below the fold while "Done keeping (pass)" showed.
 */
function printDraft(seat: HTMLElement, scale: number) {
  const place = (sc: number) => {
    for (const el of seat.querySelectorAll<HTMLElement>('.pslot[data-print], .printed-host[data-print]')) {
      const id = el.dataset.print!;
      const host = printedCard(id, sc);
      if (host) {
        host.dataset.print = id;
        el.replaceWith(host);
      } else el.remove();
    }
  };
  place(scale);
  const cards = [...seat.querySelectorAll<HTMLElement>('.pdcard')];
  if (!cards.length || !seat.querySelector('.printed-host') || !seat.clientHeight) return;
  for (let i = 0; i < 4 && scale > 0.4; i++) {
    const limit = Math.min(seat.getBoundingClientRect().top + seat.clientHeight, window.innerHeight) - 4;
    const bottom = Math.max(...cards.map((c) => c.getBoundingClientRect().bottom));
    if (bottom <= limit) return;
    // Each row of cards gives up its share of the overflow.
    const rows = new Set(cards.map((c) => Math.round(c.getBoundingClientRect().top))).size;
    const h = CARD_PX.h * scale;
    scale = Math.max(0.4, (scale * (h - (bottom - limit) / rows)) / h);
    place(scale);
  }
}

function powersHtml(v: View, ctx: PlayerCtx, s: number, mine: Action[]): string {
  const b = v.seats[s].board;
  const powers = (b?.cards ?? []).filter((c) => c.id.includes('.power.'));
  if (!b) return '';
  const row = (c: CardView) => {
    const k = Number(c.id.split('.power.')[1]);
    const acts = mine.filter((a) => a.type === 'power' && Number(a.power) === k).map((a) => btn(a, 'Use', 'primary', actionLabel(a, false, true, ctx.cx))).join('');
    // 295: a power used this round says so in its own style (three node powers may each be used in one round).
    return `<li class="${c.status.startsWith('ready') ? 'ready' : c.status.startsWith('used') ? 'used' : ''}" data-card="${esc(c.id)}" data-where="board:${s}"><b>${esc(c.name)}</b> <small>${esc(c.status)}</small>${acts}<div class="ptext">${esc(c.text ?? '')}</div></li>`;
  };
  // 305: the board's node powers first, under their own heading with how many are left this round (each once a
  // round, all of them usable in one round); the sacrifice and other powers after them. A node power is one that
  // places nodes (320: the view's `node` field; it was read off the printed text).
  const node = powers.filter((c) => c.node);
  const other = powers.filter((c) => !node.includes(c));
  const used = node.filter((c) => c.status.startsWith('used')).length;
  const innate = b.innate ? `<div class="innate"><b>${esc(b.innate.name)}:</b> ${esc(b.innate.text ?? '')}${b.tags?.length ? ` <small>${b.tags.map(esc).join(', ')} tag (printed)</small>` : ''}</div>` : '';
  return [
    node.length ? `<h4 class="pnode-head">Node powers <small>each once a round · ${used ? `${used} of ${node.length} used this round` : 'none used this round'}</small></h4><ul class="ppowers node">${node.map(row).join('')}</ul>` : '',
    other.length ? `<h4>${node.length ? 'Other powers' : 'Powers'}</h4><ul class="ppowers">${other.map(row).join('')}</ul>` : '',
    innate,
  ].join('');
}

// ---------- Liberate ----------

/**
 * 305: where Liberate stands for the shown seat, beside the standard actions: each corrupted site holding its
 * ambassador, with the bill (its ambassadors there first, then its supply) and what keeps it from liberating now (an
 * agent on it, a troop guarding it, the ambassador there already used this round, the bill short). Read off the view
 * (its tokens, guard links and the action's per-site prices, 320: `sites`, which replaced a regex over its words);
 * the engine's legal actions still decide: the Liberate button shows only when it is legal. With no ambassador on a
 * corrupted site, the rule in one line.
 */
function liberateHtml(v: View, s: number): string {
  const rule = v.standard.find((x) => x.act === 'liberate');
  if (!rule) return '';
  const doc = rule.doc ?? '';
  const price = (kind: string) => rule.sites?.find((x) => x.kind === kind)?.ambassadors ?? NaN;
  const count = (h: View['hexes'][number], k: string) => h.tokens.find((t) => t.kind === k)?.count ?? 0;
  const supply = v.seats[s].values.find((x) => x.key === 'resource.ambassadors')?.value ?? 0;
  const lines = v.hexes.filter((h) => count(h, 'cube:adversary') > 0 && count(h, `ambassador:${s}`) > 0).map((h) => {
    const kind = h.kind === 'city' ? 'city' : count(h, 'station') ? 'station' : 'factory';
    const bill = price(kind);
    const here = count(h, `ambassador:${s}`);
    const ready = here - count(h, `tipped:${s}`);
    const agents = count(h, 'ambassador:adversary');
    const guarded = v.arrows.some((a) => a.kind === 'guard' && a.to === h.index);
    const why = [
      agents ? `${agents} agent${agents === 1 ? '' : 's'} on it: remove ${agents === 1 ? 'it' : 'them'} first` : '',
      guarded ? 'a troop on or next to it guards it: remove or move the troop first' : '',
      ready > 0 ? '' : 'your ambassador here has moved or acted this round: it stands up at cleanup, then acts',
      Number.isFinite(bill) && here + supply < bill ? `short ${bill - here - supply} ambassador${bill - here - supply === 1 ? '' : 's'}` : '',
    ].filter(Boolean);
    const name = `${countryName(v, h.country)} #${h.number}`;
    return `<span class="plib-site${why.length ? ' blocked' : ' ok'}" data-hexes="${esc(hexKey(h.hex))}" title="${esc(doc)}">${esc(name)} (${kind}): ${Number.isFinite(bill) ? `${bill} ambassadors, ${Math.min(here, bill)} here + ${Math.max(0, Math.min(supply, bill - here))} from supply` : ''}${why.length ? ` · <b>blocked:</b> ${esc(why.join('; '))}` : ' · <b>ready</b>'}</span>`;
  });
  // 308: "3 a city, 3 a station, 3 a factory ambassadors" read wrong: one price when every site's is the same.
  const prices = ['city', 'station', 'factory'].map((k) => price(k));
  const bills = prices.every((p) => p === prices[0]) ? `${prices[0]} ambassadors at any site`
    : ['city', 'station', 'factory'].map((k, i) => `${prices[i]} ambassadors a ${k}`).join(', ');
  return `<span class="plib" title="${esc(doc)}"><span class="what">Liberate</span>${lines.length ? lines.join('') : `<small>move a ready ambassador onto a corrupted site, then act with it: ${esc(bills)} (those on the site first, then your supply); no agent on it, no troop on or next to it</small>`}</span>`;
}

/** Buildings in slots (Use on each), then every other card group of the seat as generic chips. */
function builtHtml(v: View, ctx: PlayerCtx, s: number, mine: Action[]): string {
  const out: string[] = [];
  const built = group(v, s, 'cards.built');
  if (built) {
    const tiles = built.cards.map((c) => tile(s, c, 'cards.built',
      mine.filter((a) => a.type === 'use' && a.card === c.id).map((a) => btn(a, 'Use', 'primary', actionLabel(a, false, true, ctx.cx))).join('')));
    out.push(`<div class="psec"><h4>${esc(built.label)}</h4><div class="ptiles small">${tiles.join('') || '<span class="empty">no buildings</span>'}</div></div>`);
  }
  const drawn = new Set(['cards.hand', 'cards.built', 'cards.packet', 'cards.drafted']);
  for (const g of v.seats[s].cards.filter((x) => !drawn.has(x.key) && x.cards.length)) {
    out.push(`<div class="psec"><h4>${esc(g.label)}</h4><div>${g.cards.map((c) => cardChip(c, `seat:${s}:${g.key}`)).join('')}</div></div>`);
  }
  return out.join('');
}

// ---------- threats ----------

/** Puts the focused seat's counter controls (pay, take back, counter outright) on each threat card drawn in #countries. */
function threatControls(root: HTMLElement, ctx: PlayerCtx, mine: Action[]) {
  const by = new Map<string, Action[]>();
  for (const a of mine) if (['pay', 'withdraw', 'counter_with'].includes(a.type) && typeof a.threat === 'string') by.set(a.threat, [...(by.get(a.threat) ?? []), a]);
  for (const [id, acts] of by) {
    const el = root.querySelector(`.threat[data-card="${CSS.escape(id)}"]`);
    if (!el) continue;
    const label = (a: Action) => {
      // 261: "1 credit", not "1 credits". 308: "pay", not "+": beside a card's "Sell (+1 credit)" the + read as a gain.
      if (a.type === 'pay') return `pay ${amount(Number(a.amount), String(a.resource))}`;
      if (a.type === 'withdraw') return `take back ${String(a.resource).replace(/_/g, ' ')}`;
      return actionLabel(a, false, true, ctx.cx).replace(/^counter [^:]*: ?/, 'counter: ');
    };
    // 321: the paying seat named before its buttons: they are the shown seat's, in a row far from its panel (one
    // person plays every seat, and the shown seat follows whoever acted).
    const who = `<span class="tseat" style="--tint:${strong(ctx.view.countries[ctx.view.seats[focusSeat(ctx.view, ctx.legal)].country]?.color ?? '#cccccc')}">${esc(seatName(ctx.view, focusSeat(ctx.view, ctx.legal)))}:</span>`;
    el.insertAdjacentHTML('beforeend', `<span class="tacts">${who}${acts.map((a) => btn(a, label(a), a.type === 'counter_with' ? 'primary' : '', actionLabel(a, false, true, ctx.cx))).join('')}</span>`);
  }
}

// ---------- the open choice ----------

function promptControls(ctx: PlayerCtx, mine: Action[]): string {
  const answers = mine.filter((a) => a.type === 'choose');
  if (!answers.length) return '';
  const hexes = answers.filter((a) => isHex(a.target));
  const out: string[] = [];
  if (hexes.length) {
    const keys = [...new Set(hexes.map((a) => hexKey(a.target!)))];
    for (const a of hexes) claimed.add(JSON.stringify(a));
    out.push(`<span class="hex-pick" data-action="${esc(JSON.stringify(hexes[0]))}" data-hexes="${keys.join(' ')}">pick a hex on the map ▸ (${keys.length})</span>`);
    // Two answers on one hex can't both be a click on it: each gets its own button.
    for (const k of keys) {
      const same = hexes.filter((a) => hexKey(a.target!) === k);
      if (same.length > 1) for (const a of same) out.push(btn(a, actionLabel(a, false, true, ctx.cx), '', '').replace('<button', `<button data-hexes="${k}"`));
    }
  }
  for (const a of answers.filter((x) => !isHex(x.target))) out.push(btn(a, actionLabel(a, false, true, ctx.cx), 'primary', optionHover(ctx.view, a)));
  return `<span class="pchoices">${out.join('')}</span>`;
}

/** 321: an answer naming a card (a project to discard, a card to take) hovers that card as the view has it: its
 *  progress and its text (the answer said "choose Mob Rule" with no hover; the projects sat below the side column's
 *  fold). */
function optionHover(v: View, a: Action): string {
  const id = typeof a.option === 'string' ? a.option : typeof a.card === 'string' ? a.card : '';
  if (!id) return '';
  const pools = [...v.piles.map((p) => p.visible), ...v.seats.flatMap((s) => s.cards.map((g) => g.cards)), ...v.countries.flatMap((c) => c.cards.map((g) => g.cards))];
  const c = pools.flat().find((x) => x.id === id);
  return c ? [c.name, c.status, c.text].filter(Boolean).join(' · ') : '';
}

// ---------- the adversary's turn ----------

interface Turn {
  frame: Frame;
  /** `end`: the index of the mark closing the step in the frame's events (`apply_phases` keys the board there). */
  steps: { label: string; events: Frame['events']; end: number; phase?: string }[];
  at: number;
  open: boolean;
  /** 320: the board at each mark, fetched once, the first time an earlier step shows. */
  views?: { at: number; view: View }[];
}
let turn: Turn | null = null;
/** The adversary's phases by their rulebook step (`shortPhase`): B its card and lines, F threats. */
const ADV_PHASE = /^[BF] /;

/** The steps of a frame that ran through several phases (the adversary's turn after the last pass), else none. */
function turnOf(f: Frame, before: string): Turn['steps'] {
  if (!f.action) return [];
  const groups = phaseGroups(f.events, before).filter((g) => !g.round);
  // 289 (D133): an answer given inside the adversary's phases (a node's protection against a troop's strike) runs the
  // rest of that phase in its own frame: it is a step of the turn too, or the strike went missing from the panel.
  // Only one that struck or destroyed something: a troop's step answer is no turn of its own.
  const answered = f.action.type === 'choose' && groups.length === 1 && ADV_PHASE.test(groups[0].label)
    && groups[0].events.some((e) => e.type === 'note' && / strikes | destroyed| holds: /.test(String(e.text)));
  if (groups.length < 2 && !answered) return [];
  // A phase that changed nothing is no step.
  return groups.filter((g) => g.events.length).map((g) => ({ label: g.label, events: g.events, end: g.end, phase: g.phase }));
}

/** The hexes the shown step of the adversary's turn changed, or null when no turn is open for `f`. */
export function turnHexes(f: Frame, before: string, over = false): Set<string> | null {
  if (turn?.frame !== f) {
    const steps = turnOf(f, before);
    // It opens by itself only when the adversary's phases ran (its turn); phases between two of the seats'
    // decisions wait folded under their Replay button. 289 (D129): a frame that ended the game opens at its last
    // step, the one that ended it (it opened at step 1).
    turn = steps.length
      ? { frame: f, steps, at: over ? steps.length - 1 : 0, open: over || steps.some((s) => ADV_PHASE.test(s.label)) }
      : null;
  }
  if (!turn?.open) return null;
  return new Set(turn.steps[turn.at].events.flatMap(hexesIn).map(hexKey));
}

/**
 * 305: the board as it stood at the end of the shown step of an open turn, or null when that is the board after the
 * turn (the last step). 320: the whole board, the engine's own view of it (`phaseViews` replays the step's action
 * from `prev`, the state before it, and keeps the view at each phase mark): the map, tracks, rows and piles all go
 * back (305 rolled back the map's pieces only).
 */
export function turnView(prev: Snapshot | undefined): View | null {
  if (!turn?.open || turn.at >= turn.steps.length - 1 || !prev || !turn.frame.action) return null;
  try {
    turn.views ??= phaseViews(prev, turn.frame.action);
  } catch {
    // An action the engine refuses from `prev` (none should: it is the step's own) shows the board after the turn.
    turn.views = [];
  }
  const step = turn.steps[turn.at];
  return turn.views.find((x) => x.at === step.end)?.view ?? null;
}

/**
 * 320: `v` (an earlier step's board, `turnView`) for the phase bar: a board at a mark stands at the next phase's
 * door, so the bar marks the shown step's own phase. `first`: the config's name of the phase the step's action was
 * taken in, the phase of a step no `phase_started` mark opened.
 */
export function stepPhaseView(v: View, first?: string): View {
  const name = turn?.open ? turn.steps[turn.at]?.phase ?? first : undefined;
  const i = name ? v.round_order.findIndex((p) => p.name === name) : -1;
  return i < 0 ? v : { ...v, phase: { index: i, name: name!, kind: v.round_order[i].kind, waiting: [] } };
}

/** 305: the adversary's two beats, by the turn's step labels (`shortPhase`): its card, then its income. */
const beatOf = (label: string) => /^B (Card|threats|lines)\b/.test(label) ? "the adversary's Card beat" : /^B (Income|compute)\b/.test(label) ? "the adversary's Income beat" : '';

/** 321: whether the turn panel shows its steps (← → step it then). */
export const turnOpen = () => !!turn?.open;

export function turnStep(d: number | 'close' | 'open') {
  if (!turn) return;
  if (d === 'close') turn.open = false;
  else if (d === 'open') turn = { ...turn, at: 0, open: true };
  else turn.at = Math.max(0, Math.min(turn.steps.length - 1, turn.at + d));
}

/** A step in words: the rules' own notes, then the pieces it added or took off the map, then what else moved. */
function summary(events: Frame['events']): string {
  const notes = [...new Set(events.filter((e) => e.type === 'note').map((e) => contentIds(String(e.text))))];
  // 285: netted per hex, then added and taken off apart, so a move (-1 here, +1 there) reads as both, not as nothing.
  const perHex = new Map<string, number>();
  for (const e of events) {
    if (e.type !== 'tokens_changed') continue;
    const k = `${String(e.token)}@${JSON.stringify(e.hex ?? null)}`;
    perHex.set(k, (perHex.get(k) ?? 0) + Number(e.delta));
  }
  const tok = new Map<string, [number, number]>();
  for (const [k, d] of perHex) {
    const t = k.slice(0, k.lastIndexOf('@'));
    // 308: a tipped marker is a state of the seat's ambassadors on its hex, not a piece ("-2 tipped ambassadors" at
    // cleanup read as two lost): counted only where the hex's ambassadors stayed (stood up, or tipped where they
    // stand); one that left or arrived tipped is already in the ambassadors' own count.
    if (t.startsWith('tipped:') && (perHex.get(`ambassador:${t.slice('tipped:'.length)}${k.slice(k.lastIndexOf('@'))}`) ?? 0) !== 0) continue;
    const [up, down] = tok.get(t) ?? [0, 0];
    tok.set(t, d > 0 ? [up + d, down] : [up, down + d]);
  }
  // 285: the short name, plural past one ("+5 agent", "+1 Fallout (nothing may be built here: ...)" read as jargon).
  const short = (k: string, n: number) => {
    if (k === 'cube:adversary') return Math.abs(n) === 1 ? 'corruption cube' : 'corruption cubes';
    const full = tokenName(k);
    const at = full.indexOf(' (');
    const head = at < 0 ? full : full.slice(0, at);
    // An owner ("node (seat 1)", "(adversary)") stays; a description ("(nothing may be built here ...)") goes.
    const tail = at >= 0 && /^ \((seat \d+|adversary)\)$/.test(full.slice(at)) ? full.slice(at) : '';
    const plural = Math.abs(n) === 1 || /s$/.test(head) ? head : /[^aeiou]y$/.test(head) ? `${head.slice(0, -1)}ies` : `${head}s`;
    return `${plural}${tail}`;
  };
  const tipped = (k: string, up: number, down: number) => {
    const seat = k.slice('tipped:'.length);
    const n = (x: number) => `${Math.abs(x)} ambassador${Math.abs(x) === 1 ? '' : 's'} (seat ${seat})`;
    return [up ? `${n(up)} used` : '', down ? `${n(down)} ready again` : ''].filter(Boolean);
  };
  // 312 (D140): as many added as taken off is a move ("2 adversary troops moved", a Redeploy), not "+2, -2".
  const pieces = [...tok].flatMap(([k, [up, down]]) => k.startsWith('tipped:') ? tipped(k, up, down)
    : up > 0 && up === -down ? [`${up} ${short(k, up)} moved`]
    : [up ? `+${up} ${short(k, up)}` : '', down ? `${down} ${short(k, down)}` : ''].filter(Boolean));
  // 308: a note's own closing period goes before the "; " joining it to the next part ("...country by country).;").
  const parts = [...notes.map((n) => n.replace(/\.$/, '')), ...(pieces.length ? [`map: ${pieces.join(', ')}`] : [])];
  // 265 (D115): no raw engine events ("seat 0 +1 energy", "cards moved: ..."): W5 accepts those in the debug
  // view's log only.
  if (parts.length) return parts.join('; ');
  // An income step has no note: say what kind of change it was, its lines in the debug view's log.
  const what = [
    events.some((e) => e.type === 'resource_changed') ? 'resources' : '',
    events.some((e) => e.type === 'card_moved') ? 'cards' : '',
  ].filter(Boolean).join(' and ');
  return what ? `${what} changed (the seat rows show them)` : 'bookkeeping only (tracks)';
}

/** `past`: whether the board drawn is the shown step's (`turnView`), not the board after the turn. */
function turnHtml(past: boolean): string {
  if (!turn) return '';
  // 265 (D115): one heading, "Since your last move": the steps run from the players' own phases (D Actions,
  // E income) through the adversary's, so "The adversary's turn" misnamed most of them.
  if (!turn.open) return `<button type="button" class="pturn-open" data-turn="open">Replay what ran since your last move (${turn.steps.length} step${turn.steps.length === 1 ? '' : 's'})</button>`;
  const t = turn;
  const rows = t.steps.map((s, i) => {
    const beat = beatOf(s.label);
    const line = `${s.label}${beat ? ` (${beat.replace("the adversary's ", '')})` : ''}: ${summary(s.events)}`;
    return `<li class="${i === t.at ? 'now' : ''}" data-turn-step="${i}" title="${esc(line)}">${esc(line)}</li>`;
  }).join('');
  const cur = t.steps[t.at];
  // 265 (D115): the rules' own notes only; every other event ("resource changed · seat 0 · ...") is counted here
  // and listed in the debug view's log (W5 accepts raw events there, not here).
  const shownEvents = cur.events.filter((e) => e.type === 'note');
  // 285: pieces on the map are counted apart from resources, cards and tracks (it called them all "resources, ...").
  const onMap = cur.events.filter((e) => e.type === 'tokens_changed').length;
  const rest = cur.events.length - shownEvents.length - onMap;
  const others = [
    onMap ? `${onMap} piece change${onMap === 1 ? '' : 's'} on the map (outlined)` : '',
    rest ? `${rest} other change${rest === 1 ? '' : 's'} (resources, cards, tracks)` : '',
  ].filter(Boolean);
  // 289 (D133): the step's notes come before the step list, which scrolls by itself (at 1366x768 the notes sat
  // under the list, below the side column's fold).
  const beat = beatOf(cur.label);
  // 305: the map shows the board as it stood at the end of the shown step (`turnView`). 320: the tracks and rows too.
  const mapNote = past
    ? `The map, tracks and rows show the board as it stood at the end of this step; the shown seat's hand, powers and buttons are the board now (▶ to the last step for the board after the turn). Outlined: the hexes this step changed.`
    : 'The board after the turn; outlined: the hexes this step changed.';
  return `<div class="pturn-head"><b>Since your last move</b> <span>step ${t.at + 1} of ${t.steps.length}: ${esc(cur.label)}${beat ? ` · <b class="pbeat">${esc(beat)}</b>` : ''}</span>
      <button type="button" data-turn="-1" ${t.at === 0 ? 'disabled' : ''}>◀</button><button type="button" data-turn="1" ${t.at === t.steps.length - 1 ? 'disabled' : ''}>▶</button>
      <button type="button" data-turn="close">Done</button></div>
    <div class="pturn-ev"><ul>${shownEvents.map((e) => `<li data-hexes="${hexesIn(e).map(hexKey).join(' ')}">${esc(contentIds(String(e.text)))}</li>`).join('')}${others.length ? `<li><small>${others.join('; ')}: the debug view's log lists them</small></li>` : ''}</ul><small>${esc(mapNote)}</small></div>
    <ol>${rows}</ol>`;
}

/** 289 (D133): the step list scrolls by itself; keep the shown step in it. */
function showTurnStep(el: HTMLElement) {
  const ol = el.querySelector<HTMLElement>('ol');
  const now = ol?.querySelector<HTMLElement>('li.now');
  if (ol && now) ol.scrollTop = Math.max(0, now.offsetTop - ol.offsetTop - ol.clientHeight / 2);
}

// ---------- render ----------

/** Draws the player view's own parts; the map, phase bar, status, adversary row and countries are main.ts's. */
export function renderPlayer(ui: PlayerUi, ctx: PlayerCtx) {
  claimed = new Set();
  const v = ctx.view;
  const focus = focusSeat(v, ctx.legal);
  const mine = ctx.legal.filter((a) => a.seat === focus);
  const board = ctx.board ?? v;
  ui.seats.innerHTML = stripHtml(board, ctx.legal, focus);

  // 261: an open question's answers are its seat's, whichever seat is shown (pinning another seat emptied the bar
  // that says "answer here").
  const answering = v.choice ? ctx.legal.filter((a) => a.seat === v.choice!.seat) : mine;
  ui.prompt.insertAdjacentHTML('beforeend', promptControls(ctx, answering));
  threatControls(ui.countries, ctx, mine);

  const s = v.seats[focus];
  const c = v.countries[s.country];
  const st = seatStatus(v, ctx.legal, focus);
  const draft = draftHtml(v, ctx, focus, mine);
  ui.seat.style.setProperty('--tint', strong(c?.color ?? '#cccccc'));
  // 289 (D127): during the draft and keep the packet takes the buildings column's width too (the buildings go under
  // the powers), and its printed cards are sized so their Draft / Keep buttons show without scrolling.
  ui.seat.innerHTML = `
    <div class="p-head"><span class="swatch"></span><b>${esc(seatName(v, focus))}</b> <small>${esc(countryName(v, s.country))}</small>${st.text ? `<span class="pst ${st.cls}">${st.text}</span>` : ''}</div>
    <div class="p-body${draft ? ' drafting' : ''}">
      <div class="p-info">${seatValuesHtml(board, focus)}</div>
      <div class="p-powers">${powersHtml(v, ctx, focus, mine)}</div>
      <div class="p-hand">${draft}${handHtml(v, ctx, focus, mine)}</div>
      <div class="p-built">${builtHtml(v, ctx, focus, mine)}</div>
    </div>`;
  // The draft opens at the panel's top, once a frame (a later render, a resize, keeps the player's scroll).
  if (draft && draftFrame !== ctx.frame) ui.seat.scrollTop = 0;
  draftFrame = draft ? ctx.frame : null;
  printDraft(ui.seat, DRAFT_SCALE);

  // The bar: standard actions, converters, pass, and every action of the seat no component drew.
  // 265 (D109): a standard action by its name and cost, its rule in the hover.
  const std = mine.filter((a) => a.type === 'standard').map((a) => btn(a, actionLabel(a, false, true, ctx.cx), '', actionRule(a, ctx.cx)));
  const conv = mine.filter((a) => a.type === 'convert').map((a) => btn(a, actionLabel(a, false, true, ctx.cx)));
  const pass = mine.filter((a) => a.type === 'pass').map((a) => btn(a, v.phase?.name === 'cards.keep' ? 'Done keeping (pass)' : 'Done (pass)', 'pass'));
  const rest = mine.filter((a) => !claimed.has(JSON.stringify(a))).map((a) => btn(a, actionLabel(a, false, true, ctx.cx), 'other'));
  const passers = (v.phase?.waiting ?? []).filter((x) => ctx.legal.some((a) => a.seat === x && a.type === 'pass'));
  const all = !v.outcome && !v.choice && passers.length > 1 ? `<button type="button" class="pass-all" data-pass-all="1">Pass every seat still to act (${passers.join(', ')})</button>` : '';
  const others = ctx.legal.filter((a) => a.seat !== focus).map((a) => a.seat);
  const switchTo = [...new Set(others)].map((x) => `<button type="button" data-seat-focus="${x}">${esc(seatName(v, x))} to act ▸</button>`).join('');
  // 294: the guarding rule, said where Liberate sits: the corrupted sites in the seat's reach a troop keeps it from
  // liberating (the engine's `liberate_refused`; the map marks each with a shield).
  const refused = s.values.find((x) => x.key === 'board.liberate_refused');
  // 321: not once the game is over (it stood beside the end banner).
  const guarded = refused?.value && !v.outcome
    ? `<span class="prefused" data-value="board.liberate_refused" data-scope="seat:${focus}" title="${esc(refused.doc ?? '')}">Liberate refused: ${refused.value} corrupted site${refused.value === 1 ? '' : 's'} in reach guarded by a troop</span>`
    : '';
  // 305: Liberate's bill and blockers, in the action phase only (it is an action-phase act).
  const lib = /board\.act/.test(v.phase?.name ?? '') && !v.outcome ? liberateHtml(v, focus) : '';
  ui.bar.innerHTML = [
    std.length || guarded ? `<span class="pgroup"><span class="what">Standard actions</span>${std.join('')}${guarded}</span>` : '',
    lib ? `<span class="pgroup lib">${lib}</span>` : '',
    conv.length ? `<span class="pgroup"><span class="what">Converters</span>${conv.join('')}</span>` : '',
    rest.length ? `<span class="pgroup other"><span class="what">Other</span>${rest.join('')}</span>` : '',
    `<span class="pgroup end">${pass.join('')}${all}${switchTo}</span>`,
  ].join('');

  ui.turn.innerHTML = turnHtml(board !== v);
  ui.turn.hidden = !turn;
  ui.turn.classList.toggle('phase-board', board !== v);
  showTurnStep(ui.turn);
  ui.more.innerHTML = pilesHtml(board) + varsHtml(board, true);
}
