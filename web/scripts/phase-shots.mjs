// 326: the player view's cards phases and first action phase as shots, for a before/after: a solo game (seed 24, the
// tutorial's, the Builder) from a fresh browser, the new-game screen closed, played by the first legal action of each
// kind (each tier a tier-1 card, each draft the first card, each keep the first offered), shot at the build, the
// draft (when the config has one), the keep and the action phase.
//
//   node scripts/phase-shots.mjs [--size 1920x1080] [--seed 24] [--config ../configs/solo.json] --out DIR
//
// Needs a served viewer (AIC_URL). Writes DIR/<size>-<n>-<phase>.png.
import { mkdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { openPage, shotTo, sleep, viewerUrl } from './cdp.mjs';

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const size = arg('size', '1920x1080');
const [width, height] = size.split('x').map(Number);
const seed = arg('seed', '24');
const configPath = resolve(arg('config', new URL('../../configs/solo.json', import.meta.url).pathname.replace(/^\/([A-Z]:)/, '$1')));
const out = resolve(arg('out', 'phase-shots'));
mkdirSync(out, { recursive: true });
const config = readFileSync(configPath, 'utf8');

const page = await openPage({ width, height, fresh: true });
try {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view', 'the viewer to start', 60000);
  await page.js(`(() => { const d = document.getElementById('new-dialog'); if (d?.open) d.close(); window.aic.tutorial?.stop?.(); })()`);
  await page.js(`window.aic.setView('player')`);
  await page.js(`window.aic.play(${JSON.stringify(config)}, '${seed}', 'solo.json')`);
  await page.until(`window.aic.timeline.seed === '${seed}'`, 'the game', 30000);
  const shot = new Set();
  let n = 0;
  const snap = async (name) => {
    if (shot.has(name)) return;
    shot.add(name);
    await sleep(900);
    await shotTo(page, out, `${size}-${++n}-${name}`);
    console.log('shot', name);
  };
  for (let i = 0; i < 400; i++) {
    const st = await page.js(`({ phase: aic.view.phase?.name ?? '', round: aic.view.round, types: aic.legal.map((a) => a.type), outcome: !!aic.view.outcome })`);
    if (st.outcome || !st.types.length) break;
    if (st.phase === 'cards.tier' && st.types.includes('tier')) {
      await snap('build');
      await page.js(`aic.act(aic.legal.find((a) => a.type === 'tier' && Number(a.tier) === 1) ?? aic.legal.find((a) => a.type === 'tier'))`);
      continue;
    }
    if (st.types.includes('draft')) {
      await snap('draft');
      await page.js(`aic.act(aic.legal.find((a) => a.type === 'draft'))`);
      continue;
    }
    if (st.phase === 'cards.keep') {
      await snap('keep');
      await page.js(`aic.act(aic.legal.find((a) => a.type === 'keep') ?? aic.legal.find((a) => a.type === 'pass'))`);
      continue;
    }
    if (/board\.act/.test(st.phase) && !(await page.js('!!aic.view.choice'))) {
      await snap('actions');
      break;
    }
    await page.js(`aic.act(aic.legal.find((a) => a.type === 'pass') ?? aic.legal[0])`);
  }
  console.log(`${n} shots in ${out}`);
} finally {
  await page.close();
}
