// `npm run derive`: every frame variant from the one master, art/frames/r3/master.jpg
// (Matt's pick, steel-event-3), into art/frames/r3/<kind>-t<tier>.jpg. Nothing is redrawn,
// because independent draws drift by up to 3.5 mm: the steel stays the master's pixels.
// Numbers in derive.json. Four steps, in this order:
//   split:    buildings only. Restack the inside of the frame row by row, so master rows
//             also make a divider above the art. The outer border is not touched.
//   gold:     tier 2 only. Tint the steel to gold, leaving enamel and plates.
//   recolor: remap the enamel, found by hue, through the kind's color ramp.
//   inset:    scale the frame in so its steel lands inside the trim (layout.json's
//             inset_mm, 0.5 mm inside it), and fill the bleed by mirroring the frame's edge.
import path from 'node:path';

import sharp from 'sharp';

import { ART, TOOL, layout, readJson } from './lib.mjs';

const dir = path.join(ART, 'frames', 'r3');
const master = path.join(dir, 'master.jpg');

const cfg = await readJson(path.join(TOOL, 'derive.json'));
const l = await layout();
const [W, H] = l.print_px;

const smooth = (a, b, x) => {
  const t = Math.min(1, Math.max(0, (x - a) / (b - a)));
  return t * t * (3 - 2 * t);
};
const mix = (a, b, t) => a + (b - a) * t;

/** Hue in degrees, saturation 0..1, value 0..1. */
function hsv(r, g, b) {
  const max = Math.max(r, g, b);
  const c = max - Math.min(r, g, b);
  let h = 0;
  if (c > 0) h = max === r ? ((g - b) / c + 6) % 6 : max === g ? (b - r) / c + 2 : (r - g) / c + 4;
  return [h * 60, max ? c / max : 0, max / 255];
}

const inBox = (p) => {
  const [x0, y0, x1, y1] = cfg.enamel.box;
  const [x, y] = [p % W, Math.floor(p / W)];
  return x >= x0 && y >= y0 && x < x1 && y < y1;
};

/** The share of a pixel that is enamel. Steel and the plates score about 0. */
function enamel(r, g, b) {
  const [h, s] = hsv(r, g, b);
  const { hue: [a, b0, c, d], saturation: [s0, s1] } = cfg.enamel;
  return smooth(a, b0, h) * (1 - smooth(c, d, h)) * smooth(s0, s1, s);
}

function split(src) {
  const rows = new Float32Array(H);
  for (const [d0, d1, s0, s1] of cfg.split) for (let y = d0; y < d1; y++) rows[y] = mix(s0, s1, (y - d0) / (d1 - d0));
  const out = Buffer.from(src);
  const [xa, xb] = cfg.inside;
  for (let y = 0; y < H; y++) {
    const s0 = Math.floor(rows[y]);
    const s1 = Math.min(H - 1, s0 + 1);
    const f = rows[y] - s0;
    for (let x = xa; x < xb; x++) {
      const t = Math.min(1, (x - xa + 1) / cfg.feather, (xb - x) / cfg.feather);
      for (let k = 0; k < 3; k++) {
        const v = mix(src[(s0 * W + x) * 3 + k], src[(s1 * W + x) * 3 + k], f);
        out[(y * W + x) * 3 + k] = Math.round(mix(src[(y * W + x) * 3 + k], v, t));
      }
    }
  }
  return out;
}

/** The ramp through color stops, by value. */
function ramp(stops, v) {
  let i = 1;
  while (i < stops.length - 1 && v > stops[i][0]) i++;
  const [[v0, c0], [v1, c1]] = [stops[i - 1], stops[i]];
  const t = Math.min(1, Math.max(0, (v - v0) / (v1 - v0)));
  return [0, 1, 2].map((k) => mix(c0[k], c1[k], t));
}

function plates(kind) {
  const mask = new Uint8Array(W * H);
  for (const [x0, y0, x1, y1] of [...cfg.plates.common, ...cfg.plates[kind]]) for (let y = y0; y < y1; y++) mask.fill(1, y * W + x0, y * W + x1);
  const { center: [cx, cy], radius: r } = cfg.octagon;
  for (let y = cy - r; y <= cy + r; y++) {
    for (let x = cx - r; x <= cx + r; x++) if (Math.abs(x - cx) + Math.abs(y - cy) <= r * cfg.octagon.diamond) mask[y * W + x] = 1;
  }
  return mask;
}

function gild(src, kind) {
  const out = Buffer.from(src);
  const keep = plates(kind);
  for (let p = 0; p < W * H; p++) {
    if (keep[p]) continue;
    const i = p * 3;
    const m = inBox(p) ? 1 - enamel(src[i], src[i + 1], src[i + 2]) : 1;
    const c = ramp(cfg.gold, Math.max(src[i], src[i + 1], src[i + 2]) / 255);
    for (let k = 0; k < 3; k++) out[i + k] = Math.round(mix(src[i + k], c[k], m));
  }
  return out;
}

function recolor(src, stops) {
  if (!stops) return src;
  const out = Buffer.from(src);
  for (let p = 0; p < W * H; p++) {
    if (!inBox(p)) continue; // blue reflected in the outer steel
    const i = p * 3;
    const m = enamel(src[i], src[i + 1], src[i + 2]);
    if (!m) continue;
    const c = ramp(stops, Math.max(src[i], src[i + 1], src[i + 2]) / 255);
    for (let k = 0; k < 3; k++) out[i + k] = Math.round(mix(src[i + k], c[k], m));
  }
  return out;
}

/** The finished frame, scaled in, its bleed the steel band mirrored outward (twice, so the enamel never shows). */
async function inset(px) {
  const left = Math.round((l.inset_mm * W) / l.bleed_mm[0]);
  const top = Math.round((l.inset_mm * H) / l.bleed_mm[1]);
  const band = Math.floor((cfg.steel * (W - 2 * left)) / W);
  const mirror = (buf, n) => sharp(buf).extend({ left: n, right: n, top: n, bottom: n, extendWith: 'mirror' });
  const small = await sharp(px, { raw: { width: W, height: H, channels: 3 } })
    .resize(W - 2 * left, H - 2 * top, { fit: 'fill', kernel: 'lanczos3' })
    .png()
    .toBuffer();
  const once = await mirror(small, Math.min(band, left)).png().toBuffer();
  return mirror(once, left - Math.min(band, left));
}

const { data: base } = await sharp(master).removeAlpha().raw().toBuffer({ resolveWithObject: true });
for (const kind of Object.keys(l.kinds)) {
  for (const tier of [1, 2]) {
    let px = kind === 'building' ? split(base) : Buffer.from(base);
    if (tier === 2) px = gild(px, kind);
    px = recolor(px, cfg.ramps[kind]);
    await (await inset(px))
      .jpeg({ quality: 92, mozjpeg: true, chromaSubsampling: '4:4:4' })
      .toFile(path.join(dir, `${kind}-t${tier}.jpg`));
    console.log(`derive: ${kind}-t${tier}`);
  }
}
