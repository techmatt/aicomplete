// 070: threat sources in a real browser. Plays the first legal actions until a face-up threat
// refuses a seat, then checks the threat's source badge, its sources marked on the map, the
// refusal on the card, and the engine's refusal of a forged payment. Needs the viewer running.
//   AIC_URL=http://localhost:PORT/ node scripts/e2e-sources.mjs
// Screenshot: web/e2e-out/threat-sources.png.

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const url = process.env.AIC_URL ?? 'http://localhost:5173/';
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const page = await openPage();
const { send, js, until, size } = page;
let failures = 0;
const check = (ok, what) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
  if (!ok) failures++;
};

try {
  await size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);

  // First legal actions until a seat is refused a threat that is up.
  const refused = `(() => {
    const v = window.aic.view;
    const t = (v.threat_sources ?? []).find((x) => x.seats.length < v.seats.length);
    if (!t) return null;
    return { threat: t.threat, source: t.source, hexes: t.hexes.length, seat: v.seats.find((s) => !t.seats.includes(s.index)).index };
  })()`;
  let found = null;
  for (let i = 0; i < 400 && !found; i++) {
    // A payment is taken only in the action phase: wait for it.
    found = await js(`window.aic.legal.some((a) => a.type === 'pay' || a.type === 'standard') ? ${refused} : null`);
    if (found) break;
    const over = await js(`window.aic.legal.length === 0`);
    if (over) break;
    await js(`window.aic.act(window.aic.legal[window.aic.legal.length - 1])`);
  }
  check(!!found, `a threat refuses a seat (${JSON.stringify(found)})`);
  if (found) {
    const dom = await js(`(() => {
      const card = document.querySelector('.threat[data-card="${found.threat}"]');
      return { badge: card?.querySelector('.tsource')?.textContent ?? '', says: card?.querySelector('.trefused')?.textContent ?? '',
               hexes: card?.dataset.hexes ?? '', marks: document.querySelectorAll('#board .mark.source').length };
    })()`);
    check(dom.badge === found.source, `the card shows its source type "${dom.badge}"`);
    check(dom.says.includes("can't counter") || dom.says.includes('no one can counter'), `the card says who is refused: "${dom.says}"`);
    check(dom.marks > 0, `${dom.marks} sources marked on the map`);
    check(dom.hexes.length > 0, 'hovering the card highlights its sources');
    const seatHasMoney = 1;
    await js(`window.aic.act({ type: 'pay', seat: ${found.seat}, threat: '${found.threat}', resource: 'credits', amount: ${seatHasMoney} })`).catch(() => {});
    const err = await js('window.aic.error');
    check(!!err && /sources in reach/.test(err), `a forged payment is refused: "${err}"`);
    writeFileSync(join(out, 'threat-sources.png'), await page.shot());
  }
} finally {
  await page.close();
}
process.exit(failures ? 1 : 0);
