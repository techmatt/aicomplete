"""128: keeps and plays per board of card groups (per-tag six, the tier-2 per-tag pair, the draw five, each sink),
a seat-game, for each run's strategist and greedy.

    python experiments/results/128_draft_and_price/keep_groups.py DIR=TAG...
"""
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import read128 as R  # noqa: E402

GROUPS = {
    'per-tag six': R.PERTAG[:6], 'Antitrust': ['antitrust_action'], 'HSA + Supergrid': R.PERTAG[7:],
    'Embassy': ['orbital_embassy'], 'draw five': R.DRAWS, 'Accelerator': ['particle_accelerator'],
    'Grid Desk': ['demand_response_grid'], 'Microgrid': ['citywide_microgrid'],
}


def main():
    acc = collections.defaultdict(collections.Counter)
    n = collections.Counter()
    for arg in sys.argv[1:]:
        for label, pair, g in R.load(arg):
            acts, _ = R.seat_stream(g, len(g['peaks']))
            for s, a in enumerate(acts):
                key = (label, R.BOARD[pair[s]])
                n[key] += 1
                for gname, cards in GROUPS.items():
                    for c in cards:
                        acc[key][(gname, 'keep')] += a[('keep', c)]
                        acc[key][(gname, 'play')] += a[('play', c)]
                        acc[key][(gname, 'use')] += a[('use', c)]
    print('| run | board | ' + ' | '.join(GROUPS) + ' |')
    print('|---|---|' + '---|' * len(GROUPS))
    for key in sorted(n):
        cells = []
        for gname in GROUPS:
            k, p, u = (acc[key][(gname, x)] / n[key] for x in ('keep', 'play', 'use'))
            cells.append(f'{k:.2f} / {p:.2f}' + (f' / {u:.2f}' if gname in ('Accelerator', 'Grid Desk', 'Microgrid') else ''))
        print(f'| {key[0]} | {key[1]} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
