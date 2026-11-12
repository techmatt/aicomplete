"""190: facilities on the map by round and owner, neutral claims by round, city growth and the board upkeep it costs,
and troops, from a ladder's games.jsonl (its `.rNN` per-round counters). Usage:

    python experiments/results/190_rules_growth_chips/map_read.py NEW/games.jsonl OLD/games.jsonl [--md OUT.md]

Per game, averaged over games still running at that round (a finished game stops counting). Rounds 1-10.
"""
import json
import sys
from collections import defaultdict

ROUNDS = range(1, 11)
SERIES = [
    ('factories, player', 'map.factories.player'),
    ('factories, neutral', 'map.factories.neutral'),
    ('factories, adversary', 'map.factories.adversary'),
    ('stations, player', 'map.stations.player'),
    ('stations, neutral', 'map.stations.neutral'),
    ('stations, adversary', 'map.stations.adversary'),
    ('neutral factory claims', 'map.claims.factory'),
    ('neutral station claims', 'map.claims.station'),
    ('city builds: factory', 'map.built.factory'),
    ('city builds: station', 'map.built.station'),
    ('chips waiting on cities', 'map.city_chips'),
    ('growth token moves', 'map.upkeep'),
    ('adversary troops on the map', 'map.troops'),
]
TOTALS = ['board.troops_built', 'threats.troops', 'adversary.project_troops', 'board.troop_steps',
          'board.factories_destroyed', 'board.war_nodes', 'board.war_destroyed', 'board.city_waits',
          'board.factory_built_by_city', 'board.station_built_by_city', 'board.factories_built.adversary',
          'board.factories_built.player', 'board.factories_built.neutral', 'board.chips_made',
          'board.claimed.factory', 'board.claimed.station', 'board.claimed.city']


def read(path):
    by_round = defaultdict(lambda: defaultdict(float))
    alive = defaultdict(int)
    totals = defaultdict(float)
    n = 0
    troops_by_round = defaultdict(float)
    with open(path, encoding='utf-8') as f:
        f.readline()
        for line in f:
            g = json.loads(line)
            if 'counters' not in g:
                continue
            n += 1
            c = g['counters']
            for r in ROUNDS:
                if r <= g['rounds']:
                    alive[r] += 1
            for k, v in c.items():
                if '.r' in k:
                    base, _, rr = k.rpartition('.r')
                    if rr.isdigit():
                        by_round[base][int(rr)] += v
            for k in TOTALS:
                totals[k] += c.get(k, 0)
    return n, alive, by_round, totals


def fmt(x):
    return f'{x:.2f}'


def main():
    new, old = sys.argv[1], sys.argv[2]
    out = []
    a = read(new)
    b = read(old)
    out.append(f'Games: r83 {a[0]}, r82 {b[0]}. Per game still running at that round.\n')
    head = '| measure | side | ' + ' | '.join(f'r{r}' for r in ROUNDS) + ' |'
    out.append(head)
    out.append('|' + '---|' * (len(ROUNDS) + 2))
    for label, key in SERIES:
        for side, (n, alive, by_round, _) in (('r83', a), ('r82', b)):
            vals = [by_round[key].get(r, 0) / max(1, alive[r]) for r in ROUNDS]
            if side == 'r82' and not any(vals):
                continue
            out.append(f'| {label} | {side} | ' + ' | '.join(fmt(v) for v in vals) + ' |')
    out.append('\nGames running at each round: r83 ' + ', '.join(str(a[1][r]) for r in ROUNDS)
               + '; r82 ' + ', '.join(str(b[1][r]) for r in ROUNDS) + '\n')
    out.append('| total a game | r83 | r82 |')
    out.append('|---|---|---|')
    for k in TOTALS:
        out.append(f'| {k} | {fmt(a[3][k] / max(1, a[0]))} | {fmt(b[3][k] / max(1, b[0]))} |')
    text = '\n'.join(out) + '\n'
    if '--md' in sys.argv:
        with open(sys.argv[sys.argv.index('--md') + 1], 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)
    sys.stdout.reconfigure(encoding='utf-8')
    print(text)


main()
