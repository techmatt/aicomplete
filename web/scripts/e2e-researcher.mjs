// `npm run e2e:researcher` (075): the Researcher's board in a real browser. Plays the baseline at
// 3 countries with `station_cubes` and `owned_station_cubes` (142: a claimed station's) 4, so Onyx's 8-cube card fills in round 2 (184: at 3 it filled in round 3 and round 4 held no cubes): seat 2, the Researcher, builds
// a station in Onyx (262: no Claim action; neutral stations make nothing here) and passes its turns. Checks its tracks and powers on the page, Patience's
// carried insight and the stored research cubes, and spends Peer Review's insight (its price from content; 252: insight,
// not cubes) on it, leaving the cubes stored.
// Needs the viewer running.
//   AIC_URL  viewer URL (default http://localhost:5173/)
// Screenshot: web/e2e-out/researcher.png.

import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checker, openPage, playConfig, toRound } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const SEAT = 2;
const config = JSON.parse(readFileSync(join(web, '..', 'configs', 'baseline.json'), 'utf8'));
config.countries = 3;
// 262 cut the Claim action: seat 2 builds its station instead (`station_chips` 0, free), beside Onyx's neutral
// station, so Onyx's card fills by round 2 and seat 2's station stores its cubes from then.
Object.assign(config.modules.find((m) => m.module === 'board').params, { station_cubes: 4, owned_station_cubes: 4, troop_steps: 0, station_chips: 0 });
// Nothing diverts a cube or takes the bound city while the script passes its turns.
// No agents: a creation line would land one on the claimed station, blocking its cubes.
const adversary = config.modules.find((m) => m.module === 'adversary').params;
Object.assign(adversary, { corrupt_at: 20, corrupt_station_at: 20, corrupt_factory_at: 20, supply_per_country: 0 });
// 293: the corrupted stations' cubes (at this script's `station_cubes`) would run the adversary's research row to
// its loss slot by round 2: a slot costs 30 cubes a country (the param's most) instead.
if ('research_at' in adversary) adversary.research_at = Array.isArray(adversary.research_at) ? adversary.research_at.map(() => 30) : 30;

const page = await openPage();
const { send, js, until, size, consoleErrors } = page;
const { check, failed } = checker();

const value = (key) => js(`(window.aic.view.seats[${SEAT}].values.find((x) => x.key === '${key}'))?.value ?? 0`);
const panel = `[...document.querySelectorAll('.seat')][${SEAT}]`;

try {
  await size(1366, 900);
  await playConfig(page, config);

  check(await toRound(page, 1), 'reached the first action phase');
  check(await js(`window.aic.view.seats[${SEAT}].board?.name ?? ''`) === 'Researcher', 'seat 2 plays the Researcher');
  check(await js(`${panel}.querySelectorAll('.board .track-row').length`) === 2, 'its two tracks are on the page');
  const marks = await js(`[...${panel}.querySelectorAll('.board .mark')].map((m) => m.textContent)`);
  check(marks.includes('free station') && marks.includes('T2 keep'), `the tracks show the free station and the keep upgrade: ${marks.join(', ')}`);
  check(marks.includes('Open Archive') && marks.includes('Open Science'), 'both capstones are on the tracks');
  const powers = await js(`window.aic.view.seats[${SEAT}].board.cards.map((c) => c.name)`);
  const ais = JSON.parse(readFileSync(join(web, '..', 'content', 'ais.json'), 'utf8'));
  const missing = ais.find((b) => b.id === 'researcher').powers.map((p) => p.name).filter((p) => !powers.includes(p));
  check(missing.length === 0, `its powers are all listed${missing.length ? `; missing ${missing}` : ''}`);
  const innate = await js(`window.aic.view.seats[${SEAT}].board.innate?.name ?? ''`);
  check(innate === 'Patience', 'its special rule is Patience');

  // Seat 2 builds a research station in its reach, in Onyx (262: the Claim action is cut; `station_chips` 0 here),
  // so the check asks for any station of country 2 now seat 2's.
  await js(`window.aic.act({ type: 'standard', seat: ${SEAT}, act: 'station' })`);
  for (let i = 0; i < 4 && await js(`window.aic.legal.some((a) => a.type === 'choose')`); i++) {
    await js(`(() => {
      const l = window.aic.legal.filter((a) => a.type === 'choose');
      window.aic.act(l.find((a) => a.option.startsWith('Onyx #')) ?? l[0]);
    })()`);
  }
  check(await js(`window.aic.view.hexes.some((h) => h.country === 2 && h.tokens.some((t) => t.kind === 'station') && h.tokens.some((t) => t.kind === 'cube:${SEAT}'))`), 'seat 2 built a station in Onyx');

  check(await toRound(page, 2), 'reached round 2');
  const carried = await value('economy.insight_carried');
  check(carried > 0, `Patience carried ${carried} insight into round 2`);
  const chip = await js(`${panel}.textContent`);
  check(chip.includes('insight carried'), 'the carried insight shows on its panel');
  const insight = await value('resource.insight');
  const income = await value('economy.income.insight');
  // Keeps in round 2's draft may have spent some before the action phase, all of the carried at r59 (145: this
  // failed on main since 142), so the carried amount above is Patience's check; here, no more than income plus it.
  check(insight >= income && insight <= carried + income, `insight ${insight}: at least its income ${income}, at most that plus the carried ${carried}`);
  // 184: the badge shows what next round adds: the income, plus any gift, less any insight lost next round.
  const next = income + await value('economy.insight_gift') - await value('economy.insight_debt');
  const badge = await js(`${panel}.querySelector('.res.r-insight .inc')?.textContent ?? ''`);
  check(badge === `${next >= 0 ? '+' : ''}${next}`, `its insight reads as an addition, not a reset: ${badge} (income ${income})`);

  // Round 3 stores cubes once the card has completed in round 2. 221: round 3, not 4: at r97 the passing seats
  // lose their last nodes in round 4.
  const at3 = await toRound(page, 3);
  check(at3, `reached round 3${at3 ? '' : `: round ${await js('window.aic.view.round')}, outcome ${await js('JSON.stringify(window.aic.view.outcome ?? null)')}`}`);
  const stored = await value('resource.research_cubes');
  check(stored >= 3, `the Researcher stores ${stored} research cubes`);
  const shown = await js(`${panel}.querySelector('.res.r-research_cubes')?.textContent ?? ''`);
  check(/research cubes\s*\d/.test(shown), `its panel shows them: "${shown.trim()}"`);
  await js(`${panel}.scrollIntoView()`);
  writeFileSync(join(out, 'researcher.png'), await page.shot());

  const k = await js(`window.aic.view.seats[${SEAT}].board.cards.filter((c) => c.id.includes('.power.')).findIndex((c) => c.name === 'Peer Review')`);
  // 252: Peer Review's price from content, in insight (Patience carries it).
  const peer = ais.find((b) => b.id === 'researcher').powers.find((p) => p.name === 'Peer Review');
  const price = (peer.cost.find(([r]) => r === 'insight') ?? [, 0])[1];
  // 261: `held`, not a second `insight` (a SyntaxError since 252: the lane never ran).
  const held = await value('resource.insight');
  check(held >= price, `the Researcher holds ${held} insight, Peer Review's ${price}`);
  check(await js(`window.aic.legal.some((a) => a.type === 'power' && a.seat === ${SEAT} && a.power === ${k})`), 'Peer Review, a Growth option, is on the menu');
  await js(`window.aic.act({ type: 'power', seat: ${SEAT}, power: ${k} })`);
  for (let i = 0; i < 4 && await js(`window.aic.legal.some((a) => a.type === 'choose')`); i++) {
    await js(`window.aic.act(window.aic.legal.filter((a) => a.type === 'choose')[0])`);
  }
  // 252: it spends insight, never the stored cubes (Matt's ruling 1).
  check(await value('resource.research_cubes') === stored, 'it spends no research cubes');
  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  check(false, 'the run stopped');
} finally {
  await page.close();
}
console.log(failed() ? `e2e:researcher FAILED (${failed()})` : 'e2e:researcher ok');
process.exit(failed() ? 1 : 0);
