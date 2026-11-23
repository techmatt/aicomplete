// `npm run dev [-- vite args]`: what conductor's *Play game* runs
// (`npm run dev -- --port 5261 --strictPort`, in the main checkout's web/).
//
// 1. Installs web/node_modules (`npm ci`) when it is missing or doesn't match
//    package-lock.json: the main checkout never gets the `npm ci` a worktree does.
// 2. Starts vite at once, with these arguments. vite.config.ts builds the wasm when its
//    inputs changed and serves a "building the engine" page until it is done, so the
//    URL answers in seconds even when a cold build takes minutes.

import { spawn, spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const vite = join(web, 'node_modules', 'vite', 'bin', 'vite.js');

/** Does an optional package (a native binding) install on this machine? Its lock entry's `os` and `cpu`
 *  lists, `!x` excluding x; one with a `libc` list (Linux glibc or musl) is left unchecked. */
function forHere(p) {
  const fits = (list, v) =>
    !list || (list.some((x) => !x.startsWith('!')) ? list.includes(v) : !list.includes('!' + v));
  return !p.libc && fits(p.os, process.platform) && fits(p.cpu, process.arch);
}

/** Why node_modules needs `npm ci`, or '' when every locked package is installed at its version. */
function stale() {
  if (!existsSync(vite)) return 'node_modules has no vite';
  const installed = join(web, 'node_modules', '.package-lock.json');
  if (!existsSync(installed)) return 'node_modules has no .package-lock.json';
  try {
    const want = JSON.parse(readFileSync(join(web, 'package-lock.json'), 'utf8')).packages ?? {};
    const have = JSON.parse(readFileSync(installed, 'utf8')).packages ?? {};
    for (const [path, p] of Object.entries(want)) {
      if (!path || p.peer || (p.optional && !forHere(p))) continue; // the root, or a binding for another platform
      if (have[path]?.version !== p.version) return `${path} is not ${p.version}`;
    }
  } catch (e) {
    return `reading the lock files failed (${e.message})`;
  }
  return '';
}

const why = stale();
if (why) {
  console.log(`dev: ${why}; running npm ci`);
  const ci = spawnSync('npm ci --no-audit --no-fund', { cwd: web, stdio: 'inherit', shell: true });
  if (ci.status !== 0 || !existsSync(vite)) {
    console.error('dev: npm ci failed; the dev server cannot start');
    process.exit(ci.status || 1);
  }
}

const child = spawn(process.execPath, [vite, ...process.argv.slice(2)], { cwd: web, stdio: 'inherit' });
for (const sig of ['SIGINT', 'SIGTERM', 'SIGBREAK']) process.on(sig, () => child.kill(sig));
child.on('exit', (code, sig) => process.exit(code ?? (sig ? 1 : 0)));
