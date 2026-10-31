// `npm run fonts`: the card font picker. For each choice below, one sheet of the same eight
// cards at true size (300 dpi, trimmed) in art-originals/picks/card_fonts_r1/, and two counts
// over every player card, in picks/card_fonts_r1.json. Stops if the folder isn't reachable
// (lib.mjs). `--measure` prints the counts and writes nothing.
//
// A choice never changes a size: the sheets print each card at the size it prints today. The
// counts ask what that size would be if it were chosen for the font: renderCard's ladder is
// an estimate from character counts and cannot see a font, so here each size is tried on
// the real text, and a card counts if any of its rules text needs less than the default.
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import sharp from 'sharp';

import { CARD, FRAMES, MM, ORIGINALS, OUT, REPO, cardPage, frameOf, frameUrl, layout, pickerOriginals, readJson } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';
import { plainFace } from '../../web/cards/plain.js';
import { exportCards } from '../../web/scripts/cards-export.mjs';

os.setPriority(os.constants.priority.PRIORITY_BELOW_NORMAL);

const NAME = 'card_fonts_r1';
const SHEET = ['recursive_self_improvement', 'materials_hedge_desk', 'agency_directorate', 'development_loan', 'cryo_compute_burn', 'beamed_power_constellation', 'blackout', 'blight'];
// The clearance renderCard's ladder keeps in a panel, 1.6 mm in all, shared top and bottom.
const MARGIN_PX = 0.8 * MM;
const SCENE_ROUNDS = ['b4', 'b3', 'b2', 'b1'];
const SAIRA = { name: 'Saira Condensed', faces: ['600 10pt "Saira Condensed"', '700 10pt "Saira Condensed"'] };
const SERIF = { name: 'Source Serif 4', faces: ['400 10pt "Source Serif 4"'] };
// `key` is the picker's key; `cls` the block in web/card/fonts.css, none for today's.
const CHOICES = [
  { key: 'current', label: 'Current', title: SAIRA, body: SERIF },
  { key: 'all_serif', label: 'All serif', cls: 'all-serif', title: { name: 'Source Serif 4 display semibold', faces: ['600 10pt "Source Serif Var"'] }, body: SERIF },
  { key: 'sturdy_serif', label: 'Sturdier serif', cls: 'sturdy-serif', title: SAIRA, body: { name: 'Source Serif 4 caption optical size', faces: ['400 10pt "Source Serif Var"'] } },
  { key: 'slab', label: 'Slab bridge', cls: 'zilla-slab', title: SAIRA, body: { name: 'Zilla Slab medium', faces: ['500 10pt "Zilla Slab"'] } },
  { key: 'barlow_title', label: 'Matched condensed title', cls: 'barlow-title', title: { name: 'Barlow Condensed', faces: ['600 10pt "Barlow Condensed"', '700 10pt "Barlow Condensed"'] }, body: SERIF },
  { key: 'barlow_body', label: 'Sans body, condensed', cls: 'barlow-body', title: SAIRA, body: { name: 'Barlow Semi Condensed medium', faces: ['500 10pt "Barlow Semi Condensed"'] } },
  { key: 'source_sans', label: 'Sans body', cls: 'source-sans', title: SAIRA, body: { name: 'Source Sans 3', faces: ['400 10pt "Source Sans 3"'] } },
];

const measureOnly = process.argv.includes('--measure');
const images = measureOnly ? null : path.join(pickerOriginals('fonts'), 'picks', NAME);
if (images) await mkdir(images, { recursive: true });
await mkdir(OUT, { recursive: true });

const l = inset(await layout());
const fits = await readJson(path.join(FRAMES, 'fit.json'));
const data = exportCards(REPO);
const players = data.cards.filter((c) => c.render);
const byId = Object.fromEntries(data.cards.map((c) => [c.id, c]));

const sceneUrl = (id) => {
  const f = SCENE_ROUNDS.map((r) => path.join(ORIGINALS, 'scenes', r, `${id}.png`)).find(existsSync);
  if (!f) throw new Error(`fonts: no scene ${id}`);
  return pathToFileURL(f).href;
};
const face = (c, cls) => {
  const url = c.scene ? sceneUrl(c.scene) : undefined;
  if (!c.render) return plainFace(c, c.kind, url, cls);
  const f = frameOf(c);
  return `<div class="trim">${renderCard(c.render, l, frameUrl(f), fits[f], url, cls)}</div>`;
};

const FONTS_CSS = await readFile(path.join(CARD, 'fonts.css'), 'utf8');
const VIEWER_CSS = await readFile(path.join(REPO, 'web', 'cards', 'viewer.css'), 'utf8');
const { p, show, close } = await cardPage(l, 1140, 900);

/** What fonts this choice needs, which of them failed, and any character they don't have. */
const fontProblems = (choice) =>
  p.evaluate(
    async ({ faces, text }) => {
      const bad = [];
      await Promise.all(faces.map((f) => document.fonts.load(f, 'A')));
      const ctx = document.createElement('canvas').getContext('2d');
      const width = (font, ch) => {
        ctx.font = font;
        return ctx.measureText(ch).width;
      };
      for (const f of faces) {
        if (!document.fonts.check(f)) bad.push(`font ${f}`);
        // A character the font lacks falls through, so it moves with the fallback.
        const missing = [...new Set(text)].filter((ch) => ch.trim() && width(`${f}, monospace`, ch) !== width(`${f}, serif`, ch));
        if (missing.length) bad.push(`${f} lacks ${missing.join(' ')}`);
      }
      return bad;
    },
    { faces: [...choice.title.faces, ...choice.body.faces], text: players.map((c) => `${c.name}${c.text}${c.cost}`).join('') },
  );

/** Every player card on one page: the ladder tried on the real text, titles on one line. */
const measure = () =>
  p.evaluate((MARGIN) => {
    const CLASSES = ['fit-0', 'fit-1', 'fit-2', 'fit-3'];
    const TOLERANCE = 0.5;
    const overflows = (box, margin = -TOLERANCE) => {
      const b = box.getBoundingClientRect();
      return [...box.children].some((c) => {
        const r = c.getBoundingClientRect();
        return r.top < b.top + margin || r.bottom > b.bottom - margin;
      });
    };
    const cards = [];
    for (const card of document.querySelectorAll('[data-id]')) {
      const row = { id: card.dataset.id, needs: 0, overflow: false, printedBelow: false, title: 0, room: 0 };
      for (const box of card.querySelectorAll('.effect')) {
        const rules = box.querySelector('.rules');
        if (!rules) continue;
        if (overflows(box)) row.overflow = true;
        const printed = rules.className;
        const printedDense = box.classList.contains('dense');
        if (!/fit-0/.test(printed)) row.printedBelow = true;
        let step = 0;
        for (; step < CLASSES.length; step++) {
          rules.className = `rules ${CLASSES[step]}`;
          box.classList.remove('dense');
          if (!overflows(box, MARGIN)) break;
        }
        row.needs = Math.max(row.needs, step);
        rules.className = printed;
        box.classList.toggle('dense', printedDense);
      }
      const name = card.querySelector('.name');
      const span = name.querySelector('span');
      span.style.whiteSpace = 'nowrap';
      const r = document.createRange();
      r.selectNodeContents(span);
      row.title = r.getBoundingClientRect().width;
      row.room = name.getBoundingClientRect().width;
      span.style.whiteSpace = '';
      cards.push(row);
    }
    return cards;
  }, MARGIN_PX);

const results = {};
for (const choice of CHOICES) {
  const extra = `${FONTS_CSS}${VIEWER_CSS}body{background:#fff}`;
  await show(players.map((c) => { const f = frameOf(c); return `<div data-id="${c.id}">${renderCard(c.render, l, frameUrl(f), fits[f], undefined, choice.cls)}</div>`; }).join(''), extra);
  const problems = await fontProblems(choice);
  if (problems.length) throw new Error(`fonts: ${choice.key}: ${problems.join('; ')}`);
  const rows = await measure();
  const over = rows.filter((r) => r.title > r.room);
  results[choice.key] = {
    below: rows.filter((r) => r.needs > 0).map((r) => r.id),
    printedBelow: rows.filter((r) => r.printedBelow).length,
    overflow: rows.filter((r) => r.overflow).map((r) => r.id),
    atSmallest: rows.filter((r) => r.needs >= 4).map((r) => r.id),
    titles: over.map((r) => r.id),
    tightest: Math.min(...rows.map((r) => (r.room - r.title) / MM)),
  };
  const v = results[choice.key];
  console.log(`${choice.key.padEnd(14)} below ${String(v.below.length).padStart(3)}  panel overflow at today's size ${String(v.overflow.length).padStart(3)}  fit nowhere ${String(v.atSmallest.length).padStart(3)}  titles over ${String(v.titles.length).padStart(3)}  tightest title ${v.tightest.toFixed(2)} mm  (ladder today ${v.printedBelow})`);
}

if (measureOnly) {
  await writeFile(path.join(OUT, 'fonts-measure.json'), JSON.stringify(results, null, 1));
  await close();
  process.exit(0);
}

const count = (n, what) => `${n} ${what}${n === 1 ? '' : 's'}`;
const caption = (choice, n) => `${n}. ${choice.label}: ${choice.title.name} titles, ${choice.body.name} body`;
const stats = (v) => `${count(v.below.length, 'card')} of ${players.length} below 7.5 pt, ${count(v.titles.length, 'title')} over the plate${v.overflow.length ? `, ${count(v.overflow.length, 'panel')} overflowing at today's size` : ''}`;

const options = [];
for (const [i, choice] of CHOICES.entries()) {
  const v = results[choice.key];
  const body = `<div class="fonts-sheet"><p>${caption(choice, i + 1)}<br>${stats(v)}</p><div class="grid">${SHEET.map((id) => face(byId[id], choice.cls)).join('')}</div></div>`;
  await show(body, `${FONTS_CSS}${VIEWER_CSS}body{background:#fff}.fonts-sheet{padding:4mm;width:max-content}.fonts-sheet>p{margin:0 0 3mm;font:10pt/1.3 Arial,sans-serif;color:#222}.fonts-sheet .grid{display:grid;grid-template-columns:repeat(4,63mm);grid-auto-rows:88mm;gap:3mm}.trim,.plain{box-shadow:none}`);
  await p.evaluate(() => Promise.all([...document.images].map((im) => { im.loading = 'eager'; return im.decode().catch(() => {}); })));
  const png = path.join(OUT, `fonts-${choice.key}.png`);
  await (await p.$('.fonts-sheet')).screenshot({ path: png });
  await sharp(png).jpeg({ quality: 93, chromaSubsampling: '4:4:4', mozjpeg: true }).toFile(path.join(images, `${choice.key}.jpg`));
  options.push({ key: choice.key, image: `../art-originals/picks/${NAME}/${choice.key}.jpg`, label: `${choice.label}: ${choice.title.name} / ${choice.body.name}. ${stats(v)}` });
}
await close();

await writeFile(
  path.join(REPO, 'picks', `${NAME}.json`),
  `${JSON.stringify(
    {
      name: NAME,
      title: 'Card fonts, round 1',
      intro: `The same eight cards in each pairing, trimmed, at 300 dpi: print a sheet at 300 dpi for true size. Body text stays at the size each card prints today (default 7.5 pt) in every sheet. Below 7.5 pt: how many of the ${players.length} player cards would need a smaller body size in that font, tried on the real text of the whole set; ${results.current.printedBelow} print smaller today, because today's sizing is a character-count estimate. Over the plate: titles wider than their banner on one line. Figures (cost, insight, numbers on icon lines) stay Saira Condensed in every sheet. All serif sets titles at 13.5 pt, not 15: at 15 pt, 16 titles were over the plate. The threat and Discord card have no frame yet, so they show the viewer's plain face.`,
      rows: [{ id: 'favorite', label: 'Your favorite', kind: 'one', note: 'Use the note for what to change, a mix of two pairings included.', options }],
    },
    null,
    1,
  )}\n`,
);
console.log(`fonts: ${options.length} sheets in art-originals/picks/${NAME}/, picks/${NAME}.json`);
