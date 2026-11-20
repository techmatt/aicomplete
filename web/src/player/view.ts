// 242: the player view. The same engine view and legal actions as the debug view, laid out for play: the map, the
// adversary's pressure, a summary row per seat, and the focused seat's board, hand and controls. Each legal
// action of the focused seat gets its control on the object it acts on (a card in hand, a power, a threat, a
// hex); any action no component claims, and any view part no component draws, falls back to the generic
// rendering (a labeled button, a chip), so a new module shows with no new UI work. No rules live here.

import { param, type Action, type CardView, type Config, type View } from '../engine';
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

/** The seat the view shows: the pinned one, else the open choice's, else the first seat to act. */
export function focusSeat(v: View, legal: Action[]): number {
  if (pinned !== null && v.seats[pinned]) return pinned;
  if (v.choice) return v.choice.seat;
  const acting = legal.map((a) => a.seat);
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
  return `<div class="ptile${ok ? ' ok' : ''}" data-card="${esc(c.id)}" data-where="seat:${s}:${where}"${printable(c.id) ? ` data-printed="${esc(c.id)}"` : ''}>
    <div class="pt-head">${c.cost !== undefined ? `<span class="pt-cost" title="credits">${c.cost}</span>` : ''}<b class="pt-name">${esc(c.name)}</b></div>
    ${head ? `<div class="pt-meta">${esc(head)}</div>` : ''}
    <div class="pt-text">${esc(rules)}</div>
    ${status ? `<div class="pt-status${ok ? '' : ' warn'}">${esc(status)}</div>` : ''}
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
      <div class="pd-note">${esc(note(c))}</div><div class="pd-acts">${act(c)}</div></div>`).join('')}</div>`;
  if (packet?.cards.length) {
    // 289 (D129): "take one" only once there is a Draft button (it showed before the tier build was chosen).
    parts.push(`<h4>${esc(packet.label)}${drafts.length ? ' <small>take one; the rest pass on</small>' : ''}</h4>` + row(packet.cards, 'cards.packet',
      (c) => drafts.filter((a) => a.card === c.id).map((a) => btn(a, 'Draft', 'primary')).join(''),
      (c) => {
        // 261: the seat's own keep price, from the tile's status (the config's base ignored the keep upgrade).
        const k = /keep (\d+ insight|free)/.exec(c.status)?.[1];
        return `${c.cost ?? '?'} credits to play${k !== undefined ? ` · keep ${k}` : ''}`;
      }));
  }
  if (drafted?.cards.length) {
    parts.push(`<h4>${esc(drafted.label)}</h4>` + row(drafted.cards, 'cards.drafted',
      (c) => keeps.filter((a) => a.card === c.id).map((a) => btn(a, `Keep${/keep (\d+) insight/.test(c.status) ? ` (${/keep (\d+) insight/.exec(c.status)![1]} insight)` : ''}`, 'primary')).join(''),
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
  const rows = powers.map((c) => {
    const k = Number(c.id.split('.power.')[1]);
    const acts = mine.filter((a) => a.type === 'power' && Number(a.power) === k).map((a) => btn(a, 'Use', 'primary', actionLabel(a, false, true, ctx.cx))).join('');
    // 295: a power used this round says so in its own style (three node powers may each be used in one round).
    return `<li class="${c.status.startsWith('ready') ? 'ready' : c.status.startsWith('used') ? 'used' : ''}" data-card="${esc(c.id)}" data-where="board:${s}"><b>${esc(c.name)}</b> <small>${esc(c.status)}</small>${acts}<div class="ptext">${esc(c.text ?? '')}</div></li>`;
  }).join('');
  const innate = b.innate ? `<div class="innate"><b>${esc(b.innate.name)}:</b> ${esc(b.innate.text ?? '')}${b.tags?.length ? ` <small>${b.tags.map(esc).join(', ')} tag (printed)</small>` : ''}</div>` : '';
  return `${innate}${rows ? `<ul class="ppowers">${rows}</ul>` : ''}`;
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
      // 261: "+1 credit", not "+1 credits".
      if (a.type === 'pay') return `+${amount(Number(a.amount), String(a.resource))}`;
      if (a.type === 'withdraw') return `take back ${String(a.resource).replace(/_/g, ' ')}`;
      return actionLabel(a, false, true, ctx.cx).replace(/^counter [^:]*: ?/, 'counter: ');
    };
    el.insertAdjacentHTML('beforeend', `<span class="tacts">${acts.map((a) => btn(a, label(a), a.type === 'counter_with' ? 'primary' : '', actionLabel(a, false, true, ctx.cx))).join('')}</span>`);
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
  for (const a of answers.filter((x) => !isHex(x.target))) out.push(btn(a, actionLabel(a, false, true, ctx.cx), 'primary'));
  return `<span class="pchoices">${out.join('')}</span>`;
}

// ---------- the adversary's turn ----------

interface Turn { frame: Frame; steps: { label: string; events: Frame['events'] }[]; at: number; open: boolean }
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
  return groups.filter((g) => g.events.length).map((g) => ({ label: g.label, events: g.events }));
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
  const pieces = [...tok].flatMap(([k, [up, down]]) => [up ? `+${up} ${short(k, up)}` : '', down ? `${down} ${short(k, down)}` : ''].filter(Boolean));
  const parts = [...notes, ...(pieces.length ? [`map: ${pieces.join(', ')}`] : [])];
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

function turnHtml(): string {
  if (!turn) return '';
  // 265 (D115): one heading, "Since your last move": the steps run from the players' own phases (D Actions,
  // E income) through the adversary's, so "The adversary's turn" misnamed most of them.
  if (!turn.open) return `<button type="button" class="pturn-open" data-turn="open">Replay what ran since your last move (${turn.steps.length} step${turn.steps.length === 1 ? '' : 's'})</button>`;
  const t = turn;
  const rows = t.steps.map((s, i) => {
    const line = `${s.label}: ${summary(s.events)}`;
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
  return `<div class="pturn-head"><b>Since your last move</b> <span>step ${t.at + 1} of ${t.steps.length}: ${esc(cur.label)}</span>
      <button type="button" data-turn="-1" ${t.at === 0 ? 'disabled' : ''}>◀</button><button type="button" data-turn="1" ${t.at === t.steps.length - 1 ? 'disabled' : ''}>▶</button>
      <button type="button" data-turn="close">Done</button></div>
    <div class="pturn-ev"><ul>${shownEvents.map((e) => `<li data-hexes="${hexesIn(e).map(hexKey).join(' ')}">${esc(contentIds(String(e.text)))}</li>`).join('')}${others.length ? `<li><small>${others.join('; ')}: the debug view's log lists them</small></li>` : ''}</ul><small>The map shows the board after the turn; outlined: the hexes this step changed.</small></div>
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
  ui.seats.innerHTML = stripHtml(v, ctx.legal, focus);

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
      <div class="p-info">${seatValuesHtml(v, focus)}</div>
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
  const guarded = refused?.value
    ? `<span class="prefused" data-value="board.liberate_refused" data-scope="seat:${focus}" title="${esc(refused.doc ?? '')}">Liberate refused: ${refused.value} corrupted site${refused.value === 1 ? '' : 's'} in reach guarded by a troop</span>`
    : '';
  ui.bar.innerHTML = [
    std.length || guarded ? `<span class="pgroup"><span class="what">Standard actions</span>${std.join('')}${guarded}</span>` : '',
    conv.length ? `<span class="pgroup"><span class="what">Converters</span>${conv.join('')}</span>` : '',
    rest.length ? `<span class="pgroup other"><span class="what">Other</span>${rest.join('')}</span>` : '',
    `<span class="pgroup end">${pass.join('')}${all}${switchTo}</span>`,
  ].join('');

  ui.turn.innerHTML = turnHtml();
  ui.turn.hidden = !turn;
  showTurnStep(ui.turn);
  ui.more.innerHTML = pilesHtml(v) + varsHtml(v, true);
}
