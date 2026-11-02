// Screenshots of the adversary's project row (017) in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-017.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at 1366x768:
//   viewer-projects-r2.png    round 2, a visible project part-filled with compute
//   viewer-projects-r<n>.png  a later round (4+), a project part-filled and at least one resolved
// Drives the page with cdp.mjs (headless Edge or Chrome, no extra packages).

import { newGame, shotsPage, viewerUrl as url } from './cdp.mjs';

const { page, shot } = await shotsPage();
const { send, js, until } = page;

// What the project row shows now: each visible project's fill, the resolved count, the board size.
const STATE = `(() => {
  const v = window.aic.view;
  const pile = (k) => v.piles.find((p) => p.key === k);
  const row = (pile('adversary.projects')?.visible ?? []).map((c) => {
    const m = /(\\d+)\\s*\\/\\s*(\\d+)/.exec(c.status ?? '');
    return { name: c.name, have: m ? +m[1] : 0, cost: m ? +m[2] : 0, status: c.status };
  });
  const r = document.getElementById('board').getBoundingClientRect();
  return { round: v.round, outcome: !!v.outcome, row, resolved: pile('adversary.projects_done')?.count ?? 0,
           ongoing: (pile('adversary.projects_ongoing')?.visible ?? []).map((c) => c.name),
           compute: v.globals.find((g) => g.key === 'adversary.compute')?.value,
           drawn: document.querySelectorAll('#pipeline .projects-row .project .prog').length,
           board: [Math.round(r.width), Math.round(r.height)] };
})()`;
const midFill = (s) => s.row.some((p) => p.have > 0 && p.have < p.cost);

const saved = {};
try {
  await page.size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  for (let seed = 1; seed <= 30 && !(saved.r2 && saved.late); seed++) {
    await newGame(page, 'baseline.json', seed);
    for (let i = 0; i < 2000; i++) {
      const s = await js(STATE);
      if (s.outcome) { console.log(`seed ${seed}: ended in round ${s.round}, ${s.resolved} resolved`); break; }
      if (!saved.r2 && s.round === 2 && midFill(s)) {
        await shot('viewer-projects-r2');
        saved.r2 = { seed, ...s };
      }
      // A resolved project leaves its slot empty until all three have gone (rules.md), so
      // this row has fewer than three.
      if (!saved.late && s.round >= 4 && s.resolved > 0 && midFill(s)) {
        await shot(`viewer-projects-r${s.round}`);
        saved.late = { seed, ...s };
        break;
      }
      await js(`document.getElementById('random-action').click()`);
    }
  }
} finally {
  await page.close();
}
console.log(JSON.stringify(saved, null, 1));
process.exit(saved.r2 && saved.late ? 0 : 1);
