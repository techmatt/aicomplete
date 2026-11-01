// `npm run guides`: draw a guide per variant from web/card/layout.json.
// art/guides/<kind>-t<tier>.png is the labeled one, for people. The unlabeled
// twin (<kind>-t<tier>-blocks.png) is what the image tool sees: labels invite text.
// Only the fixed structure is here: the renderer draws what varies by card.
import { mkdir } from 'node:fs/promises';
import path from 'node:path';

import { ART, layout, openPage } from './lib.mjs';

const ZONE_COLOR = {
  cost: '#e2574c',
  banner: '#f0a0c0',
  art: '#ffffff',
  effect: '#e8e4d8',
  top: '#d6e8c8',
};

function svg(l, kind, tier, labeled) {
  const [w, h] = l.bleed_mm;
  const zones = { strip: l.common.strip, banner: l.common.banner, ...l.kinds[kind], cost: l.common.cost };
  const box = ([x0, y0, x1, y1], fill) => `<rect x="${x0}" y="${y0}" width="${x1 - x0}" height="${y1 - y0}" fill="${fill}"/>`;
  const parts = [`<rect width="${w}" height="${h}" fill="#8f96a0"/>`, box(l.common.frame, l.kind_color[kind])];
  if (tier === 2) {
    const [x0, y0, x1, y1] = l.common.frame;
    parts.push(`<rect x="${x0 + 0.25}" y="${y0 + 0.25}" width="${x1 - x0 - 0.5}" height="${y1 - y0 - 0.5}" fill="none" stroke="#e0b83a" stroke-width="0.5"/>`);
  }
  for (const [name, r] of Object.entries(zones)) {
    if (name === 'cost') {
      const [x0, y0, x1, y1] = r;
      const c = (x1 - x0) * 0.29;
      parts.push(`<polygon points="${x0 + c},${y0} ${x1 - c},${y0} ${x1},${y0 + c} ${x1},${y1 - c} ${x1 - c},${y1} ${x0 + c},${y1} ${x0},${y1 - c} ${x0},${y0 + c}" fill="${ZONE_COLOR.cost}"/>`);
    } else parts.push(box(r, ZONE_COLOR[name] ?? l.kind_color[kind]));
    if (labeled) parts.push(`<text x="${(r[0] + r[2]) / 2}" y="${(r[1] + r[3]) / 2 + 0.7}" font-size="2" font-family="sans-serif" text-anchor="middle" fill="${name === 'strip' ? '#fff' : '#000'}">${name}</text>`);
  }
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${w} ${h}" width="100%" height="100%" style="display:block">${parts.join('')}</svg>`;
}

const l = await layout();
const [W, H] = l.print_px;
await mkdir(path.join(ART, 'guides'), { recursive: true });
const { page, close } = await openPage(W, H);
for (const kind of Object.keys(l.kinds))
  for (const tier of [1, 2])
    for (const labeled of [true, false]) {
      await page.setContent(`<!doctype html><body style="margin:0;width:${W}px;height:${H}px">${svg(l, kind, tier, labeled)}</body>`);
      await page.screenshot({ path: path.join(ART, 'guides', `${kind}-t${tier}${labeled ? '' : '-blocks'}.png`) });
    }
await close();
console.log('guides: 6 labeled, 6 unlabeled in art/guides/');
