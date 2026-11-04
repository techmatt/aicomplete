#!/usr/bin/env python3
"""The r38 read (098): per cell of each DIR, the win rate and win routes, Unity Sway and tips,
troops removed, dark city-rounds (neutral `board.city_dark`, aligned `board.city_dark_aligned`,
from r38), energy received from aligned cities (`board.city_energy_aligned`) and bought from
neutral ones (`board.city_energy_bought`), nodes placed and lost. All per game.

    python scripts/cards/r38_read.py DIR [DIR ...]

Each DIR holds an experiment's games.jsonl and results.json. A binary older than the counters
reads 0 for them. Prints Markdown tables. Seconds.
"""
import collections
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cnt, load  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def main():
    dirs = sys.argv[1:]
    if not dirs:
        sys.exit(__doc__)
    rows = []
    for d in dirs:
        cells, games = load(d)
        for cid in sorted(games):
            gs = games[cid]
            c = cells.get(cid, {})
            rows.append((os.path.basename(d.rstrip('/\\')), c.get('variant', str(cid)), gs))

    def m(gs, f):
        return sum(f(g) for g in gs) / len(gs)

    print('| run | cell | games | win % | wins: purge / Unity deck / other | losses by reason '
          '| Unity Sway | Unity tips | Discord tips | troops removed |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for run, name, gs in rows:
        won = [g for g in gs if g['won']]
        purge = sum(g['reason'] == 'purge' for g in won)
        deck = sum(g['reason'] == 'unity_deck' for g in won)
        losses = collections.Counter(g['reason'] for g in gs if not g['won'])
        lost = ', '.join(f'{k} {v}' for k, v in losses.most_common()) or '-'
        print(f"| {run} | {name} | {len(gs)} | {100 * len(won) / len(gs):.1f} "
              f"| {purge} / {deck} / {len(won) - purge - deck} | {lost} "
              f"| {m(gs, lambda g: cnt(g, 'alignment.sway_unity')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'alignment.tips_unity')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'alignment.tips_discord')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'adversary.troops_removed')):.2f} |")
    print()
    print('| run | cell | dark neutral city-rounds | dark aligned city-rounds | energy from aligned cities '
          '| energy bought from neutral | nodes placed | nodes lost | corruptions |')
    print('|---|---|---|---|---|---|---|---|---|')
    for run, name, gs in rows:
        print(f"| {run} | {name} | {m(gs, lambda g: cnt(g, 'board.city_dark')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'board.city_dark_aligned')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'board.city_energy_aligned')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'board.city_energy_bought')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'board.node_placed')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'board.node_lost')):.2f} "
              f"| {m(gs, lambda g: cnt(g, 'adversary.corruptions')):.2f} |")


if __name__ == '__main__':
    main()
