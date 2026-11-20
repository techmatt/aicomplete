// `npm run e2e:corps` (074): research cubes and the megacorps in a real browser. Plays the baseline
// with `station_cubes` 2 and `owned_station_cubes` (142: a claimed station's) 5 so a card overflows in round 2: seat 0
// claims Horizon's neutral station with a card, Cintamani's two stay neutral. 285: each country's printed station starts
// corrupted with a botnet (board.md, setup); 293: its cubes go to the adversary's research row, so a megaproject
// carries player and neutral cubes only. Checks the cubes by color, the completion with its overflow
// paid, the stored cubes and a usable converter beside an unusable one. Needs the viewer running.
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
Object.assign(config.modules.find((m) => m.module === 'board').params, { station_cubes: 2, owned_station_cubes: 5, troop_steps: 0 });
// 285: `station_cubes` 5 (to 284) made Horizon 5 player + 5 adversary = 10 in round 1's production, past its 8 before
// the round-2 read (both cards read 0 cubes, completed). At 2: Horizon 5 + 2 = 7 and Cintamani 2 x 2 neutral + 2 = 6
// after round 1, both open; round 2 fills them (14 and 12), every cube paying.
// 262 cut the Claim action (cards claim): the tier-1 deck is Power Purchase Agreement alone, so seat 0's opening
// hand (dealt first) holds it, and it claims the station.
const tiers = Object.fromEntries(JSON.parse(readFileSync(join(web, '..', 'content', 'cards.json'), 'utf8')).map((c) => [c.id, c.tier]));
const cardsModule = config.modules.find((m) => m.module === 'cards');
cardsModule.cards = cardsModule.cards.filter((id) => tiers[id] === 2 || id === 'power_purchase_agreement');
// Nothing diverts a cube or takes the bound city while the script passes its turns.
// No agents: a creation line would land one on the claimed station, blocking its cubes. 285: the thresholds keep
// Infiltrate from corrupting another site; they don't undo setup's corrupted station (Horizon #4, Cintamani #12).
const adversary = config.modules.find((m) => m.module === 'adversary').params;
Object.assign(adversary, { corrupt_at: 20, corrupt_station_at: 20, corrupt_factory_at: 20, supply_per_country: 0 });
// 293: the corrupted stations' cubes (at this script's `station_cubes`) would run the adversary's research row to
// its loss slot within the rounds played here: a slot costs 30 cubes a country (the param's most) instead.
if ('research_at' in adversary) adversary.research_at = Array.isArray(adversary.research_at) ? adversary.research_at.map(() => 30) : 30;

const page = await openPage();
const { send, js, until, size, consoleErrors } = page;
const { check, failed } = checker();

const value = (where, key) => `(${where}.values.find((x) => x.key === '${key}'))`;
const resource = (r) => js(`${value('window.aic.view.seats[0]', `resource.${r}`)}?.value ?? 0`);

try {
  await size(1366, 768);
  await playConfig(page, config);

  check(await toRound(page, 1, 600), 'reached the first action phase');
  // Seat 0 claims the neutral station next to its node with Power Purchase Agreement (2 credits, 2 energy).
  const station = await js(`(() => {
    const v = window.aic.view;
    const h = v.hexes.find((x) => x.country === 0 && x.tokens.some((t) => t.kind === 'station') && !x.tokens.some((t) => t.kind === 'cube:adversary'));
    return h?.number;
  })()`);
  check(station === 10, `Horizon's neutral station stands on hex ${station}`);
  check(await js(`window.aic.legal.some((a) => a.type === 'play' && a.seat === 0 && a.card === 'power_purchase_agreement')`), 'seat 0 may play Power Purchase Agreement');
  await js(`window.aic.act({ type: 'play', seat: 0, card: 'power_purchase_agreement' })`);
  for (let i = 0; i < 4 && await js(`window.aic.legal.some((a) => a.type === 'choose')`); i++) {
    await js(`(() => {
      const l = window.aic.legal.filter((a) => a.type === 'choose');
      window.aic.act(l.find((a) => a.option === 'Horizon #${station}') ?? l[0]);
    })()`);
  }
  const owned = await js(`window.aic.view.hexes.some((h) => h.country === 0 && h.number === ${station} && h.tokens.some((t) => t.kind === 'cube:0'))`);
  check(owned, 'the card claimed the station');

  check(await toRound(page, 2, 600), 'reached round 2');
  const colors = await js(`(() => {
    const cells = [...document.querySelectorAll('#countries .project')].filter((p) => p.querySelector('.cubes'));
    const count = (p, c) => p.querySelectorAll('.pc[data-color="' + c + '"]').length;
    return { horizon: [count(cells[0], 'seat 0'), count(cells[0], 'adversary')], cintamani: [count(cells[1], 'neutral'), count(cells[1], 'adversary')] };
  })()`);
  // 293: round 1's production: Horizon 5 player (the claimed station); Cintamani 2 + 2 neutral (#2, #11). No
  // adversary cube: a corrupted station's goes to the adversary's research row (285: 2 sat on each card).
  check(colors.horizon[0] === 5 && colors.horizon[1] === 0, `Horizon's card shows its cubes by color: ${colors.horizon} (player, adversary)`);
  check(colors.cintamani[0] === 4 && colors.cintamani[1] === 0, `Cintamani's shows neutral cubes: ${colors.cintamani} (neutral, adversary)`);

  // Round 2 fills each 8-cube card past its size: every cube pays, overflow included. 293: Horizon 10 player;
  // Cintamani 8 neutral.
  check(await toRound(page, 3, 600), 'reached round 3');
  const paid = await js(`[...document.querySelectorAll('#countries .ppaid')].map((e) => e.textContent)`);
  check(paid.length === 2 && paid[0] === '10 cubes paid (10 player, 0 neutral)', `the card completed with its overflow paid: "${paid[0]}"`);
  check(paid[1] === '8 cubes paid (0 player, 8 neutral)', `Cintamani's paid its neutral cubes: "${paid[1]}"`);
  check(await js(`document.querySelectorAll('#countries .project.done').length`) === 2, 'both cards show completed');

  // Round 3 stores 5 cubes on seat 0's board; round 4 can spend them.
  check(await toRound(page, 4, 600), 'reached round 4');
  const stored = await resource('research_cubes');
  check(stored === 5, `seat 0 stores ${stored} research cubes`);
  const shown = await js(`document.querySelector('.seat .res.r-research_cubes')?.textContent ?? ''`);
  check(/research cubes\s*5/.test(shown), `its board shows them: "${shown.trim()}"`);
  // 099: a converter needs a city of its country in reach (r75; r37 asked a node on one): seat 0 places a node
  // on a Horizon city with a node power (answering each prompt toward it), which puts the city in reach.
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
  // 184: r75 asks for any city of the country in reach (r37 asked for a node on one).
  // 285: name the converter the check matched (the first unusable one is now Horizon's chip converter).
  const far = ([c, t]) => c.includes('unusable') && /no city of this country in reach/.test(t);
  check(conv.some(far), `beside an unusable one: ${JSON.stringify(conv.find(far))}`);
  writeFileSync(join(out, 'corps.png'), await page.shot());

  const energy = await resource('energy');
  check(await js(`window.aic.legal.some((a) => a.type === 'convert' && a.seat === 0 && a.country === 0)`), 'the converter is on the menu');
  await js(`window.aic.act({ type: 'convert', seat: 0, country: 0 })`);
  // 282: Horizon's converter is 2 research cubes for 4 energy (Tidal Grid's content; 262: 3 for 5).
  check(await resource('research_cubes') === stored - 2 && await resource('energy') === energy + 4, 'using it turns 2 cubes into 4 energy');
  // 184: r75 prints every converter once a round per player (105 had them as often as you can pay).
  check(!(await js(`window.aic.legal.some((a) => a.type === 'convert' && a.seat === 0 && a.country === 0)`)), 'not offered again this round (once a round, r75)');
  check(consoleErrors.length === 0, `no page errors${consoleErrors.length ? `: ${consoleErrors.join(' | ')}` : ''}`);
} catch (e) {
  console.error(e);
  check(false, 'the run stopped');
} finally {
  await page.close();
}
console.log(failed() ? `e2e:corps FAILED (${failed()})` : 'e2e:corps ok');
process.exit(failed() ? 1 : 0);
