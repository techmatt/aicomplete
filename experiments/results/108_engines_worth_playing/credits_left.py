"""108: resources left at game end per seat, by agent, pair and board, from an experiment's games.jsonl.

usage: python credits_left.py DIR [DIR2]   (DIR2: a second run on the same seeds, read as after)
Reads `margin.unspent` (per seat: the resources held at the end) and the cell's agent and variant.
"""
import json
import sys
from collections import defaultdict

BOARDS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}


def load(d):
    lines = open(f'{d}/games.jsonl', encoding='utf-8').read().splitlines()
    head = json.loads(lines[0])
    spec = head['spec']
    cells = []
    for v in spec['variants']:
        for diff in spec['difficulties']:
            for a in spec['agents']:
                cells.append((v['name'], a))
    out = defaultdict(list)
    names = None
    for ln in lines[1:]:
        g = json.loads(ln)
        if 'cell' not in g:
            continue
        m = g.get('margin', {})
        un = m.get('unspent')
        if un is None:
            continue
        names = names or m.get('unspent_names')
        v, a = cells[g['cell']]
        for seat, vals in enumerate(un):
            out[(a, v, BOARDS[v][seat])].append(vals)
    return out, names


def main():
    runs = [load(d) for d in sys.argv[1:]]
    names = runs[0][1] or ['credits', '?']
    print('resource order:', names)
    keys = sorted(runs[0][0])
    hdr = '| agent | pair | board | ' + ' | '.join(f'{n} {i}' for i in range(len(runs)) for n in names[:1]) + ' |'
    print(hdr)
    for k in keys:
        cols = []
        for r, _ in runs:
            vals = r.get(k, [])
            cols.append(sum(v[0] for v in vals) / max(1, len(vals)))
        print(f'| {k[0]} | {k[1]} | {k[2]} | ' + ' | '.join(f'{c:.1f}' for c in cols) + ' |')


if __name__ == '__main__':
    main()
