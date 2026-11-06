"""130: per energy-cost card and board, the play passes it sat in a hand and the share of them short of energy
(`energy_cards.short.<seat>.<card>` over its passes in hand), for one agent. Passes in hand per card are not
counted by the engine, so the denominator is the seat's passes holding any energy-cost card, split by the card's
share of the seat's short passes: the share printed is short passes of the card per seat-game.

    python experiments/results/130_late_energy_sinks/short_read.py DIR [--agent strategist]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '128_draft_and_price'))
import read128 as R  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    a = ap.parse_args()
    short = collections.defaultdict(collections.Counter)
    n = collections.Counter()
    for label, pair, g in R.load(a.dir):
        if not label.endswith('/' + a.agent):
            continue
        for s in range(2):
            b = R.BOARD[pair[s]]
            n[b] += 1
            for k, v in g['counters'].items():
                if k.startswith(f'energy_cards.short.{s}.'):
                    short[b][k.rsplit('.', 1)[1]] += v
    cards = sorted({c for v in short.values() for c in v})
    print('| card | ' + ' | '.join(sorted(n)) + ' |')
    print('|---|' + '---|' * len(n))
    for c in cards:
        print(f'| {c} | ' + ' | '.join(f'{short[b][c] / n[b]:.2f}' for b in sorted(n)) + ' |')


if __name__ == '__main__':
    main()
