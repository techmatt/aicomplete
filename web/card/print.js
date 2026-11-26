// 327: the printed faces beyond player cards (Matt, Nov 25: show the printed component, rules text on hover, as
// Terraforming Mars and Spirit Island digital do): an AI board as a physical panel, and the adversary's event
// cards, projects and threats in the card frame, plus the research strip. Icon lines come from
// web/card/print-faces.json (`aic-sim docs`, crates/aic-core/src/print_face.rs); words are the content's own, in
// each face's `title` for the hover. No I/O: faces in, HTML out. Needs card.css and print.css.
//
// The three adversary kinds print on a player frame recolored (`adv-*` in print.css), with the building layout:
// the top box, the art window (its scene, else an emblem), the bottom box. fitCards sizes them as any card.
import { esc, part, renderCard } from './card.js';
import { icon, tagMedallion } from './icons.js';

const place = ([x0, y0, x1, y1]) => `left:${x0}mm;top:${y0}mm;width:${(x1 - x0).toFixed(2)}mm;height:${(y1 - y0).toFixed(2)}mm`;
const line = (parts) => `<div class="fx-line" data-safe>${parts.map(part).join('')}</div>`;
const ROMAN = ['', 'I', 'II', 'III'];
const tagName = (p) => p.replace(/^#/, '');

/** The frame each adversary kind prints on, and its layout kind. */
export const PRINT_FRAME = { threat: 'building-t1', project: 'event-t1', event: 'building-t1' };
const LAYOUT_KIND = { threat: 'building', project: 'event', event: 'building' };

/**
 * One adversary card in the card frame: renderCard's structure, so fitCards measures it unchanged.
 * @param kind   threat | project | event (its recolor, `adv-<kind>`)
 * @param f      { name, octagon, tags (names), top (lines), effect (lines), emblem (HTML), hover }
 */
function advCard(kind, f, layout, frameUrl, fit, artUrl) {
  const lk = LAYOUT_KIND[kind];
  const card = { kind: lk, tier: 1, cost: f.octagon, insight: 0, needs: [], replaces: null, tags: f.tags, name: f.name, top: f.top ?? [], top_text: '', effect: f.effect, text: '' };
  const z = { ...layout.common, ...layout.kinds[lk], ...fit };
  let html = renderCard(card, layout, frameUrl, fit, artUrl);
  html = html.replace(`class="card kind-${lk} tier-1"`, `class="card kind-${lk} tier-1 adv adv-${kind}" title="${esc(f.hover)}" data-print="${kind}"`);
  // With a scene, the emblem (a project's compute track, a threat's sites) rides on a band at the window's foot.
  if (f.emblem) html = html.replace('<div class="content">', `<div class="emblem${artUrl ? ' over-art' : ''}" style="${place(z.art)}">${f.emblem}</div>\n  <div class="content">`);
  return html;
}

/** A threat: tags and tier atop, what it does unpaid in the dark top box, its sites in the window, the pay below. */
export function renderThreat(t, layout, frameUrl, fit, artUrl) {
  const pay = t.pay.flatMap((p, i) => (i ? ['OR', ...p] : p));
  return advCard('threat', {
    name: t.name,
    octagon: ROMAN[t.tier],
    tags: t.tags.map(tagName),
    top: t.unpaid,
    effect: [[...pay, '→', 'counter']],
    emblem: `<div class="emblem-row">${icon('threat')}<span class="emblem-on">${t.sites.map(icon).join('<span class="emblem-or">/</span>')}</span></div><div class="emblem-say">Unpaid: the top box. Pay one to counter it.</div>`,
    hover: `${t.name} (tier ${t.tier} threat). Unpaid: ${t.text} Counter it: pay one of the costs below.`,
  }, layout, frameUrl, fit, artUrl);
}

/**
 * A project: its compute cost in the octagon, what it does when filled in the top box, the compute track in the
 * window. `filled` compute cubes show on the track (the mockup's state; 0 on a plain face).
 */
export function renderProject(p, layout, frameUrl, fit, artUrl, filled = 0) {
  const cells = Array.from({ length: p.cost }, (_, i) => `<span class="cell${i < filled ? ' on' : ''}"></span>`).join('');
  return advCard('project', {
    name: p.name,
    octagon: p.cost,
    tags: p.tags.map(tagName),
    effect: p.lines,
    emblem: `<div class="compute-track" style="--cols:${Math.min(p.cost, 8)}">${cells}</div>`,
    hover: `${p.name} (adversary project, ${p.cost} compute). When its compute fills it: ${p.text}`,
  }, layout, frameUrl, fit, artUrl);
}

/**
 * A card from the viewer's export (`cards-export.mjs` puts print-faces.json's face in `c.print`), drawn by its kind.
 * @param frame  (name) => the frame's URL, `building-t1` and the like
 * @param fits   art/frames/r3/fit.json
 * @returns HTML, or null for a kind print.js doesn't draw
 */
export function printFace(c, layout, frame, fits, artUrl) {
  const f = PRINT_FRAME[c.kind === 'adversary' ? 'event' : c.kind];
  if (c.kind === 'ai') return renderBoard(c.print);
  if (c.kind === 'threat') return renderThreat(c.print, layout, frame(f), fits[f], artUrl);
  if (c.kind === 'project') return renderProject(c.print, layout, frame(f), fits[f], artUrl);
  if (c.kind === 'adversary') return renderEvent(c.print, layout, frame(f), fits[f], artUrl);
  return null;
}

/** The beats an event card's lines belong to, in print order, top box first. */
const TOP_BEATS = ['card', 'infiltrate'];

/** An adversary event card: the Card beat and Infiltrate above the window, Strike, Redeploy and Escalate below. */
export function renderEvent(e, layout, frameUrl, fit, artUrl) {
  const top = e.lines.filter((l) => TOP_BEATS.includes(l.beat)).map((l) => l.icons);
  const effect = e.lines.filter((l) => !TOP_BEATS.includes(l.beat)).map((l) => l.icons);
  return advCard('event', {
    name: e.name,
    octagon: ROMAN[e.stage],
    tags: [],
    top,
    effect,
    emblem: `<div class="emblem-stage">Stage ${ROMAN[e.stage]}</div>`,
    hover: `${e.name} (adversary event, stage ${ROMAN[e.stage]}). ${e.text} Escalate: each site of last round's type holding an Agent takes 1 more, and every factory holding one.`,
  }, layout, frameUrl, fit, artUrl);
}

/**
 * The research strip: its slots in order, the last the players' loss, a marker on `at` (0: none advanced).
 * @param slots print-faces.json's `research`
 */
export function renderResearch(slots, at = 0, name = 'Research') {
  const cells = slots.map((s, i) => `<div class="rslot${i + 1 === at ? ' at' : ''}${i + 1 < at ? ' past' : ''}${i === slots.length - 1 ? ' last' : ''}" title="${esc(`${i + 1}. ${s.name}: ${s.text}`)}">
      <span class="rnum">${i + 1}</span>${line(s.icons)}${i + 1 === at ? `<span class="rmark">${icon('marker')}</span>` : ''}</div>`).join('');
  return `<div class="research-strip" title="${esc(name)}">${cells}</div>`;
}

/** One track space: what it costs to cover and what it prints; a node token over it while covered. */
function space(s, i, covered, kind) {
  const get = [...s.get];
  const parts = [];
  if (s.insight) parts.push(`<span class="insight-total" title="Insight each round: ${s.insight}">${icon('insight')}<b>${s.insight}</b></span>`);
  if (get.length) parts.push(line(get));
  if (s.capstone) parts.push(`<span class="capstone" title="${esc(`${s.capstone[0]}: ${s.capstone[1]}`)}">★ ${esc(s.capstone[0])}</span>`);
  const pay = s.pay.length ? `<div class="pay">${line(s.pay)}</div>` : '';
  const title = [s.pay.length ? `Cover: pay ${s.pay[0].replace('−', '')} energy.` : 'Free.', s.capstone ? `${s.capstone[0]}: ${s.capstone[1]}` : ''].join(' ');
  return `<div class="space ${kind}${covered ? ' covered' : ''}" title="${esc(title)}">${pay}<div class="get">${parts.join('')}</div>${covered ? `<span class="node-token">${icon('node')}</span>` : ''}</div>`;
}

/**
 * An AI board as a physical panel, about 180×110 mm (327's staging default, Spirit Island's spirit panel scale):
 * the name, printed income and tags; the special rule as one icon line; the two node tracks as spaces a node token
 * covers; its node powers and tiered powers as icon boxes, each with its used-marker socket (a tiered power's tiers
 * share one).
 * @param b      print-faces.json's board
 * @param state  { economy, play: spaces uncovered on each track (the first economy space from setup);
 *                 used: power and group names used this round }
 */
export function renderBoard(b, state = {}) {
  // With no state, the face as printed: every space showing (no tokens on it).
  const economy = state.economy ?? b.economy.length;
  const play = state.play ?? b.play.length;
  const used = new Set(state.used ?? []);
  const track = (name, spaces, n, kind) => `<div class="track ${kind}"><div class="track-name">${name}</div>${spaces.map((s, i) => space(s, i, i >= n, kind)).join('')}</div>`;
  const socket = (on) => `<span class="used-socket${on ? ' on' : ''}" title="Used marker: once a round">${on ? icon('marker') : ''}</span>`;
  const plain = b.powers.filter((p) => !p.group);
  const groups = [...new Set(b.powers.filter((p) => p.group).map((p) => p.group))];
  const nodeBox = (p) => `<div class="power node-power" title="${esc(`${p.name}: ${p.text}`)}"><div class="power-head"><span>${esc(p.name)}</span>${socket(used.has(p.name))}</div>${line(p.line)}</div>`;
  const groupBox = (g) => {
    const tiers = b.powers.filter((p) => p.group === g);
    return `<div class="power tiered" title="${esc(tiers.map((p) => `${p.name}: ${p.text}`).join(' / '))}"><div class="power-head"><span>${esc(g)}</span>${socket(used.has(g))}</div>${tiers.map((p, i) => `<div class="tier-row"><span class="tier-num">${ROMAN[i + 1]}</span>${line(p.line)}</div>`).join('')}</div>`;
  };
  return `<article class="board-face" data-print="board" title="${esc(`${b.name}: ${b.text}`)}">
  <header class="board-head">
    <div class="board-name">${esc(b.name)}</div>
    <div class="board-tags">${b.tags.map((t) => tagMedallion(tagName(t))).join('')}</div>
    <div class="board-income" title="Printed income, each round">${line(b.income)}</div>
    <div class="board-rule" title="${esc(`${b.rule[0]}: ${b.rule[1]}`)}"><span class="rule-name">${esc(b.rule[0])}</span>${b.rule_line.map(line).join('')}</div>
  </header>
  <section class="tracks">
    ${track('Economy', b.economy, economy, 'economy')}
    ${track('Card play', b.play, play, 'play')}
  </section>
  <section class="powers">
    <div class="power-set">${plain.map(nodeBox).join('')}</div>
    <div class="power-set">${groups.map(groupBox).join('')}</div>
  </section>
</article>`;
}
