"""Measures gone quiet: counter names the read scripts and the sweep read that no game writes.

    python scripts/cards/counter_names.py DIR... [--all]

Scans scripts/**/*.py, web/scripts/*.mjs and crates/aic-sim/src (not its agents) for quoted counter keys (`board.`, `adversary.`, `threats.`, ...;
a key ending in `.` is a prefix) and checks each against the counters of every game in the given experiment
outputs (`games.jsonl`). Prints the keys no game wrote, by file: a name a rules change retired, or one the
engine writes under another name (145: the sweep's `adversary.sites_created`, written only with a site suffix).
A key a run simply never reached (a rare event, a variant off) also lists: read the list, don't gate on it.
`--all` prints the keys found too. Exits 0.
"""
import argparse
import glob
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
PREFIXES = ('adversary', 'alignment', 'board', 'cards', 'threats', 'economy', 'ai', 'effects', 'slots', 'actions',
            'energy_cards', 'unspent')
KEY = re.compile(r"""['"`]((?:%s)\.[a-z0-9_.]+)['"`]""" % '|'.join(PREFIXES))


def written(dirs):
    keys = set()
    for d in dirs:
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            for line in f:
                g = json.loads(line)
                c = g.get('counters') or {}
                if isinstance(c, list):
                    c = dict(c)
                keys.update(k for k, v in c.items() if v)
    return keys


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--all', action='store_true')
    a = ap.parse_args()
    keys = written(a.dirs)
    # The sim's reports and gates read counters too (crates/aic-sim, not its agents: they read state).
    files = sorted(glob.glob(os.path.join(ROOT, 'scripts', '**', '*.py'), recursive=True)
                   + glob.glob(os.path.join(ROOT, 'web', 'scripts', '*.mjs'))
                   + [f for f in glob.glob(os.path.join(ROOT, 'crates', 'aic-sim', 'src', '**', '*.rs'), recursive=True)
                      if os.sep + 'agents' + os.sep not in f])
    quiet = 0
    for path in files:
        if os.path.abspath(path) == os.path.abspath(__file__):
            continue
        with open(path, encoding='utf-8') as f:
            text = f.read()
        names = sorted(n for n in set(KEY.findall(text)) if not re.search(r'\.(json|csv|md|html|png)$', n))
        # A config path (`board.unit_cap` in a --set) is not a counter: only names some game wrote, or that
        # look like one, are checked; config params are read from the baseline and left out.
        missing = [n for n in names if n not in PARAMS and not any(k == n or k.startswith(n if n.endswith('.') else n + '.')
                                                                   for k in keys)]
        if missing or a.all:
            rel = os.path.relpath(path, ROOT).replace(os.sep, '/')
            print(f'{rel}: ' + ', '.join(missing) if missing else f'{rel}: all {len(names)} written')
            quiet += len(missing)
    print(f'\n{quiet} names no game in {len(a.dirs)} output(s) wrote ({len(keys)} counter keys written)')


def params():
    """Every `<module>.<param>` of the baseline config, so a --set isn't read as a counter."""
    with open(os.path.join(ROOT, 'configs', 'baseline.json'), encoding='utf-8') as f:
        cfg = json.load(f)
    out = set()
    for m in cfg.get('modules', []):
        name = m.get('module') or m.get('name')
        for k in (m.get('params') or {}):
            out.add(f'{name}.{k}')
    return out


PARAMS = params()

if __name__ == '__main__':
    main()
