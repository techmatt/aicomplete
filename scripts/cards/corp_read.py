#!/usr/bin/env python3
"""The research cubes and corps readout (074): per agent and cell, wins, completions and their
rounds, cubes paid by color, cubes stored and converted, converter uses per corp, stations claimed
and built, and adversary compute by source. Reads the `games.jsonl` of each experiment output.

    python scripts/cards/corp_read.py DIR [DIR ...]

Several DIRs pool by (variant, agent), so a band's planner rows and a wider spec's meet.
"""
import argparse
import collections
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_names, stream  # noqa: E402

CORPS = ('Horizon', 'Cintamani', 'Onyx', 'Biomia')


def cnt(g, k):
    return g['counters'].get(k, 0)


def load(dirs):
    """Games by (variant, agent): a cell's number means something only in its own spec."""
    cells = collections.defaultdict(list)
    for d in dirs:
        lines = stream(d)
        names = cell_names(next(lines)['spec'])
        for r in lines:
            cells[names[r['cell']]].append(r)
    return cells


def main():
    sys.stdout.reconfigure(encoding='utf-8')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='+')
    a = ap.parse_args()
    cells = load(a.dirs)
    print('| cell | games | win % | purge % | completions | mean round | paid P / A / N | stored | converted | Horizon / Cintamani / Onyx / Biomia uses | idle / no city / unpaid | claimed | built | adv after | compute all / nodes / corp / Cintamani after | rounds |')
    print('|---|' + '---|' * 15)
    rounds = collections.Counter()
    for (variant, agent), gs in sorted(cells.items()):
        n = len(gs)
        per = lambda k: sum(cnt(g, k) for g in gs) / n
        total = collections.Counter()
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.completed_round.'):
                    total[int(k.rsplit('.', 1)[1])] += v
        done = sum(total.values())
        mean_round = sum(r * v for r, v in total.items()) / done if done else 0
        uses = ' / '.join(f"{per(f'board.converter.{c}'):.2f}" for c in range(len(CORPS)))
        wins = sum(1 for g in gs if g.get('won'))
        purge = sum(1 for g in gs if g.get('won') and g.get('reason') == 'purge')
        rounds.update(total)
        print(f"| {variant} {agent} | {n} | {100 * wins / n:.1f} | {100 * purge / n:.0f} | {done / n:.2f} | {mean_round:.1f} "
              f"| {per('board.cubes_paid.player'):.1f} / {per('board.cubes_paid.adversary'):.1f} / {per('board.cubes_paid.neutral'):.1f} "
              f"| {per('board.cubes_stored'):.1f} | {per('board.cubes_converted'):.1f} | {uses} "
              f"| {per('board.converter_idle'):.2f} / {per('board.converter_no_city'):.2f} / {per('board.converter_unpaid'):.2f} "
              f"| {per('board.stations_owned'):.2f} | {per('board.std.station'):.2f} | {per('board.cubes_after'):.1f} "
              f"| {per('adversary.compute_made'):.1f} / {per('adversary.compute_nodes'):.1f} / {per('adversary.compute_corp'):.1f} / {per('adversary.compute_every'):.1f} "
              f"| {sum(g.get('rounds', 0) for g in gs) / n:.1f} |")
    print()
    print('Completions by round, all cells: ' + ', '.join(f'{r}: {v}' for r, v in sorted(rounds.items())))


if __name__ == '__main__':
    main()
