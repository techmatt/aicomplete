// 242: the player view. The same engine view and legal actions as the debug view, laid out for play: the map, the
// adversary's pressure, a summary row per seat, and the focused seat's board, hand and controls. Each legal
// action of the focused seat gets its control on the object it acts on (a card in hand, a power, a threat, a
// hex); any action no component claims, and any view part no component draws, falls back to the generic
// rendering (a labeled button, a chip), so a new module shows with no new UI work. No rules live here.

import { param, type Action, type CardView, type Config, type View } from '../engine';
import { actionLabel, contentIds, countryName, escapeHtml as esc, eventText, hexKey, hexesIn, isHex, strong, tokenName, type LabelContext } from '../format';
import { phaseGroups } from '../log';
import { cardChip, pilesHtml, seatValuesHtml, varsHtml } from '../panels';
import type { Frame } from '../session';
import { printable, printedCard } from './printed';
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
      const inc = val(`economy.income.${r}`)?.value;
      return `<span class="sres" data-value="resource.${r}" data-scope="seat:${s.index}">${esc(r.replace(/_/g, ' '))} <b>${val(`resource.${r}`)?.value ?? 0}</b>${inc ? `<small>${r === 'insight' ? '→' : '+'}${inc}</small>` : ''}</span>`;
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
  const status = c.status && !['ready', 'draft', 'built', 'in play'].includes(c.status) ? c.status : '';
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
    return tile(s, c, 'cards.hand', acts);
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
      const k = keepCost(Number(a.tier));
      return btn(a, `Tier ${a.tier} deck${typeof k === 'number' ? ` · keep ${k} insight` : ''}`, 'primary big');
    }).join('')}</div>`);
  }
  const row = (cards: CardView[], key: string, act: (c: CardView) => string, note: (c: CardView) => string) =>
    `<div class="pdraft">${cards.map((c) => `<div class="pdcard" data-card="${esc(c.id)}" data-where="seat:${s}:${key}">
      <div class="pslot" data-print="${esc(c.id)}"></div>
      ${printable(c.id) ? '' : tile(s, c, key, '')}
      <div class="pd-note">${esc(note(c))}</div><div class="pd-acts">${act(c)}</div></div>`).join('')}</div>`;
  if (packet?.cards.length) {
    parts.push(`<h4>${esc(packet.label)} <small>take one; the rest pass on</small></h4>` + row(packet.cards, 'cards.packet',
      (c) => drafts.filter((a) => a.card === c.id).map((a) => btn(a, 'Draft', 'primary')).join(''),
      (c) => {
        const t = /tier (\d+)/.exec(c.status)?.[1];
        const k = t ? keepCost(Number(t)) : undefined;
        return `${c.cost ?? '?'} credits to play${typeof k === 'number' ? ` · keep ${k} insight` : ''}`;
      }));
  }
  if (drafted?.cards.length) {
    parts.push(`<h4>${esc(drafted.label)}</h4>` + row(drafted.cards, 'cards.drafted',
      (c) => keeps.filter((a) => a.card === c.id).map((a) => btn(a, `Keep${/keep (\d+) insight/.test(c.status) ? ` (${/keep (\d+) insight/.exec(c.status)![1]} insight)` : ''}`, 'primary')).join(''),
      (c) => `${c.cost ?? '?'} credits to play · ${c.status}`));
  }
  return `<div class="psec draft">${parts.join('')}</div>`;
}

function powersHtml(v: View, ctx: PlayerCtx, s: number, mine: Action[]): string {
  const b = v.seats[s].board;
  const powers = (b?.cards ?? []).filter((c) => c.id.includes('.power.'));
  if (!b) return '';
  const rows = powers.map((c) => {
    const k = Number(c.id.split('.power.')[1]);
    const acts = mine.filter((a) => a.type === 'power' && Number(a.power) === k).map((a) => btn(a, 'Use', 'primary', actionLabel(a, false, true, ctx.cx))).join('');
    return `<li class="${c.status.startsWith('ready') ? 'ready' : ''}" data-card="${esc(c.id)}" data-where="board:${s}"><b>${esc(c.name)}</b> <small>${esc(c.status)}</small>${acts}<div class="ptext">${esc(c.text ?? '')}</div></li>`;
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
      if (a.type === 'pay') return `+${a.amount} ${String(a.resource).replace(/_/g, ' ')}`;
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
/** The adversary's phases by their rulebook step (`shortPhase`): B its card and lines, F threats, G war. */
const ADV_PHASE = /^[BFG] /;

/** The steps of a frame that ran through several phases (the adversary's turn after the last pass), else none. */
function turnOf(f: Frame, before: string): Turn['steps'] {
  if (!f.action) return [];
  const groups = phaseGroups(f.events, before).filter((g) => !g.round);
  if (groups.length < 2) return [];
  // A phase that changed nothing is no step.
  return groups.filter((g) => g.events.length).map((g) => ({ label: g.label, events: g.events }));
}

/** The hexes the shown step of the adversary's turn changed, or null when no turn is open for `f`. */
export function turnHexes(f: Frame, before: string): Set<string> | null {
  if (turn?.frame !== f) {
    const steps = turnOf(f, before);
    // It opens by itself only when the adversary's phases ran (its turn); phases between two of the seats'
    // decisions wait folded under their Replay button.
    turn = steps.length ? { frame: f, steps, at: 0, open: steps.some((s) => ADV_PHASE.test(s.label)) } : null;
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
  const tok = new Map<string, number>();
  for (const e of events) if (e.type === 'tokens_changed') tok.set(String(e.token), (tok.get(String(e.token)) ?? 0) + Number(e.delta));
  const pieces = [...tok].filter(([, d]) => d).map(([k, d]) => `${d > 0 ? '+' : ''}${d} ${tokenName(k)}`);
  const parts = [...notes, ...(pieces.length ? [`map: ${pieces.join(', ')}`] : [])];
  if (parts.length) return parts.join('; ');
  const res = events.filter((e) => e.type === 'resource_changed').map((e) => `seat ${e.seat} ${Number(e.delta) > 0 ? '+' : ''}${e.delta} ${String(e.resource).replace(/_/g, ' ')}`);
  const cards = events.filter((e) => e.type === 'card_moved').map((e) => contentIds(String(e.card)));
  const other = [...res, ...(cards.length ? [`cards moved: ${cards.join(', ')}`] : [])];
  return other.length ? other.join('; ') : 'bookkeeping only (tracks)';
}

function turnHtml(): string {
  if (!turn) return '';
  // Whether the adversary's phases ran (its turn), or only phases between two of the seats' decisions.
  const adv = turn.steps.some((s) => ADV_PHASE.test(s.label));
  const what = adv ? "the adversary's turn" : 'what ran since the last move';
  if (!turn.open) return `<button type="button" class="pturn-open" data-turn="open">Replay ${what} (${turn.steps.length} step${turn.steps.length === 1 ? '' : 's'})</button>`;
  const t = turn;
  const rows = t.steps.map((s, i) => {
    const line = `${s.label}: ${summary(s.events)}`;
    return `<li class="${i === t.at ? 'now' : ''}" data-turn-step="${i}" title="${esc(line)}">${esc(line)}</li>`;
  }).join('');
  const cur = t.steps[t.at];
  // Track changes are bookkeeping under engine keys: counted here, listed in the debug view's log.
  const shownEvents = cur.events.filter((e) => e.type !== 'var_changed');
  const tracks = cur.events.length - shownEvents.length;
  return `<div class="pturn-head"><b>${adv ? "The adversary's turn" : 'Since the last move'}</b> <span>step ${t.at + 1} of ${t.steps.length}</span>
      <button type="button" data-turn="-1" ${t.at === 0 ? 'disabled' : ''}>◀</button><button type="button" data-turn="1" ${t.at === t.steps.length - 1 ? 'disabled' : ''}>▶</button>
      <button type="button" data-turn="close">Done</button></div>
    <ol>${rows}</ol>
    <div class="pturn-ev"><small>The map shows the board after the turn; outlined: the hexes this step changed.</small><ul>${shownEvents.map((e) => `<li data-hexes="${hexesIn(e).map(hexKey).join(' ')}">${esc(eventText(e))}</li>`).join('')}${tracks ? `<li><small>${tracks} track change${tracks === 1 ? '' : 's'}: the debug view's log has them</small></li>` : ''}</ul></div>`;
}

// ---------- render ----------

/** Draws the player view's own parts; the map, phase bar, status, adversary row and countries are main.ts's. */
export function renderPlayer(ui: PlayerUi, ctx: PlayerCtx) {
  claimed = new Set();
  const v = ctx.view;
  const focus = focusSeat(v, ctx.legal);
  const mine = ctx.legal.filter((a) => a.seat === focus);
  ui.seats.innerHTML = stripHtml(v, ctx.legal, focus);

  ui.prompt.insertAdjacentHTML('beforeend', promptControls(ctx, mine));
  threatControls(ui.countries, ctx, mine);

  const s = v.seats[focus];
  const c = v.countries[s.country];
  const st = seatStatus(v, ctx.legal, focus);
  const draft = draftHtml(v, ctx, focus, mine);
  ui.seat.style.setProperty('--tint', strong(c?.color ?? '#cccccc'));
  ui.seat.innerHTML = `
    <div class="p-head"><span class="swatch"></span><b>${esc(seatName(v, focus))}</b> <small>${esc(countryName(v, s.country))}</small>${st.text ? `<span class="pst ${st.cls}">${st.text}</span>` : ''}</div>
    <div class="p-body">
      <div class="p-info">${seatValuesHtml(v, focus)}${powersHtml(v, ctx, focus, mine)}</div>
      <div class="p-hand">${draft}${handHtml(v, ctx, focus, mine)}</div>
      <div class="p-built">${builtHtml(v, ctx, focus, mine)}</div>
    </div>`;
  for (const slot of ui.seat.querySelectorAll<HTMLElement>('.pslot[data-print]')) {
    const host = printedCard(slot.dataset.print!, 0.8);
    if (host) slot.replaceWith(host);
    else slot.remove();
  }

  // The bar: standard actions, converters, pass, and every action of the seat no component drew.
  const std = mine.filter((a) => a.type === 'standard').map((a) => btn(a, actionLabel(a, false, true, ctx.cx)));
  const conv = mine.filter((a) => a.type === 'convert').map((a) => btn(a, actionLabel(a, false, true, ctx.cx)));
  const pass = mine.filter((a) => a.type === 'pass').map((a) => btn(a, v.phase?.name === 'cards.keep' ? 'Done keeping (pass)' : 'Done (pass)', 'pass'));
  const rest = mine.filter((a) => !claimed.has(JSON.stringify(a))).map((a) => btn(a, actionLabel(a, false, true, ctx.cx), 'other'));
  const passers = (v.phase?.waiting ?? []).filter((x) => ctx.legal.some((a) => a.seat === x && a.type === 'pass'));
  const all = !v.outcome && !v.choice && passers.length > 1 ? `<button type="button" class="pass-all" data-pass-all="1">Pass every seat still to act (${passers.join(', ')})</button>` : '';
  const others = ctx.legal.filter((a) => a.seat !== focus).map((a) => a.seat);
  const switchTo = [...new Set(others)].map((x) => `<button type="button" data-seat-focus="${x}">${esc(seatName(v, x))} to act ▸</button>`).join('');
  ui.bar.innerHTML = [
    std.length ? `<span class="pgroup"><span class="what">Standard actions</span>${std.join('')}</span>` : '',
    conv.length ? `<span class="pgroup"><span class="what">Converters</span>${conv.join('')}</span>` : '',
    rest.length ? `<span class="pgroup other"><span class="what">Other</span>${rest.join('')}</span>` : '',
    `<span class="pgroup end">${pass.join('')}${all}${switchTo}</span>`,
  ].join('');

  ui.turn.innerHTML = turnHtml();
  ui.turn.hidden = !turn;
  ui.more.innerHTML = pilesHtml(v) + varsHtml(v, true);
}
