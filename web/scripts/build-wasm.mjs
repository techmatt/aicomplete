// Builds crates/aic-wasm into web/src/wasm/ (git-ignored), so the viewer always runs
// the engine from this checkout. Run by `npm run build`, by `npm run wasm`, and by the
// dev server (vite.config.ts) at its start and when crates/ or content/ change.
//   --force                build even when the inputs are unchanged
//   AIC_WASM_PROFILE=dev   faster unoptimized build (default: release)
//
// Skips the build when web/src/wasm/.inputs holds the hash of what the wasm is built
// from (aic-core minus its tests, aic-wasm, content/, Cargo.toml, Cargo.lock, the
// profile): a no-op wasm-pack still costs ~50 s of wasm-opt. One build at a time per
// checkout: target/aic-wasm-build.lock, its mtime refreshed every few seconds while held;
// a second build waits, then rechecks the hash. A lock whose pid is gone, or not
// refreshed for 30 s (a killed build, its pid reused), is taken over.

import { spawn } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import {
  closeSync, copyFileSync, existsSync, mkdirSync, openSync, readdirSync, readFileSync, renameSync, rmSync, statSync,
  utimesSync, writeFileSync,
} from 'node:fs';
import { dirname, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const repo = resolve(web, '..');
const crate = join(repo, 'crates', 'aic-wasm');
const outDir = join(web, 'src', 'wasm');
const stamp = join(outDir, '.inputs');
const lock = join(repo, 'target', 'aic-wasm-build.lock');
const profile = process.env.AIC_WASM_PROFILE === 'dev' ? '--dev' : '--release';
const force = process.argv.includes('--force');
const HEARTBEAT_MS = 5000;
const STALE_MS = 30000;

function files(dir, out = []) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) files(p, out);
    else out.push(p);
  }
  return out;
}

/** The hash of every file the wasm is built from. aic-core's tests module is test-only. */
function inputs() {
  const coreTests = join(repo, 'crates', 'aic-core', 'src', 'tests') + sep;
  const list = [
    join(repo, 'Cargo.toml'),
    join(repo, 'Cargo.lock'),
    ...files(join(repo, 'crates', 'aic-core')).filter((f) => !f.startsWith(coreTests)),
    ...files(crate),
    ...files(join(repo, 'content')),
  ].filter(existsSync);
  const h = createHash('sha256').update(profile);
  for (const f of list.sort()) {
    h.update('\0' + relative(repo, f).replaceAll('\\', '/') + '\0');
    h.update(readFileSync(f));
  }
  return h.digest('hex');
}

const built = () =>
  existsSync(join(outDir, 'aic_wasm.js')) && existsSync(join(outDir, 'aic_wasm_bg.wasm'));

const alive = (pid) => {
  try {
    process.kill(pid, 0);
    return true;
  } catch (e) {
    return e.code === 'EPERM';
  }
};

const nap = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

/** Takes the checkout's build lock, waiting for a live holder; returns its token. */
function takeLock() {
  mkdirSync(dirname(lock), { recursive: true });
  const token = `${process.pid} ${randomUUID()}`;
  let told = false;
  for (;;) {
    try {
      const fd = openSync(lock, 'wx');
      writeFileSync(fd, token);
      closeSync(fd);
      return token;
    } catch (e) {
      if (e.code !== 'EEXIST') throw e;
    }
    let held, age;
    try {
      held = readFileSync(lock, 'utf8');
      age = Date.now() - statSync(lock).mtimeMs;
    } catch {
      continue; // released meanwhile
    }
    const pid = Number(held.split(' ')[0]) || 0;
    // An empty file is a lock being written this instant: only its age makes it stale.
    if ((pid && !alive(pid)) || age > STALE_MS) {
      const aside = `${lock}.${process.pid}.stale`;
      try {
        renameSync(lock, aside); // one waiter wins the takeover; the others find no file and retry
        rmSync(aside, { force: true });
      } catch {}
      continue;
    }
    if (!told) {
      console.log(`another wasm build (pid ${pid || '?'}) is running in this checkout; waiting for it`);
      told = true;
    }
    nap(500);
  }
}

const quote = (a) => (/[\s"]/.test(a) ? `"${a.replaceAll('"', '\\"')}"` : a);

const token = takeLock();
const beat = setInterval(() => {
  try {
    utimesSync(lock, new Date(), new Date());
  } catch {}
}, HEARTBEAT_MS);
process.on('exit', () => {
  try {
    if (readFileSync(lock, 'utf8') === token) rmSync(lock, { force: true });
  } catch {}
});
for (const sig of ['SIGINT', 'SIGTERM', 'SIGBREAK']) process.on(sig, () => process.exit(130));

const hash = inputs();
if (!force && built() && existsSync(stamp) && readFileSync(stamp, 'utf8').trim() === hash) {
  console.log(`web/src/wasm is up to date (${profile.slice(2)}); not rebuilt (--force rebuilds)`);
  process.exit(0);
}

const started = Date.now();
// Built beside target/ and copied in when done: a dev server watching web/src/wasm
// reloads the page once, onto a whole engine, not on each file wasm-pack writes.
const staging = join(repo, 'target', `aic-wasm-pkg-${process.pid}`);
rmSync(staging, { recursive: true, force: true });
const args = ['build', crate, '--target', 'web', profile, '--out-dir', staging, '--out-name', 'aic_wasm', '--no-pack'];
// On Windows wasm-pack may be a .cmd shim, which only a shell runs; one command line
// (not an args array) keeps Node's DEP0190 warning out of the log. Async, so the lock's
// heartbeat runs while it builds.
const build = await new Promise((done) => {
  const p =
    process.platform === 'win32'
      ? spawn(['wasm-pack', ...args].map(quote).join(' '), { stdio: 'inherit', shell: true })
      : spawn('wasm-pack', args, { stdio: 'inherit' });
  p.on('error', (error) => done({ error }));
  p.on('exit', (status) => done({ status }));
});
clearInterval(beat);
if (build.error) {
  console.error(`Could not run wasm-pack (cargo install wasm-pack --locked): ${build.error.message}`);
  process.exit(1);
}
if (build.status !== 0) {
  rmSync(staging, { recursive: true, force: true });
  rmSync(stamp, { force: true });
  process.exit(build.status ?? 1);
}
mkdirSync(outDir, { recursive: true });
rmSync(stamp, { force: true });
const out = readdirSync(staging).filter((f) => f !== '.gitignore' && f !== 'package.json');
// The .wasm first: the .js that loads it is what a reload re-imports.
out.sort((a, b) => Number(b.endsWith('.wasm')) - Number(a.endsWith('.wasm')));
for (const f of out) copyFileSync(join(staging, f), join(outDir, f));
rmSync(staging, { recursive: true, force: true });
rmSync(join(outDir, '.gitignore'), { force: true });
writeFileSync(stamp, hash + '\n');
console.log(`web/src/wasm built (${profile.slice(2)}) in ${((Date.now() - started) / 1000).toFixed(1)} s`);
