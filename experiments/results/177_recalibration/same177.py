"""177: how many games two agents of one run play differently (same variant, level and seed), by the game's
action count and end (a cheap proxy: an identical game has the same of each).

    python experiments/results/177_recalibration/same177.py DIR AGENT_A AGENT_B
"""
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def sig(g):
    return (g.get('won'), g.get('reason'), g.get('rounds'), len(g.get('actions') or []))


def main():
    d, a, b = sys.argv[1:4]
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    by = {}
    for g in it:
        if 'cell' not in g:
            continue
        c = cells[g['cell']]
        by[(c['agent'], c['variant'], c['difficulty'], g['seed'])] = sig(g)
    n = diff = 0
    for (agent, v, lvl, seed), s in by.items():
        if agent != a:
            continue
        o = by.get((b, v, lvl, seed))
        if o is None:
            continue
        n += 1
        diff += s != o
    print(f'{diff} of {n} games differ between {a} and {b}')


if __name__ == '__main__':
    main()
