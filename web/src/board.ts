// The SVG board, drawn from the engine's generic view: every hex of every country piece
// (pointy-top axial, `data-hex="q,r"`), filled by what it is (city, military base, open
// land, ocean), with a band in the country's color along each country border, the
// printed hex number, an industrial-zone mark, a pennant in the facing country's color
// on each military base, the printed fronts (the land edges between two countries with
// bases facing each other, and a link between the two bases, both at war every round), and each token
// stack drawn as a small glyph with its count. Knows no
// rules: what to highlight is handed in. Token kinds it has no glyph for are drawn as
// plain counters.

import type { HexView, Token, View } from './engine';
import { ADVERSARY_COLOR, contentIds, countryName, hexKey, kindFill, strong, tokenColor } from './format';

const SQRT3 = Math.sqrt(3);
const NS = 'http://www.w3.org/2000/svg';

export interface Marks {
  /** Hexes the armed choice can target: clicking one takes that action. */
  targets: Set<string>;
  /** Hexes the events of the shown step touched. */
  changed: Set<string>;
  /** The hexes of face-up threats' sources (070). */
  sources?: Set<string>;
}

const center = (q: number, r: number): [number, number] => [SQRT3 * (q + r / 2), 1.5 * r];

const corner = (x: number, y: number, k: number, s: number): [number, number] => {
  const a = (Math.PI / 180) * (60 * k - 30);
  return [x + s * Math.cos(a), y + s * Math.sin(a)];
};

const corners = (x: number, y: number, s = 0.97) =>
  Array.from({ length: 6 }, (_, k) => corner(x, y, k, s).map((v) => v.toFixed(3)).join(',')).join(' ');

/** Axial neighbor across the edge from corner k to corner k + 1. */
const EDGE: [number, number][] = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];

function el<K extends keyof SVGElementTagNameMap>(tag: K, attrs: Record<string, string | number>, text?: string) {
  const e = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, String(v));
  if (text !== undefined) e.textContent = text;
  return e;
}

const f = (n: number) => n.toFixed(3);

/** Colors a glyph may need besides its kind's own: a seat's, the hex's country's. */
interface Paint {
  seat: (seat: number) => string;
  country: string;
}

/** The owner color of `kind:owner` tokens: the adversary's, or the seat's. */
const ownerColor = (arg: string | undefined, paint: Paint, fallback: string) =>
  arg === 'adversary' ? ADVERSARY_COLOR : arg !== undefined ? paint.seat(Number(arg)) : fallback;

/** A glyph for one token kind, centered on 0,0 with half-size `s`. */
function glyph(kind: string, s: number, paint: Paint): SVGElement {
  const g = el('g', {});
  const [base, arg] = kind.split(':');
  const c = tokenColor(kind);
  const p = (d: string, fill: string, cls = 'glyph') => el('path', { d, fill, class: cls });
  switch (base) {
    case 'cube': {
      // An owner cube, drawn in perspective: the top face lighter.
      const col = ownerColor(arg, paint, c);
      const h = 0.87 * s;
      g.append(
        p(`M0,${f(-s)} L${f(h)},${f(-0.5 * s)} V${f(0.5 * s)} L0,${f(s)} L${f(-h)},${f(0.5 * s)} V${f(-0.5 * s)} Z`, col),
        p(`M0,${f(-s)} L${f(h)},${f(-0.5 * s)} L0,0 L${f(-h)},${f(-0.5 * s)} Z`, '#ffffff55', 'face'),
        p(`M0,0 V${f(s)} M0,0 L${f(h)},${f(-0.5 * s)} M0,0 L${f(-h)},${f(-0.5 * s)}`, 'none', 'detail'),
      );
      break;
    }
    case 'ambassador': {
      const col = ownerColor(arg, paint, c);
      g.append(
        el('circle', { cx: 0, cy: f(-0.5 * s), r: f(0.38 * s), fill: col, class: 'glyph' }),
        p(`M${f(-0.75 * s)},${f(0.95 * s)} Q${f(-0.75 * s)},${f(-0.05 * s)} 0,${f(-0.05 * s)} Q${f(0.75 * s)},${f(-0.05 * s)} ${f(0.75 * s)},${f(0.95 * s)} Z`, col),
      );
      break;
    }
    case 'factory':
      g.append(p(`M${f(-s)},${f(0.85 * s)} V${f(-0.15 * s)} L${f(-0.45 * s)},${f(-0.6 * s)} V${f(-0.15 * s)} L${f(0.1 * s)},${f(-0.6 * s)} V${f(-0.15 * s)} L${f(0.45 * s)},${f(-0.5 * s)} V${f(-s)} H${f(s)} V${f(0.85 * s)} Z`, c));
      break;
    case 'station':
      // A research station: an observatory dome on a block, with an antenna.
      g.append(
        p(`M${f(-0.9 * s)},${f(0.85 * s)} V${f(0.2 * s)} H${f(-0.7 * s)} A${f(0.7 * s)},${f(0.7 * s)} 0 0 1 ${f(0.7 * s)},${f(0.2 * s)} H${f(0.9 * s)} V${f(0.85 * s)} Z`, c),
        p(`M${f(0.15 * s)},${f(-0.45 * s)} L${f(0.6 * s)},${f(-s)}`, 'none', 'detail'),
        el('circle', { cx: f(0.62 * s), cy: f(-s), r: f(0.12 * s), fill: '#e0f2f1' }),
      );
      break;
    case 'node': {
      // A player's node: a server rack in the seat's color.
      const col = ownerColor(arg, paint, c);
      g.append(el('rect', { x: f(-0.7 * s), y: f(-0.95 * s), width: f(1.4 * s), height: f(1.9 * s), rx: f(0.15 * s), fill: col, class: 'glyph' }));
      for (const yy of [-0.5, 0, 0.5]) {
        g.append(el('line', { x1: f(-0.45 * s), x2: f(0.2 * s), y1: f(yy * s), y2: f(yy * s), class: 'detail' }));
        g.append(el('circle', { cx: f(0.42 * s), cy: f(yy * s), r: f(0.09 * s), fill: '#7cfc8a' }));
      }
      break;
    }
    case 'botnet':
      // A botnet: a diamond with a core.
      g.append(
        p(`M0,${f(-s)} L${f(0.8 * s)},0 L0,${f(s)} L${f(-0.8 * s)},0 Z`, ADVERSARY_COLOR),
        el('circle', { cx: 0, cy: 0, r: f(0.28 * s), fill: '#ffcdd2' }),
      );
      break;
    case 'troop':
      // An adversary troop: a spearhead.
      g.append(p(`M0,${f(-s)} L${f(0.85 * s)},${f(0.8 * s)} L0,${f(0.35 * s)} L${f(-0.85 * s)},${f(0.8 * s)} Z`, c));
      break;
    case 'unit':
      // Corp units at a base: rank chevrons in the base's country's color.
      g.append(
        p(`M${f(-0.9 * s)},${f(-0.15 * s)} L0,${f(-0.85 * s)} L${f(0.9 * s)},${f(-0.15 * s)} V${f(0.25 * s)} L0,${f(-0.45 * s)} L${f(-0.9 * s)},${f(0.25 * s)} Z`, paint.country),
        p(`M${f(-0.9 * s)},${f(0.45 * s)} L0,${f(-0.25 * s)} L${f(0.9 * s)},${f(0.45 * s)} V${f(0.85 * s)} L0,${f(0.15 * s)} L${f(-0.9 * s)},${f(0.85 * s)} Z`, paint.country),
      );
      break;
    case 'harden':
      // A hardening token: a shield.
      g.append(p(`M0,${f(-s)} L${f(0.85 * s)},${f(-0.62 * s)} V${f(0.05 * s)} Q${f(0.8 * s)},${f(0.75 * s)} 0,${f(1.02 * s)} Q${f(-0.8 * s)},${f(0.75 * s)} ${f(-0.85 * s)},${f(0.05 * s)} V${f(-0.62 * s)} Z`, c));
      g.append(p(`M0,${f(-0.6 * s)} V${f(0.65 * s)}`, 'none', 'detail'));
      break;
    case 'corruption':
      // A corruption marker: a burned patch with a cross through it, where a node fell.
      g.append(
        el('circle', { cx: 0, cy: 0, r: f(0.9 * s), fill: '#e1bee7', stroke: ADVERSARY_COLOR, 'stroke-width': f(0.22 * s), class: 'glyph' }),
        el('path', { d: `M${f(-0.45 * s)},${f(-0.45 * s)} L${f(0.45 * s)},${f(0.45 * s)} M${f(0.45 * s)},${f(-0.45 * s)} L${f(-0.45 * s)},${f(0.45 * s)}`, fill: 'none', stroke: ADVERSARY_COLOR, 'stroke-width': f(0.24 * s), 'stroke-linecap': 'round' }),
      );
      break;
    case 'land_corruption':
      // 145: land corruption (138): a scorched square of land with the marker's cross; no node may be placed.
      g.append(
        el('rect', { x: f(-0.85 * s), y: f(-0.85 * s), width: f(1.7 * s), height: f(1.7 * s), rx: f(0.2 * s), fill: '#bcaaa4', stroke: ADVERSARY_COLOR, 'stroke-width': f(0.22 * s), class: 'glyph' }),
        el('path', { d: `M${f(-0.45 * s)},${f(-0.45 * s)} L${f(0.45 * s)},${f(0.45 * s)} M${f(0.45 * s)},${f(-0.45 * s)} L${f(-0.45 * s)},${f(0.45 * s)}`, fill: 'none', stroke: ADVERSARY_COLOR, 'stroke-width': f(0.24 * s), 'stroke-linecap': 'round' }),
      );
      break;
    default:
      g.append(el('circle', { cx: 0, cy: 0, r: f(0.9 * s), fill: c, class: 'glyph' }));
  }
  return g;
}

/** Glyph scale per kind: owner cubes and facilities a little larger than pawns. */
const SCALE: Record<string, number> = { cube: 1.05, station: 1.1, factory: 1, node: 1, ambassador: 0.9, troop: 0.95, unit: 0.95, harden: 0.85, botnet: 1, corruption: 0.95, land_corruption: 0.95 };
/** The adversary's tokens: its owner cube, its ambassadors, its troops, its botnets, a corruption marker, land corruption. */
const isAdversary = (kind: string) => kind === 'troop' || kind === 'botnet' || kind === 'corruption' || kind === 'land_corruption' || kind.endsWith(':adversary');
const ORDER = ['cube', 'station', 'factory', 'node', 'harden', 'unit', 'ambassador', 'corruption', 'land_corruption', 'botnet', 'troop'];
const rank = (k: string) => {
  const i = ORDER.indexOf(k.split(':')[0]);
  return i < 0 ? ORDER.length : i;
};

/** Players' pieces and facilities on the upper row, the adversary's on the lower one. */
function rows(tokens: Token[]): Token[][] {
  const sorted = [...tokens].sort((a, b) => rank(a.kind) - rank(b.kind) || a.kind.localeCompare(b.kind));
  const top = sorted.filter((t) => !isAdversary(t.kind));
  const bottom = sorted.filter((t) => isAdversary(t.kind));
  return [top, bottom].filter((r) => r.length);
}

function drawTokens(layer: SVGGElement, h: HexView, x: number, y: number, paint: Paint) {
  const key = hexKey(h.hex);
  const rs = rows(h.tokens);
  const ys = rs.length === 1 ? [y + 0.12] : [y - 0.07, y + 0.45];
  rs.forEach((row, ri) => {
    const step = Math.min(0.47, 1.5 / row.length);
    const s = step * 0.44;
    row.forEach((t, j) => {
      const tx = x + (j - (row.length - 1) / 2) * step;
      const ty = ys[ri];
      const k = s * (SCALE[t.kind.split(':')[0]] ?? 1);
      const tg = el('g', { class: 'token', 'data-hex': key, 'data-token': t.kind });
      const hit = el('circle', { cx: f(tx), cy: f(ty), r: f(step / 2), class: 'hit' });
      const gl = glyph(t.kind, k, paint);
      gl.setAttribute('transform', `translate(${f(tx)} ${f(ty)})`);
      tg.append(hit, gl);
      if (t.count !== 1) {
        tg.append(el('text', { x: f(tx + k * 0.95), y: f(ty + k * 0.85), class: 'count', 'font-size': f(Math.max(0.15, k * 1.25)) }, String(t.count)));
      }
      layer.append(tg);
    });
  });
}

/** A military base's pennant, in the color of the country across its front. */
function pennant(x: number, y: number, color: string): SVGElement {
  const g = el('g', { class: 'base-mark', transform: `translate(${f(x)} ${f(y)})` });
  g.append(
    el('path', { d: 'M0,0.17 V-0.17', class: 'pole' }),
    el('path', { d: 'M0,-0.17 L0.2,-0.1 L0,-0.03 Z', fill: color, class: 'flag' }),
  );
  return g;
}

/** Each front's two bases (country a's facing b, b's facing a), linked across it. */
function frontLinks(view: View, pts: number[][]): SVGElement[] {
  const links: SVGElement[] = [];
  view.hexes.forEach((h, i) => {
    if (h.kind !== 'base' || h.facing === undefined || h.facing < h.country) return;
    const j = view.hexes.findIndex((o) => o.kind === 'base' && o.country === h.facing && o.facing === h.country);
    if (j < 0) return;
    const [[ax, ay], [bx, by]] = [pts[i], pts[j]];
    links.push(el('line', { x1: f(ax), y1: f(ay), x2: f(bx), y2: f(by), class: 'front-link', 'data-front': `${h.country}-${h.facing}` }));
  });
  return links;
}

/**
 * Each adversary troop's next target (019): a dashed arrow from the troop to the
 * piece it is heading for, solid red when it strikes in the next adversary phase.
 */
function arrowLayer(view: View, pts: number[][]): SVGElement {
  const layer = el('g', { class: 'arrows' });
  for (const a of view.arrows) {
    // From the troop (low on its hex, where the adversary's row sits) toward the
    // target's center, stopping short of it; the head follows the drawn shaft.
    const [ax, ay] = [pts[a.from][0], pts[a.from][1] + 0.3];
    const [bx, by] = pts[a.to];
    const len = Math.hypot(bx - ax, by - ay);
    const g = el('g', { class: `arrow ${a.kind}${a.rounds <= 1 ? ' soon' : ''}`, 'data-from': a.from, 'data-to': a.to });
    layer.append(g);
    if (len < 0.8) {
      // A target on the troop's own hex: a ring round the hex instead of an arrow.
      g.append(el('circle', { cx: f(bx), cy: f(by), r: 0.78, fill: 'none' }), el('title', {}, contentIds(a.doc)));
      continue;
    }
    const [ux, uy] = [(bx - ax) / len, (by - ay) / len];
    const [sx, sy] = [ax + ux * 0.25, ay + uy * 0.25];
    const [ex, ey] = [bx - ux * 0.45, by - uy * 0.45];
    g.append(el('line', { x1: f(sx), y1: f(sy), x2: f(ex), y2: f(ey) }));
    const [px, py] = [-uy, ux];
    const head = `M${f(ex + ux * 0.2)},${f(ey + uy * 0.2)} L${f(ex + px * 0.12)},${f(ey + py * 0.12)} L${f(ex - px * 0.12)},${f(ey - py * 0.12)} Z`;
    g.append(el('path', { d: head }));
    g.append(el('title', {}, contentIds(a.doc)));
  }
  return layer;
}

export function renderBoard(svg: SVGSVGElement, view: View, marks: Marks) {
  svg.replaceChildren();
  if (!view.hexes.length) return;
  const pts = view.hexes.map((h) => center(h.hex[0], h.hex[1]));
  const xs = pts.map((p) => p[0]);
  const ys = pts.map((p) => p[1]);
  const pad = 1.15;
  const [x0, y0] = [Math.min(...xs) - pad, Math.min(...ys) - pad];
  svg.setAttribute('viewBox', `${f(x0)} ${f(y0)} ${f(Math.max(...xs) + pad - x0)} ${f(Math.max(...ys) + pad - y0)}`);
  svg.setAttribute('preserveAspectRatio', 'xMidYMid meet');

  const at = new Map(view.hexes.map((h) => [hexKey(h.hex), h]));
  const countryColor = (c: number) => strong(view.countries[c]?.color ?? '#888888');
  const seatColor = (seat: number) => countryColor(view.seats[seat]?.country ?? seat);
  // The printed fronts: pairs of countries with a base facing each other.
  const fronts = new Set(view.hexes.filter((h) => h.kind === 'base' && h.facing !== undefined).map((h) => `${h.country}-${h.facing}`));

  const hexLayer = el('g', { class: 'hexes' });
  const borderLayer = el('g', { class: 'borders' });
  // 139: no clock; the fronts fight at the end of every round, so they are always drawn at war.
  const frontLayer = el('g', { class: 'fronts war' });
  const labelLayer = el('g', { class: 'labels' });
  const tokenLayer = el('g', { class: 'tokens' });
  const markLayer = el('g', { class: 'marks' });

  view.hexes.forEach((h, i) => {
    const [x, y] = pts[i];
    const key = hexKey(h.hex);
    const cls = ['hex', `k-${h.kind}`];
    if (marks.targets.has(key)) cls.push('target');
    if (marks.changed.has(key)) cls.push('changed');
    const g = el('g', { class: cls.join(' '), 'data-hex': key, 'data-index': h.index, 'data-country': h.country });
    g.append(el('polygon', { points: corners(x, y), fill: kindFill(h.kind), class: 'cell' }));
    g.append(el('text', { x: f(x), y: f(y - 0.66), class: 'num' }, String(h.number)));
    // A printed industrial zone: a cog outline low on the hex.
    if (h.industrial) {
      g.append(el('circle', { cx: f(x), cy: f(y + 0.62), r: 0.13, class: 'zone-mark' }));
      g.append(el('title', {}, 'industrial zone: cities build factories here'));
    }
    if (h.kind === 'base' && h.facing !== undefined) g.append(pennant(x + 0.42, y - 0.42, countryColor(h.facing)));
    hexLayer.append(g);

    // A band in the country's color along every edge that leaves the country; a land
    // edge into another country's land is a front, drawn over it.
    const band = countryColor(h.country);
    EDGE.forEach(([dq, dr], k) => {
      const n = at.get(`${h.hex[0] + dq},${h.hex[1] + dr}`);
      if (n && n.country === h.country) return;
      const [ax, ay] = corner(x, y, k, 0.9);
      const [bx, by] = corner(x, y, (k + 1) % 6, 0.9);
      borderLayer.append(el('line', { x1: f(ax), y1: f(ay), x2: f(bx), y2: f(by), stroke: band, class: 'border' }));
      if (n && h.kind !== 'ocean' && n.kind !== 'ocean' && h.country < n.country && fronts.has(`${h.country}-${n.country}`)) {
        const [cx, cy] = corner(x, y, k, 1);
        const [dx, dy] = corner(x, y, (k + 1) % 6, 1);
        frontLayer.append(el('line', { x1: f(cx), y1: f(cy), x2: f(dx), y2: f(dy), class: 'front-edge', 'data-front': `${h.country}-${n.country}` }));
      }
    });

    // Highlights sit above the tokens so a target stays visible under a crowd.
    // 070: a face-up threat's sources stay marked, under any target ring.
    if (marks.sources?.has(key)) markLayer.append(el('polygon', { points: corners(x, y, 0.72), class: 'mark source' }));
    if (marks.targets.has(key) || marks.changed.has(key)) {
      markLayer.append(el('polygon', { points: corners(x, y, 0.86), class: marks.targets.has(key) ? 'mark target' : 'mark changed' }));
    }

    drawTokens(tokenLayer, h, x, y, { seat: seatColor, country: band });
  });

  frontLayer.append(...frontLinks(view, pts));

  // Country labels at each piece's center, under the tokens and not catching the mouse.
  for (const c of view.countries) {
    const cp = c.hexes.map((i) => pts[i]);
    const cx = cp.reduce((s, p) => s + p[0], 0) / cp.length;
    const cy = cp.reduce((s, p) => s + p[1], 0) / cp.length;
    const g = el('g', { class: 'country-label', 'data-country': c.index });
    // The piece's printed name, a word per line, centered on the piece.
    const words = countryName(view, c.index).split(/\s+/);
    words.forEach((w, i) => {
      const dy = (i - (words.length - 1) / 2) * 0.62;
      g.append(el('text', { x: f(cx), y: f(cy + dy), fill: strong(c.color) }, w));
    });
    labelLayer.append(g);
  }

  svg.append(hexLayer, labelLayer, borderLayer, frontLayer, tokenLayer, arrowLayer(view, pts), markLayer);
}
