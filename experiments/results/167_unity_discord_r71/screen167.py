"""167: a screen's levers per level (variants `PAIR` and `PAIR_LEVER`), win % and the paired delta against base,
and, with OLD (a read of the r70 binary on the same seeds), each lever against r70.

    python experiments/results/167_unity_discord_r71/screen167.py SCREEN [OLD]
"""
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def load(d):
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    out = defaultdict(dict)
    for g in it:
        c = cells[g['cell']]
        pair, _, lever = c['variant'].partition('_')
        out[lever or 'base'][(pair, c['difficulty'], g['seed'])] = int(g['won'])
    return out


def delta(a, b):
    keys = [k for k in a if k in b]
    if not keys:
        return None
    d = [100.0 * (a[k] - b[k]) for k in keys]
    m = sum(d) / len(d)
    se = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1) / len(d))
    return m, se, len(d)


def main():
    s = load(sys.argv[1])
    old = load(sys.argv[2])['base'] if len(sys.argv) > 2 else None
    levels = sorted({k[1] for k in s['base']})
    for lever in sorted(s, key=lambda x: (x != 'base', x)):
        row = []
        for lvl in levels:
            a = {k: v for k, v in s[lever].items() if k[1] == lvl}
            win = 100.0 * sum(a.values()) / max(1, len(a))
            cell = f'd{lvl} {win:5.1f}'
            if lever != 'base':
                m, se, _ = delta(a, s['base'])
                cell += f' ({m:+.1f}±{se:.1f} v base)'
            if old is not None:
                r = delta(a, old)
                if r:
                    cell += f' [{r[0]:+.1f}±{r[1]:.1f} v r70]'
            row.append(cell)
        print(f'{lever:24s} ' + ' | '.join(row))


if __name__ == '__main__':
    main()
