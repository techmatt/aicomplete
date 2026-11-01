// `npm run picker`: picks/card_frames_r3.json, with its images in the shared folder
// (art-originals/picks/card_frames_r3/), never committed: composed cards and the splay
// from out/cards/ (run `npm run render` first), and a copy of each bare frame. One row per
// variant, composed card beside the bare frame, then the splay and the icon set. Stops if
// the folder isn't reachable (lib.mjs).
import { copyFile, mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { ART, OUT, REPO, TOOL, pickerOriginals, readJson } from './lib.mjs';

const NAME = 'card_frames_r3';
const samples = await readJson(path.join(TOOL, 'samples.json'));
const cards = Object.fromEntries((await readJson(path.join(REPO, 'content', 'cards.json'))).map((c) => [c.id, c]));
const framesDir = path.join(ART, 'frames', 'r3');
const images = path.join(pickerOriginals('picker'), 'picks', NAME);
await mkdir(images, { recursive: true });
const url = (f) => `../art-originals/picks/${NAME}/${f}`;
const label = (v) => {
  const [kind, tier] = v.split('-');
  return `${kind[0].toUpperCase()}${kind.slice(1)}, tier ${tier.slice(1)}`;
};
const preview = (name, width) => sharp(path.join(OUT, `${name}.png`)).resize(width).jpeg({ quality: 86, mozjpeg: true }).toFile(path.join(images, `${name}.jpg`));

const rows = [];
for (const [v, id] of Object.entries(samples.variants)) {
  await preview(v, 600);
  await copyFile(path.join(framesDir, `${v}.jpg`), path.join(images, `${v}-frame.jpg`));
  rows.push({
    id: v,
    label: `${label(v)}: ${cards[id].name}`,
    options: [
      { key: 'composed', image: url(`${v}.jpg`), label: 'composed' },
      { key: 'frame', image: url(`${v}-frame.jpg`), label: 'bare frame' },
    ],
  });
}
await preview('splay', 600);
rows.push({ id: 'splay', label: `Splay, oldest to newest: ${samples.splay.map((id) => cards[id].name).join(', ')}`, options: [{ key: 'splay', image: url('splay.jpg') }] });
await preview('icons', 900);
rows.push({ id: 'icons', label: 'The icon set', note: 'Tag, resource, token and piece glyphs are vector over the one painted container sheet.', options: [{ key: 'icons', image: url('icons.jpg') }] });

await writeFile(
  path.join(REPO, 'picks', `${NAME}.json`),
  `${JSON.stringify(
    {
      name: NAME,
      title: 'Card frames, round 3: one master frame, six variants',
      intro: 'Every frame is steel-event-3 with its enamel recolored in code, tier 2 gilded, buildings split: borders and bevels are the same pixels. Each card is a real card from content; only the faces (icons and box-worded rules text) are written for this round. Art windows are empty on purpose. These images show the full bleed: the outer steel or gold band sits inside the 3 mm the guillotine cuts away, so the splay, which is trimmed, is what a cut card looks like. Use the notes for what to change.',
      rows,
    },
    null,
    1,
  )}\n`,
);
console.log(`picker: ${rows.length} rows in picks/${NAME}.json`);
