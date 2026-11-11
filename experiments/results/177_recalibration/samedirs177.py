"""177: do two runs play the same games? Matches cells by (pair, level, agent) across two dirs, with a lever tag
mapped per side, and compares each seed's end (won, reason, rounds, action count).

    python experiments/results/177_recalibration/samedirs177.py DIR_A LEVER_A DIR_B LEVER_B

LEVER `base` is the variant with no lever (`bp`, `br`, `pr`).
"""
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def load(d, lever):
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    out = {}
    for g in it:
        if 'cell' not in g:
            continue
        c = cells[g['cell']]
        pair, _, lv = c['variant'].partition('_')
        if (lv or 'base') != lever:
            continue
        out[(pair, c['difficulty'], c['agent'], g['seed'])] = (g.get('won'), g.get('reason'), g.get('rounds'),
                                                               len(g.get('actions') or []))
    return out


def main():
    a = load(sys.argv[1], sys.argv[2])
    b = load(sys.argv[3], sys.argv[4])
    keys = [k for k in a if k in b]
    diff = sum(a[k] != b[k] for k in keys)
    print(f'{diff} of {len(keys)} games differ')


if __name__ == '__main__':
    main()
