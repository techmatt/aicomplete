// The one card renderer: card content in, HTML out, no I/O. Sizes are mm: print now (a
// PDF at true size), the web client later (scale it with CSS).
// Needs card.css. Geometry comes from layout.json; the icon set is in icons.js. The frame
// holds the fixed structure only: what changes card by card is drawn here. Where a thing
// sits is what it is: see README, Card art.
import { arrow, eye, icon, isIcon, isTag, stacked, tagMedallion } from './icons.js';

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

const place = ([x0, y0, x1, y1]) => `left:${x0}mm;top:${y0}mm;width:${(x1 - x0).toFixed(2)}mm;height:${(y1 - y0).toFixed(2)}mm`;

/**
 * The layout as printed: every zone as the master painted it, carried in by the frame's
 * scale (derive.mjs), so a frame's steel lands inside the trim.
 */
export function inset(l) {
  const [W, H] = l.bleed_mm;
  const m = l.inset_mm;
  const r = (v) => Math.round(v * 100) / 100;
  const box = ([x0, y0, x1, y1]) => [m + (x0 * (W - 2 * m)) / W, m + (y0 * (H - 2 * m)) / H, m + (x1 * (W - 2 * m)) / W, m + (y1 * (H - 2 * m)) / H].map(r);
  const zones = (o) => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, box(v)]));
  return { ...l, common: zones(l.common), kinds: Object.fromEntries(Object.entries(l.kinds).map(([k, v]) => [k, zones(v)])) };
}

/** Tag medallions, right-aligned on the strip: only the card's own, so no socket shows empty. */
function tags(card, [, y0, x1, y1]) {
  const d = y1 - y0;
  const shown = card.tags.slice(0, 3);
  return shown.map((t, i) => `<div class="tag" style="${place([x1 - d - (shown.length - 1 - i) * (d + 1), y0, x1 - (shown.length - 1 - i) * (d + 1), y1])}" data-safe>${tagMedallion(t)}</div>`).join('');
}

/** The insight cost: a hexagon in front of the octagon, 0 omits it. Its rim matches the octagon's; the halo is the strip colour. */
function insight(card, box) {
  if (!card.insight) return '';
  const pts = '26 3 74 3 97 43.300 74 83.600 26 83.600 3 43.300';
  const rim = `rim-t${card.tier >= 2 ? 2 : 1}`; // by tier: a page of cards shares one id space
  return `<div class="insight" style="${place(box)}">
      <svg class="hexagon" viewBox="-4 -4 108 95"><defs><linearGradient id="${rim}" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="var(--rim-a)"/><stop offset=".45" stop-color="var(--rim-b)"/><stop offset=".6" stop-color="var(--rim-c)"/><stop offset="1" stop-color="var(--rim-a)"/></linearGradient></defs>
        <polygon points="${pts}" class="halo"/><polygon points="${pts}" class="edge"/><polygon points="${pts}" class="rim" stroke="url(#${rim})"/><polygon points="${pts}" class="face"/></svg>
      <span class="insight-fig" data-safe><b>${esc(card.insight)}</b>${eye()}</span>
    </div>`;
}

/** Requirements, then replaces: at least is yellow, at most red, replaces the stacked cards. */
function requirement(card, x, box) {
  const count = ([tag, n]) => `${n > 1 ? `<span class="fx-num">${n}</span>` : ''}${tagMedallion(tag)}`;
  const plate = (cls, body) => `<div class="requirement ${cls}">${body}</div>`;
  const plates = [
    card.needs?.length ? plate('at-least', card.needs.map(count).join('')) : '',
    card.at_most?.length ? plate('at-most', card.at_most.map(count).join('')) : '',
    card.replaces ? plate('replaces', stacked() + tagMedallion(card.replaces)) : '',
  ].join('');
  const crowd = (card.needs?.length ?? 0) + (card.replaces ? 1 : 0) >= 3 ? ' crowded' : '';
  return plates ? `<div class="requirements${crowd}" style="left:${x.toFixed(2)}mm;top:${box[1]}mm;height:${(box[3] - box[1]).toFixed(2)}mm">${plates}</div>` : '';
}

const WORD = (w) => () => `<span class="fx-word">${w}</span>`;
const TEXT = { '→': arrow, '/': () => '<span class="fx-per">/</span>', ':': () => '<span class="fx-num">:</span>', OR: WORD('OR'), ALL: WORD('ALL'), ACT: WORD('ACT'), ELSE: WORD('ELSE'), '*': () => '<span class="fx-num">*</span>' };

/** One part of an icon line: a box of parts, an icon, a connector or a number. */
function part(p) {
  if (p.income) return `<span class="income">${p.income.map(part).join('')}</span>`;
  if (p.adversary) return `<span class="adversary">${p.adversary.map(part).join('')}</span>`;
  if (p.other) return `<span class="other">${p.other.map(part).join('')}</span>`;
  if (TEXT[p]) return TEXT[p]();
  if (isTag(p)) return `<span class="fx-tag">${tagMedallion(p.slice(1))}</span>`;
  if (isIcon(p)) return icon(p);
  if (/^[−+]?[\dn]+$/.test(p)) return `<span class="fx-num">${esc(p)}</span>`;
  throw new Error(`no icon or number ${p}`);
}
const iconLine = (parts) => `<div class="fx-line" data-safe>${parts.map(part).join('')}</div>`;

/** The rules-text size that fits, largest first: pt, class. */
const SIZES = [[7.5, 'fit-0'], [6.9, 'fit-1'], [6.3, 'fit-2'], [5.8, 'fit-3']];
const textClass = (text, [x0, y0, x1, y1], rows, rowMm) => {
  const room = y1 - y0 - 1.6 - rows * rowMm;
  for (const [pt, cls] of SIZES) {
    const perLine = ((x1 - x0 - 3) / (pt * 0.3528)) * 1.95;
    if (Math.ceil(text.length / perLine) * pt * 0.3528 * 1.22 <= room) return cls;
  }
  return `${SIZES.at(-1)[1]} dense`;
};

/** Icon lines over rules text: the panel below the art, and a building's top box. */
const effectBox = (cls, box, lines = [], text = '') => {
  const boxed = lines.some((l) => l.some((p) => typeof p === 'object'));
  const rowMm = (cls === 'lasting' ? 5 : 6.2) + (boxed ? 1.6 : 0);
  const width = (l) => l.reduce((n, p) => n + (typeof p === 'object' ? Object.values(p)[0].length * (rowMm + 0.1) + 3 : rowMm + 0.1), 0);
  const rows = Math.ceil(lines.reduce((n, l) => n + width(l) + 3.5, 0) / (box[2] - box[0] - 3));
  const fit = text ? textClass(text, box, rows, rowMm) : '';
  return `<div class="effect ${cls}${fit.endsWith('dense') ? ' dense' : ''}" style="${place(box)}">
      ${lines.length ? `<div class="fx-icons">${lines.map(iconLine).join('')}</div>` : ''}
      ${text ? `<p class="rules ${fit}" data-safe>${esc(text)}</p>` : ''}
    </div>`;
};

/**
 * @param card     content: name, kind, cost, insight, needs, at_most, replaces, tags, and
 *                 the generated face (faces.json): top and top_text above the art
 *                 (buildings), effect and text below it
 * @param layout   web/card/layout.json, through inset()
 * @param frameUrl the frame for this kind and tier
 * @param fit      optional measured zones of this frame (art/frames/r3/fit.json), in mm:
 *                 cost, banner, top, art, effect, the guide's where absent; plate, a colour
 * @param artUrl   optional scene for the art window, centre-cropped to it
 * @param fonts    optional font choice, a `.fonts-<name>` block in web/card/fonts.css (which
 *                 the page must load); left out, card.css's fonts. Sizes don't change with it
 */
export function renderCard(card, layout, frameUrl, fit = {}, artUrl, fonts) {
  const z = { ...layout.common, ...layout.kinds[card.kind], ...fit };
  const reqX = card.insight ? z.insight[2] + 0.5 : z.cost[2] + 2.4;
  return `<article class="card kind-${esc(card.kind)} tier-${card.tier >= 2 ? 2 : 1}${fonts ? ` fonts-${esc(fonts)}` : ''}" style="--strip:${layout.kind_colour[card.kind]}${fit.plate ? `;--plate:${esc(fit.plate)}` : ''}">
  <img class="frame" src="${esc(frameUrl)}" alt="">
  ${artUrl ? `<img class="art" src="${esc(artUrl)}" alt="" style="${place(z.art)}">` : ''}
  <div class="content">
    <div class="cost${card.insight ? ' beside' : ''}" style="${place(z.cost)}"><span data-safe>${esc(card.cost)}</span></div>
    ${insight(card, z.insight)}
    ${requirement(card, reqX, z.requirement)}
    ${tags(card, z.tags)}
    <div class="name${card.name.length > 26 ? ' long' : ''}" style="${place(z.banner)}"><span data-safe>${esc(card.name)}</span></div>
    ${z.top ? effectBox('lasting', z.top, card.top, card.top_text) : ''}
    ${effectBox('once', z.effect, card.effect, card.text)}
  </div>
</article>`;
}
