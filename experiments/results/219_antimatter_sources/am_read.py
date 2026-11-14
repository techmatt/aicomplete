"""219: antimatter sources and spends a seat-game, by board and by plant, from a read's games.jsonl.

    python experiments/results/219_antimatter_sources/am_read.py READ [READ...]

READ: a folder holding games.jsonl, or the file (a farm's out file too: no header needed, then three pair cells
variant-major). With a header, variants named `PAIR` or `PAIR_LEVER` are read one lever at a time. Per lever:
antimatter gained and spent a seat-game; the top sources' shares; each board power that makes antimatter, uses a
seat-game of its board's seats; each plant's antimatter a seat-game from its Action and the share of games and
seat-games that played it. Seat 0 is the pair's first board (bp: Builder, Protector; br: Builder, Researcher;
pr: Protector, Researcher).
"""
import json
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4  # games.jsonl's resource order: credits, insight, energy, chips, antimatter, cubes, card tokens, ambassadors
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
POWERS = {'Teardown': 'builder', 'Refine': 'builder', 'Seize': 'protector', 'Joint Facility': 'protector',
          'Overload': 'researcher', 'Particle Run': 'researcher'}
PLANTS = ['penning_trap_array', 'antiproton_foundry', 'positron_storage_ring', 'dark_fiber_collider',
          'treaty_isotope_bank', 'cyclotron_lease']


def load(path):
    f = os.path.join(path, 'games.jsonl') if os.path.isdir(path) else path
    rows, spec = [], None
    with open(f, encoding='utf-8') as fh:
        for line in fh:
            g = json.loads(line)
            if 'sources' in g:
                rows.append(g)
            elif 'spec' in g:
                spec = g['spec']
    if spec:
        names = [v['name'] for v in spec['variants'] for _ in spec['difficulties'] for _ in spec['agents']]
    else:
        per = (max(g['cell'] for g in rows) + 1) // 3
        names = [['bp', 'br', 'pr'][c // per] for c in range(per * 3)]
    for g in rows:
        pair, _, lv = names[g['cell']].partition('_')
        g['pair'], g['lever'] = pair, lv or 'base'
    return rows


def read(label, rows):
    n = seats = 0
    board_seats = defaultdict(int)
    gained = spent = make = 0.0
    src = defaultdict(float)
    power = defaultdict(float)
    plant_am = defaultdict(float)
    plant_games = defaultdict(int)
    plant_seats = defaultdict(int)
    wins = 0
    for g in rows:
        n += 1
        wins += bool(g.get('won'))
        pair = PAIRS[g['pair']]
        in_game = set()
        uses = g.get('uses') or []
        for i, per in enumerate(g['sources']):
            seats += 1
            b = pair[i] if i < len(pair) else None
            board_seats[b] += 1
            keys = set(per) | set(uses[i] if i < len(uses) else {})
            for k in keys:
                m = re.match(r'(own:)?play:(\w+)$', k)
                if m and m.group(2) in PLANTS:
                    plant_seats[m.group(2)] += 1
                    in_game.add(m.group(2))
            for k, row in per.items():
                v = row[AM]
                if not v:
                    continue
                k = re.sub(r'^(own|other):', '', k)
                gained += v
                src[k] += v
                m = re.match(r'power:(.+)$', k)
                if m and m.group(1) in POWERS:
                    power[m.group(1)] += v
                m = re.match(r'use:(\w+)$', k)
                if m and m.group(1) in PLANTS:
                    plant_am[m.group(1)] += v
                if k == 'standard:antimatter':
                    make += v
        for per in uses:
            spent += sum(row[AM] for row in per.values())
        for c in in_game:
            plant_games[c] += 1
        # amendment 9855: games with one or more tier-2 plants built (any seat), and with two or more
        t2 = in_game - {'cyclotron_lease'}
        plant_games['any tier-2'] += bool(t2)
        plant_games['2+ tier-2'] += len(t2) >= 2
    s = seats or 1
    print(f'{label}: {n} games, {seats} seat-games, won {wins / max(n, 1):.1%}')
    print(f'  antimatter a seat-game: gained {gained / s:.2f}, spent {spent / s:.2f} ({spent / max(gained, 1e-9):.0%});'
          f' Make antimatter {make / s:.2f}')
    tot = gained or 1
    for k, v in sorted(src.items(), key=lambda x: -x[1])[:10]:
        print(f'  source {k}: {v / s:.3f} a seat-game ({v / tot:.0%})')
    for p, b in POWERS.items():
        if power[p]:
            print(f'  power {b} {p}: {power[p] / max(board_seats[b], 1):.3f} a {b} seat-game')
    plants_used = sum(plant_am.values())
    print(f'  plants: {plants_used / s:.3f} antimatter a seat-game from their Actions ({plants_used / tot:.0%}); '
          f'games with 1+ tier-2 plant built {plant_games["any tier-2"] / max(n, 1):.0%}, '
          f'2+ {plant_games["2+ tier-2"] / max(n, 1):.0%}')
    for c in PLANTS:
        print(f'  plant {c}: {plant_am[c] / s:.3f} a seat-game from its Action; played in '
              f'{plant_games[c] / max(n, 1):.0%} of games, {plant_seats[c] / s:.1%} of seat-games')


for p in sys.argv[1:]:
    rows = load(p)
    order = []
    for g in rows:
        if g['lever'] not in order:
            order.append(g['lever'])
    for lv in order:
        read(f'{p} [{lv}]', [g for g in rows if g['lever'] == lv])
