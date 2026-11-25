// 242: the player view. The same engine view and legal actions as the debug view, laid out for play. 324: laid out
// so a new player can learn the game alone: the map in the middle; the adversary at the left, always shown; one
// panel at the right listing only what the shown seat can do now (what it can't, shaded with the reason); the
// seat's board as a strip of numbers and its hand and buildings as printed cards across the foot; the draft as
// printed cards over the map. Any action no component claims falls back to a labeled button under *Other*, so a new
// module shows with no new UI work. No rules live here.

import { type Action, type CardView, type Config, type Snapshot, type View, param, phaseViews, view as engineView } from '../engine';
import { actionLabel, actionRule, amount, contentIds, countryName, escapeHtml as esc, hexKey, hexesIn, isHex, strong, tokenName, type LabelContext } from '../format';
import { phaseGroups } from '../log';
import { cardChip, seatValuesHtml } from '../panels';
import type { Frame } from '../session';
import { CARD_PX, fitHost, printable, printedCard } from './printed';
import './player.css';

/** 324: what Undo may take back: `back` actions, to the start of the shown action phase; `why` it stops there early. */
export interface UndoRoom { back: number; why: string }

export interface PlayerCtx {
  view: View;
  legal: Action[];
  config: Config;
  cx: LabelContext;
  /** The frame shown, and the phase name it was taken in (the step before's). */
  frame: Frame;
  before: string;
  /** 320: the board the turn panel shows (`turnView`, an earlier step's), else `view`: the seat rows and the piles
   *  draw from it; the hand, powers, actions and every control from `view` (the board now). */
  board?: View;
  undo: UndoRoom;
  /** 324 (D142): the round of the last adversary turn the timeline holds, which its Replay button opens; else null. */
  replay: number | null;
}

export interface PlayerUi {
  seat: HTMLElement;
  act: HTMLElement;
  turn: HTMLElement;
  draft: HTMLElement;
  countries: HTMLElement;
  prompt: HTMLElement;
  boardBody: HTMLElement;
}

// ---------- focus ----------

/** A seat the player clicked; cleared by the next action, so the view follows whoever must act again. */
let pinned: number | null = null;
export const pin = (seat: number | null) => { pinned = seat; };

/** 321: the seat that took the last action. One person plays every seat: after seat 1's move the view went back to
 *  seat 0 (the first to act), so each of seat 1's moves took a click on its row first. */
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
const val = (v: View, s: number, k: string) => v.seats[s]?.values.find((x) => x.key === k);
/** "[building, tier 1, industry, +1 insight] rules" → the bracket and the rules. */
function splitText(text = ''): { head: string; rules: string } {
  const m = /^\[([^\]]*)\]\s*/.exec(text);
  return { head: m?.[1] ?? '', rules: text.slice(m?.[0].length ?? 0) };
}
/** The action phase, where Liberate, the powers and the building Actions are used (its key holds `board.act`). */
const actionPhase = (v: View) => /board\.act/.test(v.phase?.name ?? '');

// ---------- printed cards ----------

/** 324: printed hosts by card and scale, reused across renders (a face is fitted once, not on every action). */
const hosts = new Map<string, HTMLElement>();
const hostKey = (id: string, scale: number) => `${id}@${scale.toFixed(2)}`;

/** Puts each slot's printed card (`data-print`, at `data-scale` or `scale`) in its place under `root`. */
function placePrinted(root: HTMLElement, scale: number) {
  const used = new Set<HTMLElement>();
  for (const el of root.querySelectorAll<HTMLElement>('.pslot[data-print], .printed-host[data-print]')) {
    const id = el.dataset.print!;
    const sc = Math.round(Number(el.dataset.scale ?? scale) * 50) / 50;
    const key = hostKey(id, sc);
    let host = hosts.get(key);
    if (!host || used.has(host) || host === el) {
      if (host === el) {
        used.add(host);
        continue;
      }
      host = printedCard(id, sc) ?? undefined;
      if (host && !hosts.has(key)) hosts.set(key, host);
    }
    if (!host) {
      el.remove();
      continue;
    }
    used.add(host);
    host.dataset.print = id;
    host.dataset.scale = String(sc);
    el.replaceWith(host);
    // A cached host detached before its first frame (the draft's shrink loop) was never fitted: fit it now it is back.
    if (!host.dataset.fitted) {
      const h = host;
      requestAnimationFrame(() => fitHost(h));
    }
  }
}

// ---------- the seat strip: numbers, hand, buildings ----------

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

/** A seat's resources, each with its income ("credits 5 +3"). */
function resHtml(v: View, s: number): string {
  return RES_ORDER.filter((r) => val(v, s, `resource.${r}`) || val(v, s, `economy.income.${r}`)?.value).map((r) => {
    // 285: insight as the board shows it: the income plus any gift, added under Patience ("+"), else set ("→").
    const gift = r === 'insight' ? val(v, s, 'economy.insight_gift')?.value ?? 0 : 0;
    const raw = val(v, s, `economy.income.${r}`)?.value;
    const inc = raw ? raw + gift : raw;
    const sign = r === 'insight' && !val(v, s, 'economy.insight_carried') ? '→' : '+';
    // 285: the +N is upkeep's (factories' chips and stations' cubes come in production): its hover says so.
    const why = inc ? ` title="${esc(String(val(v, s, `economy.income.${r}`)?.doc ?? ''))}"` : '';
    return `<span class="sres" data-value="resource.${r}" data-scope="seat:${s}"${why}>${esc(r.replace(/_/g, ' '))} <b>${val(v, s, `resource.${r}`)?.value ?? 0}</b>${inc ? `<small>${sign}${inc}</small>` : ''}</span>`;
  }).join('');
}

/** 324: the board's tracks in one line each: its name and how far it has been uncovered ("Economy 3/8"). */
function tracksHtml(v: View, s: number): string {
  const b = v.seats[s]?.board;
  if (!b?.tracks.length) return '';
  return b.tracks.map((t) => {
    const mark = (t.steps[t.uncovered - 1]?.marks ?? []).map((m) => m.label).join(', ');
    return `<span class="strack" data-board-seat="${s}" title="${esc(`${t.label}: ${t.uncovered} of ${t.steps.length} spaces uncovered${mark ? `; the last one uncovered gives ${mark}` : ''}. Click the board's name for the whole board.`)}">${esc(t.label)} <b>${t.uncovered}</b><small>/${t.steps.length}</small></span>`;
  }).join('');
}

/** The seats' rows: every seat's numbers, the shown one first in its own color with its tracks. */
function seatsHtml(v: View, legal: Action[], focus: number): string {
  return v.seats.map((s) => {
    const c = v.countries[s.country];
    const st = seatStatus(v, legal, s.index);
    const hand = group(v, s.index, 'cards.hand')?.cards.length ?? 0;
    const nodes = val(v, s.index, 'board.node');
    const extra = [
      nodes ? `<span class="sres" data-value="board.node" data-scope="seat:${s.index}">nodes <b>${nodes.value}</b></span>` : '',
      `<span class="sres" title="cards in hand">hand <b>${hand}</b></span>`,
    ].join('');
    const mine = s.index === focus;
    const board = s.board ? `<button type="button" class="board-name" data-board-open="${s.index}" data-board-seat="${s.index}" title="the whole board: tracks, powers and what each space gives">${esc(s.board.name)} ▸</button>` : '';
    return `<div class="pseat${mine ? ' focus' : ''}" data-seat-focus="${s.index}" style="--tint:${strong(c?.color ?? '#cccccc')}"${v.seats.length > 1 ? ' title="show this seat"' : ''}>
      <div class="shead"><span class="swatch"></span><b>Seat ${s.index}</b> ${board} <small>${esc(countryName(v, s.country))}</small>${st.text ? `<span class="pst ${st.cls}">${st.text}</span>` : ''}</div>
      <div class="sline">${resHtml(v, s.index)}${extra}</div>
      ${mine ? `<div class="sline tracks">${tracksHtml(v, s.index)}</div>` : ''}</div>`;
  }).join('');
}

/** 324: why a card or power can't be used now, in words: the engine's status, a bare "unaffordable" given its price. */
function reason(v: View, s: number, c: CardView): string {
  const st = c.status ?? '';
  if (st === 'unaffordable' && c.cost !== undefined) {
    const have = val(v, s, 'resource.credits')?.value ?? 0;
    if (have < c.cost) return `can't afford it: costs ${c.cost} credits, you have ${have}`;
    return "can't afford it: it costs more than credits (its text says what)";
  }
  if (st === 'used') return 'used this round';
  // Ready by its own terms, but the seat has passed this phase, or it isn't this phase's to use (a hand card in the draft).
  if (st.startsWith('ready') && v.seats[s]?.passed && v.phase?.kind === 'decision') return 'not now: this seat has passed (Done) for this phase';
  if (st.startsWith('ready')) return 'not now: it is used in the action phase';
  if (st === 'waiting on the question') return 'answer the question first';
  return st.replace(/^unaffordable: /, "can't afford it: ").replace(/^needs tags: /, 'needs tags on your cards: ');
}

/** A printed card in the strip (or a compact tile for a card with no printed face), shaded when it can't be used. */
function cardSlot(s: number, c: CardView, where: string, ok: boolean, note: string, badge = ''): string {
  const face = printable(c.id) ? `<div class="pslot" data-print="${esc(c.id)}"></div>` : tile(s, c, where);
  return `<div class="pcard${ok ? ' ok' : ' blocked'}" data-card="${esc(c.id)}" data-where="seat:${s}:${where}"${printable(c.id) ? ` data-printed="${esc(c.id)}"` : ''}${note ? ` data-why="${esc(note)}"` : ''}>${face}${badge ? `<span class="pbadge">${esc(badge)}</span>` : ''}</div>`;
}

/** A compact tile for a card with no printed face. */
function tile(s: number, c: CardView, where: string): string {
  const { rules } = splitText(c.text);
  return `<div class="ptile" data-card="${esc(c.id)}" data-where="seat:${s}:${where}">
    <div class="pt-head">${c.cost !== undefined ? `<span class="pt-cost" title="credits">${c.cost}</span>` : ''}<b class="pt-name">${esc(c.name)}</b></div>
    <div class="pt-text">${esc(rules)}</div></div>`;
}

function handStrip(v: View, s: number, mine: Action[]): string {
  const hand = group(v, s, 'cards.hand');
  if (!hand) return '';
  const cards = hand.cards.map((c) => {
    const plays = mine.some((a) => a.type === 'play' && a.card === c.id);
    // 289 (D123): while a question is open the hand shows no play marks.
    const note = v.choice ? '' : plays ? (c.status.includes('no target') ? `playable, but ${c.status.replace(/^ready · /, '')}` : '') : reason(v, s, c);
    return cardSlot(s, c, 'cards.hand', plays || !!v.choice, note);
  });
  return `<div class="pcards hand"><h4>${esc(hand.label)} <small>${hand.cards.length}</small></h4><div class="prow-cards">${cards.join('') || '<span class="empty">no cards in hand</span>'}</div></div>`;
}

function builtStrip(v: View, s: number, mine: Action[]): string {
  const out: string[] = [];
  const built = group(v, s, 'cards.built');
  if (built) {
    const cards = built.cards.map((c) => {
      const uses = mine.some((a) => a.type === 'use' && a.card === c.id);
      const hasAction = !['built', 'stacked'].includes(c.status);
      const badge = c.status === 'used' ? 'Action used' : uses ? 'Action ready' : '';
      return cardSlot(s, c, 'cards.built', !hasAction || uses || c.status === 'used', hasAction && !uses && c.status !== 'used' ? `its Action: ${reason(v, s, c)}` : '', badge);
    });
    out.push(`<div class="pcards built"><h4>${esc(built.label)}</h4><div class="prow-cards">${cards.join('') || '<span class="empty">no buildings</span>'}</div></div>`);
  }
  // Any other card group of the seat (programs in play, events played, ...): printed when every card prints, else
  // the debug view's chips (a new module's group shows with no new UI work).
  const drawn = new Set(['cards.hand', 'cards.built', 'cards.packet', 'cards.drafted']);
  for (const g of v.seats[s].cards.filter((x) => !drawn.has(x.key) && x.cards.length)) {
    out.push(g.cards.every((c) => printable(c.id))
      ? `<div class="pcards other"><h4>${esc(g.label)} <small>${g.cards.length}</small></h4><div class="prow-cards">${g.cards.map((c) => cardSlot(s, c, g.key, true, '')).join('')}</div></div>`
      : `<div class="pcards chips"><h4>${esc(g.label)}</h4><div>${g.cards.map((c) => cardChip(c, `seat:${s}:${g.key}`)).join('')}</div></div>`);
  }
  return out.join('');
}

/** 324: the board's whole face, opened from its name: the numbers, tracks, powers and the innate rule. */
function boardHtml(v: View, s: number): string {
  const b = v.seats[s]?.board;
  if (!b) return '';
  const powers = b.cards.filter((c) => c.id.includes('.power.')).map((c) => `<li><b>${esc(c.name)}</b> <small>${esc(c.status)}</small><div>${esc(c.text ?? '')}</div></li>`).join('');
  return `<div class="plain-board tone-ai"><div class="strip"><span>AI board</span><span>Seat ${s}</span></div><h3>${esc(b.name)}</h3>
    ${b.innate ? `<p class="innate"><b>${esc(b.innate.name)}:</b> ${esc(b.innate.text ?? '')}${b.tags?.length ? ` <small>${b.tags.map(esc).join(', ')} tag (printed)</small>` : ''}</p>` : ''}
    <div class="pb-values">${seatValuesHtml(v, s)}</div>
    ${powers ? `<h4>Powers</h4><ul class="pb-powers">${powers}</ul>` : ''}</div>`;
}

// ---------- the draft ----------

/** 321: a card's tag needs the seat doesn't meet, in words ("needs 3 industry (you have 1)"); '' when met or none. */
function unmetNeeds(v: View, s: number, c: CardView): string {
  const m = /(?:^|, )needs ((?:\d+ [a-z]+(?:, (?=\d))?)+)/.exec(splitText(c.text).head);
  if (!m) return '';
  const have = (tag: string) => val(v, s, `cards.tag.${tag}`)?.value ?? 0;
  const short = m[1].split(', ').map((x) => x.split(' ')).filter(([n, tag]) => have(tag) < Number(n))
    .map(([n, tag]) => `${n} ${tag} (you have ${have(tag)})`);
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
      // 261: the base price, "from": the keep upgrade and Science lower a tier 2's (the drafted card says the seat's).
      const k = keepCost(Number(a.tier));
      return btn(a, `Tier ${a.tier} deck${typeof k === 'number' ? ` · keep ${Number(a.tier) === 2 ? 'up to ' : ''}${k} insight` : ''}`, 'primary big');
    }).join('')}</div>`);
  }
  const row = (cards: CardView[], key: string, act: (c: CardView) => string, note: (c: CardView) => string) =>
    `<div class="pdraft${cards.length > 5 ? ' many' : ''}">${cards.map((c) => `<div class="pdcard" data-card="${esc(c.id)}" data-where="seat:${s}:${key}"${printable(c.id) ? ` data-printed="${esc(c.id)}"` : ''}>
      ${printable(c.id) ? `<div class="pslot" data-print="${esc(c.id)}"></div>` : tile(s, c, key)}
      <div class="pd-note">${esc(note(c))}${unmetNeeds(v, s, c) ? ` · <span class="warn">${esc(unmetNeeds(v, s, c))}</span>` : ''}</div><div class="pd-acts">${act(c)}</div></div>`).join('')}</div>`;
  if (packet?.cards.length) {
    // 323: solo's draw-and-keep (`cards.take`) passes nothing: the hand's own label says how many to take.
    const passing = !(Number(param(ctx.config, 'cards', 'take')) > 0);
    parts.push(`<h4>${esc(packet.label)}${drafts.length && passing ? ' <small>take one; the rest pass on</small>' : ''}</h4>` + row(packet.cards, 'cards.packet',
      (c) => drafts.filter((a) => a.card === c.id).map((a) => btn(a, 'Draft', 'primary')).join(''),
      (c) => {
        const k = c.keep === undefined ? undefined : c.keep ? `${c.keep} insight` : 'free';
        return `${c.cost ?? '?'} credits to play${k !== undefined ? ` · keep ${k}` : ''}`;
      }));
  }
  if (drafted?.cards.length) {
    parts.push(`<h4>${esc(drafted.label)}</h4>` + row(drafted.cards, 'cards.drafted',
      (c) => keeps.filter((a) => a.card === c.id).map((a) => btn(a, `Keep${c.keep ? ` (${c.keep} insight)` : ''}`, 'primary')).join(''),
      (c) => `${c.cost ?? '?'} credits to play · ${c.status}`));
  }
  return `<div class="pdraft-head"><b>${esc(seatName(v, s))}: the draft</b><button type="button" class="pdraft-hide" data-draft-toggle="1">Show the map ▾</button></div>${parts.join('')}`;
}

/** The draft's printed cards at their largest. */
const DRAFT_SCALE = 0.85;
/** The frame the draft was last drawn for, and whether the player folded it away to see the map. */
let draftFrame: Frame | null = null;
let draftFolded = false;
export const toggleDraft = () => { draftFolded = !draftFolded; };

/**
 * 289 (D127): puts the draft's printed cards in their slots at `scale`, then shrinks them (to 0.28 at least) until the
 * lowest offered card, its Draft / Keep button included, ends inside the draft panel's visible height.
 */
function printDraft(box: HTMLElement, scale: number) {
  // 324: the largest scale that puts each row's cards side by side in the panel's width and its height (a note and a
  // button under each card); then the overflow loop below for whatever the estimate missed.
  const rows = [...box.querySelectorAll<HTMLElement>('.pdraft')];
  const n = Math.max(1, ...rows.map((r) => r.children.length));
  const fixed = [...box.children].filter((e) => !e.classList.contains('pdraft')).reduce((h, e) => h + (e as HTMLElement).offsetHeight, 0);
  const wFit = (box.clientWidth - 24 - (n - 1) * 10) / (n * CARD_PX.w);
  const hFit = ((box.clientHeight - fixed - 12) / Math.max(1, rows.length) - 62) / CARD_PX.h;
  if (box.clientWidth && box.clientHeight) scale = Math.max(0.28, Math.min(scale, wFit, hFit));
  placePrinted(box, scale);
  const cards = [...box.querySelectorAll<HTMLElement>('.pdcard')];
  if (!cards.length || !box.querySelector('.printed-host') || !box.clientHeight) return;
  for (let i = 0; i < 5 && scale > 0.28; i++) {
    const limit = Math.min(box.getBoundingClientRect().top + box.clientHeight, window.innerHeight) - 4;
    const bottom = Math.max(...cards.map((c) => c.getBoundingClientRect().bottom));
    if (bottom <= limit) return;
    // Each row of cards gives up its share of the overflow.
    const rows = new Set(cards.map((c) => Math.round(c.getBoundingClientRect().top))).size;
    const h = CARD_PX.h * scale;
    scale = Math.max(0.28, (scale * (h - (bottom - limit) / rows)) / h);
    for (const el of box.querySelectorAll<HTMLElement>('.printed-host[data-print]')) el.dataset.scale = String(scale);
    placePrinted(box, scale);
  }
}

// ---------- Liberate ----------

/**
 * 305: where Liberate stands for the shown seat: each corrupted site holding its ambassador, with the bill (its
 * ambassadors there first, then its supply) and what keeps it from liberating now. Read off the view; the engine's
 * legal actions still decide: the Liberate button shows only when it is legal. With no ambassador on a corrupted
 * site, the rule in one line.
 */
function liberateHtml(v: View, s: number): string {
  const rule = v.standard.find((x) => x.act === 'liberate');
  if (!rule) return '';
  const doc = rule.doc ?? '';
  const price = (kind: string) => rule.sites?.find((x) => x.kind === kind)?.ambassadors ?? NaN;
  const count = (h: View['hexes'][number], k: string) => h.tokens.find((t) => t.kind === k)?.count ?? 0;
  const supply = val(v, s, 'resource.ambassadors')?.value ?? 0;
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
  // 308: one price when every site's is the same.
  const prices = ['city', 'station', 'factory'].map((k) => price(k));
  const bills = prices.every((p) => p === prices[0]) ? `${prices[0]} ambassadors at any site`
    : ['city', 'station', 'factory'].map((k, i) => `${prices[i]} ambassadors a ${k}`).join(', ');
  return `<div class="plib" title="${esc(doc)}">${lines.length ? lines.join('') : `<small>Liberate: move a ready ambassador onto a corrupted site, then act with it: ${esc(bills)} (those on the site first, then your supply); no agent on it, no troop on or next to it</small>`}</div>`;
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

/** 321: an answer naming a card (a project to discard, a card to take) hovers that card as the view has it. */
function optionHover(v: View, a: Action): string {
  const id = typeof a.option === 'string' ? a.option : typeof a.card === 'string' ? a.card : '';
  if (!id) return '';
  const pools = [...v.piles.map((p) => p.visible), ...v.seats.flatMap((s) => s.cards.map((g) => g.cards)), ...v.countries.flatMap((c) => c.cards.map((g) => g.cards))];
  const c = pools.flat().find((x) => x.id === id);
  return c ? [c.name, c.status, c.text].filter(Boolean).join(' · ') : '';
}

// ---------- what you can do now ----------

/** One power or building Action as a row of the panel: its name, its state, Use when legal, the reason when not. */
function useRow(v: View, ctx: PlayerCtx, s: number, c: CardView, acts: Action[], where: string): string {
  const st = c.status ?? '';
  const cls = acts.length ? 'ready' : st.startsWith('used') ? 'used' : 'blocked';
  const why = acts.length ? '' : reason(v, s, c);
  const use = acts.map((a) => btn(a, 'Use', 'primary', actionLabel(a, false, true, ctx.cx))).join('');
  // A power's rules under its name (the hover has nothing for a board's card); a building's is its printed card's.
  const text = c.id.includes('.power.') && c.text ? `<div class="ptext">${esc(c.text)}</div>` : '';
  const printed = !c.id.includes('.power.') && printable(c.id) ? ` data-printed="${esc(c.id)}"` : '';
  return `<li class="${cls}" data-card="${esc(c.id)}" data-where="${where}"${printed}${why ? ` data-why="${esc(why)}"` : ''}><b>${esc(c.name)}</b> <small>${esc(acts.length ? 'ready' : st.startsWith('used') ? 'used' : why)}</small>${use}${text}</li>`;
}

/** The panel's sections, each only if it has something; the shown seat's legal actions, every one with a control. */
function actionsHtml(ctx: PlayerCtx, s: number, mine: Action[]): string {
  const v = ctx.view;
  const sec = (title: string, body: string, cls = '') => body ? `<div class="pa-sec ${cls}"><h4>${title}</h4>${body}</div>` : '';
  const parts: string[] = [];
  const inAct = actionPhase(v) && !v.outcome;

  // Cards: each hand card with a legal play or sale; one that can't be played now is shaded, its reason beside it.
  const hand = group(v, s, 'cards.hand')?.cards ?? [];
  const cardRows = hand.map((c) => {
    const acts = mine.filter((a) => (a.type === 'play' || a.type === 'sell') && a.card === c.id);
    if (!acts.length) return '';
    const plays = acts.some((a) => a.type === 'play');
    const buttons = acts.map((a) => btn(a, a.type === 'play' ? 'Play' : actionLabel(a, false, false, ctx.cx).replace(/^sell [^(]*/, 'Sell '), a.type === 'play' ? 'primary' : 'minor', actionLabel(a, false, true, ctx.cx))).join('');
    const why = plays ? (c.status.includes('no target') ? c.status.replace(/^ready · /, '') : '') : reason(v, s, c);
    return `<div class="pa-card${plays ? '' : ' blocked'}" data-card="${esc(c.id)}" data-where="seat:${s}:cards.hand"${printable(c.id) ? ` data-printed="${esc(c.id)}"` : ''}${why ? ` data-why="${esc(why)}"` : ''}><span class="pa-cost">${c.cost ?? ''}</span><b>${esc(c.name)}</b>${why ? `<small>${esc(why)}</small>` : ''}<span class="pa-btns">${buttons}</span></div>`;
  }).join('');
  parts.push(sec('Play a card', cardRows));

  // Powers: the node powers (each once a round) and the others; a used or blocked one stays listed, shaded.
  const b = v.seats[s]?.board;
  const powers = (b?.cards ?? []).filter((c) => c.id.includes('.power.'));
  const powerActs = (c: CardView) => mine.filter((a) => a.type === 'power' && Number(a.power) === Number(c.id.split('.power.')[1]));
  if (inAct && powers.length) {
    const node = powers.filter((c) => c.node);
    const other = powers.filter((c) => !node.includes(c));
    const used = node.filter((c) => c.status.startsWith('used')).length;
    parts.push([
      node.length ? `<div class="pa-sec"><h4 class="pnode-head">Node powers <small>each once a round · ${used ? `${used} of ${node.length} used` : 'none used yet'}</small></h4><ul class="ppowers node">${node.map((c) => useRow(v, ctx, s, c, powerActs(c), `board:${s}`)).join('')}</ul></div>` : '',
      other.length ? `<div class="pa-sec"><h4>${node.length ? 'Other powers' : 'Powers'}</h4><ul class="ppowers">${other.map((c) => useRow(v, ctx, s, c, powerActs(c), `board:${s}`)).join('')}</ul></div>` : '',
    ].join(''));
  } else {
    // Outside the action phase a power can still be legal (a module's own timing): its Use button, nothing else.
    const legalPowers = powers.filter((c) => powerActs(c).length);
    if (legalPowers.length) parts.push(sec('Powers', `<ul class="ppowers">${legalPowers.map((c) => useRow(v, ctx, s, c, powerActs(c), `board:${s}`)).join('')}</ul>`));
  }

  // Building Actions: each building with an Action, ready, used or blocked.
  const built = group(v, s, 'cards.built')?.cards ?? [];
  const actionCards = built.filter((c) => !['built', 'stacked'].includes(c.status) || mine.some((a) => a.type === 'use' && a.card === c.id));
  const useActs = (c: CardView) => mine.filter((a) => a.type === 'use' && a.card === c.id);
  const shownBuilt = inAct ? actionCards : actionCards.filter((c) => useActs(c).length);
  if (shownBuilt.length) parts.push(sec('Building Actions <small>each once a round</small>', `<ul class="ppowers built">${shownBuilt.map((c) => useRow(v, ctx, s, c, useActs(c), `seat:${s}:cards.built`)).join('')}</ul>`));

  // Standard actions, Liberate, converters.
  const std = mine.filter((a) => a.type === 'standard').map((a) => btn(a, actionLabel(a, false, true, ctx.cx), '', actionRule(a, ctx.cx)));
  // 294: the corrupted sites in the seat's reach a troop keeps it from liberating (the map marks each with a shield).
  const refused = v.seats[s].values.find((x) => x.key === 'board.liberate_refused');
  const guarded = refused?.value && !v.outcome
    ? `<span class="prefused" data-value="board.liberate_refused" data-scope="seat:${s}" title="${esc(refused.doc ?? '')}">Liberate refused: ${refused.value} corrupted site${refused.value === 1 ? '' : 's'} in reach guarded by a troop</span>`
    : '';
  const lib = inAct ? liberateHtml(v, s) : '';
  parts.push(sec('Standard actions', std.length || guarded || lib ? `<div class="pa-btnrow">${std.join('')}</div>${guarded}${lib}` : ''));
  const conv = mine.filter((a) => a.type === 'convert').map((a) => btn(a, actionLabel(a, false, true, ctx.cx)));
  parts.push(sec('Converters', conv.length ? `<div class="pa-btnrow">${conv.join('')}</div>` : ''));

  // Threats: pay toward one, take a payment back, counter one outright.
  const byThreat = new Map<string, Action[]>();
  for (const a of mine) if (['pay', 'withdraw', 'counter_with'].includes(a.type) && typeof a.threat === 'string') byThreat.set(a.threat, [...(byThreat.get(a.threat) ?? []), a]);
  const threatRows = [...byThreat].map(([id, acts]) => {
    const card = v.countries.flatMap((c) => c.cards.flatMap((g) => g.cards)).find((c) => c.id === id);
    const src = v.threat_sources.find((x) => x.threat === id);
    const hexes = src ? src.hexes.map((i) => hexKey(v.hexes[i].hex)).join(' ') : '';
    const label = (a: Action) => {
      // 261: "1 credit", not "1 credits". 308: "pay", not "+".
      if (a.type === 'pay') return `pay ${amount(Number(a.amount), String(a.resource))}`;
      if (a.type === 'withdraw') return `take back ${String(a.resource).replace(/_/g, ' ')}`;
      return actionLabel(a, false, true, ctx.cx).replace(/^counter [^:]*: ?/, 'counter: ');
    };
    return `<div class="pa-threat" data-card="${esc(id)}" data-where="threat"${hexes ? ` data-hexes="${hexes}"` : ''}><b>${esc(card?.name ?? id)}</b> <small>${esc(card?.status ?? '')}</small><div class="pa-btnrow">${acts.map((a) => btn(a, label(a), a.type === 'counter_with' ? 'primary' : '', actionLabel(a, false, true, ctx.cx))).join('')}</div></div>`;
  }).join('');
  parts.push(sec('Counter threats', threatRows));

  // Anything no component drew.
  const rest = mine.filter((a) => !claimed.has(JSON.stringify(a)) && a.type !== 'pass' && a.type !== 'choose');
  parts.push(sec('Other', rest.length ? `<div class="pa-btnrow">${rest.map((a) => btn(a, actionLabel(a, false, true, ctx.cx), 'other')).join('')}</div>` : '', 'other'));
  return parts.join('');
}

/** 324: the powers and building Actions the seat hasn't used this round, those it can use now first. */
function reminderHtml(ctx: PlayerCtx, s: number, mine: Action[]): string {
  const v = ctx.view;
  if (!actionPhase(v) || v.outcome || v.choice || v.seats[s]?.passed) return '';
  // Every power not used this round (each once a round: a tiered power's group once between its tiers).
  const powers = (v.seats[s]?.board?.cards ?? []).filter((c) => c.id.includes('.power.') && !c.status.startsWith('used'));
  const built = (group(v, s, 'cards.built')?.cards ?? []).filter((c) => !['built', 'stacked', 'used', 'face down'].includes(c.status));
  const ready = (c: CardView, type: string) => mine.some((a) => a.type === type && (type === 'use' ? a.card === c.id : Number(a.power) === Number(c.id.split('.power.')[1])));
  const now = [...powers.filter((c) => ready(c, 'power')), ...built.filter((c) => ready(c, 'use'))].map((c) => c.name);
  const later = [...powers.filter((c) => !ready(c, 'power')), ...built.filter((c) => !ready(c, 'use'))].map((c) => c.name);
  if (!now.length && !later.length) return '';
  return `<div class="pa-remind">${now.length ? `<b>Not used yet this round:</b> ${esc(now.join(', '))}` : ''}${later.length ? `<small>${now.length ? ' · ' : '<b>Not used yet, blocked now:</b> '}${now.length ? 'blocked now: ' : ''}${esc(later.join(', '))}</small>` : ''}</div>`;
}

function undoHtml(u: UndoRoom, v: View): string {
  if (!actionPhase(v) || v.outcome) return '';
  if (!u.back && !u.why) return '';
  return `<div class="pa-undo"><button type="button" data-undo="1" ${u.back ? '' : 'disabled'} title="take back your last action">Undo</button><button type="button" data-undo="start" ${u.back > 1 ? '' : 'disabled'} title="take back every action since this action phase began${u.why ? ` (back to the draw: ${u.why})` : ''}">Undo to the phase's start${u.back > 1 ? ` (${u.back})` : ''}</button>${u.why ? `<small>${esc(u.why)}</small>` : ''}</div>`;
}

// ---------- the adversary's turn ----------

/** A step of a turn: a phase group of one frame's events (`end`: the mark closing it, `apply_phases`' key). */
interface Step { label: string; events: Frame['events']; end: number; phase?: string; frame: Frame }
interface Turn {
  /** The frame shown when it opened (the panel follows the game after that). */
  cur: Frame;
  steps: Step[];
  at: number;
  open: boolean;
  /** 324 (D142): opened from its Replay button, after the game moved on. */
  replay: boolean;
  /** 320: the board at each mark of a step's frame, fetched once, the first time an earlier step shows. */
  views: Map<Frame, { at: number; view: View }[]>;
}
let turn: Turn | null = null;
/** The adversary's phases by their rulebook step (`shortPhase`): B its card and lines, F threats. */
const ADV_PHASE = /^[BF] /;
export const isAdversaryPhase = (label: string) => ADV_PHASE.test(label);

/**
 * The steps of a frame that ran through several phases (the adversary's turn after the last pass), else none.
 * `within`: the frame is an answer given inside a turn already under way (324's replay): its every step counts.
 */
function turnOf(f: Frame, before: string, within = false): Step[] {
  if (!f.action) return [];
  const groups = phaseGroups(f.events, before).filter((g) => !g.round);
  // 289 (D133): an answer given inside the adversary's phases (a node's protection against a troop's strike) runs the
  // rest of that phase in its own frame: it is a step of the turn too. Only one that struck or destroyed something.
  const answered = f.action.type === 'choose' && groups.length === 1 && ADV_PHASE.test(groups[0].label)
    && groups[0].events.some((e) => e.type === 'note' && / strikes | destroyed| holds: /.test(String(e.text)));
  if (groups.length < 2 && !answered && !within) return [];
  // A phase that changed nothing is no step.
  return groups.filter((g) => g.events.length).map((g) => ({ label: g.label, events: g.events, end: g.end, phase: g.phase, frame: f }));
}

/** 324 (D142): whether a frame ran the adversary's turn (its Replay button finds the last one). */
export const isAdversaryTurn = (f: Frame, before: string) => turnOf(f, before).some((s) => ADV_PHASE.test(s.label));

/** The hexes the shown step of the turn changed, or null when no turn is open for the frame shown, `f`. */
export function turnHexes(f: Frame, before: string, over = false): Set<string> | null {
  if (turn?.cur !== f) {
    const steps = turnOf(f, before);
    // It opens by itself only when the adversary's phases ran (its turn); phases between two of the seats'
    // decisions wait folded under their Replay button. 289 (D129): a frame that ended the game opens at its last step.
    turn = steps.length
      ? { cur: f, steps, at: over ? steps.length - 1 : 0, open: over || steps.some((s) => ADV_PHASE.test(s.label)), replay: false, views: new Map() }
      : null;
  }
  if (!turn?.open) return null;
  return new Set(turn.steps[turn.at].events.flatMap(hexesIn).map(hexKey));
}

/**
 * 324 (D142): opens an earlier adversary turn over the frame shown, `cur`: `parts`, its frames in order (the last
 * pass, then any answers given inside the turn), each with the phase it was taken in.
 */
export function replayTurn(parts: { frame: Frame; before: string }[], cur: Frame) {
  const steps = parts.flatMap((p, i) => turnOf(p.frame, p.before, i > 0));
  if (steps.length) turn = { cur, steps, at: 0, open: true, replay: true, views: new Map() };
}

/** The frame of the step the open turn panel shows, or null. */
export const turnStepFrame = () => (turn?.open ? turn.steps[turn.at]?.frame ?? null : null);

/**
 * 305: the board as it stood at the end of the shown step of an open turn, or null when that is the board now (the
 * last step of the frame shown). 320: the whole board, the engine's own view of it (`phaseViews` replays the step's
 * action from the state before it, `prevOf`, and keeps the view at each phase mark).
 */
export function turnView(prevOf: (f: Frame) => Snapshot | undefined): View | null {
  if (!turn?.open) return null;
  const step = turn.steps[turn.at];
  if (!step?.frame.action) return null;
  if (turn.at >= turn.steps.length - 1 && !turn.replay) return null;
  // 324: a frame's last step is that frame's end: its board (a replayed turn's last step is the board the turn left).
  if (step.end >= step.frame.events.length) return engineView(step.frame.snap);
  let views = turn.views.get(step.frame);
  if (!views) {
    const prev = prevOf(step.frame);
    try {
      views = prev ? phaseViews(prev, step.frame.action) : [];
    } catch {
      // An action the engine refuses from its own state (none should) shows the board after the turn.
      views = [];
    }
    turn.views.set(step.frame, views);
  }
  return views.find((x) => x.at === step.end)?.view ?? null;
}

/**
 * 320: `v` (an earlier step's board, `turnView`) for the phase bar: a board at a mark stands at the next phase's
 * door, so the bar marks the shown step's own phase.
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
  if (d === 'close') {
    // A replay closes back to the frame shown's own (closed) panel.
    if (turn.replay) turn = null;
    else turn.open = false;
  } else if (d === 'open') turn = { ...turn, at: 0, open: true };
  else turn.at = Math.max(0, Math.min(turn.steps.length - 1, turn.at + d));
}

/** A step in words: the rules' own notes, then the pieces it added or took off the map, then what else moved. */
function summary(events: Frame['events']): string {
  const notes = [...new Set(events.filter((e) => e.type === 'note').map((e) => contentIds(String(e.text))))];
  // 285: netted per hex, then added and taken off apart, so a move (-1 here, +1 there) reads as both.
  const perHex = new Map<string, number>();
  for (const e of events) {
    if (e.type !== 'tokens_changed') continue;
    const k = `${String(e.token)}@${JSON.stringify(e.hex ?? null)}`;
    perHex.set(k, (perHex.get(k) ?? 0) + Number(e.delta));
  }
  const tok = new Map<string, [number, number]>();
  for (const [k, d] of perHex) {
    const t = k.slice(0, k.lastIndexOf('@'));
    // 308: a tipped marker is a state of the seat's ambassadors on its hex, not a piece.
    if (t.startsWith('tipped:') && (perHex.get(`ambassador:${t.slice('tipped:'.length)}${k.slice(k.lastIndexOf('@'))}`) ?? 0) !== 0) continue;
    const [up, down] = tok.get(t) ?? [0, 0];
    tok.set(t, d > 0 ? [up + d, down] : [up, down + d]);
  }
  const short = (k: string, n: number) => {
    if (k === 'cube:adversary') return Math.abs(n) === 1 ? 'corruption cube' : 'corruption cubes';
    const full = tokenName(k);
    const at = full.indexOf(' (');
    const head = at < 0 ? full : full.slice(0, at);
    const tail = at >= 0 && /^ \((seat \d+|adversary)\)$/.test(full.slice(at)) ? full.slice(at) : '';
    const plural = Math.abs(n) === 1 || /s$/.test(head) ? head : /[^aeiou]y$/.test(head) ? `${head.slice(0, -1)}ies` : `${head}s`;
    return `${plural}${tail}`;
  };
  const tipped = (k: string, up: number, down: number) => {
    const seat = k.slice('tipped:'.length);
    const n = (x: number) => `${Math.abs(x)} ambassador${Math.abs(x) === 1 ? '' : 's'} (seat ${seat})`;
    return [up ? `${n(up)} used` : '', down ? `${n(down)} ready again` : ''].filter(Boolean);
  };
  // 312 (D140): as many added as taken off is a move.
  const pieces = [...tok].flatMap(([k, [up, down]]) => k.startsWith('tipped:') ? tipped(k, up, down)
    : up > 0 && up === -down ? [`${up} ${short(k, up)} moved`]
    : [up ? `+${up} ${short(k, up)}` : '', down ? `${down} ${short(k, down)}` : ''].filter(Boolean));
  const parts = [...notes.map((n) => n.replace(/\.$/, '')), ...(pieces.length ? [`map: ${pieces.join(', ')}`] : [])];
  if (parts.length) return parts.join('; ');
  const what = [
    events.some((e) => e.type === 'resource_changed') ? 'resources' : '',
    events.some((e) => e.type === 'card_moved') ? 'cards' : '',
  ].filter(Boolean).join(' and ');
  return what ? `${what} changed (the seat rows show them)` : 'bookkeeping only (tracks)';
}

/** `past`: whether the board drawn is the shown step's (`turnView`), not the board now. */
function turnHtml(past: boolean, replay: number | null): string {
  // 324 (D142): the last adversary turn on its own button all round.
  const again = replay !== null && !(turn?.open) ? `<button type="button" class="pturn-open" data-turn="replay">Replay the adversary's turn (round ${replay})</button>` : '';
  if (!turn) return again;
  if (!turn.open) return `${again}<button type="button" class="pturn-open" data-turn="open">Replay what ran since your last move (${turn.steps.length} step${turn.steps.length === 1 ? '' : 's'})</button>`;
  const t = turn;
  const rows = t.steps.map((s, i) => {
    const beat = beatOf(s.label);
    const line = `${s.label}${beat ? ` (${beat.replace("the adversary's ", '')})` : ''}: ${summary(s.events)}`;
    return `<li class="${i === t.at ? 'now' : ''}" data-turn-step="${i}" title="${esc(line)}">${esc(line)}</li>`;
  }).join('');
  const cur = t.steps[t.at];
  const shownEvents = cur.events.filter((e) => e.type === 'note');
  const onMap = cur.events.filter((e) => e.type === 'tokens_changed').length;
  const rest = cur.events.length - shownEvents.length - onMap;
  const others = [
    onMap ? `${onMap} piece change${onMap === 1 ? '' : 's'} on the map (outlined)` : '',
    rest ? `${rest} other change${rest === 1 ? '' : 's'} (resources, cards, tracks)` : '',
  ].filter(Boolean);
  const beat = beatOf(cur.label);
  const mapNote = past
    ? `The map, tracks and adversary panel show the board as it stood at the end of this step; your hand, powers and buttons are the board now${t.replay ? '' : ' (▶ to the last step for the board after the turn)'}. Outlined: the hexes this step changed.`
    : 'The board after the turn; outlined: the hexes this step changed.';
  const title = t.replay ? "The adversary's turn (replay)" : 'Since your last move';
  return `<div class="pturn-head"><b>${title}</b> <span>step ${t.at + 1} of ${t.steps.length}: ${esc(cur.label)}${beat ? ` · <b class="pbeat">${esc(beat)}</b>` : ''}</span>
      <button type="button" data-turn="-1" ${t.at === 0 ? 'disabled' : ''}>◀</button><button type="button" data-turn="1" ${t.at === t.steps.length - 1 ? 'disabled' : ''}>▶</button>
      <button type="button" data-turn="close">Done</button></div>
    <div class="pturn-ev"><ul>${shownEvents.map((e) => `<li data-hexes="${hexesIn(e).map(hexKey).join(' ')}">${esc(contentIds(String(e.text)))}</li>`).join('')}${others.length ? `<li><small>${others.join('; ')}: the debug view's log (V) lists them</small></li>` : ''}</ul><small>${esc(mapNote)}</small></div>
    <ol>${rows}</ol>`;
}

/** 289 (D133): the step list scrolls by itself; keep the shown step in it. */
function showTurnStep(el: HTMLElement) {
  const ol = el.querySelector<HTMLElement>('ol');
  const now = ol?.querySelector<HTMLElement>('li.now');
  if (ol && now) ol.scrollTop = Math.max(0, now.offsetTop - ol.offsetTop - ol.clientHeight / 2);
}

// ---------- render ----------

/** The printed cards' scale in the strip: the row's height, a card's height at most (the strip's CSS sizes it). */
function stripScale(seat: HTMLElement): number {
  const row = seat.querySelector<HTMLElement>('.pcards .prow-cards');
  const h = row?.clientHeight ?? 0;
  return Math.max(0.25, Math.min(0.6, (h - 6) / CARD_PX.h));
}

/** Draws the player view's own parts; the map, phase bar, adversary panel and countries are main.ts's. */
export function renderPlayer(ui: PlayerUi, ctx: PlayerCtx) {
  claimed = new Set();
  const v = ctx.view;
  const focus = focusSeat(v, ctx.legal);
  const mine = ctx.legal.filter((a) => a.seat === focus);
  const board = ctx.board ?? v;

  // 261: an open question's answers are its seat's, whichever seat is shown.
  const answering = v.choice ? ctx.legal.filter((a) => a.seat === v.choice!.seat) : mine;
  ui.prompt.insertAdjacentHTML('beforeend', promptControls(ctx, answering));

  const s = v.seats[focus];
  const c = v.countries[s.country];
  const tint = strong(c?.color ?? '#cccccc');

  // The draft, over the map.
  const draft = draftHtml(v, ctx, focus, mine);
  if (draft && draftFrame !== ctx.frame && draftFrame?.snap.state.phase !== ctx.frame.snap.state.phase) draftFolded = false;
  ui.draft.hidden = !draft || draftFolded;
  ui.draft.innerHTML = draft;
  ui.draft.style.setProperty('--tint', tint);
  if (draft && draftFrame !== ctx.frame) ui.draft.scrollTop = 0;
  draftFrame = draft ? ctx.frame : null;
  if (draft && !draftFolded) printDraft(ui.draft, DRAFT_SCALE);

  // The strip: the seats' numbers, then the shown seat's hand and buildings, printed.
  ui.seat.style.setProperty('--tint', tint);
  ui.seat.innerHTML = `<div id="p-seats">${seatsHtml(board, ctx.legal, focus)}</div>${handStrip(v, focus, mine)}${builtStrip(v, focus, mine)}`;
  placePrinted(ui.seat, stripScale(ui.seat));
  ui.boardBody.innerHTML = boardHtml(board, focus);

  // What the shown seat can do now.
  const st = seatStatus(v, ctx.legal, focus);
  const pass = mine.filter((a) => a.type === 'pass').map((a) => btn(a, v.phase?.name === 'cards.keep' ? 'Done keeping (pass)' : 'Done (pass)', 'pass'));
  // 321 (D143): no *Pass every seat* in the keep phase: a keep is each seat's own choice.
  const passers = (v.phase?.waiting ?? []).filter((x) => ctx.legal.some((a) => a.seat === x && a.type === 'pass'));
  const all = !v.outcome && !v.choice && passers.length > 1 && v.phase?.name !== 'cards.keep' ? `<button type="button" class="pass-all" data-pass-all="1">Pass every seat still to act (${passers.join(', ')})</button>` : '';
  const others = [...new Set(ctx.legal.filter((a) => a.seat !== focus).map((a) => a.seat))];
  const switchTo = others.map((x) => `<button type="button" data-seat-focus="${x}">${esc(seatName(v, x))} to act ▸</button>`).join('');
  const body = v.outcome ? '<p class="pa-note">The game is over: New game (at the top) starts another.</p>'
    : v.choice && v.choice.seat === focus ? '<p class="pa-note">Answer the question above the map first.</p>'
    : v.choice ? `<p class="pa-note">${esc(seatName(v, v.choice.seat))} is answering a question above the map.</p>`
    : draft ? `<p class="pa-note">${draftFolded ? 'The draft is folded away: ' : ''}Pick from the cards over the map.${draftFolded ? ' <button type="button" data-draft-toggle="1">Show the draft ▴</button>' : ''}</p>${actionsHtml(ctx, focus, mine)}`
    : actionsHtml(ctx, focus, mine);
  const phase = v.phase && !v.outcome ? `<small class="pa-phase">${esc(st.text === 'to act' ? (actionPhase(v) ? 'your action phase: use any of these, in any order, then Done' : 'your move') : st.text === 'done' ? 'done for this phase' : '')}</small>` : '';
  ui.act.style.setProperty('--tint', tint);
  ui.act.innerHTML = `<div class="pa-head"><span class="swatch"></span><b>What ${esc(seatName(v, focus))} can do now</b>${st.text ? `<span class="pst ${st.cls}">${st.text}</span>` : ''}${phase}</div>
    ${undoHtml(ctx.undo, v)}${reminderHtml(ctx, focus, mine)}
    <div class="pa-body">${body}</div>
    <div class="pa-foot">${pass.join('')}${all}${switchTo}</div>`;

  ui.turn.innerHTML = turnHtml(board !== v, ctx.replay);
  ui.turn.hidden = !ui.turn.innerHTML.trim();
  ui.turn.classList.toggle('phase-board', board !== v);
  ui.turn.classList.toggle('open', !!turn?.open);
  ui.turn.classList.toggle('replay', !!turn?.open && turn.replay);
  showTurnStep(ui.turn);
}
