// The one card renderer: card content in, HTML out, no I/O. Sizes are mm: print now (a
// PDF at true size), the web client later (scale it with CSS).
// Needs card.css. Geometry comes from layout.json; the icon set is in icons.js. The frame
// holds the fixed structure only: what changes card by card is drawn here. Where a thing
// sits is what it is: see tools/card-art/README.md.
import { arrow, atRing, eye, icon, isIcon, isTag, less, stacked, tagMedallion } from './icons.js';

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

/** Tier 2 and up print gilded; tier 0 (unique) and 1 share a frame. */
export const tierOf = (tier) => (tier >= 2 ? 2 : 1);
/** The frame variant a card prints on. */
export const frameOf = (card) => `${card.kind}-t${tierOf(card.tier)}`;

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

/** The insight cost: a hexagon in front of the octagon, 0 omits it. Its rim matches the octagon's; the halo is the strip color. */
function insight(card, box) {
  if (!card.insight) return '';
  const pts = '26 3 74 3 97 43.300 74 83.600 26 83.600 3 43.300';
  const rim = `rim-t${tierOf(card.tier)}`; // by tier: a page of cards shares one id space
  return `<div class="insight" style="${place(box)}">
      <svg class="hexagon" viewBox="-4 -4 108 95"><defs><linearGradient id="${rim}" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="var(--rim-a)"/><stop offset=".45" stop-color="var(--rim-b)"/><stop offset=".6" stop-color="var(--rim-c)"/><stop offset="1" stop-color="var(--rim-a)"/></linearGradient></defs>
        <polygon points="${pts}" class="halo"/><polygon points="${pts}" class="edge"/><polygon points="${pts}" class="rim" stroke="url(#${rim})"/><polygon points="${pts}" class="face"/></svg>
      <span class="insight-fig" data-safe><b>${esc(card.insight)}</b>${eye()}</span>
    </div>`;
}

/** Requirements, then replaces: at least is yellow, replaces the stacked cards (182: no card requires at most). */
function requirement(card, x, box) {
  const count = ([tag, n]) => `${n > 1 ? `<span class="fx-num">${n}</span>` : ''}${tagMedallion(tag)}`;
  const plate = (cls, body) => `<div class="requirement ${cls}">${body}</div>`;
  const plates = [
    card.needs?.length ? plate('at-least', card.needs.map(count).join('')) : '',
    card.replaces ? plate('replaces', stacked() + tagMedallion(card.replaces)) : '',
  ].join('');
  const crowd = (card.needs?.length ?? 0) + (card.replaces ? 1 : 0) >= 3 ? ' crowded' : '';
  return plates ? `<div class="requirements${crowd}" style="left:${x.toFixed(2)}mm;top:${box[1]}mm;height:${(box[3] - box[1]).toFixed(2)}mm">${plates}</div>` : '';
}

const WORD = (w) => () => `<span class="fx-word">${w}</span>`;
const TEXT = { '→': arrow, '/': () => '<span class="fx-per">/</span>', ':': () => '<span class="fx-num">:</span>', OR: WORD('OR'), ALL: WORD('ALL'), ACT: WORD('ACT') };

/** One part of an icon line: a box of parts, an icon, a connector or a number. */
function part(p) {
  if (p.income) return `<span class="income">${p.income.map(part).join('')}</span>`;
  if (p.adversary) return `<span class="adversary">${p.adversary.map(part).join('')}</span>`;
  if (p.other) return `<span class="other">${p.other.map(part).join('')}</span>`;
  if (p.yours) return `<span class="yours">${p.yours.map(part).join('')}</span>`;
  if (TEXT[p]) return TEXT[p]();
  if (isTag(p)) return `<span class="fx-tag">${tagMedallion(p.slice(1))}</span>`;
  if (isIcon(p)) return icon(p);
  // 179: `less:N` (a discount or a jam) and `at:N` (At N), each its own mark.
  const m = /^(less|at):(\d+)$/.exec(p);
  if (m) return m[1] === 'less' ? less(m[2]) : atRing(m[2]);
  if (/^[−+]?[≤≥]?[\dn]+$/.test(p)) return `<span class="fx-num">${esc(p)}</span>`;
  throw new Error(`no icon or number ${p}`);
}
const iconLine = (parts) => `<div class="fx-line" data-safe>${parts.map(part).join('')}</div>`;

/**
 * Rules text, with any `{piece}` marker drawn as that icon inline in the sentence (amendment 8045:
 * every piece's symbol is usable in a card's sentences, as Spirit Island prints presence). The
 * generated faces print words today; a marker naming no icon prints as written.
 */
const rulesText = (s) => esc(s).replace(/\{(\w+)\}/g, (m, n) => (isIcon(n) ? `<span class="fx-inline">${icon(n)}</span>` : m));

/** Icon lines over rules text: the panel below the art, and a building's top box. */
const effectBox = (cls, box, lines = [], text = '') => `<div class="effect ${cls}" style="${place(box)}">
      ${lines.length ? `<div class="fx-icons">${lines.map(iconLine).join('')}</div>` : ''}
      ${text ? `<p class="rules" data-safe>${rulesText(text)}</p>` : ''}
    </div>`;

/**
 * Sizes what renderCard left at its defaults, by measuring: run it on anything holding cards,
 * after they are in the page. A card it never reached prints at the defaults, 7.5 pt text and
 * a 15 pt title. In the page it is self-contained, so `page.evaluate(fitCards)` and the
 * function's source both work; running it again changes nothing.
 *
 * Each box steps down the ladder alone, largest size first, until its icon rows and text
 * leave the clearance; a title shrinks until it is one line inside its plate. Measuring is
 * done on a copy at true size outside `root`, so a card comes out the same at any zoom and
 * on any page.
 *
 * @returns one row per card, in page order: `top` and `effect`, the ladder's pt for that box
 *          (null: nothing in it); `title`, pt; `overflow`, what breaks its box or plate even
 *          so; `tight`, the boxes inside it but short of the clearance.
 */
export async function fitCards(root = document) {
  const MM = 96 / 25.4;
  const CLEARANCE_MM = 1.6;
  const TITLE_MARGIN_MM = 1;
  const TITLE_PT = 15;
  const TITLE_FLOOR_PT = 10.5;
  const TITLE_STEP_PT = 0.25;
  const LADDER = [[7.5, ''], [6.9, 'fit-1'], [6.3, 'fit-2'], [5.8, 'fit-3'], [5.4, 'dense']];
  const FACES = ['600 15pt "Saira Condensed"', '700 15pt "Saira Condensed"', '400 7.5pt "Source Serif 4"'];
  const EPSILON_PX = 0.01;
  const OVERFLOW_PX = 0.5; // As render.mjs tolerates: under a pixel is rounding, not a break.

  // A font loads when something first needs it, so `document.fonts.ready` alone can pass
  // before it has started.
  if (!FACES.every((f) => document.fonts.check(f))) {
    await Promise.all(FACES.map((f) => document.fonts.load(f).catch(() => {})));
    await document.fonts.ready;
  }

  /** Does what the box holds, gaps included, leave `clear` px of its height, and stay inside its width? */
  const fits = (box, clear, slack) => {
    const b = box.getBoundingClientRect();
    const parts = [...box.querySelectorAll('.fx-line, .rules')].map((e) => e.getBoundingClientRect());
    if (!parts.length) return true;
    const high = Math.max(...parts.map((r) => r.bottom)) - Math.min(...parts.map((r) => r.top));
    return high + clear <= b.height + slack && Math.min(...parts.map((r) => r.left)) >= b.left - slack && Math.max(...parts.map((r) => r.right)) <= b.right + slack;
  };
  const step = (box, cls) => {
    box.classList.remove(...LADDER.map(([, c]) => c).filter(Boolean));
    if (cls) box.classList.add(cls);
  };

  const host = document.createElement('div');
  host.style.cssText = 'position:fixed;left:-300mm;top:0;visibility:hidden;contain:layout style';
  document.body.append(host);
  const rows = [];
  for (const card of root.querySelectorAll('.card')) {
    const content = card.querySelector(':scope > .content');
    if (!content) continue; // The icon sheet borrows .card for its look.
    const copy = card.cloneNode(false);
    copy.append(content.cloneNode(true));
    host.replaceChildren(copy);
    const row = { top: null, effect: null, title: TITLE_PT, overflow: [], tight: [] };

    const boxes = [...content.querySelectorAll('.effect')];
    [...copy.querySelectorAll('.effect')].forEach((box, i) => {
      const where = box.classList.contains('lasting') ? 'top' : 'effect';
      const held = box.childElementCount > 0;
      let [pt, cls] = LADDER[0];
      for ([pt, cls] of LADDER) {
        step(box, cls);
        if (fits(box, CLEARANCE_MM * MM, EPSILON_PX)) break;
      }
      step(boxes[i], cls);
      if (held) row[where] = pt;
      if (!fits(box, 0, OVERFLOW_PX)) row.overflow.push(where);
      else if (!fits(box, CLEARANCE_MM * MM, EPSILON_PX)) row.tight.push(where);
    });

    const name = copy.querySelector('.name');
    const room = name.getBoundingClientRect().width - 2 * TITLE_MARGIN_MM * MM;
    const wide = () => name.firstElementChild.getBoundingClientRect().width > room + EPSILON_PX;
    for (name.style.fontSize = ''; wide() && row.title > TITLE_FLOOR_PT; name.style.fontSize = `${row.title}pt`) row.title -= TITLE_STEP_PT;
    content.querySelector('.name').style.fontSize = name.style.fontSize;
    if (wide()) row.overflow.push('title');
    rows.push(row);
  }
  host.remove();
  return rows;
}

/**
 * @param card     content: name, kind, cost, insight, needs, replaces, tags, and
 *                 the generated face (faces.json): top and top_text above the art
 *                 (buildings), effect and text below it
 * @param layout   web/card/layout.json, through inset()
 * @param frameUrl the frame for this kind and tier
 * @param fit      optional measured zones of this frame (art/frames/r3/fit.json), in mm:
 *                 cost, banner, top, art, effect, the guide's where absent; plate, a color
 * @param artUrl   optional scene for the art window, center-cropped to it
 *
 * Sizes are the defaults until fitCards has run on the card in the page.
 */
export function renderCard(card, layout, frameUrl, fit = {}, artUrl) {
  const z = { ...layout.common, ...layout.kinds[card.kind], ...fit };
  const reqX = card.insight ? z.insight[2] + 0.5 : z.cost[2] + 2.4;
  return `<article class="card kind-${esc(card.kind)} tier-${tierOf(card.tier)}" style="--strip:${layout.kind_color[card.kind]}${fit.plate ? `;--plate:${esc(fit.plate)}` : ''}">
  <img class="frame" src="${esc(frameUrl)}" alt="">
  ${artUrl ? `<img class="art" src="${esc(artUrl)}" alt="" style="${place(z.art)}">` : ''}
  <div class="content">
    <div class="cost${card.insight ? ' beside' : ''}" style="${place(z.cost)}"><span data-safe>${esc(card.cost)}</span></div>
    ${insight(card, z.insight)}
    ${requirement(card, reqX, z.requirement)}
    ${tags(card, z.tags)}
    <div class="name" style="${place(z.banner)}"><span data-safe>${esc(card.name)}</span></div>
    ${z.top ? effectBox('lasting', z.top, card.top, card.top_text) : ''}
    ${effectBox('once', z.effect, card.effect, card.text)}
  </div>
</article>`;
}
