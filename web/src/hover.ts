// The hover tooltip's text: `details` picks the formatter for whatever the mouse is on.
// Board-track and board-name hovers live with their drawing in panels.ts.

import type { Action, CardView, View } from './engine';
import { actionLabel, contentIds, countryName, escapeHtml, hexKey, isHex, kindName, strong, tokenColor, tokenName, type LabelContext } from './format';
import { boardDetails, trackStepDetails } from './panels';

function tokenDetails(token: string, hex: string, shown: View): string {
  const h = shown.hexes.find((x) => hexKey(x.hex) === hex);
  const tok = h?.tokens.find((x) => x.kind === token);
  return `<b>${escapeHtml(tokenName(token))}</b> × ${tok?.count ?? 0}<br><code>${escapeHtml(token)}</code> on ${escapeHtml(countryName(shown, h?.country ?? -1))} #${h?.number} (${escapeHtml(kindName(h?.kind ?? ''))}, hex ${hex})`;
}

function hexDetails(hex: string, index: number, shown: View, legal: Action[], cx: LabelContext): string {
  const h = shown.hexes[index];
  // A seat's piece in its seat's color, as the board draws it (020: gray).
  const dot = (k: string) => {
    const seat = /:(\d+)$/.exec(k)?.[1];
    const c = seat !== undefined ? shown.countries[shown.seats[Number(seat)]?.country ?? -1]?.color : undefined;
    return c ? strong(c) : tokenColor(k);
  };
  const toks = h.tokens.map((x) => `<span class="dot" style="background:${dot(x.kind)}"></span>${escapeHtml(tokenName(x.kind))} × ${x.count}`).join('<br>') || 'no tokens';
  const here = legal.filter((a) => isHex(a.target) && hexKey(a.target) === hex).map((a) => escapeHtml(actionLabel(a, true, true, cx)));
  const facing = h.facing !== undefined ? ` · <b>front</b> facing ${escapeHtml(countryName(shown, h.facing))}` : '';
  const arrows = shown.arrows.filter((a) => a.from === h.index || a.to === h.index).map((a) => escapeHtml(contentIds(a.doc)));
  return `<b>${escapeHtml(countryName(shown, h.country))} #${h.number}</b> · ${escapeHtml(kindName(h.kind))}${facing}<br>hex ${hex} · index ${h.index} · piece position ${h.piece_pos.join(',')} · ${h.neighbors.length} neighbors<hr>${toks}${arrows.length ? `<hr>${arrows.join('<br>')}` : ''}${here.length ? `<hr>legal here:<br>${here.join('<br>')}` : ''}`;
}

function pileDetails(key: string, shown: View): string | null {
  const p = shown.piles.find((x) => x.key === key);
  if (!p) return null;
  return `<b>${escapeHtml(p.label)}</b>: ${p.count} card${p.count === 1 ? '' : 's'}<br><code>${escapeHtml(p.key)}</code> · from ${escapeHtml(p.source)}${p.visible.length ? `<hr>${p.visible.map((c) => `${escapeHtml(c.name)}${c.text ? ` <small>${escapeHtml(c.text)}</small>` : ''}`).join('<br>')}` : ''}`;
}

function valueDetails(key: string, where: string | undefined, shown: View): string | null {
  const [scope, idx] = (where ?? 'global').split(':');
  const list = scope === 'seat' ? shown.seats[Number(idx)].values : scope === 'country' ? shown.countries[Number(idx)].values : shown.globals;
  const v = list.find((x) => x.key === key);
  // A resource another seat has but this one never held (drawn as 0 since 020).
  if (!v && key.startsWith('resource.')) return `<b>${escapeHtml(key.slice('resource.'.length).replace(/_/g, ' '))}</b>: 0<br><code>${escapeHtml(key)}</code> · ${scope}${idx ? ` ${idx}` : ''} (none yet)`;
  if (!v) return null;
  return `<b>${escapeHtml(v.label)}</b>: ${v.value}${v.max !== undefined ? ` / ${v.max}` : ''}<br><code>${escapeHtml(v.key)}</code> · ${scope}${idx ? ` ${idx}` : ''} · from ${escapeHtml(v.source)}${v.doc ? `<br>${escapeHtml(v.doc)}` : ''}`;
}

function cardDetails(id: string, where: string | undefined, shown: View): string | null {
  // The same card id can sit with several seats, each with its own status: look
  // in the place the chip was drawn from.
  const [kind, a, ...rest] = (where ?? '').split(':');
  const key = rest.join(':');
  const pool: CardView[] =
    kind === 'seat' ? shown.seats[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
    : kind === 'country' ? shown.countries[Number(a)]?.cards.find((g) => g.key === key)?.cards ?? []
    : kind === 'pile' ? shown.piles.find((p) => p.key === [a, ...rest].join(':'))?.visible ?? []
    : [];
  const c = pool.find((x) => x.id === id);
  if (!c) return null;
  return `<b>${escapeHtml(c.name)}</b> <code>${escapeHtml(c.id)}</code>${c.cost !== undefined ? ` · cost ${c.cost}` : ''}<br>${escapeHtml(c.status)}${c.text ? `<br>${escapeHtml(c.text)}` : ''}<br><small>${escapeHtml(where ?? '')}</small>`;
}

/** The tooltip for element `t`, or null if it has none. */
export function details(t: HTMLElement | SVGElement, shown: View, legal: Action[], cx: LabelContext): string | null {
  const d = t.dataset;
  if (d.token !== undefined && d.hex) return tokenDetails(d.token, d.hex, shown);
  if (d.hex && d.index !== undefined) return hexDetails(d.hex, Number(d.index), shown, legal, cx);
  if (d.pile) return pileDetails(d.pile, shown);
  if (d.value) return valueDetails(d.value, d.scope, shown);
  if (d.card) return cardDetails(d.card, d.where, shown);
  if (d.track) return trackStepDetails(shown, d.track);
  if (d.boardSeat !== undefined) return boardDetails(shown, Number(d.boardSeat));
  if (d.action) return `<code>${escapeHtml(d.action)}</code>`;
  return null;
}
