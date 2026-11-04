#!/usr/bin/env python3
"""The slots readout (092): per variant and agent, how often each seat builds over a standing
building (a game, and by round) and how full each seat's slots are as each round starts
(buildings standing / slots). Reads the `slots.*` counters the experiment loop records
(`crates/aic-sim/src/experiment.rs`, `play_one`). Several DIRs pool by (variant, agent).

    python scripts/cards/slots_read.py DIR [DIR ...] [--rounds N]
"""
import argparse
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from corp_read import load  # noqa: E402


def boards(g):
    """Each seat's board, from the game's build signature (`board:tops|board:tops`)."""
    return [part.split(':')[0] for part in g.get('build', '').split('|')]


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--rounds', type=int, default=9, help='rounds shown (default 9)')
    a = ap.parse_args()
    cells = load(a.dirs)
    rounds = range(1, a.rounds + 1)
    print('Build-overs a game per seat, then by round (summed over seats, a game reaching it); '
          'fill: buildings standing / slots as the round starts, a game reaching it.\n')
    for (variant, agent), gs in sorted(cells.items()):
        n = len(gs)
        if not n or 'slots.cap.0' not in gs[0]['counters']:
            continue
        names = boards(gs[0])
        seats = len(names)
        print(f'## {variant} {agent} ({n} games)\n')
        print('| seat | slots | build-overs a game | ' + ' | '.join(f'r{r}' for r in rounds) + ' |')
        print('|---|---|---|' + '---|' * len(rounds))
        for s in range(seats):
            c = lambda g, k: g['counters'].get(k, 0)  # noqa: E731
            over = sum(c(g, f'slots.over.{s}') for g in gs) / n
            cap = gs[0]['counters'].get(f'slots.cap.{s}', 0)
            fill = []
            for r in rounds:
                reach = [g for g in gs if g['rounds'] >= r]
                if not reach:
                    fill.append('–')
                    continue
                used = sum(c(g, f'slots.used.{s}.r{r:02}') for g in reach) / len(reach)
                fill.append(f'{used:.1f}')
            print(f'| {s} {names[s]} | {cap} | {over:.2f} | ' + ' | '.join(fill) + ' |')
        by_round = []
        for r in rounds:
            reach = [g for g in gs if g['rounds'] >= r]
            k = sum(g['counters'].get(f'slots.over.{s}.r{r:02}', 0) for g in reach for s in range(seats))
            by_round.append(f'{k / len(reach):.2f}' if reach else '–')
        print('| build-overs by round | | | ' + ' | '.join(by_round) + ' |\n')


if __name__ == '__main__':
    main()
