"""178: the paired gap by board pair and level (which seats the r77 change helps).

    python experiments/results/178_rules_words/pairs178.py NEW_DIR OLD_DIR

Pairs each (cell, seed) across the two dirs (same spec grid) and prints win % new / old and the gap with its paired
SE, per pair (bp, br, pr) and level.
"""
import json
import math
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts')))
from screens import cells_of  # noqa: E402


def read(d):
    with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
        cells = cells_of(json.loads(f.readline())['spec'])
        out = {}
        for line in f:
            g = json.loads(line)
            c = cells[g['cell']]
            out[(c['variant'], c['difficulty'], c['agent'], g['seed'])] = 1.0 if g['won'] else 0.0
    return out


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    new, old = read(sys.argv[1]), read(sys.argv[2])
    by = defaultdict(list)
    for k, v in new.items():
        if k in old:
            by[(k[0], k[1])].append((v, old[k]))
    print('| pair | d | new | old | gap | ±SE | n |')
    print('|---|---|---|---|---|---|---|')
    for (pair, d) in sorted(by):
        xs = by[(pair, d)]
        n = len(xs)
        diffs = [a - b for a, b in xs]
        m = sum(diffs) / n
        sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(n - 1, 1))
        print(f'| {pair} | {d} | {100 * sum(a for a, _ in xs) / n:.1f} | {100 * sum(b for _, b in xs) / n:.1f} | '
              f'{100 * m:+.1f} | {100 * sd / math.sqrt(n):.1f} | {n} |')


if __name__ == '__main__':
    main()
