// `npm run cards:fit`: what size every player card prints at. Puts each one through the real
// renderer and fitCards in a headless browser, from content and web/card/faces.json as they
// stand in the working tree, and writes out/card-fit.json:
//   { goal, cards: { <id>: { name, top, effect, title, target, overflow, where, tight } } }
// top and effect are pt (null: nothing in that box), title is pt, target the target box's pt
// (null: none), overflow says something
// still breaks its box or plate, where says what, and tight lists boxes inside but short of
// the clearance. Prints the cards below the goal, and what overflows by how much (px, and
// roughly in characters). Exits 1 if anything overflows or any box prints below the goal (264: no
// recorded exceptions). No server.
//   AIC_BROWSER browser executable (see cdp.mjs)

import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

import { fitCards, frameOf, inset, renderCard } from '../card/card.js';
import { printFace } from '../card/print.js';
import { REPO, exportCards } from './cards-export.mjs';
import { openPage } from './cdp.mjs';

const GOAL_PT = 7.5;
const OUT = join(REPO, 'out');
const started = performance.now();

const data = exportCards(REPO);
// 327: the adversary's printed cards (event cards, projects, threats: web/card/print.js) fit as player cards do;
// the boards, drawn at a fixed size, are checked for anything spilling out of a space or power box.
const players = data.cards.filter((c) => c.render || (c.print && c.kind !== 'ai'));
const boards = data.cards.filter((c) => c.print && c.kind === 'ai');
const layout = inset(data.layout);
const frameUrl = (f) => pathToFileURL(join(REPO, 'art', 'frames', 'r3', `${f}.jpg`)).href;
const face = (c) => {
  if (!c.render) return printFace(c, layout, frameUrl, data.fits);
  const f = frameOf(c);
  return renderCard(c.render, layout, frameUrl(f), data.fits[f]);
};

mkdirSync(OUT, { recursive: true });
const html = join(OUT, 'card-fit-page.html');
writeFileSync(html, `<!doctype html><html><head><meta charset="utf-8"><base href="${pathToFileURL(join(REPO, 'web', 'card')).href}/"><link rel="stylesheet" href="card.css"><link rel="stylesheet" href="print.css"></head><body>${players.map(face).join('')}${boards.map((c) => printFace(c)).join('')}</body></html>`);

/** 327: each board's spaces and power boxes, and the board itself: what runs past its box, by name. */
function boardSpills() {
  const out = [];
  for (const b of document.querySelectorAll('.board-face')) {
    const name = b.querySelector('.board-name').textContent;
    for (const box of [b, ...b.querySelectorAll('.space, .power, .board-head')]) {
      if (box.scrollHeight > box.clientHeight + 1 || box.scrollWidth > box.clientWidth + 1) out.push(`${name}: ${box.className} ${(box.title || box.textContent).slice(0, 40)}`);
    }
  }
  return out;
}

/**
 * How far each box and title runs past its edge once fitCards has sized it, in px, measured as
 * fitCards measures (a copy at true size, the box's icon rows and text against its height and
 * width; a title against its plate less the margin). `chars` turns it into characters, roughly:
 * the overflow in text lines times the box's characters per line, or the title's px times its
 * characters per px. Null where the text gives no rate (an icon row alone). Only for the report:
 * what overflows is fitCards' call.
 */
function overruns() {
  const MM = 96 / 25.4;
  const TITLE_MARGIN_MM = 1;
  const host = document.createElement('div');
  host.style.cssText = 'position:fixed;left:-300mm;top:0;visibility:hidden;contain:layout style';
  document.body.append(host);
  const out = [];
  for (const card of document.querySelectorAll('.card')) {
    const content = card.querySelector(':scope > .content');
    if (!content) continue;
    const copy = card.cloneNode(false);
    copy.append(content.cloneNode(true));
    host.replaceChildren(copy);
    const row = {};
    for (const box of copy.querySelectorAll('.effect')) {
      const parts = [...box.querySelectorAll('.fx-line, .rules')].map((e) => e.getBoundingClientRect());
      if (!parts.length) continue;
      const b = box.getBoundingClientRect();
      const down = Math.max(...parts.map((r) => r.bottom)) - Math.min(...parts.map((r) => r.top)) - b.height;
      const side = Math.max(b.left - Math.min(...parts.map((r) => r.left)), Math.max(...parts.map((r) => r.right)) - b.right);
      const rules = box.querySelector('.rules');
      let chars = null;
      if (rules && down > 0) {
        const lineHeight = parseFloat(getComputedStyle(rules).lineHeight);
        const lines = rules.getBoundingClientRect().height / lineHeight;
        if (lineHeight > 0 && lines > 0) chars = Math.ceil((down / lineHeight) * (rules.textContent.trim().length / lines));
      }
      row[box.classList.contains('lasting') ? 'top' : 'effect'] = { down, side, chars };
    }
    const name = copy.querySelector('.name');
    const span = name.firstElementChild;
    const w = span.getBoundingClientRect().width;
    const over = w - (name.getBoundingClientRect().width - 2 * TITLE_MARGIN_MM * MM);
    row.title = { side: over, chars: w > 0 ? Math.ceil((over * span.textContent.trim().length) / w) : null };
    out.push(row);
  }
  host.remove();
  return out;
}

const page = await openPage({ width: 900, height: 900 });
let rows;
let spills = [];
let fonts;
let runs;
try {
  await page.send('Page.navigate', { url: pathToFileURL(html).href });
  await page.until(`document.readyState === 'complete' && document.querySelectorAll('.card').length === ${players.length}`, 'the cards');
  rows = await page.js(`(${fitCards})()`);
  spills = await page.js(`(${boardSpills})()`);
  fonts = await page.js(`[...document.fonts].filter((f) => f.status === 'loaded').length`);
  if (rows.some((r) => r.overflow.length)) runs = await page.js(`(${overruns})()`);
} finally {
  await page.close();
}
if (rows.length !== players.length) throw new Error(`cards-fit: ${rows.length} rows for ${players.length} cards`);
if (fonts < 3) throw new Error(`cards-fit: only ${fonts} fonts loaded: these sizes would be for a fallback`);

const cards = Object.fromEntries(
  players.map((c, i) => {
    const { top, effect, title, target, overflow, tight } = rows[i];
    return [c.id, { name: c.name, top, effect, title, target, overflow: overflow.length > 0, where: overflow, tight }];
  }),
);
writeFileSync(join(OUT, 'card-fit.json'), `${JSON.stringify({ goal: GOAL_PT, cards }, null, 1)}\n`);

const list = Object.entries(cards);
const below = (box) => list.filter(([, c]) => c[box] != null && c[box] < GOAL_PT);
const bySize = (found, box) =>
  Object.entries(Object.groupBy(found, ([, c]) => c[box]))
    .sort(([a], [b]) => b - a)
    .map(([pt, of]) => `${pt} pt: ${of.length}`)
    .join(', ');
const held = (box) => list.filter(([, c]) => c[box] != null).length;
const any = new Set([...below('top'), ...below('effect')].map(([id]) => id));
console.log(`${list.length} cards (${data.cards.filter((c) => c.render).length} player, ${list.length - data.cards.filter((c) => c.render).length} adversary), ${any.size} with a box below ${GOAL_PT} pt`);
console.log(`${boards.length} boards, ${spills.length} boxes spilling`);
for (const s of spills) console.log(`SPILL ${s}`);
for (const [box, label] of [['top', 'top box'], ['effect', 'effect box']]) {
  const found = below(box);
  console.log(`${label}: ${found.length} of ${held(box)} below${found.length ? ` (${bySize(found, box)})` : ''}`);
  for (const [id, c] of found.sort(([, a], [, b]) => a[box] - b[box])) console.log(`  ${c[box]} pt  ${id}`);
}
const titled = list.filter(([, c]) => c.title < 15).sort(([, a], [, b]) => a.title - b.title);
console.log(`titles under 15 pt: ${titled.length}${titled.length ? `, smallest ${titled[0][1].title} pt (${titled[0][0]})` : ''}`);
const targeted = list.filter(([, c]) => c.target != null).sort(([, a], [, b]) => a.target - b.target);
if (targeted.length) console.log(`target boxes: ${targeted.length}, smallest ${targeted[0][1].target} pt (${targeted[0][0]})`);
const tight = list.filter(([, c]) => c.tight.length);
if (tight.length) console.log(`short of the clearance: ${tight.map(([id]) => id).join(', ')}`);
const broken = list.filter(([, c]) => c.overflow);
/** "effect 6.2 px down, about 14 characters over": what of `where` runs past its edge. */
const overrun = (i, where) => {
  const r = runs?.[i]?.[where];
  if (!r) return where;
  const px = [r.down > 0 && `${r.down.toFixed(1)} px down`, r.side > 0 && `${r.side.toFixed(1)} px ${where === 'title' ? 'wide' : 'across'}`].filter(Boolean);
  return `${where} ${px.join(' and ') || 'under a px'}${r.chars ? `, about ${r.chars} character${r.chars === 1 ? '' : 's'} over` : ''}`;
};
for (const [id, c] of broken) console.log(`OVERFLOW ${id}: ${c.where.map((w) => overrun(list.findIndex(([k]) => k === id), w)).join('; ')}`);
console.log(`out/card-fit.json, ${((performance.now() - started) / 1000).toFixed(1)} s`);
// 264: no recorded exceptions: a box below the goal fails as an overflow does.
if (any.size) console.log(`BELOW ${GOAL_PT} pt: ${[...any].join(', ')}`);
process.exit(broken.length || any.size || spills.length ? 1 : 0);
