// `npm run e2e:sources` (070): threat sources in a real browser. Plays the last legal action until a face-up threat
// refuses a seat, then checks the threat's source badge, its sources marked on the map, the
// refusal on the card, and the engine's refusal of a forged payment. Needs the viewer running.
//   AIC_URL  viewer URL (default http://localhost:5173/)
// Screenshot: web/e2e-out/threat-sources.png.

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checker, openPage, viewerUrl as url } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const page = await openPage();
const { send, js, until, size, consoleErrors } = page;
const { check, failed } = checker();

try {
  await size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);

  // The last legal action until a seat is refused a threat that is up.
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
    await js(`window.aic.act({ type: 'pay', seat: ${found.seat}, threat: '${found.threat}', resource: 'credits', amount: 1 })`);
    const err = await js('window.aic.error');
    // 138: a seat with no source in reach is also not near the threat, which the engine checks first.
    check(!!err && /sources in reach|only a player near the threat can pay/.test(err), `a forged payment is refused: "${err}"`);
    writeFileSync(join(out, 'threat-sources.png'), await page.shot());
  }
  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  check(false, 'the run stopped');
} finally {
  await page.close();
}
console.log(failed() ? `e2e:sources FAILED (${failed()})` : 'e2e:sources ok');
process.exit(failed() ? 1 : 0);
