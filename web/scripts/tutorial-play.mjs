// 325: the tutorial clicked through, as a new player meets it: a fresh browser (the player view on solo.json, the
// new-game screen over it), *Tutorial: your first game*, then every step to the hand-off. An info step is read and
// *Next* clicked; at a "do" step (and between steps) the moves come from play-policy.mjs's small policy, each made by
// clicking the control the player view shows for it (its line is the tutorial's: a node next to a corrupted city, an
// ambassador in, onto the city, Liberate the round after). Shoots every step (OUT/NN-<step>.png).
//
//   node scripts/tutorial-play.mjs [--size 1366x768] [--out DIR]
//
// Needs a served viewer (AIC_URL). Fails (exit 1) on: no new-game screen or the tutorial not its first option; a step
// that does not move on within 60 moves (stuck); a step outlining nothing (the hand-off aside); the prompt off screen,
// or covering the middle of everything it outlines;
// page scroll; a console error; no hand-off. Then checks a reload keeps the step, *Skip step*, *Restart tutorial* (its
// game afresh, step 1) and *Skip tutorial* (the prompt gone, the game on). Writes OUT/summary.json.
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, sleep, viewerUrl } from './cdp.mjs';
import { POLICY } from './play-policy.mjs';

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const size = arg('size', '1366x768');
const [width, height] = size.split('x').map(Number);
const out = arg('out', join(web, '..', 'out', 'tutorial-play', size));
mkdirSync(out, { recursive: true });

/** In the page: the tutorial's prompt as a player sees it. */
const LOOK = `(() => {
  const box = document.getElementById('tutorial');
  const st = aic.tutorial.state;
  const r = box && !box.hidden ? box.getBoundingClientRect() : null;
  const bad = [];
  if (r && (r.left < 0 || r.top < 0 || r.right > innerWidth + 1 || r.bottom > innerHeight + 1)) bad.push('the prompt runs off screen');
  if (document.documentElement.scrollHeight > innerHeight || document.documentElement.scrollWidth > innerWidth) bad.push('page scroll');
  // What it outlines must show: the middle of each ring not under the prompt.
  let covered = 0, rings = 0;
  for (const ring of document.querySelectorAll('#tutorial-rings .tut-ring')) {
    rings++;
    const q = ring.getBoundingClientRect();
    const x = q.left + q.width / 2, y = q.top + q.height / 2;
    if (r && x > r.left && x < r.right && y > r.top && y < r.bottom) covered++;
  }
  if (rings && covered === rings) bad.push('the prompt covers everything it outlines');
  if (/undefined|NaN|level/i.test(box?.innerText ?? '')) bad.push('placeholder or level text: ' + box.innerText.slice(0, 80));
  return { ...st, rings, bad, title: box && !box.hidden ? box.querySelector('.tut-head b')?.textContent ?? '' : '', text: box && !box.hidden ? box.innerText : '' };
})()`;

const page = await openPage({ width, height, fresh: true });
const failures = [];
const steps = [];
let ok = false;
try {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view && window.aic.tutorial', 'the viewer to start', 60000);
  // A new player's browser: the player view on solo.json, the new-game screen over it, the tutorial first.
  const first = await page.js(`(() => { const d = document.getElementById('new-dialog'); const b = d?.querySelector('button');
    return { mode: aic.mode, file: aic.timeline.source.file, open: !!d?.open, first: b?.id, label: b?.textContent }; })()`);
  if (first.mode !== 'player') failures.push(`a fresh browser opened the ${first.mode} view`);
  if (first.file !== 'solo.json') failures.push(`a fresh browser opened ${first.file}, not solo.json`);
  if (!first.open || first.first !== 'nd-tutorial') failures.push(`no new-game screen with the tutorial first (${JSON.stringify(first)})`);
  await sleep(300);
  writeFileSync(join(out, '00-new-game.png'), await page.shot());
  await page.js(`document.getElementById('nd-tutorial').click()`);
  await page.until('window.aic.tutorial.active && !document.getElementById("new-dialog").open', 'the tutorial to start', 30000);
  await page.js(POLICY);

  let last = -1, since = 0, moves = 0;
  for (let i = 0; i < 1500; i++) {
    await sleep(60);
    const st = await page.js(LOOK);
    for (const b of st.bad) failures.push(`step ${st.step + 1} (${st.id}): ${b}`);
    if (!st.active) break;
    if (st.step !== last) {
      last = st.step;
      since = 0;
    }
    if (st.showing && !steps.some((s) => s.id === st.id)) {
      // Move the mouse off anything with a hover, then shoot the step.
      await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
      const name = `${String(st.step + 1).padStart(2, '0')}-${st.id}`;
      writeFileSync(join(out, `${name}.png`), await page.shot());
      steps.push({ n: st.step + 1, id: st.id, kind: st.kind, round: st.round, title: st.title, rings: st.rings, shot: `${name}.png`, text: st.text.split('\n').filter(Boolean).slice(1, -2).join(' ').slice(0, 400) });
    }
    if (st.showing && st.kind === 'info') {
      await page.js(`document.querySelector('#tutorial [data-tut="next"]').click()`);
      continue;
    }
    // A do step, or the play between steps: one move by the policy, clicked.
    if (++since > 60) {
      const board = await page.js(`aic.view.hexes.filter((h) => h.tokens.some((t) => t.kind === 'cube:adversary' || t.kind.startsWith('ambassador'))).map((h) => '#' + h.number + ' ' + h.kind + ': ' + h.tokens.map((t) => t.kind + 'x' + t.count).join(',')).join('; ')
        + ' | last moves: ' + aic.timeline.frames.slice(-12).map((f) => f.action ? f.action.type + (f.action.act ?? '') : '').join(' ')`);
      failures.push(`stuck: step ${st.step + 1} (${st.id}) did not move on in 60 moves (round ${st.round}); the board: ${board}`);
      break;
    }
    const turn = await page.js('window.__tp.turn()');
    if (turn) continue;
    const d = await page.js('window.__tp.decide()');
    if (!d) {
      failures.push(`no legal move at step ${st.step + 1} (${st.id})`);
      break;
    }
    const what = await page.js(`window.__tp.click(${JSON.stringify(d.action)})`);
    if (what === null) {
      failures.push(`no control for ${JSON.stringify(d.action)} at step ${st.step + 1} (${st.id})`);
      break;
    }
    moves++;
  }
  const end = await page.js(`({ active: aic.tutorial.active, state: aic.tutorial.state, round: aic.view.round, outcome: aic.view.outcome, shown: !document.getElementById('tutorial').hidden })`);
  const ids = end.state.ids;
  if (end.active || !steps.some((s) => s.id === 'handoff')) failures.push(`no hand-off (ended at ${end.state.id}, round ${end.round})`);
  if (end.shown) failures.push('the prompt still shows after the hand-off');
  // Every step but the hand-off points at something on screen (a selector matching nothing outlines nothing).
  for (const s of steps) if (!s.rings && s.id !== 'handoff') failures.push(`step ${s.n} (${s.id}) outlines nothing on screen`);
  const missed = ids.filter((id) => !steps.some((s) => s.id === id));
  if (missed.length) failures.push(`steps never shown: ${missed.join(', ')}`);
  if (end.outcome) failures.push(`the game ended before the hand-off: ${JSON.stringify(end.outcome)}`);
  writeFileSync(join(out, '99-after-handoff.png'), await page.shot());

  // The controls: Restart (its game afresh, step 1), a reload keeping the step, Skip step, Skip tutorial.
  const controls = {};
  await page.js('aic.tutorial.start()');
  await page.until(`aic.tutorial.active && aic.tutorial.state.step === 0 && aic.timeline.last === 0 && aic.timeline.seed === '24'`, 'the tutorial restarted', 10000);
  for (let k = 0; k < 3; k++) await page.js(`document.querySelector('#tutorial [data-tut="next"]').click()`);
  const before = await page.js('aic.tutorial.state.step');
  await page.js('(window.__aicOld = true)');
  await page.send('Page.reload');
  await page.until('!window.__aicOld && window.aic && window.aic.tutorial', 'the reload', 30000);
  await sleep(300);
  controls.reloadKeepsStep = (await page.js('aic.tutorial.state.step')) === before && (await page.js('aic.tutorial.active'));
  // Back, then through to the first do step (the build), and Skip step there.
  await page.js(`document.querySelector('#tutorial [data-tut="back"]').click()`);
  controls.back = (await page.js('aic.tutorial.state.step')) === before - 1;
  for (let k = 0; k < 10 && (await page.js(`aic.tutorial.state.kind === 'info'`)); k++) await page.js(`document.querySelector('#tutorial [data-tut="next"]').click()`);
  const doStep = await page.js('aic.tutorial.state');
  await page.js(`document.querySelector('#tutorial [data-tut="skip"]')?.click()`);
  controls.skipStep = doStep.kind === 'do' && (await page.js('aic.tutorial.state.step')) === doStep.step + 1;
  await page.js(`document.querySelector('#tutorial [data-tut="restart"]').click()`);
  controls.restartButton = await page.js(`aic.tutorial.active && aic.tutorial.state.step === 0 && aic.timeline.last === 0`);
  await page.js(`document.querySelector('#tutorial [data-tut="exit"]').click()`);
  controls.skipTutorial = await page.js(`!aic.tutorial.active && document.getElementById('tutorial').hidden && aic.legal.length > 0`);
  // The new-game screen opens from the header and offers the tutorial again.
  await page.js(`document.getElementById('new-open').click()`);
  controls.newGameAgain = await page.js(`document.getElementById('new-dialog').open`);
  for (const [k, v] of Object.entries(controls)) if (!v) failures.push(`control check failed: ${k}`);
  if (page.consoleErrors.length) failures.push(`${page.consoleErrors.length} console errors: ${page.consoleErrors.slice(0, 3).join(' | ')}`);

  const summary = { how: 'web/scripts/tutorial-play.mjs', size, url: viewerUrl, steps, moves, controls, failures, consoleErrors: page.consoleErrors };
  writeFileSync(join(out, 'summary.json'), `${JSON.stringify(summary, null, 1)}\n`);
  for (const s of steps) console.log(`${String(s.n).padStart(2)} ${s.kind.padEnd(4)} r${s.round} ${s.id}: ${s.title} (${s.rings} outlined)`);
  console.log(`${steps.length} steps shown, ${moves} moves clicked; controls ${JSON.stringify(controls)}`);
  for (const f of failures) console.log(`FAIL ${f}`);
  ok = failures.length === 0;
  console.log(ok ? `ok: the tutorial clicked through to the hand-off (${out})` : `${failures.length} failures`);
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
