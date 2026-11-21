"""306: antimatter spent and made a seat-game, paired by (cell, seed) between two farmed reads: each side's mean and
the paired gap with its SE.

    python experiments/results/306_finale_buildings/am_paired.py OLD.jsonl NEW.jsonl
"""
import json
import math
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4  # scripts/screens.py PEAKS: credits, insight, energy, chips, antimatter, ...


def read(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            seats = len(g.get('start', [])) or 2
            out[(g['cell'], g['seed'])] = (sum(x[AM] for x in g['spent']) / seats, sum(x[AM] for x in g['gained']) / seats)
    return out


def main():
    a, b = read(sys.argv[1]), read(sys.argv[2])
    keys = sorted(set(a) & set(b))
    for i, name in ((0, 'spent'), (1, 'made')):
        d = [b[k][i] - a[k][i] for k in keys]
        n = len(d)
        m = sum(d) / n
        se = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1) / n)
        print('%s a seat-game: %.2f -> %.2f, paired gap %+.2f +- %.2f (%d games)' % (
            name, sum(a[k][i] for k in keys) / n, sum(b[k][i] for k in keys) / n, m, se, n))


if __name__ == '__main__':
    main()
