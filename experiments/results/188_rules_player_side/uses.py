"""188: per game, new against old: node-power uses, node-placing card plays, nodes placed, protections spent and
botnets taken down (the four rulings' measures).

    python uses.py NEW_DIR OLD_DIR
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

POWERS = {'builder': {1: 'Expand', 2: 'Buy In'}, 'protector': {1: 'Expand', 2: 'Accord'}, 'researcher': {1: 'Expand', 2: 'Peer Review'}}
CARDS = ['relay_mast_rollout', 'transmission_corridor', 'beamed_power_constellation', 'aegis_grid_lockdown',
         'national_grid_buildout']
COUNTERS = ['board.base_absorbed', 'board.hardening_used', 'board.ambassador_shield', 'board.aegis_used',
            'board.protection_declined', 'board.botnets_taken_down', 'board.node_placed', 'board.node_lost', 'board.destroyed', 'board.factories_destroyed']


def load(d):
    sums, n = defaultdict(float), 0
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            n += 1
            c = g.get('counters', {})
            for board, ks in POWERS.items():
                for k, name in ks.items():
                    sums[f'{board} {name}'] += c.get(f'ai.power.{board}.{k}', 0)
            for k in COUNTERS:
                sums[k] += c.get(k, 0)
            plays = g.get('cards') or {}
            for card in CARDS:
                v = plays.get(card)
                if isinstance(v, dict):
                    sums[card] += v.get('played', 0)
    return {k: v / n for k, v in sums.items()}, n


new, nn = load(sys.argv[1])
old, no = load(sys.argv[2])
print(f'| per game | new ({nn}) | old ({no}) |')
print('|---|---|---|')
for k in sorted(set(new) | set(old)):
    print(f'| {k} | {new.get(k, 0):.2f} | {old.get(k, 0):.2f} |')
