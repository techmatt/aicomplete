"""121: the carry screen against the r48 read, paired on seed.

    python experiments/results/121_loose_threads/carry121.py R48_DIR CARRY_DIR [R47_DIR]

R48_DIR is screen-121-r48 (the baseline, carry 1: Unity's side only, Matt), CARRY_DIR
screen-121-carry (variants PAIR-c2: both sides, PAIR-c0: neither). Per agent, pooled over the
three pairs: win %, mean rounds, ends and tips at each carry level (and r47 with R47_DIR), and
each arm's paired gap against the baseline.
"""
import math
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402


def load(d):
    lines = stream(d)
    names = cell_labels(next(lines)['spec'])
    out = {}
    for g in lines:
        v, a = names[g['cell']].split('/')
        pair, _, carry = v.partition('-')
        out[(carry or 'c1', pair, a, g['seed'])] = g
    return out


def mean(xs):
    xs = list(xs)
    return sum(xs) / len(xs) if xs else float('nan')


def se(xs):
    xs = list(xs)
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs)) if len(xs) > 1 else float('nan')


def main():
    games = load(sys.argv[1])
    games.update(load(sys.argv[2]))
    if len(sys.argv) > 3:
        games.update({('r47',) + k[1:]: g for k, g in load(sys.argv[3]).items()})
    sys.stdout.reconfigure(encoding='utf-8')
    levels = ['r47', 'c0', 'c2', 'c1'] if len(sys.argv) > 3 else ['c0', 'c2', 'c1']
    label = {'r47': 'r47 (no carry, r47 rules)', 'c0': 'r48, carry 0 (neither)', 'c1': 'r48, carry 1 (Unity only: the baseline)',
             'c2': 'r48, carry 2 (both sides)'}
    print('| agent | arm | win % | gap vs the baseline ± SE | rounds | ends | Unity tips | Discord tips |')
    print('|---|---|---|---|---|---|---|---|')
    for agent in ['strategist', 'greedy']:
        base = {k[1:]: g for k, g in games.items() if k[0] == 'c1' and k[2] == agent}
        for lv in levels:
            arm = {k[1:]: g for k, g in games.items() if k[0] == lv and k[2] == agent}
            keys = sorted(set(arm) & set(base))
            w = [arm[k]['won'] for k in keys]
            d = [int(arm[k]['won']) - int(base[k]['won']) for k in keys]
            ends = Counter(arm[k]['reason'] for k in keys)
            ut = mean(arm[k]['margin']['unity_tips'] for k in keys)
            dt = mean(arm[k]['counters'].get('alignment.tips_discord', 0) for k in keys)
            gap = '' if lv == 'c1' else f'{100*mean(d):+.1f} ± {100*se(d):.1f} pp'
            print(f'| {agent} | {label[lv]} | {100*mean(w):.1f} | {gap} | {mean(arm[k]["rounds"] for k in keys):.2f} | '
                  f'{", ".join(f"{a} {b}" for a, b in ends.most_common())} | {ut:.2f} | {dt:.2f} |')


if __name__ == '__main__':
    main()
