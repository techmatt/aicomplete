// `npm run scene-picker`, after `npm run scene-gen`: everything below the raw draws, and
// picks/<picker>.json. Nothing it writes but the manifest is committed. Runs at
// below-normal priority, Chrome with it, one image at a time: the box is shared.
// Each draw is center-cut to the program window's shape, <id>.png beside raw/; building
// windows center-crop that again. Then its two sample cards (scenes.json) at 300 dpi in
// cards/, the same cards at true size in cards.pdf (9 to an A4 page), and one preview per
// draw, scene over cards, in art-originals/picks/<picker>/. Reads and writes the originals
// folder (lib.mjs). Stops, before any work, if that folder isn't reachable or a draw is
// missing. With --one (`npm run art:alts`), a pick-one row per card the batch was written for
// (a scene's `card`, else its `for`), one option per scene, keyed by scene id, none selected.
import { mkdir, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import sharp from 'sharp';

import { FRAMES, REPO, SHEET_CSS, TOOL, cardPage, centerBox, frameOf, frameUrl, layout, loadCards, pickerOriginals, readJson, sheetPages } from './lib.mjs';
import { inset, renderCard } from '../../web/card/card.js';

os.setPriority(os.constants.priority.PRIORITY_BELOW_NORMAL);
sharp.concurrency(1);

const s = await readJson(path.join(TOOL, 'scenes.json'));
const ONE = process.argv.includes('--one');
const NAME = s.picker;
const shared = pickerOriginals('scene-picker');
const dir = path.join(shared, 'scenes', s.round);
const images = path.join(shared, 'picks', NAME);
const missing = Object.keys(s.batch).filter((id) => !existsSync(path.join(dir, 'raw', `${id}.png`)));
if (missing.length) throw new Error(`scene-picker: no raw draw in ${path.join(dir, 'raw')} for ${missing.join(', ')}: no picker written`);
await mkdir(path.join(dir, 'cards'), { recursive: true });
await mkdir(images, { recursive: true });

const l = inset(await layout());
const fits = await readJson(path.join(FRAMES, 'fit.json'));
const { cards, faced } = await loadCards();

// The tier-1 program window, every draw's one shape.
const [wx0, wy0, wx1, wy1] = fits['program-t1'].art;
const SHAPE = (wx1 - wx0) / (wy1 - wy0);
async function cut(id) {
  const src = path.join(dir, 'raw', `${id}.png`);
  const box = centerBox(await sharp(src).metadata(), SHAPE);
  await sharp(src).extract(box).png().toFile(path.join(dir, `${id}.png`));
  return box;
}

const { p, show, clip, close } = await cardPage(l, 800, 1000);
const face = (cardId, id) => {
  const card = faced(cardId);
  const frame = frameOf(card);
  return renderCard(card, l, frameUrl(frame), fits[frame], pathToFileURL(path.join(dir, `${id}.png`)).href);
};

// The name of the card a scene was written for, which may be no player card.
const others = Object.fromEntries((await Promise.all(['adversary', 'alignment', 'threats'].map((f) => readJson(path.join(REPO, 'content', `${f}.json`))))).flat().map((c) => [c.id, c]));
const written = (id) => (cards[id] ?? others[id]).name;
const target = (scene) => scene.card ?? scene.for;

const SHEET_W = l.print_px[0] * 2 + 20;
const rows = [];
const all = [];
const tagOf = (id) => Object.keys(s.scenes).find((tag) => s.scenes[tag][id]);
const frameless = Object.keys(s.batch).some((id) => ['adversary', 'discord', 'unity'].includes(tagOf(id)));
for (const [id, pair] of Object.entries(s.batch)) {
  const size = await cut(id);
  for (const c of pair) {
    await show(face(c, id));
    await p.screenshot({ path: path.join(dir, 'cards', `${id}--${c}.png`), clip });
    all.push(face(c, id));
  }
  const sceneH = Math.round((SHEET_W * size.height) / size.width);
  const [W, H] = [SHEET_W, sceneH + 20 + l.print_px[1]];
  const left = await sharp(path.join(dir, 'cards', `${id}--${pair[0]}.png`)).toBuffer();
  const right = await sharp(path.join(dir, 'cards', `${id}--${pair[1]}.png`)).toBuffer();
  const top = await sharp(path.join(dir, `${id}.png`)).resize(W, sceneH, { kernel: 'lanczos3' }).toBuffer();
  await sharp({ create: { width: W, height: H, channels: 3, background: '#1b1d20' } })
    .composite([
      { input: top, left: 0, top: 0 },
      { input: left, left: 0, top: sceneH + 20 },
      { input: right, left: l.print_px[0] + 20, top: sceneH + 20 },
    ])
    .jpeg({ quality: 86, mozjpeg: true })
    .toFile(path.join(images, `${id}.jpg`));
  const tag = tagOf(id);
  const scene = s.scenes[tag][id];
  if (ONE) {
    const card = target(scene);
    let row = rows.find((r) => r.id === card);
    if (!row) {
      row = { id: card, label: `${rows.length + 1}. ${written(card)}`, note: '', kind: 'one', options: [] };
      rows.push(row);
    }
    const n = row.options.length + 1;
    row.options.push({ key: id, image: `../art-originals/picks/${NAME}/${id}.jpg`, label: `${n}. ${id}` });
    row.note += `${row.note ? ' ' : `${s.tags[tag].label}. Each tile is the scene over ${pair.map((c) => cards[c].name).join(' and ')}. `}${n}. ${scene.subject}`;
    continue;
  }
  rows.push({
    id,
    label: `${rows.length + 1}. ${s.tags[tag].label}: ${scene.subject}`,
    note: `Written for ${written(target(scene))}. Shown on ${pair.map((c) => cards[c].name).join(' and ')}. Tile on: keep. Tile off: reject.`,
    kind: 'many',
    default: ['keep'],
    options: [{ key: 'keep', image: `../art-originals/picks/${NAME}/${id}.jpg`, label: 'keep' }],
  });
}

const { body, pages } = sheetPages(all);
await show(body, SHEET_CSS);
await p.pdf({ path: path.join(dir, 'cards.pdf'), printBackground: true, preferCSSPageSize: true });
await close();

await writeFile(
  path.join(REPO, 'picks', `${NAME}.json`),
  `${JSON.stringify(
    {
      name: NAME,
      title: ONE ? `Card scene candidates, batch ${s.round.slice(1)}` : `Card scenes, batch ${s.round.slice(1)}`,
      intro: ONE
        ? `Pick one scene for each card, in the C and E mix. Each tile is the bare scene over two real cards, the event and the building, composed at 300 dpi with the full bleed showing: the program window is the whole scene, the building window its center. Nothing is selected to start. For true size, print art-originals/scenes/${s.round}/cards.pdf at 100%. Use the notes for what to change.`
        : `Keep or reject each scene, in the C and E mix. Each tile is the bare scene over two real cards, a program or event and a building, composed at 300 dpi with the full bleed showing: the program window is the whole scene, the building window its center. Scenes are a pool: the card a scene was written for is a suggestion. The second card only shows the other window${frameless ? ', and adversary, Discord and Unity cards have no frame yet, so their scenes sit on the nearest fit by name' : ''}. For true size, print art-originals/scenes/${s.round}/cards.pdf at 100%. Use the notes for what to change.`,
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`scene-picker: ${rows.length} rows in picks/${NAME}.json, ${all.length} cards, ${pages} pdf pages`);
