// Builds the farm's wasm bundle (crates/aic-farm) into BUNDLE: conductor's `[remote] build`.
//   node scripts/farm/build.mjs BUNDLE
// Writes aic_farm.js and aic_farm_bg.wasm (wasm-pack, --target web, release), then farm.mjs: the
// bundle's ES module (`bundle_js`), whose default export is init() and which exports fingerprint()
// and run(first, count, args). In Node, init() reads the wasm from beside farm.mjs; a browser
// fetches it. Needs wasm-pack (cargo install wasm-pack --locked) and the wasm32-unknown-unknown target.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const bundle = process.argv[2];
if (!bundle) {
  console.error('usage: node scripts/farm/build.mjs BUNDLE');
  process.exit(2);
}
const out = resolve(bundle);
mkdirSync(out, { recursive: true });

const started = Date.now();
const build = spawnSync(
  'wasm-pack',
  ['build', join(root, 'crates', 'aic-farm'), '--target', 'web', '--release', '--out-dir', out,
    '--out-name', 'aic_farm', '--no-pack', '--no-typescript'],
  // No shell: a path with a space stays one argument (libuv finds wasm-pack.exe on PATH).
  { stdio: 'inherit' },
);
if (build.error) {
  console.error(`Could not run wasm-pack (cargo install wasm-pack --locked): ${build.error.message}`);
  process.exit(1);
}
if (build.status !== 0) process.exit(build.status ?? 1);
rmSync(join(out, '.gitignore'), { force: true });

writeFileSync(join(out, 'farm.mjs'), `// The farm's bundle (crates/aic-farm; written by scripts/farm/build.mjs).
import wasmInit, * as farm from './aic_farm.js';

let ready;

// Loads the wasm once. A browser fetches aic_farm_bg.wasm from beside this file; Node reads it.
export default function init(input) {
  if (!ready) {
    if (input === undefined && typeof process !== 'undefined' && process.versions?.node) {
      ready = import('node:fs/promises')
        .then((fs) => fs.readFile(new URL('./aic_farm_bg.wasm', import.meta.url)))
        .then((bytes) => wasmInit({ module_or_path: bytes }));
    } else {
      ready = wasmInit(input === undefined ? undefined : { module_or_path: input });
    }
  }
  return ready;
}

// A short check a worker must reproduce: the rules revision, content hash and a hash of two games.
export function fingerprint() {
  return farm.fingerprint();
}

// A shard: every cell of the spec \`args\` names (one name in experiments/farm/), on seeds
// first .. first + count, one game line each, seed by seed.
export function run(first, count, args) {
  return farm.run(Number(first), Number(count), String(args));
}

// The engine this bundle plays as: 'wasm'.
export function engine() {
  return farm.engine();
}
`, { encoding: 'utf-8' });
console.log(`farm bundle built into ${out} in ${((Date.now() - started) / 1000).toFixed(1)} s`);
