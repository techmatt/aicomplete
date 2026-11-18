"""262: converter and resource counters a game, pooled, per read: what the converters did and why they sat.

    python experiments/results/262_balance_economy/conv_counts.py READ [READ...] [--prefix P]...

READ: a read's folder (games.jsonl) or a games file. Prints the mean a game of every counter whose key starts with a
prefix (default: the converter, chip-converter and standard-action tallies), side by side.
"""
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'scripts'))
import screens  # noqa: E402

DEFAULT = ['board.converter', 'board.chip_converter', 'board.std.', 'board.cubes_', 'board.chips_', 'board.claimed']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('reads', nargs='+')
    ap.add_argument('--prefix', action='append')
    ap.add_argument('--by-variant', action='store_true', help='a column per variant of each read')
    ap.add_argument('--by-agent', action='store_true', help='a column per agent of each read')
    a = ap.parse_args()
    prefixes = a.prefix or DEFAULT
    cols = []
    keys = set()
    for r in a.reads:
        cells, by_cell = screens.load(r)
        groups = {}
        for cid, gs in by_cell.items():
            name = os.path.basename(os.path.normpath(r))
            if a.by_variant:
                name += ':' + str(cells[cid].get('variant', '?'))
            if a.by_agent:
                name += ' ' + str(cells[cid].get('agent', '?'))
            groups.setdefault(name, []).extend(gs)
        for name, gs in groups.items():
            tot = {}
            for g in gs:
                for k, v in (g.get('counters') or {}).items():
                    if any(k.startswith(p) for p in prefixes):
                        tot[k] = tot.get(k, 0) + v
            cols.append((name, len(gs), tot))
            keys |= set(tot)
    print('| counter | ' + ' | '.join(name for name, _, _ in cols) + ' |')
    print('|---|' + '---|' * len(cols))
    for k in sorted(keys):
        print(f'| {k} | ' + ' | '.join(f'{t.get(k, 0) / max(n, 1):.2f}' for _, n, t in cols) + ' |')


if __name__ == '__main__':
    main()
