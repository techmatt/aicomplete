"""130: flagged seat-games of a board, split by whether the seat played any energy sink (an Action that pays energy, or
a tier-2 big-energy card), with the energy held after each action phase in the round the peak came.

    python experiments/results/130_late_energy_sinks/flag_sinks.py DIR [--agent strategist] [--board Builder]
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
SINKS = {c for c, d in R.CARDS.items()
         if any(k == 'energy' for k, _ in (d.get('ability') or {}).get('cost', [])) or d.get('energy', 0) >= 8}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--board', default='Builder')
    a = ap.parse_args()
    t = collections.Counter()
    uses = collections.Counter()
    for label, pair, g in R.load(a.dir):
        if not label.endswith('/' + a.agent):
            continue
        for s in range(len(g['peaks'])):
            if R.BOARD[pair[s]] != a.board:
                continue
            rise = g['peaks'][s][E] - (g.get('start') or [[0] * 8] * 2)[s][E]
            inc = g['gained'][s][E] / max(g['rounds'], 1)
            flag = inc > 0 and rise > 3 * inc and rise > 10
            played = {x['card'] for x in g['actions'] if x['seat'] == s and x['type'] == 'play'} & SINKS
            used = sum(1 for x in g['actions'] if x['seat'] == s and x['type'] == 'use' and x['card'] in SINKS)
            k = (flag, bool(played))
            t[k] += 1
            uses[k] += used
    print(f'{a.board}, {a.agent}. Sinks: {", ".join(sorted(SINKS))}.\n')
    print('| | played a sink | sink Action uses a seat-game | played none |')
    print('|---|---|---|---|')
    for flag in (True, False):
        y, n = t[(flag, True)], t[(flag, False)]
        print(f"| {'flagged' if flag else 'not flagged'} | {y} | {uses[(flag, True)] / max(y, 1):.2f} | {n} |")


if __name__ == '__main__':
    main()
