// 105: the demo cards of the engine wording in the card viewer's large view, one shot each.
// node scripts/shots-105.mjs [--port N]   (shots in ../out/shots-105/; the viewer serves this checkout)
import { spawn } from 'node:child_process';
import { mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, shotTo, sleep } from './cdp.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const at = process.argv.indexOf('--port');
const PORT = Number(at > 0 ? process.argv[at + 1] : 5199);
const OUT = join(here, '..', '..', 'out', 'shots-105');
const CARDS = ['citation_engine', 'quantum_decryptor', 'grid_tied_smelter', 'consulate_tower', 'lights_out_assembly',
  'darknet_exchange', 'hyperscale_botnet', 'payroll_automation', 'security_contractor_hq', 'security_ops_floor',
  'honeypot_relay', 'telemetry_archive', 'simulation_cluster', 'data_licensing_deal'];

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
const page = await openPage({ width: 1366, height: 900 });
try {
  await page.send('Page.navigate', { url: `http://localhost:${PORT}/cards.html` });
  await page.until(`document.querySelectorAll('[data-id]').length > 100`, 'the grid');
  for (const id of CARDS) {
    await page.js(`document.querySelector('[data-id="${id}"]').click()`);
    await page.until(`document.getElementById('detail').open`, `${id}'s large view`);
    await sleep(900);
    await shotTo(page, OUT, id);
    await page.js(`document.querySelector('[data-close]').click()`);
    await page.until(`!document.getElementById('detail').open`, 'the view to close');
  }
  console.log(`${CARDS.length} shots in ${OUT}`);
} finally {
  await page.close();
  const gone = new Promise((r) => server.once('exit', r));
  server.kill();
  await gone;
}
