"""196: how many games each agent cell plays differently from the plain strategist on the same variant,
difficulty and seed (an inert-switch check: identical games mean the weight is never reached).

    python same_cells.py FILE=SPEC
"""
import os
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cells import load  # noqa: E402


def sig(g):
    return (g['won'], g.get('reason'), g['rounds'], g['decisions'], g.get('options_total'))


def main():
    rows = load(sys.argv[1].split(','))
    idx = {(a, lv, p, d, s): g for lv, p, d, a, s, g in rows}
    differ, total = Counter(), Counter()
    for (a, lv, p, d, s), g in idx.items():
        if a == 'strategist':
            continue
        b = idx.get(('strategist', lv, p, d, s))
        if b is None:
            continue
        total[a] += 1
        differ[a] += sig(g) != sig(b)
    for a in sorted(total):
        print(f'{a}: {differ[a]} of {total[a]} games differ from the plain strategist')


if __name__ == '__main__':
    main()
