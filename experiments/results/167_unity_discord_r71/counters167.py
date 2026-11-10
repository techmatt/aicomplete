"""167: mean counters a game, NEW against OLD, for keys matching any PATTERN (substring).

    python experiments/results/167_unity_discord_r71/counters167.py NEW OLD PATTERN...
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import stream  # noqa: E402


def means(d, pats):
    it = stream(d)
    next(it)
    tot, n = defaultdict(float), 0
    for g in it:
        n += 1
        for k, v in g.get('counters', {}).items():
            if any(p in k for p in pats):
                tot[k] += v
        for k, v in g.get('margin', {}).items():
            if isinstance(v, (int, float)) and any(p in 'margin.' + k for p in pats):
                tot['margin.' + k] += v
    return {k: v / n for k, v in tot.items()}


def main():
    pats = sys.argv[3:]
    a, b = means(sys.argv[1], pats), means(sys.argv[2], pats)
    for k in sorted(set(a) | set(b)):
        print(f'{k:60s} {a.get(k, 0):9.2f} {b.get(k, 0):9.2f}')


if __name__ == '__main__':
    main()
