"""240: screen and read reader.

    python read.py screen GAMES.jsonl SPEC.json
        s240's arms (base, F, FG, FP, FGP): pair win % at the spec's levels, each pair's gap to the pair mean, paired
        gaps against base per pair, and the touched powers' uses.
    python read.py pairs BASE.jsonl NEW.jsonl SPEC.json [LEVELS]
        a fresh read (bp / br / pr variants, both sides on the same seeds): pairs and gaps at LEVELS (default 1,2),
        the paired new - base gap, and the touched powers' uses a seat-game (pooled over the spec's levels).

Uses: a power over its board's seats; Grant and Peer Review also a seat-round (their bands' measure).
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
POWERS = ['builder.fortify', 'protector.debrief', 'protector.patrol', 'researcher.grant', 'researcher.peer_review',
          'protector.interdict', 'builder.mint']
BOARD = {'builder': 'b', 'protector': 'p', 'researcher': 'r'}


def load(path, spec):
    cells = [(v['name'], d, a.split(':')[0]) for c in spec['configs'] for v in spec['variants']
             for d in spec['difficulties'] for a in spec['agents']]
    out = {}
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if 'cell' not in g:
            continue
        v, d, a = cells[g['cell']]
        out[(v, d, a, g['seed'])] = g
    return out


def se(xs):
    if len(xs) < 2:
        return float('nan')
    m = sum(xs) / len(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def win(gs):
    return 100 * sum(g['won'] for g in gs) / len(gs) if gs else float('nan')


def pick(G, v, levels, a):
    return {k: g for k, g in G.items() if k[0] == v and k[1] in levels and k[2] == a}


def pair_of(v):
    return next(s for s in v.split('-') if s in ('bp', 'br', 'pr'))


def uses(G,variants, a, levels):
    """Uses a seat-game and a seat-round of each power, over the variants' games whose pair holds its board."""
    row = {}
    for p in POWERS:
        board = BOARD[p.split('.')[0]]
        gs = [g for k, g in G.items() if k[2] == a and k[1] in levels and k[0] in variants
              and board in pair_of(k[0])]
        n = sum(g['counters'].get(f'ai.power.{p}', 0) for g in gs)
        rounds = sum(g['rounds'] for g in gs)
        row[p] = (n / max(1, len(gs)), n / max(1, rounds))
    return row


def pair_table(G, arms, agents, levels, base_arm):
    print('| agent | arm | bp | br | pr | bp gap | max gap | bp vs base | br vs base | pr vs base |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for a in agents:
        for arm, vs in arms.items():
            w = {p: win(list(pick(G, v, levels, a).values())) for p, v in zip(('bp', 'br', 'pr'), vs)}
            m = sum(w.values()) / 3
            gaps = {p: w[p] - m for p in w}
            worst = max(gaps.values(), key=abs)
            cmp = []
            for p, v, bv in zip(('bp', 'br', 'pr'), vs, arms[base_arm]):
                if v == bv:
                    cmp.append('=')
                    continue
                B = pick(G, bv, levels, a)
                N = pick(G, v, levels, a)
                d = [100 * (N[k]['won'] - B[(bv,) + k[1:]]['won']) for k in N if (bv,) + k[1:] in B]
                cmp.append(f'{sum(d) / len(d):+.1f} ± {se(d):.1f}')
            print(f'| {a} | {arm} | ' + ' | '.join(f'{w[p]:.1f}' for p in w)
                  + f' | {gaps["bp"]:+.1f} | {worst:+.1f} | ' + ' | '.join(cmp) + ' |')


def screen(path, spec_path):
    spec = json.load(open(spec_path, encoding='utf-8'))
    G = load(path, spec)
    levels = tuple(spec['difficulties'])
    agents = [a.split(':')[0] for a in spec['agents']]
    arms = {'base': ('bp', 'br', 'pr'), 'F': ('bp-F', 'br-F', 'pr'), 'FG': ('bp-F', 'br-FG', 'pr-G'),
            'FP': ('bp-F', 'br-FP', 'pr-P'), 'FGP': ('bp-F', 'br-FGP', 'pr-GP')}
    print(f'## Pairs, d{levels[0]}-d{levels[-1]}, {len({k[3] for k in G})} seeds\n')
    pair_table(G, arms, agents, levels, 'base')
    print('\nBoth agents pooled, bp gap to the pair mean:')
    for arm, vs in arms.items():
        w = [win([g for k, g in G.items() if k[0] == v and k[1] in levels]) for v in vs]
        print(f'  {arm}: bp {w[0]:.1f} br {w[1]:.1f} pr {w[2]:.1f} -> bp gap {w[0] - sum(w) / 3:+.1f}')
    print('\n## Uses (a seat-game / a seat-round)\n')
    print('| agent | arm | ' + ' | '.join(POWERS) + ' |')
    print('|---|---|' + '---|' * len(POWERS))
    for a in agents:
        for arm, vs in arms.items():
            u = uses(G, set(vs), a, levels)
            print(f'| {a} | {arm} | ' + ' | '.join(f'{x:.2f} / {y:.2f}' for x, y in u.values()) + ' |')


def pairs(base_path, new_path, spec_path, levels):
    spec = json.load(open(spec_path, encoding='utf-8'))
    B = load(base_path, spec)
    N = load(new_path, spec)
    agents = [a.split(':')[0] for a in spec['agents']]
    G = {('base-' + k[0],) + k[1:]: g for k, g in B.items()}
    G.update({('new-' + k[0],) + k[1:]: g for k, g in N.items()})
    arms = {'base': ('base-bp', 'base-br', 'base-pr'), 'new': ('new-bp', 'new-br', 'new-pr')}
    print(f'## Pairs, d{levels[0]}-d{levels[-1]}\n')
    pair_table(G, arms, agents, levels, 'base')
    print('\nBoth agents pooled, bp gap to the pair mean:')
    for arm, vs in arms.items():
        w = [win([g for k, g in G.items() if k[0] == v and k[1] in levels]) for v in vs]
        print(f'  {arm}: bp {w[0]:.1f} br {w[1]:.1f} pr {w[2]:.1f} -> bp gap {w[0] - sum(w) / 3:+.1f}')
    alllv = tuple(spec['difficulties'])
    print('\n## By level\n')
    print('| agent | side | ' + ' | '.join(f'd{d}' for d in alllv) + ' | new - base ± SE |')
    print('|---|---|' + '---|' * (len(alllv) + 1))
    for a in agents:
        for side, H in (('base', B), ('new', N)):
            row = [win([g for k, g in H.items() if k[1] == d and k[2] == a]) for d in alllv]
            vs = ''
            if side == 'new':
                d = [100 * (g['won'] - B[k]['won']) for k, g in N.items() if k in B and k[2] == a]
                vs = f'{sum(d) / len(d):+.1f} ± {se(d):.1f}'
            print(f'| {a} | {side} | ' + ' | '.join(f'{x:.1f}' for x in row) + f' | {vs} |')
    print(f'\n## Uses a seat-game / a seat-round (d{alllv[0]}-d{alllv[-1]})\n')
    print('| agent | side | ' + ' | '.join(POWERS) + ' |')
    print('|---|---|' + '---|' * len(POWERS))
    for a in agents:
        for side, vs in arms.items():
            u = uses(G, set(vs), a, alllv)
            print(f'| {a} | {side} | ' + ' | '.join(f'{x:.2f} / {y:.2f}' for x, y in u.values()) + ' |')
    print('\nBoth agents pooled, a seat-game:')
    for side, vs in arms.items():
        gs = {p: [] for p in POWERS}
        for p in POWERS:
            board = BOARD[p.split('.')[0]]
            sel = [g for k, g in G.items() if k[0] in vs and board in pair_of(k[0])]
            gs[p] = sum(g['counters'].get(f'ai.power.{p}', 0) for g in sel) / max(1, len(sel))
        print(f'  {side}: ' + ', '.join(f'{p.split(".")[1]} {v:.2f}' for p, v in gs.items()))


if sys.argv[1] == 'screen':
    screen(sys.argv[2], sys.argv[3])
else:
    lv = tuple(int(x) for x in sys.argv[5].split(',')) if len(sys.argv) > 5 else (1, 2)
    pairs(sys.argv[2], sys.argv[3], sys.argv[4], lv)
