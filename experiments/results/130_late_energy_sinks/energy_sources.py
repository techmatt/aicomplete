"""130: where a board's energy comes from, flagged seat-games against the rest: energy gained a seat-game by source
(the `sources` record of games.jsonl, index 2), for one agent.

    python experiments/results/130_late_energy_sinks/energy_sources.py DIR [--agent strategist] [--board Builder]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '128_draft_and_price'))
import read128 as R  # noqa: E402

E = 2


def group(key):
    """A source key, cards kept by name and the rest by kind (`phase:board.produce`, `own:power:Overcharge`)."""
    return key


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--board', default='Builder')
    ap.add_argument('--top', type=int, default=15)
    a = ap.parse_args()
    tot = {True: collections.Counter(), False: collections.Counter()}
    n = collections.Counter()
    for label, pair, g in R.load(a.dir):
        if not label.endswith('/' + a.agent):
            continue
        for s in range(len(g['peaks'])):
            if R.BOARD[pair[s]] != a.board:
                continue
            rise = g['peaks'][s][E] - (g.get('start') or [[0] * 8] * 2)[s][E]
            inc = g['gained'][s][E] / max(g['rounds'], 1)
            flag = inc > 0 and rise > 3 * inc and rise > 10
            n[flag] += 1
            for k, v in g['sources'][s].items():
                if v[E]:
                    tot[flag][group(k)] += v[E]
    keys = sorted(set(tot[True]) | set(tot[False]), key=lambda k: -(tot[True][k] / max(n[True], 1)))
    print(f'{a.board}, {a.agent}: {n[True]} flagged seat-games, {n[False]} not. Energy gained a seat-game by source.\n')
    print('| source | flagged | not flagged |')
    print('|---|---|---|')
    for k in keys[:a.top]:
        print(f'| {k} | {tot[True][k] / max(n[True], 1):.2f} | {tot[False][k] / max(n[False], 1):.2f} |')
    print(f"| all | {sum(tot[True].values()) / max(n[True], 1):.2f} | {sum(tot[False].values()) / max(n[False], 1):.2f} |")


if __name__ == '__main__':
    main()
