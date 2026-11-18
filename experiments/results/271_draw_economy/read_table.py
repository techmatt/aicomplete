"""271: a read's win % and rounds by level and agent, with cards played and drawn a game-round (all seats).

    python experiments/results/271_draw_economy/read_table.py GAMES.jsonl [GAMES.jsonl ...]

Each file is a farmed or experiment games.jsonl (a header line, if any, is skipped). Columns per file.
"""
import collections
import json
import math
import sys


AGENTS = ('strategist', 'greedy')


def agent_of(cell):
    """The spec's cells run variant-major, then difficulty, then agent: the agent is the cell's parity."""
    return AGENTS[cell % 2] if isinstance(cell, int) else cell.split('/')[-1]


def num(v):
    """A counter's value: a leaf, or a node whose own count is under ''."""
    return v.get('', 0) if isinstance(v, dict) else v


def read(path):
    rows = collections.defaultdict(list)
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if 'seed' not in g:
            continue
        c = g.get('counters', {})
        # Counters are flat, dotted keys.
        n_played = sum(num(v) for k, v in c.items() if k.startswith('cards.played.'))
        drawn = num(c.get('cards.drawn', 0))
        rows[(g['difficulty'], agent_of(g['cell']))].append((bool(g['won']), g['rounds'], n_played, drawn))
    return rows


def main():
    reads = [read(p) for p in sys.argv[1:]]
    keys = sorted(set().union(*reads))
    head = ['Level', 'Agent'] + [f'{h} ({i + 1})' for i in range(len(reads)) for h in
                                 ('games', 'win %', 'rounds', 'plays / round', 'drawn / round')]
    print('| ' + ' | '.join(head) + ' |')
    print('|' + '---|' * len(head))
    for k in keys:
        cells = [f'd{k[0]}', k[1]]
        for r in reads:
            g = r.get(k, [])
            if not g:
                cells += ['-'] * 5
                continue
            n = len(g)
            p = sum(w for w, *_ in g) / n
            se = math.sqrt(p * (1 - p) / n) * 100
            rounds = sum(x[1] for x in g) / n
            cells += [str(n), f'{p * 100:.0f} ± {se:.0f}', f'{rounds:.1f}',
                      f'{sum(x[2] for x in g) / sum(x[1] for x in g):.1f}',
                      f'{sum(x[3] for x in g) / sum(x[1] for x in g):.1f}']
        print('| ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
