"""230: antimatter supply by source, spent / made and burns by round, per lever and agent, from farmed reads.

    python experiments/results/230_antimatter_followups/am.py OUT.jsonl[,OUT2] [SPEC.json]

The spec is read from the jsonl's header when not given. A variant named `PAIR_LEVER` reads as lever LEVER of pair
PAIR (`PAIR` alone: lever `base`). Per lever and agent: antimatter made and spent a seat-game; supply shares by source
(plants' Actions, Horizon's converter, upkeep, burns, Make antimatter, the rest: cards' on-play gains and other
Actions); the cheap share (Make antimatter plus tier-1 cards, as design_check counts it); each burn's uses a seat-game
of its board, its share in rounds 1-4 and in the game's last two rounds; win rate and rounds.
"""
import json
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4  # games.jsonl's resource order: credits, insight, energy, chips, antimatter, cubes, card tokens, ambassadors
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
BURNS = {'teardown': 'builder', 'meltdown': 'researcher', 'reclaim': 'protector'}
PLANTS = {'penning_trap_array', 'antiproton_foundry', 'positron_storage_ring', 'dark_fiber_collider',
          'treaty_isotope_bank', 'cyclotron_lease'}
TIER1 = {'cyclotron_lease'}  # refreshed from content below
CLASSES = ['plants', 'converter', 'upkeep', 'burns', 'make', 'other']


def tier1():
    try:
        with open('content/cards.json', encoding='utf-8') as f:
            return {c['id'] for c in json.load(f) if c.get('tier') == 1}
    except OSError:
        return TIER1


def klass(k):
    m = re.match(r'use:(\w+)$', k)
    if m and m.group(1) in PLANTS:
        return 'plants'
    if k.startswith('convert:'):
        return 'converter'
    if k == 'phase:economy.upkeep':
        return 'upkeep'
    if re.match(r'power:(Teardown|Meltdown|Reclaim)$', k):
        return 'burns'
    if k == 'standard:antimatter':
        return 'make'
    return 'other'


def load(paths, spec):
    rows = []
    for p in paths.split(','):
        with open(p, encoding='utf-8') as fh:
            for line in fh:
                g = json.loads(line)
                if 'spec' in g and spec is None:
                    spec = g['spec']
                elif 'cell' in g:
                    rows.append(g)
    cells = [(v['name'], d, a) for c in spec.get('configs', ['x']) for v in spec['variants']
             for d in spec['difficulties'] for a in spec['agents']]
    for g in rows:
        v, d, a = cells[g['cell']]
        pair, _, lv = v.partition('_')
        g['pair'], g['lever'], g['agent'], g['d'] = pair, lv or 'base', a, d
    return rows


def read(label, games, t1):
    seats = 0
    made = spent = 0.0
    cls = defaultdict(float)
    cheap = 0.0
    board_seats = defaultdict(int)
    uses, early, late = defaultdict(int), defaultdict(int), defaultdict(int)
    for g in games:
        for b in PAIRS[g['pair']]:
            board_seats[b] += 1
        for per in g['sources']:
            seats += 1
            for k, row in per.items():
                v = row[AM]
                if not v:
                    continue
                k = re.sub(r'^(own|other):', '', k)
                made += v
                cls[klass(k)] += v
                m = re.match(r'(play|use):(\w+)$', k)
                if k.startswith('standard:') or (m and m.group(2) in t1):
                    cheap += v
        for per in g.get('uses') or []:
            spent += sum(row[AM] for row in per.values())
        for k, n in g['counters'].items():
            m = re.match(r'ai\.power\.\w+\.(teardown|meltdown|reclaim)\.r(\d+)$', k)
            if m:
                r = int(m.group(2))
                uses[m.group(1)] += n
                early[m.group(1)] += n * (r <= 4)
                late[m.group(1)] += n * (r >= g['rounds'] - 1)
    s, tot = seats or 1, made or 1
    n = len(games)
    won = sum(bool(g.get('won')) for g in games) / max(n, 1)
    rounds = sum(g['rounds'] for g in games) / max(n, 1)
    shares = ' '.join(f'{c} {cls[c] / tot:.0%}' for c in CLASSES)
    print(f'{label}: {n} games, won {won:.1%}, rounds {rounds:.2f}; antimatter a seat-game made {made / s:.2f} '
          f'spent {spent / s:.2f} ({spent / tot:.0%}); cheap {cheap / tot:.0%}')
    print(f'  shares: {shares}')
    print('  a seat-game: ' + ' '.join(f'{c} {cls[c] / s:.2f}' for c in CLASSES))
    tb = sum(uses.values())
    te = sum(early.values())
    parts = []
    for p, b in BURNS.items():
        u = uses[p]
        parts.append(f'{p} {u / max(board_seats[b], 1):.3f} ({early[p] / u:.0%} r1-4, {late[p] / u:.0%} last 2)'
                     if u else f'{p} never')
    print(f'  burns: all {tb / max(n, 1):.3f} a game, {te / max(tb, 1):.0%} in rounds 1-4; ' + '; '.join(parts))


def main():
    spec = None
    if len(sys.argv) > 2:
        with open(sys.argv[2], encoding='utf-8') as f:
            spec = json.load(f)
    games = load(sys.argv[1], spec)
    t1 = tier1()
    keys = []
    for g in games:
        k = (g['lever'], g['agent'])
        if k not in keys:
            keys.append(k)
    for lv, a in sorted(keys):
        read(f'{lv} {a}', [g for g in games if (g['lever'], g['agent']) == (lv, a)], t1)


if __name__ == '__main__':
    main()
