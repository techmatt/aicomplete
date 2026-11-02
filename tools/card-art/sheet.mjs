// `node sheet.mjs <out.png> <image>…`: a one-row inspection sheet, 390 px a tile.
import path from 'node:path';

import sharp from 'sharp';

import { layout } from './lib.mjs';

const [out, ...images] = process.argv.slice(2);
const W = 390;
const [printW, printH] = (await layout()).print_px;
const H = Math.round((W * printH) / printW);
const tiles = await Promise.all(images.map(async (f, i) => ({ input: await sharp(f).resize(W, H).toBuffer(), left: i * (W + 6), top: 0 })));
await sharp({ create: { width: images.length * (W + 6), height: H, channels: 3, background: '#fff' } })
  .composite(tiles)
  .png()
  .toFile(path.resolve(out));
