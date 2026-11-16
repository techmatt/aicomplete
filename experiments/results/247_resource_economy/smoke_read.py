"""Per agent of a smoke spec (3 pairs x d0-d3 x A agents): games, wins, end reasons, rounds, Liberate with
antimatter a seat-game, antimatter gained / spent, Teardown uses a seat-game.
usage: python smoke_read.py GAMES.jsonl AGENT [AGENT ...]"""
import json, sys
from collections import Counter, defaultdict
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
path, agents = sys.argv[1], sys.argv[2:]
A = len(agents)
d = defaultdict(lambda: defaultdict(float))
reasons = defaultdict(Counter)
with open(path, encoding='utf-8') as f:
    for l in f:
        if not l.startswith('{"cell"'):
            continue
        g = json.loads(l)
        a = g['cell'] % A
        x = d[a]
        seats = len(g['build'].split('|'))
        boards = [b.split(':')[0] for b in g['build'].split('|')]
        x['games'] += 1
        x['seats'] += seats
        x['won'] += g['won']
        x['rounds'] += g['rounds']
        reasons[a][g['reason']] += 1
        for s in range(seats):
            x['am_gain'] += g['gained'][s][4]
            x['am_spent'] += g['spent'][s][4]
        for act in g['actions']:
            if act.get('type') == 'standard' and act.get('act') == 'liberate_am':
                x['lib_am'] += 1
            if act.get('type') == 'power' and boards[act['seat']] == 'builder' and act.get('power') == 7:
                x['teardown'] += 1
for a in range(A):
    x = d[a]
    print(f"{agents[a]}: {x['games']:.0f} games, won {100 * x['won'] / x['games']:.1f}%, rounds {x['rounds'] / x['games']:.2f}, "
          f"liberate_am {x['lib_am'] / x['seats']:.2f}/seat-game, antimatter {x['am_gain'] / x['seats']:.2f} / {x['am_spent'] / x['seats']:.2f} "
          f"({x['am_spent'] / max(1, x['am_gain']):.0%}), teardown {x['teardown'] / x['seats']:.2f}/seat-game; ends {dict(reasons[a])}")
