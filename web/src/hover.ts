// The hover tooltip's text: `details` picks the formatter for whatever the mouse is on.
// Board-track and board-name hovers live with their drawing in panels.ts.

import type { Action, CardView, View } from './engine';
import { actionLabel, contentIds, countryName, escapeHtml, hexKey, isHex, kindName, seatText, strong, tokenColor, tokenName, type LabelContext } from './format';
import { boardDetails, trackStepDetails } from './panels';

function tokenDetails(token: string, hex: string, shown: View, plain = false): string {
  // 321: the player view says where, never the token's key or the hex's coordinates.
  const key = plain ? '' : `<code>${escapeHtml(token)}</code> `;
  const at = plain ? '' : `, hex ${hex}`;
  const h = shown.hexes.find((x) => hexKey(x.hex) === hex);
  const tok = h?.tokens.find((x) => x.kind === token);
  // 295: a seat's ambassadors, standing or tipped, are one stack with one hover: how many, and how many are used.
  const [base, seat] = token.split(':');
  if ((base === 'ambassador' || base === 'tipped') && seat !== 'adversary') {
    const n = h?.tokens.find((x) => x.kind === `ambassador:${seat}`)?.count ?? 0;
    const m = Math.min(n, h?.tokens.find((x) => x.kind === `tipped:${seat}`)?.count ?? 0);
    return `<b>${n} ambassador${n === 1 ? '' : 's'} (${escapeHtml(tokenName(`ambassador:${seat}`).replace(/^ambassador \((.*)\)$/, '$1'))})</b>${m ? ` (${m} tipped: moved or acted this round)` : ' (ready: each may move or act this round)'}<br>${key}on ${escapeHtml(countryName(shown, h?.country ?? -1))} #${h?.number} (${escapeHtml(kindName(h?.kind ?? ''))}${at})`;
  }
  return `<b>${escapeHtml(tokenName(token))}</b> × ${tok?.count ?? 0}<br>${key}on ${escapeHtml(countryName(shown, h?.country ?? -1))} #${h?.number} (${escapeHtml(kindName(h?.kind ?? ''))}${at})`;
}

function hexDetails(hex: string, index: number, shown: View, legal: Action[], cx: LabelContext, plain = false): string {
  const h = shown.hexes[index];
  // A seat's piece in its seat's color, as the board draws it (020: gray).
  const dot = (k: string) => {
    const seat = /:(\d+)$/.exec(k)?.[1];
    const c = seat !== undefined ? shown.countries[shown.seats[Number(seat)]?.country ?? -1]?.color : undefined;
    return c ? strong(c) : tokenColor(k);
  };
  // 295: tipped ambassadors are some of the seat's ambassadors there, said on their line.
  const tipped = (k: string) => {
    const [base, seat] = k.split(':');
    const m = base === 'ambassador' ? h.tokens.find((x) => x.kind === `tipped:${seat}`)?.count ?? 0 : 0;
    return m ? ` (${m} tipped: moved or acted this round)` : '';
  };
  const toks = h.tokens.filter((x) => !x.kind.startsWith('tipped:')).map((x) => `<span class="dot" style="background:${dot(x.kind)}"></span>${escapeHtml(tokenName(x.kind))} × ${x.count}${tipped(x.kind)}`).join('<br>') || 'no tokens';
  const here = legal.filter((a) => isHex(a.target) && hexKey(a.target) === hex).map((a) => escapeHtml(actionLabel(a, true, true, cx)));
  const arrows = shown.arrows.filter((a) => a.from === h.index || a.to === h.index).map((a) => escapeHtml(contentIds(a.doc)));
  return `<b>${escapeHtml(countryName(shown, h.country))} #${h.number}</b> · ${escapeHtml(kindName(h.kind))}${plain ? '' : `<br>hex ${hex} · index ${h.index} · piece position ${h.piece_pos.join(',')} · ${h.neighbors.length} neighbors`}<hr>${toks}${arrows.length ? `<hr>${arrows.join('<br>')}` : ''}${here.length ? `<hr>legal here:<br>${here.join('<br>')}` : ''}`;
}

function pileDetails(key: string, shown: View, plain = false): string | null {
  const p = shown.piles.find((x) => x.key === key);
  if (!p) return null;
  return `<b>${escapeHtml(p.label)}</b>: ${p.count} card${p.count === 1 ? '' : 's'}${plain ? '' : `<br><code>${escapeHtml(p.key)}</code> · from ${escapeHtml(p.source)}`}${p.visible.length ? `<hr>${p.visible.map((c) => `${escapeHtml(c.name)}${c.text ? ` <small>${escapeHtml(c.text)}</small>` : ''}`).join('<br>')}` : ''}`;
}

function valueDetails(key: string, where: string | undefined, shown: View, plain = false): string | null {
  const [scope, idx] = (where ?? 'global').split(':');
  const list = scope === 'seat' ? shown.seats[Number(idx)].values : scope === 'country' ? shown.countries[Number(idx)].values : shown.globals;
  const v = list.find((x) => x.key === key);
  // A resource another seat has but this one never held (drawn as 0 since 020).
  if (!v && key.startsWith('resource.')) return `<b>${escapeHtml(key.slice('resource.'.length).replace(/_/g, ' '))}</b>: 0${plain ? ' (none yet)' : `<br><code>${escapeHtml(key)}</code> · ${scope}${idx ? ` ${idx}` : ''} (none yet)`}`;
  if (!v) return null;
  return `<b>${escapeHtml(v.label)}</b>: ${v.value}${v.max !== undefined ? ` / ${v.max}` : ''}${plain ? '' : `<br><code>${escapeHtml(v.key)}</code> · ${scope}${idx ? ` ${idx}` : ''} · from ${escapeHtml(v.source)}`}${v.doc ? `<br>${escapeHtml(v.doc)}` : ''}`;
}

function cardDetails(id: string, where: string | undefined, shown: View, plain = false): string | null {
  // The same card id can sit with several seats, each with its own status: look
  // in the place the chip was drawn from.
  const [kind, a, ...rest] = (where ?? '').split(':');
  const key = rest.join(':');
  const pool: CardView[] =
    kind === 'seat' ? shown.seats[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
    : kind === 'country' ? shown.countries[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
    : kind === 'pile' ? shown.piles.find((p) => p.key === [a, ...rest].join(':'))?.visible ?? []
    // 324: a board's power (the player view's panel lists them by `board:<seat>`).
    : kind === 'board' ? shown.seats[Number(a)]?.board?.cards ?? []
    : [];
  const c = pool.find((x) => x.id === id);
  if (!c) return null;
  // 326: the player view's hover names seats as it does ("Paid: you 2 credits", not "seat 0").
  const words = (s: string) => (plain ? seatText(s) : s);
  return `<b>${escapeHtml(c.name)}</b>${plain ? '' : ` <code>${escapeHtml(c.id)}</code>`}${c.cost !== undefined ? ` · cost ${c.cost}` : ''}<br>${escapeHtml(words(c.status))}${c.text ? `<br>${escapeHtml(words(c.text))}` : ''}${plain ? '' : `<br><small>${escapeHtml(where ?? '')}</small>`}`;
}

/** The tooltip for element `t`, or null if it has none. */
/** `plain` (321): the player view's hover, without the debug view's keys, indexes and sources (W21's developer aid). */
export function details(t: HTMLElement | SVGElement, shown: View, legal: Action[], cx: LabelContext, map: View = shown, plain = false): string | null {
  const d = t.dataset;
  // 305: a token or hex reads the view the map was drawn from (the turn panel's earlier step, `map`).
  if (d.token !== undefined && d.hex) return tokenDetails(d.token, d.hex, map, plain);
  if (d.hex && d.index !== undefined) return hexDetails(d.hex, Number(d.index), map, legal, cx, plain);
  // 320: so does everything else drawn from it (rows, tracks, piles, cards); the shown seat's hand, powers, buildings
  // and the bar are the board now.
  // 324: the panel of what the seat can do and its printed cards are the board now; the seat rows draw from `map`.
  const v = map !== shown && !t.closest('#p-act, #p-seat .pcards, #p-draft') ? map : shown;
  if (d.pile) return pileDetails(d.pile, v, plain);
  if (d.value) return valueDetails(d.value, d.scope, v, plain);
  if (d.card) return cardDetails(d.card, d.where, v, plain);
  if (d.track) return trackStepDetails(v, d.track, plain);
  if (d.boardSeat !== undefined) return boardDetails(shown, Number(d.boardSeat), plain);
  // 321: the player view's controls (`pact`, the hex pick) hover their own words (the button's title), never the
  // action's raw JSON, the debug view's developer aid (W9): it showed over every Play, Use and pay button.
  if (d.action) return plain ? null : `<code>${escapeHtml(d.action)}</code>`;
  return null;
}
