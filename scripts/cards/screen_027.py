"""Kept for the docs that cite it (screen-027-endgame/endgame.md); new work: scripts/cards/screen.py.

The endgame screen summary (027, rules r16).

    python scripts/cards/screen_027.py DIR [DIR ...]
    python scripts/cards/screen_027.py --gate DIR [DIR ...]

Each DIR is an `aic-sim experiment` output. Prints, per cell: win %, win and loss rounds
as histograms, how games ended, Unity and Discord tips, node lost (and by source
over all cells), arrivals and removals a round, frees and corruptions.

--gate prints each check's gap to the engine (win % points, ± one standard error) for
the dominance tables.
"""
import collections
import math
import os
import re
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screens as s  # noqa: E402
from screens import hist  # noqa: E402


def rate(gs, k, per_round=True):
    n = sum(g['counters'].get(k, 0) for g in gs)
    d = sum(g['rounds'] for g in gs) if per_round else len(gs)
    return n / d if d else 0.0


def table(cells, games):
    print('| variant | d | agent | n | win % | win rounds | loss rounds | ends | Unity / Discord tips | node lost | arrivals / removed a round | peak ambassadors | botnets peak | freed / corruptions | projects | supply short |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    key = lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])
    for cell in sorted(games, key=key):
        gs = games[cell]
        c = cells[cell]
        wins = [g['rounds'] for g in gs if g['won']]
        losses = [g['rounds'] for g in gs if not g['won']]
        ends = collections.Counter(('win ' if g['won'] else '') + g['reason'] for g in gs)
        peak = 0.0
        for g in gs:
            on = [v for k, v in g['counters'].items() if re.fullmatch(r'adversary\.ambassadors_r\d\d', k)]
            peak += max(on) if on else 0
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {len(gs)} | {100 * len(wins) / len(gs):.0f} "
              f"| {hist(wins)} | {hist(losses)} | {', '.join(f'{k} {v}' for k, v in ends.most_common())} "
              f"| {rate(gs, 'alignment.tips_unity', False):.1f} / {rate(gs, 'alignment.tips_discord', False):.1f} "
              f"| {rate(gs, 'board.node_lost', False):.1f} "
              f"| {rate(gs, 'adversary.arrivals'):.1f} / {rate(gs, 'adversary.ambassadors_removed'):.1f} "
              f"| {peak / len(gs):.0f} | {rate(gs, 'adversary.botnets_peak', False):.1f} "
              f"| {rate(gs, 'board.cities_freed', False):.1f} / {rate(gs, 'adversary.corruptions', False):.1f} "
              f"| {rate(gs, 'adversary.projects_resolved', False):.1f} | {rate(gs, 'adversary.supply_short', False):.1f} |")


def sources(games):
    s.sources(games)
    print(f"Last node saved by the stage-I rule: {sum(g['counters'].get('board.last_node_safe', 0) for gs in games.values() for g in gs)}")


def gate(cells, games):
    print('| variant | check | win % | gap to engine |')
    print('|---|---|---|---|')
    by = collections.defaultdict(dict)
    for cell, gs in games.items():
        c = cells[cell]
        by[(c['variant'], c['difficulty'])][c['agent']] = gs
    for (v, d), agents in sorted(by.items()):
        base = agents.get('engine')
        if not base:
            continue
        p = sum(g['won'] for g in base) / len(base)
        for a, gs in sorted(agents.items()):
            q = sum(g['won'] for g in gs) / len(gs)
            se = math.sqrt(p * (1 - p) / len(base) + q * (1 - q) / len(gs))
            flag = '' if p - q > 0 or a == 'engine' else ' X'
            print(f'| {v} d{d} | {a} | {100 * q:.0f} | {100 * (p - q):+.0f} ±{100 * se:.0f}{flag} |')


if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if a != '--gate']
    for d in args:
        cells, games = s.load(d)
        if '--gate' in sys.argv:
            gate(cells, games)
        else:
            table(cells, games)
            sources(games)
