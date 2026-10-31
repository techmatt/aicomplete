// native = wasm across the variants (020): aic-sim plays and saves games in every
// variant of the core's variant matrix, then the wasm package replays each record
// (`check_record`) and must reach the saved final fingerprint with no warnings.
// Usage (after a release aic-sim; rebuilds the wasm package itself when it is missing
// or older than crates/ or content/):
//   node scripts/wasm-parity.mjs [games per variant, default 3]
//   AIC_SIM  aic-sim binary (default target/release/aic-sim[.exe])
import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pkg = path.join(root, 'crates', 'aic-wasm', 'pkg');
const wasmFile = path.join(pkg, 'aic_wasm_bg.wasm');

// The newest change under `dir`, in ms since the epoch.
function newest(dir) {
  let t = 0;
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (e.name === 'pkg') continue;
    const p = path.join(dir, e.name);
    t = Math.max(t, e.isDirectory() ? newest(p) : statSync(p).mtimeMs);
  }
  return t;
}

// 044: a package older than the content fails every record on the content hash.
const sources = ['crates/aic-core', 'crates/aic-wasm', 'content', 'configs'].map((d) => path.join(root, d));
if (!existsSync(wasmFile) || statSync(wasmFile).mtimeMs < Math.max(...sources.map(newest))) {
  console.log('crates/aic-wasm/pkg is missing or stale: rebuilding it');
  const built = spawnSync('wasm-pack', ['build', 'crates/aic-wasm', '--target', 'web'],
    { cwd: root, stdio: 'inherit', shell: process.platform === 'win32' });
  if (built.status !== 0) {
    console.error('wasm-pack failed, so this would test a stale package. Fix it, or run: wasm-pack build crates/aic-wasm --target web');
    process.exit(1);
  }
}
const m = await import(pathToFileURL(path.join(pkg, 'aic_wasm.js')).href);
await m.default({ module_or_path: readFileSync(wasmFile) });
const sim = process.env.AIC_SIM ?? path.join(root, 'target', 'release', process.platform === 'win32' ? 'aic-sim.exe' : 'aic-sim');
const games = process.argv[2] ?? '3';

// The variant matrix of crates/aic-core/src/tests/mod.rs, played by greedy and random.
const VARIANTS = [
  [], ['threats.variant=shared_row'], ['threats.variant=off'], ['alignment.variant=still'],
  ['alignment.variant=off'], ['alignment.momentum_every=2', 'alignment.antimatter_discord=0'],
  ['adversary.difficulty=4'], ['cards.variant=off'],
  ['board.clock=3'], ['countries=1'], ['countries=3'], ['countries=4'],
  ['adversary.difficulty=4', 'adversary.counter_bonus_all=1', 'threats.variant=shared_row'],
  ['countries=3', 'adversary.difficulty=4', 'alignment.variant=still'],
];
const out = mkdtempSync(path.join(tmpdir(), 'aic-parity-'));
let checked = 0;
let bad = 0;
let hashes = 0;
try {
  for (const [i, sets] of VARIANTS.entries()) {
    for (const agent of ['random', 'greedy']) {
      const dir = path.join(out, `${i}-${agent}`);
      const r = spawnSync(sim, ['run', path.join(root, 'configs', 'baseline.json'), '--games', games, '--seed', '50',
        '--agent', agent, ...sets.flatMap((s) => ['--set', s]), '--save', dir, '--save-when', 'all'], { encoding: 'utf8' });
      if (r.error) throw new Error(`cannot run aic-sim at ${sim} (${r.error.code}): cargo build --release -p aic-sim, or set AIC_SIM`);
      if (r.status !== 0) {
        console.log(`FAIL aic-sim ${sets.join(' ')} ${agent}: ${(r.stderr ?? '').slice(0, 300)}`);
        bad++;
        continue;
      }
      for (const f of readdirSync(dir)) {
        const c = JSON.parse(m.check_record(readFileSync(path.join(dir, f), 'utf8')));
        checked++;
        if (!c.ok || c.warnings.length) {
          bad++;
          const why = c.error ?? c.warnings.join('; ');
          hashes += why.includes('content hash');
          console.log(`FAIL ${sets.join(' ') || 'baseline'} ${agent} ${f}: ${why}`);
        }
      }
    }
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
console.log(`${checked} records over ${VARIANTS.length} variants replayed in wasm: ${bad ? `${bad} FAILED` : 'native = wasm'}`);
if (hashes) console.log(`${hashes} failed on the content hash: ${sim} and the wasm package come from different content, so that is a stale build, not a parity bug. Rebuild both: cargo build --release -p aic-sim; wasm-pack build crates/aic-wasm --target web`);
process.exit(bad ? 1 : 0);
