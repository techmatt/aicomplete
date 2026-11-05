"""Regenerates everything a content, rules or tunable change moves, in order, from anywhere.

    python scripts/regen.py [--no-build] [--check]

Builds `aic-sim` (release; `CARGO_BUILD_JOBS` from the environment, default 3), then runs
`aic-sim docs`, `scripts/cards/cards_doc.py`, `scripts/cards/effect_ledger.py` and
`scripts/configs/shared_row.py` and `scripts/configs/no_liberate.py` (the derived configs: 122 left
no_liberate.json stale when the baseline gained params), prints the content hash and rules revision from `aic-sim show
configs/baseline.json` and then `git status --short`. Stops at the first failing step, naming
it. `--no-build` uses the binary already built. `--check` writes nothing: it runs `aic-sim docs
--check` and `cards_doc.py --check` only and exits 1 if either is stale. Each step prints its
wall time.
"""
import argparse
import os
import re
import subprocess
import sys
import time

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
SIM = os.path.join(ROOT, 'target', 'release', 'aic-sim' + ('.exe' if os.name == 'nt' else ''))
PY = sys.executable


def step(name, cmd, env=None, capture=False):
    """Runs one step in ROOT; exits naming it if it fails. Returns its stdout when captured."""
    print(f'== {name}', flush=True)
    t = time.monotonic()
    r = subprocess.run(cmd, cwd=ROOT, env=env, text=True, encoding='utf-8', errors='replace',
                       stdout=subprocess.PIPE if capture else None)
    dt = time.monotonic() - t
    if r.returncode != 0:
        print(f'FAILED: {name} (exit {r.returncode}, {dt:.1f} s)', flush=True)
        sys.exit(1)
    print(f'   {name}: {dt:.1f} s', flush=True)
    return r.stdout


def need_sim():
    if not os.path.exists(SIM):
        print(f'FAILED: no binary at {SIM} (build it, or drop --no-build)')
        sys.exit(1)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('--no-build', action='store_true', help='skip cargo build; use the binary already built')
    ap.add_argument('--check', action='store_true',
                    help='only check: aic-sim docs --check and cards_doc.py --check; exit 1 if stale')
    a = ap.parse_args()
    start = time.monotonic()
    if not a.no_build:
        env = dict(os.environ)
        env.setdefault('CARGO_BUILD_JOBS', '3')
        step(f"cargo build --release -p aic-sim (jobs {env['CARGO_BUILD_JOBS']})",
             ['cargo', 'build', '--release', '-p', 'aic-sim'], env=env)
    need_sim()
    if a.check:
        step('aic-sim docs --check', [SIM, 'docs', '--check'])
        step('cards_doc.py --check', [PY, os.path.join('scripts', 'cards', 'cards_doc.py'), '--check'])
        print(f'docs current ({time.monotonic() - start:.1f} s)')
        return
    step('aic-sim docs', [SIM, 'docs'])
    step('cards_doc.py', [PY, os.path.join('scripts', 'cards', 'cards_doc.py')])
    step('effect_ledger.py', [PY, os.path.join('scripts', 'cards', 'effect_ledger.py')])
    step('shared_row.py', [PY, os.path.join('scripts', 'configs', 'shared_row.py')])
    step('no_liberate.py', [PY, os.path.join('scripts', 'configs', 'no_liberate.py')])
    out = step('aic-sim show', [SIM, 'show', os.path.join('configs', 'baseline.json')], capture=True)
    first = out.splitlines()[0] if out else ''
    m = re.search(r'content hash (\S+)\s+rules revision (\S+)', first)
    if m:
        print(f'content hash {m.group(1)}  rules revision {m.group(2)}')
    else:
        print(f'could not read the hashes from: {first!r}')
    step('git status --short', ['git', 'status', '--short'])
    print(f'done ({time.monotonic() - start:.1f} s)')


if __name__ == '__main__':
    main()
