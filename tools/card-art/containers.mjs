// `npm run containers -- [--draw]`: the icon set's containers. --draw paints the one sheet
// with Codex's image tool, the master frame as style reference, into the originals folder
// (card-frames-r3/containers.png). Without it, slices the kept sheet into
// web/card/containers/*.png, keying out its flat background. A tile comes in one neutral
// steel-and-enamel blank; card.css colors it by resource, so the sheet is painted once.
import { copyFile, mkdir } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { CARD, FRAMES, IMAGE_PREAMBLE, ORIGINALS, TOOL, readJson, runCodex } from './lib.mjs';

const p = await readJson(path.join(TOOL, 'prompts.json'));
const origDir = path.join(ORIGINALS, 'card-frames-r3');
const sheet = path.join(origDir, 'containers.png');
await mkdir(origDir, { recursive: true });

if (process.argv.includes('--draw')) {
  const run = await runCodex(
    [...IMAGE_PREAMBLE, 'Image shape: landscape, 3:2.', '', p.containers].join('\n'),
    path.join(FRAMES, 'master.jpg'),
  );
  if (!run.image) throw new Error(`containers: no image. ${run.limit ?? run.message ?? ''}`);
  await copyFile(run.image, sheet);
  console.log(`containers: drawn in ${run.seconds}s, ${JSON.stringify(run.usage)}`);
  process.exit(0);
}

// Blobs on the flat background, top-left to bottom-right, are the sheet's containers; the
// sixth, an ivory disc, is spare.
const { data, info } = await sharp(sheet).removeAlpha().raw().toBuffer({ resolveWithObject: true });
const { width: W, height: H } = info;
// How magenta a pixel is: the sheet's background scores about 255, the objects about 0.
const key = (i) => Math.min(data[i], data[i + 2]) - data[i + 1];
const far = (i) => key(i) < 90;
const NAMES = ['tile', 'disc', 'ring', 'income', 'adversary'];
const seen = new Uint8Array(W * H);
const blobs = [];
for (let y = 0; y < H; y++) {
  for (let x = 0; x < W; x++) {
    if (seen[y * W + x] || !far((y * W + x) * 3)) continue;
    const stack = [[x, y]];
    let [x0, y0, x1, y1, n] = [x, y, x, y, 0];
    while (stack.length) {
      const [a, b] = stack.pop();
      if (a < 0 || b < 0 || a >= W || b >= H || seen[b * W + a] || !far((b * W + a) * 3)) continue;
      seen[b * W + a] = 1;
      n++;
      [x0, y0, x1, y1] = [Math.min(x0, a), Math.min(y0, b), Math.max(x1, a), Math.max(y1, b)];
      stack.push([a + 1, b], [a - 1, b], [a, b + 1], [a, b - 1]);
    }
    if (n > 2000) blobs.push({ x0, y0, x1, y1 });
  }
}
blobs.sort((a, b) => Math.round(a.y0 / (H / 2)) - Math.round(b.y0 / (H / 2)) || a.x0 - b.x0);
console.log(`containers: ${blobs.length} blobs`, blobs.map((b) => [b.x0, b.y0, b.x1 - b.x0, b.y1 - b.y0].join(',')).join(' '));
const dir = path.join(CARD, 'containers');
await mkdir(dir, { recursive: true });
const rgba = Buffer.alloc(W * H * 4);
for (let i = 0; i < W * H; i++) {
  const k = key(i * 3);
  const alpha = Math.min(1, Math.max(0, (150 - k) / 110));
  // An edge pixel keeps its green, the one channel the magenta didn't lift: no fringe.
  const pale = k > 25 ? data[i * 3 + 1] : -1;
  for (let c = 0; c < 3; c++) rgba[i * 4 + c] = pale < 0 ? data[i * 3 + c] : pale;
  rgba[i * 4 + 3] = Math.round(alpha * 255);
}
for (const [i, b] of blobs.entries()) {
  if (!NAMES[i]) continue;
  await sharp(rgba, { raw: { width: W, height: H, channels: 4 } })
    .extract({ left: b.x0, top: b.y0, width: b.x1 - b.x0 + 1, height: b.y1 - b.y0 + 1 })
    .resize(NAMES[i] === 'income' || NAMES[i] === 'adversary' ? 300 : 160)
    .png({ compressionLevel: 9 })
    .toFile(path.join(dir, `${NAMES[i]}.png`));
}
