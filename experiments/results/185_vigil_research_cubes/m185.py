"""185's measures, old against new on the same seeds: Vigil's insight, adversary cubes landed and paid, and the D77
guards (amendment 8456), a game, by difficulty (and by pair with --pairs).

    python experiments/results/185_vigil_research_cubes/m185.py NEW_DIR OLD_DIR [--pairs]

Vigil pays 1 insight a firing, so `board.vigil` a game is its insight a game.
"""
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts', 'bench')))
from ladder import load, rev  # noqa: E402

KEYS = [
    ('Vigil insight', 'board.vigil'),
    ('Vigil capped', 'board.vigil_capped'),
    ('adv cubes landed (stations)', 'board.adversary_cubes'),
    ('adv cubes paid', 'board.cubes_paid.adversary'),
    ('station compute', 'board.station_compute'),
    ('projects completed', 'board.projects_completed'),
    ('guard: flip', 'adversary.guard_held.flip'),
    ('guard: forced flip', 'adversary.guard_held.force_flip'),
    ('guard: corrupt city', 'adversary.guard_held.corrupt_city'),
    ('stations flipped', 'adversary.stations_flipped'),
]


def table(d, pairs):
    by = defaultdict(list)
    for cell, g in load(d):
        name = cell.get('variant') or cell.get('name') or ''
        key = (cell['difficulty'], name) if pairs else (cell['difficulty'], '')
        by[key].append(g)
    rows = []
    for (diff, name), games in sorted(by.items()):
        n = len(games)
        win = 100 * sum(1 for g in games if g['won']) / n
        vals = [sum(g['counters'].get(k, 0) for g in games) / n for _, k in KEYS]
        rows.append((diff, name, n, win, vals))
    return rows


def main():
    new, old = sys.argv[1], sys.argv[2]
    pairs = '--pairs' in sys.argv
    head = '| side | d | pair | games | win % | ' + ' | '.join(l for l, _ in KEYS) + ' |'
    print(head)
    print('|' + '---|' * (len(KEYS) + 5))
    for d in (old, new):
        side = rev(d)
        for diff, name, n, win, vals in table(d, pairs):
            print(f'| {side} | d{diff} | {name} | {n} | {win:.1f} | ' + ' | '.join(f'{v:.2f}' for v in vals) + ' |')


if __name__ == '__main__':
    main()
