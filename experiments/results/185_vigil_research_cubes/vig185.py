"""The Vigil cap as a lever (f185-vigil: caps 2 and 10) against the ladder's cap 1 on the same seeds, the strategist.
Win % and Vigil insight a game by difficulty, the Protector pairs (bp, pr) pooled and br alone.

    python experiments/results/185_vigil_research_cubes/vig185.py VIG_DIR LADDER_DIR

The ladder's games are read only on the lever's seeds.
"""
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts', 'bench')))
from ladder import load  # noqa: E402


def collect(d, cap_of, seeds=None):
    out = defaultdict(list)
    for cell, g in load(d):
        if seeds is not None and g['seed'] not in seeds:
            continue
        name = cell.get('variant') or cell.get('name') or ''
        pair, cap = cap_of(name)
        out[(cap, 'br' if pair == 'br' else 'bp+pr', cell['difficulty'])].append(g)
    return out


def main():
    vig, lad = sys.argv[1], sys.argv[2]
    v = collect(vig, lambda n: (n.split('-')[0], int(n.split('-c')[1])))
    seeds = {g['seed'] for gs in v.values() for g in gs}
    v.update(collect(lad, lambda n: (n, 1), seeds))
    print('| cap | pairs | d | games | win % | Vigil insight a game |')
    print('|---|---|---|---|---|---|')
    for key in sorted(v):
        gs = v[key]
        win = 100 * sum(1 for g in gs if g['won']) / len(gs)
        vig_n = sum(g['counters'].get('board.vigil', 0) for g in gs) / len(gs)
        print(f'| {key[0]} | {key[1]} | d{key[2]} | {len(gs)} | {win:.1f} | {vig_n:.1f} |')


if __name__ == '__main__':
    main()
