// 110: after `npm run scene-picker`, adds a row per reused scene to picks/scene_batch_6.json:
// the scene from the newest round that has it, over the deck card it now goes on, at 300 dpi.
// Previews go to art-originals/picks/scene_batch_6/reuse-<id>.jpg. Run from the repo root:
//   node experiments/results/110_missing_card_art/reuse_picker.mjs
import { readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { FRAMES, REPO, TOOL, cardPage, frameOf, frameUrl, layout, loadCards, pickerOriginals, readJson } from '../../../tools/card-art/lib.mjs';
import { inset, renderCard } from '../../../web/card/card.js';

const sharp = createRequire(path.join(TOOL, 'package.json'))('sharp');
os.setPriority(os.constants.priority.PRIORITY_BELOW_NORMAL);
sharp.concurrency(1);

const s = await readJson(path.join(TOOL, 'scenes.json'));
const shared = pickerOriginals('reuse-picker');
const images = path.join(shared, 'picks', s.picker);
const manifestFile = path.join(REPO, 'picks', `${s.picker}.json`);
const manifest = JSON.parse(await readFile(manifestFile, 'utf8'));
manifest.rows = manifest.rows.filter((r) => !r.id.startsWith('reuse-'));

const rounds = ['b6', 'b5', 'b4', 'b3', 'b2', 'b1'];
const cut = (id) => rounds.map((r) => path.join(shared, 'scenes', r, `${id}.png`)).find(existsSync);

// Reused this item: a scene with both `card` and `was` set by 110, or reassigned from a cut card.
const REUSED = JSON.parse(await readFile(path.join(REPO, 'experiments', 'results', '110_missing_card_art', 'reused.json'), 'utf8'));
const tagOf = (id) => Object.keys(s.scenes).find((tag) => s.scenes[tag][id]);

const l = inset(await layout());
const fits = await readJson(path.join(FRAMES, 'fit.json'));
const { cards, faced } = await loadCards();
const { p, show, clip, close } = await cardPage(l, 800, 1000);
const W = l.print_px[0] * 2 + 20;
let n = manifest.rows.length;
for (const [id, cardId] of Object.entries(REUSED)) {
  const file = cut(id);
  if (!file) throw new Error(`reuse-picker: no cut scene for ${id}`);
  const card = faced(cardId);
  const frame = frameOf(card);
  await show(renderCard(card, l, frameUrl(frame), fits[frame], pathToFileURL(file).href));
  const face = await p.screenshot({ clip });
  const meta = await sharp(file).metadata();
  const sceneW = W - l.print_px[0] - 20;
  const sceneH = Math.round((sceneW * meta.height) / meta.width);
  const H = Math.max(sceneH, l.print_px[1]);
  await sharp({ create: { width: W, height: H, channels: 3, background: '#1b1d20' } })
    .composite([
      { input: face, left: 0, top: 0 },
      { input: await sharp(file).resize(sceneW, sceneH, { kernel: 'lanczos3' }).toBuffer(), left: l.print_px[0] + 20, top: 0 },
    ])
    .jpeg({ quality: 86, mozjpeg: true })
    .toFile(path.join(images, `reuse-${id}.jpg`));
  const tag = tagOf(id);
  const sc = s.scenes[tag][id];
  manifest.rows.push({
    id: `reuse-${id}`,
    label: `${++n}. Reused, ${s.tags[tag].label}: ${sc.subject}`,
    note: `Drawn for ${sc.was ?? 'another card'}; now on ${cards[cardId].name}. Tile on: keep the reuse. Tile off: draw a new scene for ${cards[cardId].name}.`,
    kind: 'many',
    default: ['keep'],
    options: [{ key: 'keep', image: `../art-originals/picks/${s.picker}/reuse-${id}.jpg`, label: 'keep' }],
  });
  console.log(`reuse-picker: ${id} on ${cardId}`);
}
await close();
if (!manifest.intro.includes('Rows marked Reused')) manifest.intro += ` Rows marked Reused are scenes already drawn for a card that was cut or never used, now on a new card (110): the card on the left, the whole scene on the right.`;
await writeFile(manifestFile, `${JSON.stringify(manifest, null, 1)}\n`, { encoding: 'utf8' });
console.log(`reuse-picker: ${manifest.rows.length} rows in picks/${s.picker}.json`);
