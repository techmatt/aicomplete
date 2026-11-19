"""Regenerates everything a content, rules or tunable change moves, in order, from anywhere.

    python scripts/regen.py [--no-build] [--check | --golden]

Checks the drafts (`drafts/adversary/check.py`, 182: 178 left it red unseen) and writes `content/` from
`drafts/adversary/` (`scripts/content/load_adversary.py`, which refuses on a check ERROR), both before the build
because content is compiled into the binary; then builds `aic-sim` (release; `CARGO_BUILD_JOBS` from the environment,
default 3), sets docs/rules/README.md's "at rules revision N" line and docs/rulebook.md's "Rules revision N" (283)
from `RULES_REVISION` in the source, then runs
`aic-sim docs`, `scripts/cards/cards_doc.py`, `scripts/cards/effect_ledger.py` and `scripts/configs/no_liberate.py`,
`no_tier2.py` and `three_seat.py` (the derived configs: 122 left no_liberate.json stale when the baseline gained
params), prints the content hash and rules revision from `aic-sim show configs/baseline.json` (warning if the binary's
revision isn't the source's), and then `git status --short`. Stops at the first failing step, naming it. `--no-build`
uses the binary already built (warning if the loader just rewrote the content it embeds).
`--check` writes nothing: it runs the drafts check, the loader's, `aic-sim docs`'s, `cards_doc.py`'s and the derived
configs' `--check` and the revision lines, and exits 1 if any is stale. Each step prints its wall time.
`--golden` (281) also sets the golden constants, after the steps above: `GOLDEN_REVISION`, `GOLDEN_FINGERPRINT` and
`GOLDEN_CONTENT_HASH` in crates/aic-core/src/tests/mod.rs and `GOLDEN` / `GOLDEN_CONTENT` in scripts/wasm-smoke.mjs.
The revision is the source's `RULES_REVISION`, the content hash `aic-sim show`'s, and the fingerprint the left side
of a failing `cargo test -p aic-core --lib golden` (debug, the environment's `CARGO_TARGET_DIR`); it rewrites only
those values, prints each change and reruns the tests until they pass (at most 3 runs).
"""
import argparse
import os
import re
import subprocess
import sys
import time

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
# The build's binary: under CARGO_TARGET_DIR when the environment sets one (cargo reads it relative to ROOT here).
SIM = os.path.join(ROOT, os.environ.get('CARGO_TARGET_DIR') or 'target', 'release',
                   'aic-sim' + ('.exe' if os.name == 'nt' else ''))
PY = sys.executable


def step(name, cmd, env=None, capture=False):
    """Runs one step in ROOT; exits naming it if it fails. Returns its stdout when captured."""
    print(f'== {name}', flush=True)
    t = time.monotonic()
    r = subprocess.run(cmd, cwd=ROOT, env=env, text=True, encoding='utf-8', errors='replace',
                       stdout=subprocess.PIPE if capture else None)
    dt = time.monotonic() - t
    if r.returncode != 0:
        if capture and r.stdout:
            print(r.stdout, end='')
        print(f'FAILED: {name} (exit {r.returncode}, {dt:.1f} s)', flush=True)
        sys.exit(1)
    print(f'   {name}: {dt:.1f} s', flush=True)
    return r.stdout


# The docs' revision lines `aic-sim docs` checks against RULES_REVISION (283: the rulebook's too, which a bump
# otherwise failed on until edited by hand).
REVISION_LINES = [
    ('docs/rules/README.md', re.compile(r'(What the code implements, at rules revision )(\d+)')),
    ('docs/rulebook.md', re.compile(r'(\*Rules revision )(\d+)')),
]


def readme_revision(revision, check):
    """Sets each doc's revision line (REVISION_LINES: the rules README's "at rules revision N", the rulebook's
    "Rules revision N") to `revision`; with `check`, only reports whether one is stale. Returns True when every
    one matched already or was set."""
    ok = True
    for rel, line in REVISION_LINES:
        path = os.path.join(ROOT, *rel.split('/'))
        with open(path, encoding='utf-8') as f:
            text = f.read()
        m = line.search(text)
        if not m:
            print(f'FAILED: no revision line in {path}')
            sys.exit(1)
        if m.group(2) == revision:
            continue
        if check:
            print(f'{rel} says rules revision {m.group(2)}, the code {revision}')
            ok = False
            continue
        with open(path, 'w', encoding='utf-8', newline='\n') as f:
            f.write(line.sub(lambda x: x.group(1) + revision, text, count=1))
        print(f'{rel}: rules revision {m.group(2)} -> {revision}')
    return ok


def source_revision():
    """RULES_REVISION as crates/aic-core/src/lib.rs states it: `aic-sim docs` refuses a stale README line, so the
    line is set from the source before that step (144)."""
    with open(os.path.join(ROOT, 'crates', 'aic-core', 'src', 'lib.rs'), encoding='utf-8') as f:
        m = re.search(r'RULES_REVISION: u32 = (\d+)', f.read())
    if not m:
        print('FAILED: no RULES_REVISION in crates/aic-core/src/lib.rs')
        sys.exit(1)
    return m.group(1)


def content_form():
    """221: warns of each content/*.json (alignment.json aside, hand-laid by design) not in the form
    `scripts/cards/apply_overlay.py` writes, which refuses such a file: 212 (cards.json) and 213 (map.json) each lost
    an overlay to a hand-joined line."""
    import json
    d = os.path.join(ROOT, 'content')
    for name in sorted(os.listdir(d)):
        if not name.endswith('.json') or name == 'alignment.json':
            continue
        with open(os.path.join(d, name), encoding='utf-8') as f:
            text = f.read()
        if text != json.dumps(json.loads(text), indent=1, ensure_ascii=True) + '\n':
            print(f'warning: content/{name} is not in json.dumps(indent=1) form, so apply_overlay.py refuses it: '
                  f'rewrite it once (data unchanged; the content hash moves)')


# The golden constants (281): name -> [(file, pattern)], each pattern's group 2 the value.
GOLDEN_RS = 'crates/aic-core/src/tests/mod.rs'
GOLDEN_JS = 'scripts/wasm-smoke.mjs'
GOLDEN = {
    'GOLDEN_REVISION': [(GOLDEN_RS, r'(const GOLDEN_REVISION: u32 = )(\d+)(;)')],
    'GOLDEN_FINGERPRINT': [(GOLDEN_RS, r'(const GOLDEN_FINGERPRINT: &str = ")([0-9a-f]+)(";)'),
                           (GOLDEN_JS, r"(const GOLDEN = ')([0-9a-f]+)(';)")],
    'GOLDEN_CONTENT_HASH': [(GOLDEN_RS, r'(const GOLDEN_CONTENT_HASH: &str = ")([0-9a-f]+)(";)'),
                            (GOLDEN_JS, r"(const GOLDEN_CONTENT = ')([0-9a-f]+)(';)")],
}


def set_golden(name, value):
    """Sets golden constant `name` to `value` in every file holding it; returns True if any changed."""
    changed = False
    for rel, pat in GOLDEN[name]:
        path = os.path.join(ROOT, rel)
        with open(path, encoding='utf-8') as f:
            text = f.read()
        found = list(re.finditer(pat, text))
        if len(found) != 1:
            print(f'FAILED: {len(found)} matches of {name} in {rel} (want 1)')
            sys.exit(1)
        old = found[0].group(2)
        if old == value:
            continue
        with open(path, 'w', encoding='utf-8', newline='\n') as f:
            f.write(re.sub(pat, lambda x: x.group(1) + value + x.group(3), text, count=1))
        print(f'{rel}: {name} {old} -> {value}')
        changed = True
    return changed


def golden(content_hash):
    """Sets the golden constants (see the module docstring) and confirms `golden_game` passes."""
    set_golden('GOLDEN_REVISION', source_revision())
    if content_hash:
        set_golden('GOLDEN_CONTENT_HASH', content_hash)
    env = dict(os.environ)
    env.setdefault('CARGO_BUILD_JOBS', '3')
    cmd = ['cargo', 'test', '-p', 'aic-core', '--lib', 'golden']
    for run in range(3):
        print(f"== {' '.join(cmd)} (run {run + 1})", flush=True)
        t = time.monotonic()
        r = subprocess.run(cmd, cwd=ROOT, env=env, text=True, encoding='utf-8', errors='replace',
                           stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        print(f'   {time.monotonic() - t:.1f} s', flush=True)
        if r.returncode == 0:
            print('golden tests pass')
            return
        # The (revision, fingerprint) pair is asserted first, then the content hash: one fix a run.
        pair = re.search(r'left: \((\d+), "([0-9a-f]+)"\)', r.stdout)
        hash_ = re.search(r'left: "([0-9a-f]+)"\s*\n\s*right: "[0-9a-f]+"', r.stdout)
        if pair:
            changed = set_golden('GOLDEN_REVISION', pair.group(1))
            changed |= set_golden('GOLDEN_FINGERPRINT', pair.group(2))
        elif hash_:
            print(f'warning: the test reads content hash {hash_.group(1)}, aic-sim show {content_hash}: '
                  f'is the binary stale?')
            changed = set_golden('GOLDEN_CONTENT_HASH', hash_.group(1))
        else:
            changed = False
        if not changed:
            print('\n'.join(r.stdout.splitlines()[-30:]))
            print('FAILED: golden tests fail with nothing --golden can set (output above)')
            sys.exit(1)
    print('FAILED: golden tests still fail after 3 runs')
    sys.exit(1)


def need_sim():
    if not os.path.exists(SIM):
        print(f'FAILED: no binary at {SIM} (build it, or drop --no-build)')
        sys.exit(1)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('--no-build', action='store_true', help='skip cargo build; use the binary already built')
    ap.add_argument('--check', action='store_true',
                    help='only check: load_adversary.py --check, aic-sim docs --check, cards_doc.py --check, the derived configs '
                         'and the revision lines (docs/rules/README.md, docs/rulebook.md); exit 1 if stale')
    ap.add_argument('--golden', action='store_true',
                    help='also set the golden constants (tests/mod.rs, wasm-smoke.mjs) and confirm golden_game')
    a = ap.parse_args()
    if a.check and a.golden:
        ap.error('--golden writes; --check writes nothing')
    start = time.monotonic()
    loader = [PY, os.path.join('scripts', 'content', 'load_adversary.py')]
    step('drafts/adversary/check.py', [PY, os.path.join('drafts', 'adversary', 'check.py')])
    if a.check:
        step('load_adversary.py --check', loader + ['--check'])
    else:
        # content/ is written from drafts/adversary/ (its source) before the build: the binary embeds it.
        out = step('load_adversary.py', loader, capture=True)
        if out:
            print(out, end='')
            if a.no_build and 'wrote ' in out:
                print('warning: the loader rewrote content/ and --no-build keeps a binary built from the old: rebuild')
    if not a.no_build:
        env = dict(os.environ)
        env.setdefault('CARGO_BUILD_JOBS', '3')
        step(f"cargo build --release -p aic-sim (jobs {env['CARGO_BUILD_JOBS']})",
             ['cargo', 'build', '--release', '-p', 'aic-sim'], env=env)
    need_sim()
    if a.check:
        step('aic-sim docs --check', [SIM, 'docs', '--check'])
        step('cards_doc.py --check', [PY, os.path.join('scripts', 'cards', 'cards_doc.py'), '--check'])
        for derived in ('no_liberate.py', 'no_tier2.py', 'three_seat.py'):  # 284: the derived configs
            step(f'{derived} --check', [PY, os.path.join('scripts', 'configs', derived), '--check'])
        if not readme_revision(source_revision(), check=True):
            sys.exit(1)
        print(f'docs current ({time.monotonic() - start:.1f} s)')
        return
    readme_revision(source_revision(), check=False)
    step('aic-sim docs', [SIM, 'docs'])
    step('cards_doc.py', [PY, os.path.join('scripts', 'cards', 'cards_doc.py')])
    step('effect_ledger.py', [PY, os.path.join('scripts', 'cards', 'effect_ledger.py')])
    step('no_liberate.py', [PY, os.path.join('scripts', 'configs', 'no_liberate.py')])
    step('no_tier2.py', [PY, os.path.join('scripts', 'configs', 'no_tier2.py')])
    step('three_seat.py', [PY, os.path.join('scripts', 'configs', 'three_seat.py')])
    content_form()
    out = step('aic-sim show', [SIM, 'show', os.path.join('configs', 'baseline.json')], capture=True)
    first = out.splitlines()[0] if out else ''
    m = re.search(r'content hash (\S+)\s+rules revision (\S+)', first)
    content_hash = m.group(1) if m else None
    if m:
        print(f'content hash {m.group(1)}  rules revision {m.group(2)}')
        if m.group(2) != source_revision():
            print(f'warning: the binary reads rules revision {m.group(2)}, the source {source_revision()}: rebuild')
    else:
        print(f'could not read the hashes from: {first!r}')
    if a.golden:
        golden(content_hash)
    step('git status --short', ['git', 'status', '--short'])
    print(f'done ({time.monotonic() - start:.1f} s)')


if __name__ == '__main__':
    main()
