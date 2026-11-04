"""Kept for the docs that cite it (screen-028-rollout/rollout.md); new work: screen.py.
The rollout-agent screen summary (028, rules r17).

    python scripts/cards/screen_028.py DIR [DIR ...]
    python scripts/cards/screen_028.py --gate DIR [DIR ...]

Each DIR is an `aic-sim experiment` output. Prints, per cell: win %, win and loss rounds,
how losses and wins split, Unity and Discord tips, node lost (wins / losses), frees,
and the node lost by source over all cells.

--gate prints each check's gap to its own agent: an `engine:...` check against `engine`, a
`planner:...` check against `planner` (win % points, ± one standard error).
"""
import collections
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screens as s  # noqa: E402
from screens import avg, cnt, gate, hist, sources  # noqa: E402


def table(cells, games):
    print('| variant | d | agent | n | win % | win rounds | loss rounds | wins: purge / Unity deck | losses: Discord / node / deck-out | Unity / Discord tips | node lost: wins / losses | freed | bases united |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    key = lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])
    for cell in sorted(games, key=key):
        gs = games[cell]
        c = cells[cell]
        wins = [g for g in gs if g['won']]
        losses = [g for g in gs if not g['won']]
        ends = collections.Counter(g['reason'] for g in gs)
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {len(gs)} | {100 * len(wins) / len(gs):.0f} "
              f"| {hist(g['rounds'] for g in wins)} | {hist(g['rounds'] for g in losses)} "
              f"| {ends['purge']} / {ends['unity_deck']} "
              f"| {ends['discord_deck']} / {ends['node']} / {ends['deck_empty']} "
              f"| {avg([cnt(g, 'alignment.tips_unity') for g in gs]):.1f} / {avg([cnt(g, 'alignment.tips_discord') for g in gs]):.1f} "
              f"| {avg([cnt(g, 'board.node_lost') for g in wins]):.1f} / {avg([cnt(g, 'board.node_lost') for g in losses]):.1f} "
              f"| {avg([cnt(g, 'board.cities_freed') for g in gs]):.1f} "
              f"| {avg([cnt(g, 'board.bases_united') for g in gs]):.1f} |")


if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if a != '--gate']
    for d in args:
        cells, games = s.load(d)
        if '--gate' in sys.argv:
            gate(cells, games)
        else:
            table(cells, games)
            sources(games)
