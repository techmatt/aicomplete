"""196: whether two reads of the same spec played the same games (outcome, end reason, rounds, decisions) on the
seeds both played, per difficulty. Either side may be a farmed file (FILE=SPEC) or an experiment dir.

    python same_games.py A[=SPEC] B[=SPEC]
"""
import os
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cells import load  # noqa: E402


def index(arg):
    return {(p, d, a, str(s)): (g['won'], g.get('reason'), g['rounds'], g['decisions'])
            for _, p, d, a, s, g in load([arg])}


def main():
    a, b = index(sys.argv[1]), index(sys.argv[2])
    both = [k for k in a if k in b]
    same, total = Counter(), Counter()
    for k in both:
        total[k[1]] += 1
        same[k[1]] += a[k] == b[k]
    for d in sorted(total):
        print(f'd{d}: {total[d] - same[d]} of {total[d]} shared games differ')


if __name__ == '__main__':
    main()
