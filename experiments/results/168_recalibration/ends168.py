"""168: how each lever's games end, by level: win %, loss reasons, mean rounds and Unity tips at the end.

    python experiments/results/168_recalibration/ends168.py DIR [DIR ...]
"""
import os
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402


def main():
    rows = defaultdict(list)
    for d in sys.argv[1:]:
        it = stream(d)
        cells = cells_of(next(it)['spec'])
        for g in it:
            c = cells[g['cell']]
            lever = c['variant'].partition('_')[2] or 'base'
            rows[(lever, c['agent'][:10], c['difficulty'])].append(g)
    print('| Lever | Agent | d | Games | Win % | Losses by reason | Rounds | Unity tips | Cities corrupted at end (losses) |')
    print('|---|---|---|---|---|---|---|---|---|')
    for k in sorted(rows):
        gs = rows[k]
        n = len(gs)
        won = sum(1 for g in gs if g['won'])
        why = Counter(g['reason'] for g in gs if not g['won'])
        lost = [g for g in gs if not g['won']]
        tips = sum(g['margin'].get('unity_tips', 0) for g in gs) / n
        rounds = sum(g['rounds'] for g in gs) / n
        cc = sum(g['margin'].get('cities_corrupted_end', 0) for g in lost) / max(1, len(lost))
        print(f'| {k[0]} | {k[1]} | {k[2]} | {n} | {100 * won / n:.1f} | '
              + ', '.join(f'{r} {c}' for r, c in why.most_common()) + f' | {rounds:.1f} | {tips:.1f} | {cc:.1f} |')


if __name__ == '__main__':
    main()
