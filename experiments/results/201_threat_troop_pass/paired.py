"""200: paired win rates by lever and level, against a reference cell on the same seeds.

    python experiments/results/200_engine_overhaul/paired.py REF=DIR[:LEVER] CELL=DIR[:LEVER] ...

Each argument names a side: LABEL=DIR, optionally :LEVER (a variant name's suffix after `_`; `base` is the bare pair
name). The first side is the reference; each other side is paired with it game by game on (pair, level, seed). Prints
win % a level and over all, the paired gap and its SE.
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(arg):
    label, _, rest = arg.partition('=')
    d, _, lever = rest.partition(':')
    lever = lever or 'base'
    out = {}
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        head = json.loads(next(f))
        sp = head['spec']
        names = [v['name'] for v in sp['variants']]
        diffs = sp['difficulties']
        agents = sp.get('agents') or ['?']
        for line in f:
            g = json.loads(line)
            cell = int(g['cell'])
            per_variant = len(diffs) * len(agents)
            name = names[cell // per_variant]
            pair, _, lv = name.partition('_')
            if (lv or 'base') != lever:
                continue
            out[(pair, int(g['difficulty']), int(g['seed']))] = 1.0 if g['won'] else 0.0
    return label, out


def main():
    sides = [load(a) for a in sys.argv[1:]]
    ref_label, ref = sides[0]
    levels = sorted({k[1] for k in ref})
    print('| side | ' + ' | '.join(f'd{d}' for d in levels) + ' | all | gap ± SE (paired, all) | games |')
    print('|---|' + '---|' * len(levels) + '---|---|---|')
    for label, s in sides:
        keys = [k for k in s if k in ref]
        row = []
        for d in levels:
            ks = [k for k in keys if k[1] == d]
            if not ks:
                row.append('-')
                continue
            w = sum(s[k] for k in ks) / len(ks)
            gap = [s[k] - ref[k] for k in ks]
            m = sum(gap) / len(gap)
            row.append(f'{100 * w:.1f} ({100 * m:+.1f})' if label != ref_label else f'{100 * w:.1f}')
        diffs = [s[k] - ref[k] for k in keys]
        n = len(diffs)
        if not n:
            continue
        m = sum(diffs) / n
        var = sum((x - m) ** 2 for x in diffs) / max(n - 1, 1)
        se = math.sqrt(var / n)
        allw = sum(s[k] for k in keys) / n
        print(f'| {label} | ' + ' | '.join(row) + f' | {100 * allw:.1f} | {100 * m:+.1f} ± {100 * se:.1f} | {n} |')


if __name__ == '__main__':
    main()
