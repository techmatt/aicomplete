// `node fit.mjs`: measure where each frame really put its zones, into
// art/frames/r3/fit.json (mm boxes the renderer uses instead of the guide's). The master
// was painted, so its zones sit a little off the guide; every variant derives from it, so
// all of them should measure alike, and this prints where one doesn't. It seeds from
// layout.json's zones, inset as derive.mjs insets the frames. Each zone is light inside a
// darker rim: flood from the guide's center over the lightly blurred image (texture would
// stop it), take the bounding box. When no threshold gives a box of about the right size,
// the guide's stays. Also records the octagon's inside color, so the plates the renderer
// draws match it.
import { writeFile } from 'node:fs/promises';
import path from 'node:path';

import sharp from 'sharp';

import { ART, layout } from './lib.mjs';
import { inset } from '../../web/card/card.js';

const l = inset(await layout());
const mm = l.print_px[0] / l.bleed_mm[0];
// A pixel below this share of the seed's brightness ends the plate. Shiny rims leak at
// the lower ones, textured plates stop short at the higher: first size that fits wins.
const DARK = [0.92, 0.88, 0.84, 0.8, 0.74, 0.66];
const SIZE_SLACK = 2.8; // mm a plate may differ from the guide in width or height

const at = (W, x, y) => (y * W + x) * 3;

function flood(data, W, H, [cx, cy], [x0, y0, x1, y1], dark) {
  const sx = Math.round(cx * mm);
  const sy = Math.round(cy * mm);
  const lum = (i) => data[i] + data[i + 1] + data[i + 2];
  const floor = lum(at(W, sx, sy)) * dark;
  const lim = [Math.max(0, Math.floor((x0 - 3) * mm)), Math.max(0, Math.floor((y0 - 3) * mm)), Math.min(W - 1, Math.ceil((x1 + 3) * mm)), Math.min(H - 1, Math.ceil((y1 + 3) * mm))];
  const seen = new Uint8Array(W * H);
  const stack = [[sx, sy]];
  let [bx0, by0, bx1, by1] = [sx, sy, sx, sy];
  while (stack.length) {
    const [x, y] = stack.pop();
    if (x < lim[0] || y < lim[1] || x > lim[2] || y > lim[3] || seen[y * W + x]) continue;
    seen[y * W + x] = 1;
    const i = at(W, x, y);
    if (lum(i) < floor) continue;
    if (x < bx0) bx0 = x;
    if (x > bx1) bx1 = x;
    if (y < by0) by0 = y;
    if (y > by1) by1 = y;
    stack.push([x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]);
  }
  return [bx0 / mm, by0 / mm, (bx1 + 1) / mm, (by1 + 1) / mm];
}

const r2 = (b) => b.map((v) => Math.round(v * 100) / 100);

function measure(data, W, H, zone) {
  const seed = [(zone[0] + zone[2]) / 2, (zone[1] + zone[3]) / 2];
  for (const dark of DARK) {
    const b = flood(data, W, H, seed, zone, dark);
    if (Math.abs(b[2] - b[0] - (zone[2] - zone[0])) <= SIZE_SLACK && Math.abs(b[3] - b[1] - (zone[3] - zone[1])) <= SIZE_SLACK) return { box: r2(b), ok: true };
  }
  return { box: zone, ok: false };
}

const hex = (data, W, [x, y]) => {
  const i = at(W, Math.round(x * mm), Math.round(y * mm));
  return `#${[0, 1, 2].map((k) => data[i + k].toString(16).padStart(2, '0')).join('')}`;
};

const dir = path.join(ART, 'frames', 'r3');
const out = {};
const frames = Object.keys(l.kinds).sort().flatMap((k) => [`${k}-t1`, `${k}-t2`]);
for (const id of frames) {
  const { data, info } = await sharp(path.join(dir, `${id}.jpg`)).blur(1.2).removeAlpha().raw().toBuffer({ resolveWithObject: true });
  const { cost, banner } = l.common;
  const { top, art, effect } = l.kinds[id.split('-')[0]];
  const zones = { cost, banner, ...(top ? { top } : {}), art, effect };
  const m = Object.fromEntries(Object.entries(zones).map(([k, z]) => [k, measure(data, info.width, info.height, z)]));
  const missed = Object.entries(m).filter(([, v]) => !v.ok).map(([k]) => k);
  const [x0, y0, x1, y1] = m.cost.box;
  out[id] = { ...Object.fromEntries(Object.entries(m).map(([k, v]) => [k, v.box])), plate: hex(data, info.width, [(x0 + x1) / 2, (y0 + y1) / 2]) };
  console.log(`fit: ${id}: ${missed.length ? `guide kept for ${missed.join(', ')}` : 'all measured'}`);
}
const base = out['event-t1'];
for (const [id, zones] of Object.entries(out)) {
  const off = ['cost', 'banner', 'effect'].filter((k) => zones[k].some((v, i) => Math.abs(v - base[k][i]) > 0.05) && !(id.startsWith('building') && k === 'effect'));
  if (off.length) console.log(`fit: ${id} differs from event-t1: ${off.map((k) => `${k} ${zones[k]} vs ${base[k]}`).join('; ')}`);
}
await writeFile(path.join(dir, 'fit.json'), `${JSON.stringify(out, null, 1)}\n`);
