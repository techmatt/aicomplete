"""167: a screen variant's fx tallies a resolution, per card (Unity), for levers named LEVER (`PAIR_LEVER`; base: none).

    python experiments/results/167_unity_discord_r71/fx167.py SCREEN LEVER
"""
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def main():
    d, want = sys.argv[1], sys.argv[2]
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    res = defaultdict(int)
    tot = defaultdict(lambda: defaultdict(float))
    n = 0
    for g in it:
        pair, _, lever = cells[g['cell']]['variant'].partition('_')
        if (lever or 'base') != want:
            continue
        n += 1
        for k, v in g.get('counters', {}).items():
            if not k.startswith('fx.'):
                continue
            _, cid, m = k.split('.', 2)
            if m in ('t1', 't2', 't3'):
                res[cid] += v
            else:
                tot[cid][m] += v
    print(f'{n} games')
    for cid in sorted(res, key=lambda c: -res[c]):
        what = ', '.join(f'{m} {v / res[cid]:+.1f}' for m, v in sorted(tot[cid].items()) if abs(v / res[cid]) >= 0.05)
        print(f'{cid:22s} {res[cid] / n:.2f}/game  {what}')


if __name__ == '__main__':
    main()
