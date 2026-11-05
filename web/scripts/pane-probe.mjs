// `npm run panes` (123): the pane heights behind e2e's `board fills its pane` check.
// Plays e2e's fixed game (baseline seed 11, first legal action, to its end or 150 actions)
// and prints, at 1366x768, 1920x1080 and 2560x1440, the board's size and the height of every
// element with an id in the page's top three layout levels and 30 px or taller, tallest first. Run it on two
// revisions' viewers to see which panel grew when a rules change moves that check.
// Needs the dev server running (AIC_URL, as e2e).

import { newGame, openPage, viewerUrl as url } from './cdp.mjs';

const page = await openPage();
const { send, js, until, size } = page;
try {
  await size(1366, 768);
  await send('Page.navigate', { url });
  await until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await js('localStorage.clear()');
  await newGame(page, 'baseline.json', 11);
  for (let i = 0; i < 150 && !(await js('!!window.aic.view.outcome')); i++) await js('window.aic.act(window.aic.legal[0])');
  console.log(`baseline seed 11, round ${await js('window.aic.view.round')}, ${await js('window.aic.timeline.last')} actions`);
  for (const [w, h] of [[1366, 768], [1920, 1080], [2560, 1440]]) {
    await size(w, h);
    const rows = await js(`(() => {
      const r = (e) => e.getBoundingClientRect();
      const b = r(document.getElementById('board'));
      const els = [...document.querySelectorAll('body > *, body > * > *, body > * > * > *')].filter((e) => e.id && r(e).height >= 30);
      return { board: [b.width, b.height].map(Math.round), panes: els.map((e) => [e.id, Math.round(r(e).height), Math.round(r(e).width)]) };
    })()`);
    const ratio = (rows.board[1] / h).toFixed(2);
    console.log(`\n${w}x${h}: board ${rows.board.join('x')} (${ratio} of the height)`);
    for (const [id, ph, pw] of rows.panes.sort((a, b) => b[1] - a[1])) console.log(`  ${String(ph).padStart(5)} px tall, ${String(pw).padStart(5)} wide  #${id}`);
  }
} finally {
  await page.close();
}
