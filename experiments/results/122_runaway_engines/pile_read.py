"""122's pile table: per lever (the part of a variant name after the pair, `bp-<lever>`; a bare pair is `base`),
board and resource, the mean and highest peak, mean income a round, the share of seats flagged by
runaway_read's peak rule (peak over 3x a round's income and over 10), rounds and the win rate.

    python experiments/results/122_runaway_engines/pile_read.py DIR... [--res credits,energy,...] [--agent strategist]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--res', default='credits,insight,energy,chips,ambassadors')
    ap.add_argument('--agent')
    a = ap.parse_args()
    res = a.res.split(',')
    seats = collections.defaultdict(list)
    games = collections.defaultdict(list)
    for d in a.dirs:
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        tag = os.path.basename(os.path.normpath(d))
        for g in lines:
            variant, agent = labels[g['cell']].split('/')[:2]
            if a.agent and agent != a.agent:
                continue
            pair, _, lever = variant.partition('-')
            key = (tag, lever or 'base', agent)
            games[key].append((g['won'], g['rounds']))
            for s, p in enumerate(g['peaks']):
                board = BOARD[pair[s]]
                for r in res:
                    i = PEAKS.index(r)
                    inc = g['gained'][s][i] / g['rounds']
                    seats[key + (board, r)].append((p[i], inc))
    print('| run | lever | agent | games | win | rounds |')
    print('|---|---|---|---|---|---|')
    for key, v in sorted(games.items()):
        print(f'| {key[0]} | {key[1]} | {key[2]} | {len(v)} | {sum(w for w, _ in v) / len(v):.1%} | '
              f'{sum(r for _, r in v) / len(v):.2f} |')
    for r in res:
        print(f'\n### {r}\n')
        print('| run | lever | agent | board | mean peak | max peak | income / round | flagged |')
        print('|---|---|---|---|---|---|---|---|')
        for key, v in sorted(seats.items()):
            if key[4] != r:
                continue
            fl = sum(1 for pk, inc in v if inc > 0 and pk > 10 and pk > 3 * inc)
            print(f'| {key[0]} | {key[1]} | {key[2]} | {key[3]} | {sum(x[0] for x in v) / len(v):.1f} | '
                  f'{max(x[0] for x in v)} | {sum(x[1] for x in v) / len(v):.2f} | {fl}/{len(v)} |')


if __name__ == '__main__':
    main()
