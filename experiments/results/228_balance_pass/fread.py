"""228: fresh read reader. Base and new run the same spec (f228.json: six seat orders, d0-d3) on the same seeds.

    python fread.py BASE.jsonl NEW.jsonl SPEC.json

Prints, per agent and side: win % by unordered pair (both seat orders pooled) at d1-d2 and d0-d3 with each pair's gap
to the pair mean, by board, by level, the new-against-base paired gap, the seat-order split (reported only), and the
uses a seat-game of each board's powers and of the standard actions this item touched.
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIR = {'bp': 'bp', 'pb': 'bp', 'br': 'br', 'rb': 'br', 'pr': 'pr', 'rp': 'pr'}


def load(path, spec):
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    out = {}
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if 'cell' not in g:
            continue
        v, d, a = cells[g['cell']]
        out[(v, d, a.split(':')[0], g['seed'])] = g
    return out


def se(xs):
    m = sum(xs) / len(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def win(gs):
    return 100 * sum(g['won'] for g in gs) / len(gs) if gs else float('nan')


spec = json.load(open(sys.argv[3], encoding='utf-8'))
B = load(sys.argv[1], spec)
N = load(sys.argv[2], spec)
agents = [a.split(':')[0] for a in spec['agents']]
for lname, levels in (('d1-d2', (1, 2)), ('d0-d3', (0, 1, 2, 3))):
    print(f'\n## Pairs, {lname} (both seat orders pooled)\n')
    print('| agent | side | bp | br | pr | mean | max gap | Builder | Protector | Researcher | all | new − base ± SE |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for a in agents:
        for side, G in (('base', B), ('new', N)):
            w = {p: win([g for k, g in G.items() if PAIR[k[0]] == p and k[1] in levels and k[2] == a])
                 for p in ('bp', 'br', 'pr')}
            m = sum(w.values()) / 3
            gaps = {p: w[p] - m for p in w}
            worst = max(gaps.values(), key=abs)
            boards = [(w['bp'] + w['br']) / 2, (w['bp'] + w['pr']) / 2, (w['br'] + w['pr']) / 2]
            allw = win([g for k, g in G.items() if k[1] in levels and k[2] == a])
            vs = ''
            if side == 'new':
                d = [100 * (g['won'] - B[k]['won']) for k, g in N.items() if k in B and k[1] in levels and k[2] == a]
                vs = f'{sum(d) / len(d):+.1f} ± {se(d):.1f}'
            print(f'| {a} | {side} | ' + ' | '.join(f'{w[p]:.1f} ({gaps[p]:+.1f})' for p in w)
                  + f' | {m:.1f} | {worst:+.1f} | ' + ' | '.join(f'{x:.1f}' for x in boards) + f' | {allw:.1f} | {vs} |')

print('\n## By level (all pairs and orders)\n')
print('| agent | side | d0 | d1 | d2 | d3 | rounds |')
print('|---|---|---|---|---|---|---|')
for a in agents:
    for side, G in (('base', B), ('new', N)):
        row = [win([g for k, g in G.items() if k[1] == d and k[2] == a]) for d in (0, 1, 2, 3)]
        gs = [g for k, g in G.items() if k[2] == a]
        print(f'| {a} | {side} | ' + ' | '.join(f'{x:.1f}' for x in row) + f' | {sum(g["rounds"] for g in gs) / len(gs):.2f} |')

print('\n## Seat order, d0-d3 (reported only): first-named board in seat 0\n')
print('| agent | side | bp | pb | br | rb | pr | rp |')
print('|---|---|---|---|---|---|---|---|')
for a in agents:
    for side, G in (('base', B), ('new', N)):
        print(f'| {a} | {side} | ' + ' | '.join(f'{win([g for k, g in G.items() if k[0] == v and k[2] == a]):.1f}'
                                         for v in ('bp', 'pb', 'br', 'rb', 'pr', 'rp')) + ' |')

print('\n## Uses a seat-game (pooled d0-d3; a power over its board\'s seats, a standard action over all seats)\n')
keys = sorted({c for G in (B, N) for g in G.values() for c in g['counters']
               if (c.startswith('ai.power.') and '.r' not in c[len('ai.power.x'):].split('.', 1)[-1][:2]
                   and c.count('.') == 3) or c in ('board.std.remove', 'board.std.liberate', 'board.std.envoy')})
print('| counter | ' + ' | '.join(f'{a} base | {a} new' for a in agents) + ' |')
print('|---|' + '---|' * (2 * len(agents)))
for c in keys:
    board = c.split('.')[2][0] if c.startswith('ai.power.') else None
    row = []
    for a in agents:
        for G in (B, N):
            gs = [(k, g) for k, g in G.items() if k[2] == a]
            seats = sum((1 if board in k[0] else 0) if board else 2 for k, _ in gs)
            row.append(f'{sum(g["counters"].get(c, 0) for k, g in gs if board is None or board in k[0]) / max(1, seats):.2f}')
    print(f'| {c} | ' + ' | '.join(row) + ' |')
