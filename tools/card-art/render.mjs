// `npm run render -- [variants…] [--pdf]`: compose each frame variant in art/frames/r3/
// with its real card (samples.json: card from content/cards.json, face from the generated
// web/card/faces.json) into out/cards/<variant>.png at 300 dpi. With no variants named, also
// the splay, the icon sheet and a check of every card in content. Composed cards are build
// outputs, never committed. Fails if a font fell back, anything printed leaves the safe
// zone, a panel or title overflows or a requirement plate reaches the tags. `--pdf` also writes
// out/cards/<variant>.pdf at true size and, with no variants named, all-cards.pdf (every
// real card at true size, 9 to an A4 page) and splay.pdf.
import { mkdir } from 'node:fs/promises';
import path from 'node:path';

import { FRAMES, MM, OUT, SHEET_CSS, TOOL, cardPage, frameOf, frameUrl, layout, loadCards, readJson, sheetPages } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';
import { ICON_NAMES, icon, stacked, tagMedallion } from '../../web/card/icons.js';

const argv = process.argv.slice(2);
const pdf = argv.includes('--pdf');
const named = argv.filter((a) => !a.startsWith('--'));
const samples = await readJson(path.join(TOOL, 'samples.json'));
const ids = named.length ? named : Object.keys(samples.variants);
const l = inset(await layout());
const fits = await readJson(path.join(FRAMES, 'fit.json'));
const { cards, faced } = await loadCards();
await mkdir(OUT, { recursive: true });

const { p, show, clip, close } = await cardPage(l, 700, 900);

const inspect = () =>
  p.evaluate(({ safe, mm }) => {
    const bad = [];
    for (const f of ['600 10pt "Saira Condensed"', '700 10pt "Saira Condensed"', '400 10pt "Source Serif 4"'])
      if (!document.fonts.check(f)) bad.push(`font ${f}`);
    for (const el of document.querySelectorAll('[data-safe]')) {
      const r = document.createRange();
      r.selectNodeContents(el);
      // Text width from its glyph run, height from its line box: a range's height is the font's whole content area.
      const t = el.textContent.trim() ? r.getBoundingClientRect() : el.getBoundingClientRect();
      const e = el.getBoundingClientRect();
      const [x0, y0, x1, y1] = [t.left / mm, e.top / mm, t.right / mm, e.bottom / mm];
      if (x0 < safe.x0 - 0.01 || y0 < safe.y0 - 0.01 || x1 > safe.x1 + 0.01 || y1 > safe.y1 + 0.01) bad.push(`${el.className || el.textContent.slice(0, 20)} at ${[x0, y0, x1, y1].map((v) => v.toFixed(1))}`);
    }
    for (const box of document.querySelectorAll('.effect')) {
      const b = box.getBoundingClientRect();
      for (const c of box.children) {
        const r = c.getBoundingClientRect();
        if (r.top < b.top - 0.5 || r.bottom > b.bottom + 0.5) bad.push(`${box.className} overflows`);
      }
    }
    for (const name of document.querySelectorAll('.name'))
      if (name.firstElementChild.getBoundingClientRect().width > name.getBoundingClientRect().width + 0.5) bad.push(`title ${name.textContent} wider than its plate`);
    const plate = [...document.querySelectorAll('.requirement')].at(-1)?.getBoundingClientRect();
    const tag = document.querySelector('.tag')?.getBoundingClientRect();
    if (plate && tag && plate.right > tag.left) bad.push(`requirement reaches the tags at ${(plate.right / mm).toFixed(1)}`);
    return bad;
  }, { safe: l.safe, mm: MM });

let failed = 0;
const report = (id, check) => {
  if (!check.length) return;
  failed++;
  console.error(`render: ${id}: ${check.join('; ')}`);
};

for (const id of ids) {
  const card = faced(samples.variants[id]);
  if (frameOf(card) !== id) console.error(`render: ${id}: closest fit, ${card.id} is ${frameOf(card)}`);
  await show(renderCard(card, l, frameUrl(id), fits[id]));
  report(id, await inspect());
  await p.screenshot({ path: path.join(OUT, `${id}.png`), clip });
  if (pdf) await p.pdf({ path: path.join(OUT, `${id}.pdf`), width: `${l.bleed_mm[0]}mm`, height: `${l.bleed_mm[1]}mm`, printBackground: true });
}

if (!named.length) {
  // Every real card, whole.
  for (const c of Object.values(cards)) {
    await show(renderCard(faced(c.id), l, frameUrl(frameOf(c)), fits[frameOf(c)]));
    report(c.id, await inspect());
  }

  // The splay: trimmed, each newer card over the older's name, only its top strip showing.
  const stack = samples.splay.map(faced);
  const trim = (l.bleed_mm[0] - l.trim_mm[0]) / 2;
  const [tw, th] = l.trim_mm;
  const showing = fits['building-t1'].banner[1] - trim;
  const slot = (card, i) =>
    `<div class="slot-card" style="top:${i * showing}mm;width:${tw}mm;height:${th}mm"><div style="margin:${-trim}mm">${renderCard(card, l, frameUrl(frameOf(card)), fits[frameOf(card)])}</div></div>`;
  const height = th + (stack.length - 1) * showing;
  await show(`<div class="splay" style="width:${tw}mm;height:${height}mm;margin:2mm">${stack.map(slot).join('')}</div>`, `@page{size:${tw + 4}mm ${height + 4}mm;margin:0}body{background:#3a3f46}`);
  await p.screenshot({ path: path.join(OUT, 'splay.png'), clip: { x: 0, y: 0, width: (tw + 4) * MM, height: (height + 4) * MM } });
  if (pdf) await p.pdf({ path: path.join(OUT, 'splay.pdf'), printBackground: true, preferCSSPageSize: true });

  // Every real card at true size: 9 to an A4 page, bleeds touching, in content order.
  if (pdf) {
    const all = Object.values(cards);
    const { body, pages } = sheetPages(all.map((c) => renderCard(faced(c.id), l, frameUrl(frameOf(c)), fits[frameOf(c)])));
    await show(body, SHEET_CSS);
    await p.pdf({ path: path.join(OUT, 'all-cards.pdf'), printBackground: true, preferCSSPageSize: true });
    console.log(`render: all-cards.pdf, ${all.length} cards on ${pages} pages`);
  }

  // Every icon, for review.
  const set = (names) => names.map((n) => `<div class="cell">${n.startsWith('#') ? tagMedallion(n.slice(1)) : icon(n)}<small>${n.replace('#', '')}</small></div>`).join('');
  const sheet = `<div class="card sheet" style="--strip:#333;height:auto;width:120mm;background:#efeadc"><div class="row">${set(ICON_NAMES)}</div>
    <div class="row">${['<span class="income">', '<span class="adversary">', '<span class="other">', '<span class="yours">'].map((o) => `<div class="fx-line">${o}<span class="fx-num">2</span>${icon('credits')}</span></div>`).join('')}</div>
    <div class="row plates"><div class="requirement at-least"><span class="fx-num">2</span>${tagMedallion('power')}</div><div class="requirement at-most"><span class="fx-num">1</span>${tagMedallion('cyber')}</div><div class="requirement replaces">${stacked()}${tagMedallion('science')}</div></div></div>`;
  await show(sheet, '.sheet .row{display:flex;flex-wrap:wrap;gap:2mm;padding:2mm}.cell{width:17mm;display:flex;flex-direction:column;align-items:center;gap:.5mm}.cell small{font:6pt sans-serif}.cell .medallion{width:6.2mm;height:6.2mm}.plates>div{height:5.2mm}');
  await (await p.$('.sheet')).screenshot({ path: path.join(OUT, 'icons.png') });
}
await close();
console.log(`render: ${ids.length} variants in out/cards/${failed ? `, ${failed} with problems` : ''}`);
process.exit(failed ? 1 : 0);
