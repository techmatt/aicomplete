// `npm run cards:check -- [--port N]`: the card viewer's browser check, from a fresh server:
// every kind shows, a tag filters, cost sorts, a note survives a restart and is badged after
// its card's text changes, a card added to content is badged new, and with no results the
// stat columns go away without an error. Works on a copy under out/cards-check/, so it
// never touches content or real notes. About 20 s; screenshots land in that folder.
//   AIC_BROWSER browser executable (see cdp.mjs)

import { spawn } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { REPO } from './cards-export.mjs';
import { openPage, shotTo, sleep } from './cdp.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const at = process.argv.indexOf('--port');
const PORT = Number(at > 0 ? process.argv[at + 1] : 5199);
const URL_ = `http://localhost:${PORT}/cards.html`;
const OUT = join(REPO, 'out', 'cards-check');
const ROOT = join(OUT, 'root');

rmSync(ROOT, { recursive: true, force: true });
for (const p of ['content', 'configs', 'web/card/faces.json', 'web/card/layout.json', 'art/frames/r3/fit.json', 'docs/card-pricing.md', 'tools/card-art/scenes.json', 'crates/aic-core/src/lib.rs', 'experiments/results/screen-027-endgame']) {
  mkdirSync(dirname(join(ROOT, p)), { recursive: true });
  cpSync(join(REPO, p), join(ROOT, p), { recursive: true });
}

let failed = 0;
const check = (ok, what) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
  if (!ok) failed++;
};

async function serve() {
  const p = spawn(process.execPath, [join(here, 'cards-server.mjs'), '--port', String(PORT), '--root', ROOT], { stdio: ['ignore', 'pipe', 'inherit'] });
  await new Promise((ok, fail) => {
    let text = '';
    p.stdout.on('data', (d) => {
      text += d;
      if (text.includes('Ctrl+C')) ok();
    });
    p.on('exit', (c) => fail(new Error(`server exited ${c}`)));
  });
  p.removeAllListeners('exit');
  return p;
}

const stop = async (p) => {
  const gone = new Promise((r) => p.once('exit', r));
  p.kill();
  await gone;
};

const notes = () => JSON.parse(readFileSync(join(ROOT, 'review', 'card-notes.json'), 'utf8'));
const edit = (file, fn) => {
  const f = join(ROOT, 'content', file);
  const rows = JSON.parse(readFileSync(f, 'utf8'));
  fn(rows);
  writeFileSync(f, JSON.stringify(rows));
};
const click = (page, sel) => page.js(`document.querySelector(${JSON.stringify(sel)}).click()`);
const ids = (page) => page.js(`[...document.querySelectorAll('[data-id]')].map((e) => e.dataset.id)`);
const hasBadge = (page, id, cls) => page.js(`!!document.querySelector('[data-id="${id}"] .badge.${cls}')`);

let server = await serve();
const page = await openPage({ width: 1500, height: 950 });
try {
  await page.send('Page.navigate', { url: URL_ });
  await page.until(`document.querySelectorAll('.tile').length > 0`, 'the cards');
  const data = await page.js(`(({ cards, stats }) => ({ cards: cards.map((c) => ({ id: c.id, kind: c.kind, tags: c.tags, cost: c.cost, render: !!c.render })), columns: stats.columns.length }))(window.cardViewer.data)`);
  check((await ids(page)).length === data.cards.length, `all ${data.cards.length} cards on the page`);
  for (const k of ['program', 'building', 'event', 'threat', 'unity', 'discord', 'adversary', 'project', 'megaproject', 'ai'])
    check(data.cards.some((c) => c.kind === k) && (await page.js(`!!document.querySelector('#kinds [data-v="${k}"]')`)), `kind ${k} has a filter and cards`);
  check((await page.js(`document.querySelectorAll('.tile .card').length`)) === data.cards.filter((c) => c.render).length, 'every player card went through the renderer');
  check(await page.js(`[...document.querySelectorAll('.tile .plain h3')].every((h) => h.textContent)`), 'every frameless card has a named face');
  await shotTo(page, OUT, 'grid');
  for (const k of ['ai', 'megaproject', 'unity', 'adversary']) await click(page, `#kinds [data-v="${k}"]`);
  await sleep(800);
  await page.js(`window.scrollTo(0, 0)`);
  await shotTo(page, OUT, 'plain');
  await page.js(`document.getElementById('clear').click()`);

  await click(page, '#tags [data-v="power"]');
  const power = await ids(page);
  const want = data.cards.filter((c) => c.tags.includes('power')).map((c) => c.id);
  check(power.length === want.length && power.every((i) => want.includes(i)), `tag filter: ${power.length} power cards`);
  await page.js(`(() => { const s = document.getElementById('sort'); s.value = 'cost'; s.onchange(); })()`);
  const costs = (await ids(page)).map((i) => data.cards.find((c) => c.id === i).cost);
  check(costs.every((c, i) => !i || costs[i - 1] <= c), `sorted by cost: ${costs[0]} to ${costs.at(-1)}`);
  await page.js(`document.getElementById('dir').click()`);
  const down = (await ids(page)).map((i) => data.cards.find((c) => c.id === i).cost);
  check(down.every((c, i) => !i || down[i - 1] >= c), 'and reversed');
  await page.js(`document.getElementById('clear').click()`);
  await page.js(`document.getElementById('dir').click()`);

  await click(page, '#search');
  await page.js(`(() => { const s = document.getElementById('search'); s.value = 'requisition'; s.oninput(); })()`);
  check((await ids(page)).includes('requisition'), 'text search finds a threat');
  await page.js(`document.getElementById('clear').click()`);

  // A note.
  await click(page, '[data-id="requisition"]');
  await page.until(`document.getElementById('detail').open`, 'the large view');
  check(await page.js(`document.querySelector('.detail').textContent.includes('Counter')`), 'large view: price inputs');
  check(await page.js(`document.querySelector('pre.ops').textContent.includes('requisition')`), 'large view: ops');
  await shotTo(page, OUT, 'detail-threat');
  await click(page, '[data-flag="too cheap"]');
  await page.js(`document.getElementById('comment').value = 'Keep <this> & that'`);
  await page.js(`document.getElementById('note').requestSubmit()`);
  await page.until(`document.getElementById('saved')?.textContent === 'Saved.'`, 'the save');
  await page.js(`document.querySelector('[data-close]').click()`);
  await page.until(`!document.getElementById('detail').open`, 'the view to close');
  check(notes().requisition?.flag === 'too cheap', 'note on disk, with its flag');
  check(await hasBadge(page, 'requisition', 'too-cheap'), 'flag badge on the card');

  await click(page, '[data-id="commandeer"]');
  await page.until(`document.getElementById('comment')`, 'the next card');
  await page.js(`document.getElementById('comment').value = 'typed, never saved'`);
  await page.js(`document.querySelector('[data-close]').click()`);
  await page.until(`!document.getElementById('detail').open`, 'the view to close');
  check(notes().commandeer?.comment === 'typed, never saved', 'leaving a card keeps what was typed');
  check((await fetch(`http://127.0.0.1:${PORT}/cards.html`)).ok, 'served on 127.0.0.1 as well as localhost');

  await click(page, '[data-id="field_sensor_grid"]');
  await page.until(`document.getElementById('detail').open`, 'a player card');
  check(await page.js(`document.querySelector('.detail').textContent.includes('card-pricing.md')`), 'large view: a pricing line');
  check(await page.js(`!!document.querySelector('.detail .card img.art')`), 'large view: scene art');
  await sleep(1500);
  await shotTo(page, OUT, 'detail-card');
  await page.js(`document.querySelector('[data-close]').click()`);

  // Past a restart.
  const hash = notes().requisition.hash;
  await stop(server);
  server = await serve();
  await page.send('Page.navigate', { url: URL_ });
  await page.until(`document.querySelectorAll('.tile').length > 0 && !!window.cardViewer`, 'the cards after a restart');
  check(await hasBadge(page, 'requisition', 'too-cheap'), 'note survives a server restart');
  check(!(await hasBadge(page, 'requisition', 'changed')), 'not changed yet');
  check(!(await page.js(`!!document.querySelector('.badge.new')`)), 'nothing new on the first run');
  await click(page, '[data-id="requisition"]');
  await page.until(`document.getElementById('comment')?.value === 'Keep <this> & that'`, 'the comment back, escaped');
  await page.js(`document.querySelector('[data-close]').click()`);

  // Content moves under it: Rebuild, no restart.
  edit('threats.json', (rows) => {
    rows.find((t) => t.id === 'requisition').text = 'Each player here discards 3 cards or pays 6 credits.';
    rows.push({ id: 'check_new_threat', name: 'Check New Threat', tier: 1, text: 'A row the check added.', counter: [['credits', 4]], effects: rows[0].effects });
  });
  await page.js(`document.getElementById('rebuild').click()`);
  await page.until(`document.querySelector('[data-id="check_new_threat"]')`, 'the rebuild');
  check(await hasBadge(page, 'requisition', 'changed'), 'badged "changed" after its text changed');
  check(await hasBadge(page, 'check_new_threat', 'new'), 'a card never seen is badged new');
  check(!(await page.js(`document.querySelector('[data-id="field_sensor_grid"] .badge.changed')`)), 'the other cards are not badged');
  await shotTo(page, OUT, 'badges');
  await click(page, '[data-id="check_new_threat"]');
  await page.until(`!document.querySelector('[data-id="check_new_threat"] .badge.new')`, 'opening a card to clear "new"');
  await page.js(`document.querySelector('[data-close]').click()`);
  await page.until(`!document.getElementById('detail').open`, 'the view to close');
  await click(page, '[data-id="requisition"]');
  await page.until(`document.querySelector('.note .was')`, 'the old text');
  check(await page.js(`document.querySelector('.note .was').textContent.includes('discards 2 cards')`), 'large view: the text at the time of the note');
  await page.js(`document.getElementById('note').requestSubmit()`);
  await page.until(`document.getElementById('saved')?.textContent === 'Saved.'`, 'the save');
  await page.js(`document.querySelector('[data-close]').click()`);
  await page.until(`!document.getElementById('detail').open`, 'the view to close');
  check(notes().requisition.hash !== hash && !(await hasBadge(page, 'requisition', 'changed')), 'saving again clears "changed"');

  // Stats: a reasoned price, then none at all.
  writeFileSync(join(ROOT, 'experiments', 'results', 'card-prices.csv'), 'card,reason,reasoned_price\nfield_sensor_grid,"worth more, on\ntwo lines",3.5\n');
  await page.js(`document.getElementById('rebuild').click()`);
  await page.until(`[...document.getElementById('sort').options].some((o) => o.value === 'price_gap')`, 'the reasoned column');
  await page.js(`document.getElementById('view').value = 'table'`);
  await page.js(`document.getElementById('view').onchange()`);
  await shotTo(page, OUT, 'table');
  check(await page.js(`[...document.querySelectorAll('th')].some((t) => t.textContent.startsWith('Kept %'))`), 'table has the stat columns');
  check((await page.js(`window.cardViewer.data.cards.find((c) => c.id === 'field_sensor_grid').priceGap`)) === -1.5, 'printed minus reasoned, from a CSV with a words column');
  rmSync(join(ROOT, 'experiments'), { recursive: true });
  await page.js(`document.getElementById('rebuild').click()`);
  await page.until(`![...document.getElementById('sort').options].some((o) => o.value === 'kept_pct')`, 'the stats gone');
  check(!(await page.js(`[...document.querySelectorAll('th')].some((t) => t.textContent.startsWith('Kept %'))`)), 'no results: stat columns blank');
  check(await page.js(`document.querySelectorAll('[data-id]').length > 200`), 'no results: the cards still show');

  // The bad paths.
  const post = (path, init) => fetch(`http://localhost:${PORT}${path}`, { method: 'POST', ...init }).then((r) => r.status);
  check((await post('/api/notes', { headers: { 'content-type': 'application/json', origin: 'http://evil.example' }, body: '{"id":"requisition"}' })) === 403, 'another origin cannot write');
  check((await post('/api/notes', { headers: { 'content-type': 'text/plain' }, body: '{"id":"requisition"}' })) === 415, 'a form post cannot write');
  check((await post('/api/notes', { headers: { 'content-type': 'application/json' }, body: '{"id":"nope","flag":"ok"}' })) === 404, 'no note on an unknown card');
  check((await fetch(`http://localhost:${PORT}/card/..%2f..%2fCLAUDE.md`)).status === 404, 'no reading outside the served folders');
  writeFileSync(join(ROOT, 'review', 'card-notes.json'), '{ broken');
  check((await post('/api/notes', { headers: { 'content-type': 'application/json' }, body: '{"id":"requisition","flag":"ok"}' })) === 500, 'a broken notes file stops a save');
  check(readFileSync(join(ROOT, 'review', 'card-notes.json'), 'utf8') === '{ broken', 'and is not overwritten');

  const errors = page.consoleErrors.filter((e) => !/status of 40[34]/.test(e));
  check(!errors.length, `no console errors${errors.length ? `: ${errors.slice(0, 3).join(' | ')}` : ''}`);
} catch (e) {
  const said = await page.js(`document.getElementById('saved')?.textContent ?? ''`).catch(() => '');
  check(false, `${e.message}${said ? ` (the page says: ${said})` : ''}`);
} finally {
  await page.close();
  await stop(server);
}
check(existsSync(join(ROOT, 'out', 'cards-export.json')), 'the export is on disk');
console.log(failed ? `cards:check: ${failed} failed` : 'cards:check: all good');
process.exit(failed ? 1 : 0);
