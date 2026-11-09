"""158: mean counters a game of two cell groups (wins158.py's group names) side by side, the largest gaps first.

    python diff158.py DIR GROUP_A GROUP_B [--top 40] [--grep SUBSTR]

Run from the repo root.
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from screens import cell_labels, stream  # noqa: E402
from wins158 import group  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('a')
    ap.add_argument('b')
    ap.add_argument('--top', type=int, default=40)
    ap.add_argument('--grep', default='')
    x = ap.parse_args()
    sums = {x.a: collections.Counter(), x.b: collections.Counter()}
    n = collections.Counter()
    it = stream(x.dir)
    labels = cell_labels(next(it)['spec'])
    for g in it:
        k = group(labels[g['cell']])
        if k not in sums:
            continue
        n[k] += 1
        for c, v in (g.get('counters') or {}).items():
            if isinstance(v, (int, float)):
                sums[k][c] += v
        sums[k]['rounds'] += g['rounds']
    keys = set(sums[x.a]) | set(sums[x.b])
    rows = []
    for c in keys:
        if x.grep and x.grep not in c:
            continue
        va, vb = sums[x.a][c] / max(n[x.a], 1), sums[x.b][c] / max(n[x.b], 1)
        rows.append((abs(va - vb) / (abs(va) + abs(vb) + 1.0), c, va, vb))
    rows.sort(reverse=True)
    print(f'| counter | {x.a} ({n[x.a]}) | {x.b} ({n[x.b]}) |')
    print('|---|---|---|')
    for _, c, va, vb in rows[:x.top]:
        print(f'| {c} | {va:.2f} | {vb:.2f} |')


if __name__ == '__main__':
    main()
