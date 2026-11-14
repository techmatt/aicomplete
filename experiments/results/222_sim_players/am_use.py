"""222: are 219's antimatter burns and converter plants used sensibly? Per agent of a farmed read: each burn power's uses
a seat-game of its board, the round it fires in and its share in the game's last two rounds or its first four; each
plant's plays a game, its Action's antimatter a seat-game that built it, and Actions a round once built.

    python am_use.py OUT.jsonl[,OUT2] SPEC.json
"""
import json
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
BURNS = {'teardown': 'builder', 'overload': 'researcher', 'seize': 'protector'}
PLANTS = ['penning_trap_array', 'antiproton_foundry', 'positron_storage_ring', 'dark_fiber_collider',
          'treaty_isotope_bank', 'cyclotron_lease']


def main():
    with open(sys.argv[2], encoding='utf-8') as f:
        spec = json.load(f)
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    by = defaultdict(list)
    for p in sys.argv[1].split(','):
        with open(p, encoding='utf-8') as f:
            for line in f:
                g = json.loads(line)
                if 'cell' in g:
                    v, d, a = cells[g['cell']]
                    g['pair'] = v
                    by[a].append(g)
    for a, games in by.items():
        board_seats = defaultdict(int)
        uses = defaultdict(int)
        late = defaultdict(int)
        early = defaultdict(int)
        rsum = defaultdict(float)
        built = defaultdict(int)
        plant_am = defaultdict(float)
        plant_rounds = defaultdict(float)
        for g in games:
            for b in PAIRS[g['pair']]:
                board_seats[b] += 1
            for k, n in g['counters'].items():
                m = re.match(r'ai\.power\.(\w+)\.(teardown|overload|seize)\.r(\d+)$', k)
                if m:
                    r = int(m.group(3))
                    uses[m.group(2)] += n
                    rsum[m.group(2)] += n * r
                    late[m.group(2)] += n * (r >= g['rounds'] - 1)
                    early[m.group(2)] += n * (r <= 4)
            srcs = g.get('sources') or []
            for i, per in enumerate(g.get('uses') or []):
                for k in per:
                    m = re.match(r'(own:)?play:(\w+)$', k)
                    if m and m.group(2) in PLANTS:
                        built[m.group(2)] += 1
                        plant_rounds[m.group(2)] += g['rounds']
                        src = srcs[i] if i < len(srcs) else {}
                        plant_am[m.group(2)] += sum(row[AM] for kk, row in src.items()
                                                    if re.sub(r'^(own|other):', '', kk) == f'use:{m.group(2)}')
        print(f'{a}: {len(games)} games')
        for p, b in BURNS.items():
            n = uses[p]
            if n:
                print(f'  burn {p}: {n / board_seats[b]:.3f} a {b} seat-game, mean round {rsum[p] / n:.1f}, '
                      f'last two rounds {late[p] / n:.0%}, rounds 1-4 {early[p] / n:.0%}')
            else:
                print(f'  burn {p}: never')
        for c in PLANTS:
            if built[c]:
                print(f'  plant {c}: built {built[c] / len(games):.3f} a game; its Action {plant_am[c] / built[c]:.2f} '
                      f'antimatter a seat-game that built it')


if __name__ == '__main__':
    main()
