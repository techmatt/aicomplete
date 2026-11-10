// `npm run e2e:corps` (074): research cubes and the megacorps in a real browser. Plays the baseline
// with `station_cubes` and `owned_station_cubes` (142: a claimed station's) 5 so a card overflows in round 2 (137: adversary stations make compute, not cubes): seat 0 claims Horizon's neutral station,
// Cintamani's stays neutral. Checks the cubes by color, the completion with its overflow paid, the
// stored cubes and a usable converter beside an unusable one. Needs the viewer running.
//   AIC_URL  viewer URL (default http://localhost:5173/)
// Screenshot: web/e2e-out/corps.png.

import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checker, openPage, playConfig, toRound } from './cdp.mjs';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const out = join(web, 'e2e-out');
mkdirSync(out, { recursive: true });

const config = JSON.parse(readFileSync(join(web, '..', 'configs', 'baseline.json'), 'utf8'));
Object.assign(config.modules.find((m) => m.module === 'board').params, { station_cubes: 5, owned_station_cubes: 5, troop_steps: 0, claim_station: 1 });
// Nothing diverts a cube or takes the bound city while the script passes its turns.
// No agents: a creation line would land one on the claimed station, blocking its cubes.
Object.assign(config.modules.find((m) => m.module === 'adversary').params, { corrupt_at: 20, corrupt_station_at: 20, corrupt_factory_at: 20, corrupt_land_at: 20, supply_per_country: 0 });

const page = await openPage();
const { send, js, until, size, consoleErrors } = page;
const { check, failed } = checker();

const value = (where, key) => `(${where}.values.find((x) => x.key === '${key}'))`;
const resource = (r) => js(`${value('window.aic.view.seats[0]', `resource.${r}`)}?.value ?? 0`);

try {
  await size(1366, 768);
  await playConfig(page, config);

  check(await toRound(page, 1, 600), 'reached the first action phase');
  // Seat 0's starting ambassador claims the neutral station next to its node (137's Claim;
  // `claim_station` 1, so the one ambassador pays for it).
  const station = await js(`(() => {
    const v = window.aic.view;
    const h = v.hexes.find((x) => x.country === 0 && x.tokens.some((t) => t.kind === 'station') && !x.tokens.some((t) => t.kind === 'cube:adversary'));
    return h?.number;
  })()`);
  check(station === 10, `Horizon's neutral station stands on hex ${station}`);
  await js(`window.aic.act({ type: 'standard', seat: 0, act: 'claim' })`);
  for (let i = 0; i < 4 && await js(`window.aic.legal.some((a) => a.type === 'choose')`); i++) {
    await js(`(() => {
      const l = window.aic.legal.filter((a) => a.type === 'choose');
      window.aic.act(l.find((a) => a.option === 'Horizon #${station}' || a.option === 'from your supply') ?? l[0]);
    })()`);
  }
  const owned = await js(`window.aic.view.hexes.some((h) => h.country === 0 && h.number === ${station} && h.tokens.some((t) => t.kind === 'cube:0'))`);
  check(owned, 'the ambassador claimed the station');

  check(await toRound(page, 2, 600), 'reached round 2');
  const colors = await js(`(() => {
    const cells = [...document.querySelectorAll('#countries .project')].filter((p) => p.querySelector('.cubes'));
    const count = (p, c) => p.querySelectorAll('.pc[data-color="' + c + '"]').length;
    return { horizon: [count(cells[0], 'seat 0'), count(cells[0], 'adversary')], cintamani: [count(cells[1], 'neutral'), count(cells[1], 'adversary')] };
  })()`);
  check(colors.horizon[0] === 5 && colors.horizon[1] === 0, `Horizon's card shows its cubes by color: ${colors.horizon} (player, adversary)`);
  check(colors.cintamani[0] === 5 && colors.cintamani[1] === 0, `Cintamani's shows neutral cubes: ${colors.cintamani} (neutral, adversary)`);

  // Round 2 fills an 8-cube card past its size (the claimed station's 10, a neutral station's 5): every one pays.
  check(await toRound(page, 3, 600), 'reached round 3');
  const paid = await js(`[...document.querySelectorAll('#countries .ppaid')].map((e) => e.textContent)`);
  check(paid.length === 2 && parseInt(paid[0]) > 8 && /^\d+ cubes paid/.test(paid[0]) && paid[0].includes('player'), `the card completed with its overflow paid: "${paid[0]}"`);
  check(await js(`document.querySelectorAll('#countries .project.done').length`) === 2, 'both cards show completed');

  // Round 3 stores 5 cubes on seat 0's board; round 4 can spend them.
  check(await toRound(page, 4, 600), 'reached round 4');
  const stored = await resource('research_cubes');
  check(stored === 5, `seat 0 stores ${stored} research cubes`);
  const shown = await js(`document.querySelector('.seat .res.r-research_cubes')?.textContent ?? ''`);
  check(/research cubes\s*5/.test(shown), `its board shows them: "${shown.trim()}"`);
  // 099: since r37 a converter needs a node in a city of its country: seat 0 places
  // one on a Horizon city with a node power (answering each prompt toward it).
  const horizonCities = await js(`window.aic.view.hexes.filter((h) => h.country === 0 && h.kind === 'city').map((h) => 'Horizon #' + h.number)`);
  // The board's powers that place a node (their index is the id's last part).
  const placing = await js(`(window.aic.view.seats[0].board?.cards ?? []).filter((c) => c.id.includes('.power.') && /place/.test(c.text ?? '')).map((c) => Number(c.id.split('.').pop()))`);
  for (const k of placing) {
    if (await js(`window.aic.view.seats[0].values.some((x) => x.key === 'board.converter.0' && x.value > 0)`)) break;
    if (!(await js(`window.aic.legal.some((a) => a.type === 'power' && a.seat === 0 && a.power === ${k})`))) continue;
    await js(`window.aic.act({ type: 'power', seat: 0, power: ${k} })`);
    for (let i = 0; i < 6 && await js(`window.aic.legal.some((a) => a.type === 'choose')`); i++) {
      await js(`(() => {
        const l = window.aic.legal.filter((a) => a.type === 'choose');
        const cities = ${JSON.stringify(horizonCities)};
        window.aic.act(l.find((a) => cities.includes(a.option)) ?? l[0]);
      })()`);
    }
  }
  const conv = await js(`[...document.querySelectorAll('.seat')][0].querySelectorAll('.converter').length ? [...[...document.querySelectorAll('.seat')][0].querySelectorAll('.converter')].map((e) => [e.className, e.textContent]) : []`);
  check(conv.some(([c]) => c.includes(' usable')), `a usable converter: ${JSON.stringify(conv.find(([c]) => c.includes(' usable')))}`);
  check(conv.some(([c, t]) => c.includes('unusable') && /no node/.test(t)), `beside an unusable one: ${JSON.stringify(conv.find(([c]) => c.includes('unusable')))}`);
  writeFileSync(join(out, 'corps.png'), await page.shot());

  const anti = await resource('antimatter');
  check(await js(`window.aic.legal.some((a) => a.type === 'convert' && a.seat === 0 && a.country === 0)`), 'the converter is on the menu');
  await js(`window.aic.act({ type: 'convert', seat: 0, country: 0 })`);
  check(await resource('research_cubes') === stored - 1 && await resource('antimatter') === anti + 1, 'using it turns a cube and 2 energy into 1 antimatter');
  // 105: as often as you can pay (only Onyx's is once a round).
  const payable = await resource('research_cubes') >= 1 && await resource('energy') >= 2;
  check(await js(`window.aic.legal.some((a) => a.type === 'convert' && a.seat === 0 && a.country === 0)`) === payable, `offered again while it can be paid (${payable})`);
  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  check(false, 'the run stopped');
} finally {
  await page.close();
}
console.log(failed() ? `e2e:corps FAILED (${failed()})` : 'e2e:corps ok');
process.exit(failed() ? 1 : 0);
