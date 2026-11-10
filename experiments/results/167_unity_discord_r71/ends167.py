"""167: how the lost games end, NEW against OLD: Unity tips, cities corrupted and blockers left, by level.

    python experiments/results/167_unity_discord_r71/ends167.py NEW OLD
"""
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def ends(d):
    it = stream(d)
    cells = cells_of(next(it)['spec'])
    out = defaultdict(list)
    for g in it:
        out[cells[g['cell']]['difficulty']].append(g)
    return out


def main():
    for name, d in (('new', sys.argv[1]), ('old', sys.argv[2])):
        for lvl, gs in sorted(ends(d).items()):
            lost = [g for g in gs if not g['won']]
            won = [g for g in gs if g['won']]
            if not lost:
                continue
            m = lambda xs, k: sum(g['margin'].get(k, 0) for g in xs) / max(1, len(xs))  # noqa: E731
            tips = defaultdict(int)
            for g in lost:
                tips[min(g['margin'].get('unity_tips', 0), 6)] += 1
            print(f"{name} d{lvl}: lost {len(lost)}, tips {dict(sorted(tips.items()))}, cities corrupted "
                  f"{m(lost, 'cities_corrupted_end'):.1f}, blockers {m(lost, 'blockers'):.1f}; "
                  f"won {len(won)} in {sum(g['rounds'] for g in won) / max(1, len(won)):.1f} rounds, "
                  f"tips {m(won, 'unity_tips'):.1f}")


if __name__ == '__main__':
    main()
