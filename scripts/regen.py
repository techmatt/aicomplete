"""Regenerates everything a content, rules or tunable change moves, in order, from anywhere.

    python scripts/regen.py [--no-build] [--check]

Builds `aic-sim` (release; `CARGO_BUILD_JOBS` from the environment, default 3), checks the drafts
(`drafts/adversary/check.py`, 182: 178 left it red unseen) and that `content/` matches `drafts/adversary/`
(`load_adversary.py --check`; run the loader without it to write), sets docs/rules/README.md's "at rules revision N"
line from `RULES_REVISION` in the source, then runs `aic-sim docs`, `scripts/cards/cards_doc.py`,
`scripts/cards/effect_ledger.py` and `scripts/configs/shared_row.py` and `scripts/configs/no_liberate.py` and `no_tier2.py` (the
derived configs: 122 left no_liberate.json stale when the baseline gained params), prints the content hash and rules
revision from `aic-sim show configs/baseline.json` (warning if the binary's revision isn't the source's), and then
`git status --short`. Stops at the first failing step, naming it. `--no-build` uses the binary already built.
`--check` writes nothing: it runs the drafts check, the loader's, `aic-sim docs`'s and `cards_doc.py`'s `--check`
and the README revision line, and exits 1 if any is stale. Each step prints its wall time.
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


README_LINE = re.compile(r'(What the code implements, at rules revision )(\d+)')


def readme_revision(revision, check):
    """Sets the rules README's "at rules revision N" line to `revision`; with `check`, only
    reports whether it is stale. Returns True when it matched already."""
    path = os.path.join(ROOT, 'docs', 'rules', 'README.md')
    with open(path, encoding='utf-8') as f:
        text = f.read()
    m = README_LINE.search(text)
    if not m:
        print(f'FAILED: no "at rules revision N" line in {path}')
        sys.exit(1)
    if m.group(2) == revision:
        return True
    if check:
        print(f'docs/rules/README.md says rules revision {m.group(2)}, the code {revision}')
        return False
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        f.write(README_LINE.sub(lambda x: x.group(1) + revision, text, count=1))
    print(f'docs/rules/README.md: rules revision {m.group(2)} -> {revision}')
    return True


def source_revision():
    """RULES_REVISION as crates/aic-core/src/lib.rs states it: `aic-sim docs` refuses a stale README line, so the
    line is set from the source before that step (144)."""
    with open(os.path.join(ROOT, 'crates', 'aic-core', 'src', 'lib.rs'), encoding='utf-8') as f:
        m = re.search(r'RULES_REVISION: u32 = (\d+)', f.read())
    if not m:
        print('FAILED: no RULES_REVISION in crates/aic-core/src/lib.rs')
        sys.exit(1)
    return m.group(1)


def need_sim():
    if not os.path.exists(SIM):
        print(f'FAILED: no binary at {SIM} (build it, or drop --no-build)')
        sys.exit(1)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('--no-build', action='store_true', help='skip cargo build; use the binary already built')
    ap.add_argument('--check', action='store_true',
                    help='only check: load_adversary.py --check, aic-sim docs --check, cards_doc.py --check '
                         'and the rules README revision line; exit 1 if stale')
    a = ap.parse_args()
    start = time.monotonic()
    if not a.no_build:
        env = dict(os.environ)
        env.setdefault('CARGO_BUILD_JOBS', '3')
        step(f"cargo build --release -p aic-sim (jobs {env['CARGO_BUILD_JOBS']})",
             ['cargo', 'build', '--release', '-p', 'aic-sim'], env=env)
    need_sim()
    loader = [PY, os.path.join('scripts', 'content', 'load_adversary.py'), '--check']
    step('drafts/adversary/check.py', [PY, os.path.join('drafts', 'adversary', 'check.py')])
    if a.check:
        step('load_adversary.py --check', loader)
        step('aic-sim docs --check', [SIM, 'docs', '--check'])
        step('cards_doc.py --check', [PY, os.path.join('scripts', 'cards', 'cards_doc.py'), '--check'])
        if not readme_revision(source_revision(), check=True):
            sys.exit(1)
        print(f'docs current ({time.monotonic() - start:.1f} s)')
        return
    # content/ must match drafts/adversary/ (its source) before the docs read it; the loader writes it.
    step('load_adversary.py --check', loader)
    readme_revision(source_revision(), check=False)
    step('aic-sim docs', [SIM, 'docs'])
    step('cards_doc.py', [PY, os.path.join('scripts', 'cards', 'cards_doc.py')])
    step('effect_ledger.py', [PY, os.path.join('scripts', 'cards', 'effect_ledger.py')])
    step('shared_row.py', [PY, os.path.join('scripts', 'configs', 'shared_row.py')])
    step('no_liberate.py', [PY, os.path.join('scripts', 'configs', 'no_liberate.py')])
    step('no_tier2.py', [PY, os.path.join('scripts', 'configs', 'no_tier2.py')])
    out =step('aic-sim show', [SIM, 'show', os.path.join('configs', 'baseline.json')], capture=True)
    first = out.splitlines()[0] if out else ''
    m = re.search(r'content hash (\S+)\s+rules revision (\S+)', first)
    if m:
        print(f'content hash {m.group(1)}  rules revision {m.group(2)}')
        if m.group(2) != source_revision():
            print(f'warning: the binary reads rules revision {m.group(2)}, the source {source_revision()}: rebuild')
    else:
        print(f'could not read the hashes from: {first!r}')
    step('git status --short', ['git', 'status', '--short'])
    print(f'done ({time.monotonic() - start:.1f} s)')


if __name__ == '__main__':
    main()
