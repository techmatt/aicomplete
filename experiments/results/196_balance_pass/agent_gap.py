"""196: an agent cell against the plain strategist on the same variant, difficulty, pair and seed (a weight screen).

    python agent_gap.py FILE=SPEC[,FILE=SPEC...] [--lever LEVER]
"""
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cells import load  # noqa: E402


def main():
    lever = sys.argv[sys.argv.index('--lever') + 1] if '--lever' in sys.argv else 'base'
    rows = load(sys.argv[1].split(','))
    idx = {}
    for lv, pair, d, agent, seed, g in rows:
        if lv == lever:
            idx[(agent, pair, d, seed)] = g
    agents = sorted({a for a, _, _, _ in idx})
    print(f'lever {lever}: each agent against `strategist`, paired (win % gap ± SE)\n')
    print('| agent | d0 | d1 | d2 | d3 | all |')
    print('|---|---|---|---|---|---|')
    for a in agents:
        if a == 'strategist':
            continue
        by = defaultdict(list)
        for (ag, pair, d, seed), g in idx.items():
            if ag != a:
                continue
            b = idx.get(('strategist', pair, d, seed))
            if b is not None:
                x = int(g['won']) - int(b['won'])
                by[d].append(x)
                by['all'].append(x)
        cells = []
        for d in (0, 1, 2, 3, 'all'):
            xs = by.get(d, [])
            if not xs:
                cells.append('-')
                continue
            mu = sum(xs) / len(xs)
            sd = math.sqrt(sum((x - mu) ** 2 for x in xs) / max(1, len(xs) - 1)) / math.sqrt(len(xs))
            cells.append(f'{100 * mu:+.1f} ± {100 * sd:.1f}')
        print(f'| {a} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
