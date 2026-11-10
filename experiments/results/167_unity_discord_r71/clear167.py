"""167: what Matt's ruling against tier-III clearing near corrupted cities costs, on the final deck (H): a dev screen
spec, `PAIR` (H) against `PAIR_clear3` (the three agent-removal cards' tier III by a corrupted city instead of
within 2 of a player piece; not adopted, information for the ruling). d1 only (the level that missed).

    python experiments/results/167_unity_discord_r71/clear167.py   # writes out/clear167.json
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def tier3(cid, effects):
    return f'alignment.{cid}.tiers.2.effects={json.dumps(effects)}'


CLEAR = [
    tier3('civil_defense', [{'op': 'remove_where', 'pieces': 'agents', 'count': 6, 'per': 'country', 'where': 'by_corrupted_city'}]),
    tier3('town_hall_accord', [{'op': 'remove_where', 'pieces': 'agents', 'count': 6, 'per': 'player', 'where': 'by_corrupted_city'}]),
    tier3('reconciliation_panel', [{'op': 'remove_where', 'pieces': 'agents', 'count': 3, 'per': 'hex', 'where': 'by_corrupted_city'}]),
]

spec = {'name': 'clear167', 'note': '167: H against H with tier-III clearing by corrupted cities (information only), d1',
        'configs': ['configs/baseline.json'],
        'variants': [v for pair, sets in PAIRS for v in ({'name': pair, 'set': sets}, {'name': f'{pair}_clear3', 'set': sets + CLEAR})],
        'difficulties': [1], 'agents': ['strategist'], 'games': 8, 'records': 0}
os.makedirs(os.path.join(ROOT, 'out', 'r71h', 'experiments', 'farm'), exist_ok=True)
with open(os.path.join(ROOT, 'out', 'r71h', 'experiments', 'farm', 'clear167.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec, f, indent=2)
print('wrote out/r71h/experiments/farm/clear167.json')
