"""275: the agents' sources and sinks a game-round by level, the judge, for `AGENT_FLOW` in
crates/aic-sim/src/docs/harms.rs (docs/harms.md's *Agents in and out, by level*).

    python flow.py READ/games.jsonl [--agent strategist]

Prints the Rust rows: (level, arrivals, of them Infiltrate, canceled on arrival, removed by players, back by
corruption, cities corrupted by round 4 a game, by the end a game).
"""
import argparse
import collections
import json


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('games')
    ap.add_argument('--agent', default='strategist')
    a = ap.parse_args()
    agg = collections.defaultdict(collections.Counter)
    with open(a.games, encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        cells = [(d, ag) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties'] for ag in spec['agents']]
        for line in f:
            g = json.loads(line)
            d, ag = cells[g['cell']]
            if ag != a.agent:
                continue
            c = g['counters']
            x = agg[d]
            x['games'] += 1
            x['rounds'] += g['rounds']
            for k in ('adversary.arrivals', 'adversary.infiltrated', 'adversary.canceled', 'adversary.ambassadors_removed',
                      'adversary.returned_by_corruption'):
                x[k] += c.get(k, 0)
            x['r4'] += sum(v for k, v in c.items() if k.startswith('map.cities_corrupted.r') and int(k[-2:]) <= 4)
            x['all'] += sum(v for k, v in c.items() if k.startswith('map.cities_corrupted.r'))
    for d in sorted(agg):
        x = agg[d]
        r, n = x['rounds'], x['games']
        vals = [x['adversary.arrivals'] / r, x['adversary.infiltrated'] / r, x['adversary.canceled'] / r,
                x['adversary.ambassadors_removed'] / r, x['adversary.returned_by_corruption'] / r, x['r4'] / n, x['all'] / n]
        print(f'    ({d}, ' + ', '.join(f'{v:.1f}' for v in vals) + '),')


main()
