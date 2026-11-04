#!/usr/bin/env python3
"""The r37 read (096): per cell of each DIR, the win rate, converter uses per game (all
countries, `board.converter.<c>`), cube-holding seat-rounds a converter stood idle for want of a
city node (`board.converter_no_city`; before r37, of any node), and plays per game of Popular
Uprising (with its one-shot liberations, `cards.once_discarded`: it is the only `once` card) and
Continental Supergrid.

    python scripts/cards/r37_read.py DIR [DIR ...]

Each DIR holds an experiment's games.jsonl and results.json. Prints a Markdown table. Seconds.
"""
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cnt, load, seats  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def converter_uses(g):
    return sum(v for k, v in g['counters'].items() if k.startswith('board.converter.') and k[16:].isdigit())


def main():
    dirs = sys.argv[1:]
    if not dirs:
        sys.exit(__doc__)
    loaded = [(d, *load(d)) for d in dirs]

    def cells_of():
        for d, cells, games in loaded:
            for cid in sorted(games):
                gs = games[cid]
                c = cells.get(cid, {})
                name = c.get('variant', cid) + ('' if len({x.get('agent') for x in cells.values()}) < 2
                                                 else ' ' + c.get('agent', ''))
                yield os.path.basename(d.rstrip('/')), name, gs, lambda f, gs=gs: sum(f(g) for g in gs) / len(gs), seats(c)

    print('| run | cell | games | win % | converter uses | idle, no city node | PU played | PU liberations | Supergrid played |')
    print('|---|---|---|---|---|---|---|---|---|')
    for run, name, gs, m, _ in cells_of():
        print(f"| {run} | {name} | {len(gs)} | {100 * m(lambda g: g['won']):.1f} "
              f"| {m(converter_uses):.2f} | {m(lambda g: cnt(g, 'board.converter_no_city')):.2f} "
              f"| {m(lambda g: cnt(g, 'cards.played.popular_uprising')):.2f} "
              f"| {m(lambda g: cnt(g, 'cards.once_discarded')):.2f} "
              f"| {m(lambda g: cnt(g, 'cards.played.continental_supergrid')):.2f} |")
    # Amendment #4040: dark neutral cities (`board.city_dark`, from r37), energy bought, the
    # seats' energy at their last round's action-phase end, nodes placed, corruptions.
    print()
    print('| run | cell | dark city-rounds | energy bought | end energy (all seats) | nodes placed | corruptions |')
    print('|---|---|---|---|---|---|---|')
    for run, name, gs, m, ns in cells_of():
        def end_energy(g, ns=ns):
            # 099: each seat's last round whose action phase ended (energy_read's), every seat:
            # the final round's key is missing when the game ended mid-round, and c3 has three.
            def last(s):
                return max(g['rounds'] - 1, max((r for r in range(1, g['rounds'] + 1)
                                                 if f'ai.tag_top.{s}.r{r:02d}' in g['counters']), default=0))
            return sum(cnt(g, f'unspent.{s}.energy.r{last(s):02d}') for s in range(ns) if last(s))
        print(f"| {run} | {name} | {m(lambda g: cnt(g, 'board.city_dark')):.2f} "
              f"| {m(lambda g: cnt(g, 'board.city_energy_bought')):.2f} | {m(end_energy):.2f} "
              f"| {m(lambda g: cnt(g, 'board.node_placed')):.2f} | {m(lambda g: cnt(g, 'adversary.corruptions')):.2f} |")


if __name__ == '__main__':
    main()
