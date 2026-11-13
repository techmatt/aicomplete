"""Node losses by source and level for given levers of a screen: what ends a game by `node`, and when.

    python node_read.py READ LEVER [LEVER ...]
READ as screen_read.py takes it. Prints per lever and level: node-loss games' share and rounds, and per game the
nodes lost by source (`board.node_lost.*`, `adversary.corruptions.node`).
"""
import collections
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
import screens  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8')


def main():
    read, levers = sys.argv[1], sys.argv[2:]
    it = screens.stream(read)
    cells = screens.cells_of(next(it)['spec'])
    by = collections.defaultdict(list)
    for g in it:
        v = cells[g['cell']]['variant']
        lever = v.partition('_')[2] or 'base'
        if lever in levers:
            by[(lever, g['difficulty'])].append(g)
    keys = ['board.node_lost', 'board.node_lost.threat2', 'board.node_lost.project', 'board.node_lost.war',
            'adversary.corruptions.node']
    print('lever | d | node-loss % | its rounds | ' + ' | '.join(k.replace('board.node_lost', 'lost') for k in keys))
    for (lever, d), gs in sorted(by.items()):
        nl = [g for g in gs if g['reason'] == 'node']
        r = sum(g['rounds'] for g in nl) / len(nl) if nl else float('nan')
        per = [sum(g['counters'].get(k, 0) for g in gs) / len(gs) for k in keys]
        print(f'{lever} | {d} | {100 * len(nl) / len(gs):.0f} | {r:.2f} | ' + ' | '.join(f'{x:.2f}' for x in per))


if __name__ == '__main__':
    main()
