"""167: paired win % per level, NEW against OLD, on the seeds both played (any number of farmed reads).

    python experiments/results/167_unity_discord_r71/pair167.py NEW OLD

Prints per level: games, NEW win %, OLD win %, the paired delta and its SE, and each side's loss reasons.
"""
import json
import math
import os
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def load(d):
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    out = {}
    for g in it:
        c = cells[g['cell']]
        out[(c['variant'], c['difficulty'], g['seed'])] = g
    return out


def main():
    new, old = load(sys.argv[1]), load(sys.argv[2])
    by = defaultdict(list)
    why = defaultdict(lambda: (Counter(), Counter()))
    for k, g in new.items():
        if k in old:
            o = old[k]
            by[k[1]].append((int(g['won']), int(o['won'])))
            if not g['won']:
                why[k[1]][0][g['reason']] += 1
            if not o['won']:
                why[k[1]][1][o['reason']] += 1
    allp = []
    for lvl in sorted(by):
        p = by[lvl]
        allp += p
        n = len(p)
        d = [100.0 * (a - b) for a, b in p]
        m = sum(d) / n
        se = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, n - 1) / n)
        nw = 100.0 * sum(a for a, _ in p) / n
        ow = 100.0 * sum(b for _, b in p) / n
        print(f'd{lvl}: n {n}, new {nw:.1f}, old {ow:.1f}, delta {m:+.1f} ± {se:.1f}; '
              f'new losses {dict(why[lvl][0])}, old {dict(why[lvl][1])}')


if __name__ == '__main__':
    main()
