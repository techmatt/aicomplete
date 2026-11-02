// Screenshots of 021's cards and resources in a baseline (d0, 2 countries) game.
//   AIC_URL=http://localhost:<port>/ AIC_SHOTS=<dir> node scripts/shots-021.mjs
// Plays baseline games with the viewer's Random action, seed by seed, and saves at
// 1366x768 the first moment each of these shows:
//   viewer-021-1-opening.png  round 1's keep: the opening hand of 8
//   viewer-021-2-tier.png     a draft hand being built card by card (tier picks)
//   viewer-021-3-passed.png   the draft after a pick: hands passed to the next seat
//   viewer-021-4-stacked.png  a building built over another (status "stacked")
//   viewer-021-5-tokens.png   a token card with 2 or more tokens on it
//   viewer-021-6-power.png    an AI-board power used this round (paid in resources, or by sacrificing tags)
// Drives the page with cdp.mjs (headless Edge or Chrome, no extra packages).

import { newGame, shotsPage, viewerUrl as url } from './cdp.mjs';

const { page, shot } = await shotsPage();
const { send, js, until } = page;

// What the 021 scenes need, read off the view.
const STATE = `(() => {
  const v = window.aic.view;
  const group = (s, k) => (v.seats[s].cards.find((g) => g.key === k) ?? { cards: [] }).cards;
  const seats = v.seats.map((_, s) => ({
    drafted: group(s, 'cards.drafted').length,
    packet: group(s, 'cards.packet').length,
    stacked: group(s, 'cards.built').some((c) => c.status === 'stacked'),
    tokens: Math.max(0, ...[...group(s, 'cards.built'), ...group(s, 'cards.played')].map((c) => {
      const m = /\\((\\d+) (datasets|followers|capital|agents) on it\\)/.exec(c.text ?? '');
      return m ? Number(m[1]) : 0;
    })),
    powerUsed: (v.seats[s].board?.cards ?? []).some((c) => c.id.includes('.power.') && c.status === 'used'),
  }));
  return { round: v.round, outcome: !!v.outcome, phase: v.phase?.name ?? null, seats };
})()`;

const want = {
  'viewer-021-1-opening': (s) => s.round === 1 && s.phase === 'cards.keep' && s.seats[0].drafted === 8,
  'viewer-021-2-tier': (s) => s.phase === 'cards.tier' && s.seats.some((x) => x.packet >= 1 && x.packet <= 3),
  'viewer-021-3-passed': (s) => s.phase === 'cards.draft' && s.seats[0].drafted >= 1 && s.seats[0].packet >= 1,
  'viewer-021-4-stacked': (s) => s.seats.some((x) => x.stacked),
  'viewer-021-5-tokens': (s) => s.seats.some((x) => x.tokens >= 2),
  'viewer-021-6-power': (s) => s.seats.some((x) => x.powerUsed),
};
const saved = {};
try {
  await page.size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  for (let seed = 1; seed <= 20 && Object.keys(saved).length < Object.keys(want).length; seed++) {
    await newGame(page, 'baseline.json', seed);
    for (let i = 0; i < 4000; i++) {
      const s = await js(STATE);
      if (s.outcome) break;
      for (const [name, f] of Object.entries(want)) {
        if (!saved[name] && f(s)) {
          await shot(name);
          saved[name] = { seed, round: s.round, phase: s.phase };
          console.log(name, JSON.stringify(saved[name]));
        }
      }
      if (Object.keys(saved).length === Object.keys(want).length) break;
      await js(`document.getElementById('random-action').click()`);
    }
  }
} finally {
  await page.close();
}
console.log(JSON.stringify(saved, null, 1));
process.exit(Object.keys(saved).length === Object.keys(want).length ? 0 : 1);
