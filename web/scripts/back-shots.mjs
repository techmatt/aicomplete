// The card backs (191) in the viewer's Backs view, one shot (as card-shots.mjs does for faces).
// node scripts/back-shots.mjs [--port N] [--out NAME]   (the shot in ../out/NAME/backs.png, default card-shots)
import { spawn } from 'node:child_process';
import { mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, shotTo, sleep } from './cdp.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const arg = (name, dflt) => {
  const at = process.argv.indexOf(name);
  return at > 0 ? process.argv[at + 1] : dflt;
};
const PORT = Number(arg('--port', 5199));
const OUT = join(here, '..', '..', 'out', arg('--out', 'card-shots'));

mkdirSync(OUT, { recursive: true });
const server = spawn(process.execPath, [join(here, 'cards-server.mjs'), '--port', String(PORT)], { stdio: ['ignore', 'pipe', 'inherit'] });
await new Promise((ok, fail) => {
  let text = '';
  server.stdout.on('data', (d) => {
    text += d;
    if (text.includes('Ctrl+C')) ok();
  });
  server.on('exit', (c) => fail(new Error(`server exited ${c}`)));
});
server.removeAllListeners('exit');
const page = await openPage({ width: 1000, height: 700 });
try {
  await page.send('Page.navigate', { url: `http://localhost:${PORT}/cards.html` });
  await page.until(`document.querySelectorAll('[data-id]').length > 100`, 'the grid');
  await page.js(`(() => { const v = document.getElementById('view'); v.value = 'backs'; v.dispatchEvent(new Event('change')); return 1; })()`);
  await page.until(`document.querySelectorAll('.card.back').length === 2`, 'the backs');
  await sleep(900);
  await shotTo(page, OUT, 'backs');
  console.log(`backs in ${OUT}`);
} finally {
  await page.close();
  const gone = new Promise((r) => server.once('exit', r));
  server.kill();
  await gone;
}
