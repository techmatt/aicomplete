"""Rounds by level and by how a game ended, from one or more games.jsonl files.

python rounds_read.py GAMES.jsonl [GAMES.jsonl ...] [--by-variant]
Prints per difficulty: games, win %, mean rounds (all, wins, losses), and the end mix
(reason: share, mean rounds). A deck_empty loss counts one past the cap (docs/experiments.md).
"""
import json
import sys
from collections import defaultdict


def read(path):
    with open(path, encoding='utf-8') as f:
        head = json.loads(f.readline())
        for line in f:
            line = line.strip()
            if line:
                yield head, json.loads(line)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    by_variant = '--by-variant' in sys.argv
    for path in args:
        cells = defaultdict(list)
        variants = None
        for head, g in read(path):
            if variants is None:
                variants = [v['name'] for v in head['spec'].get('variants', [{'name': 'base'}])]
                nag = len(head['spec']['agents'])
            key = g['difficulty']
            if by_variant:
                vi = g['cell'] // (len(head['spec']['difficulties']) * nag) if False else None
                key = (g.get('variant', g['cell']), g['difficulty'])
            cells[key].append(g)
        print(f'== {path}')
        print('key | n | win% | rounds | win rds | loss rds | ends (share, rounds)')
        for key in sorted(cells):
            gs = cells[key]
            n = len(gs)
            wins = [g for g in gs if g['won']]
            loss = [g for g in gs if not g['won']]
            mean = lambda xs: sum(xs) / len(xs) if xs else float('nan')
            ends = defaultdict(list)
            for g in gs:
                ends[g['reason']].append(g['rounds'])
            mix = ', '.join(f'{r} {len(v) / n:.0%} {mean(v):.1f}'
                            for r, v in sorted(ends.items(), key=lambda kv: -len(kv[1])))
            print(f'{key} | {n} | {100 * len(wins) / n:.1f} | {mean([g["rounds"] for g in gs]):.2f} | '
                  f'{mean([g["rounds"] for g in wins]):.2f} | {mean([g["rounds"] for g in loss]):.2f} | {mix}')


if __name__ == '__main__':
    main()
