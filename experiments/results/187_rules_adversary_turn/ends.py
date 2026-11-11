"""187: end reasons per difficulty, new against old, and a few counters' means (land corruption, tip drones).

    python ends.py NEW_DIR OLD_DIR
"""
import json
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KEYS = ['adversary.corruptions.node', 'board.tip_factory_troops', 'board.units_built', 'board.war_destroyed',
        'adversary.wall_troops', 'prohibit.rally.ambassador.made', 'alignment.tips_discord']


def load(d):
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        cells = [(v['name'], dd) for v in spec['variants'] for dd in spec['difficulties'] for _ in spec['agents']]
        ends = defaultdict(Counter)
        sums = defaultdict(Counter)
        n = Counter()
        for line in f:
            g = json.loads(line)
            _, dd = cells[g['cell']]
            ends[dd][g['reason']] += 1
            n[dd] += 1
            for k in KEYS:
                sums[dd][k] += g.get('counters', {}).get(k, 0)
    return ends, sums, n


for label, d in (('new', sys.argv[1]), ('old', sys.argv[2])):
    ends, sums, n = load(d)
    for dd in sorted(n):
        reasons = ', '.join(f'{r} {c}' for r, c in ends[dd].most_common())
        means = ', '.join(f'{k.split(".", 1)[1]} {sums[dd][k] / n[dd]:.2f}' for k in KEYS)
        print(f'{label} d{dd} ({n[dd]}): {reasons} | {means}')
