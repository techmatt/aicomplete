// `npm run gen -- <direction>-<kind>-<seed>… [--pool=N]`: draw tier-1 frames.
// The image tool has no seed parameter: a "seed" is an independent draw, keyed 1, 2, …
// A redraw of `steel-program-1` replaces it; the earlier original stays as
// steel-program-1.try<n>.png. Originals and log.jsonl go to the originals folder
// (lib.mjs). Print-size PNGs go next to them in frames/; a web-size JPEG, the committed
// template, to art/frames/r3/. A draw is raw: `npm run derive` only reads master.jpg, so
// promoting one means copying it over the master.
// Stops at the first usage-limit message. Exits 1 if a draw failed or the limit left some
// undrawn.
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { ART, FRAMES, IMAGE_PREAMBLE, ORIGINALS, TOOL, centerBox, drawAll, layout, parsePool, readJson, runCodex } from './lib.mjs';

const argv = process.argv.slice(2);
const pool = parsePool(argv, 'gen');
const ids = argv.filter((a) => !a.startsWith('--'));
const p = await readJson(path.join(TOOL, 'prompts.json'));
for (const id of ids) {
  const [dir, kind] = id.split('-');
  if (!p.directions[dir]) throw new Error(`gen: ${id}: no direction ${dir}: nothing drawn`);
  if (!p.kinds[kind]) throw new Error(`gen: ${id}: no kind ${kind}: nothing drawn`);
}
const l = await layout();
const [W, H] = l.print_px;
const origDir = path.join(ORIGINALS, 'card-frames-r3');
const printDir = path.join(origDir, 'frames');
const webDir = FRAMES;
await mkdir(printDir, { recursive: true });
await mkdir(webDir, { recursive: true });

const framePrompt = (dir, kind) =>
  [
    ...IMAGE_PREAMBLE,
    'Image shape: portrait, width to height 69:94, the attached guide\'s shape.',
    '',
    p.frame.replace('{top}', kind === 'building' ? p.top : '').replace('{kind}', p.kinds[kind].color) + p.directions[dir].style,
  ].join('\n');

/** Crop centered to the bleed's shape, then scale: the kept original is untouched. */
async function cut(src, id) {
  const m = await sharp(src).metadata();
  const img = sharp(src).extract(centerBox(m, W / H)).resize(W, H, { kernel: 'lanczos3' });
  await img.clone().png().toFile(path.join(printDir, `${id}.png`));
  await img.clone().jpeg({ quality: 88, mozjpeg: true }).toFile(path.join(webDir, `${id}.jpg`));
  return { w: m.width, h: m.height };
}

await drawAll(ids, pool, 'gen', {
  dir: origDir,
  async draw(id) {
    const [dir, kind] = id.split('-');
    const guide = path.join(ART, 'guides', `${kind}-t1-blocks.png`);
    let run = await runCodex(framePrompt(dir, kind), guide);
    let ref = !!run.image;
    if (!run.image && !run.limit) {
      console.error(`gen: ${id}: no image with the guide (${run.message ?? '?'}); retrying with text alone`);
      run = await runCodex(framePrompt(dir, kind));
      ref = false;
    }
    return { run, ref };
  },
  async kept(id, dest, run, { ref }) {
    const s = await cut(await readFile(dest), id);
    console.log(`gen: ${id}: ${s.w}×${s.h}, ${run.seconds}s, ref ${ref}, usage ${JSON.stringify(run.usage)}`);
  },
});
