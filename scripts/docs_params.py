"""307: hand-written docs prose quoting a tunable as "`param` N" or "`param` (N)", against configs/baseline.json.

    python scripts/docs_params.py

Prints each quote whose number is not the baseline's (a list param: any of its entries) and exits 1 if any. 298's
r144 left three such in docs/rules/ and principles.md (Remove agents' `remove_ambassadors` (2), `ambassador_credits`
16), which a diff doesn't show. A quote naming a param's other setting on purpose (an off arm, a past value) goes in
`ALLOW` as (file, param, number). Scans README.md, CLAUDE.md and docs/ (generated pages included: they print the
baseline, so they agree). `test_input_checks.py` runs it.
"""
import glob
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
# a number followed by a range or a change ("2 to 3", "2 → 3") quotes a history, not the baseline
QUOTE = re.compile(r'`([a-z_][a-z0-9_.]*)` \(?(\d+(?:\.\d+)?)(?![\d.,]*\s*(?:to|→|->|-))')
ALLOW = {
    ('docs/rules/README.md', 'factory_chips', 0),  # the off switch
    ('docs/rules/README.md', 'adversary.botnet_compute', 0),  # the off switch
    ('docs/rules/ambassadors.md', 'cities_unowned', 0),  # r114's off arm
    ('docs/rules/board.md', 'cities_unowned', 0),
    ('docs/rules/economy.md', 'cities_unowned', 0),
    ('docs/rules/threats.md', 'adversary.jam_floor', 1),  # the floor's on value, off at baseline
}


def leaves():
    with open(os.path.join(ROOT, 'configs', 'baseline.json'), encoding='utf-8') as f:
        base = json.load(f)
    out = {}

    def walk(o, path):
        if isinstance(o, dict):
            for k, v in o.items():
                walk(v, path + [k])
        else:
            out.setdefault(path[-1], []).append(o)
    for m in base['modules']:
        walk(m.get('params', {}), [m['module']])
    return out


def pages():
    out = ['README.md', 'CLAUDE.md'] + glob.glob('docs/**/*.md', recursive=True, root_dir=ROOT)
    return sorted(p.replace('\\', '/') for p in out)


def stale():
    """(file, line, param, number, baseline values) for each quote off the baseline and not in ALLOW."""
    vals = leaves()
    bad = []
    for p in pages():
        with open(os.path.join(ROOT, p), encoding='utf-8') as f:
            for i, line in enumerate(f, 1):
                for m in QUOTE.finditer(line):
                    name, n = m[1], float(m[2])
                    vs = vals.get(name.split('.')[-1])
                    if vs is None or (p, name, n) in ALLOW:
                        continue
                    if not any(n in v if isinstance(v, list) else n == v for v in vs):
                        bad.append((p, i, name, m[2], vs))
    return bad


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    bad = stale()
    for p, i, name, n, vs in bad:
        print(f'{p}:{i}: `{name}` {n}, the baseline {", ".join(map(str, vs))}')
    if bad:
        sys.exit(1)
    print('docs quote the baseline')


if __name__ == '__main__':
    main()
