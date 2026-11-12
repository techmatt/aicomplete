"""191: the credits screen's spec: the pass as built, and the same insight moves with the credits moved less.

    python experiments/results/191_card_pass_insight_harms/screen_spec.py > experiments/farm/f191-screen.json

Levers (overlays on the built pass, for each card the spread moved; the node and D82 cards keep their prices):
`half` moves credits 2 an insight instead of the pass's (about 4); `flat` leaves credits at the pre-pass price.
"""
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
SKIP = {'national_grid_buildout', 'beamed_power_constellation', 'relay_mast_rollout', 'aegis_grid_lockdown',
        'rally_the_allies', 'mass_defection', 'surgical_strike', 'double_agents', 'air_gap'}

old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', 'de6b0aa:content/cards.json'], cwd=ROOT,
                                                     capture_output=True, check=True).stdout.decode('utf-8'))}
new = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
half, flat = [], []
for i, c in new.items():
    o = old[i]
    if i in SKIP or c['insight'] == o['insight']:
        continue
    flat.append(f'cards.{i}.cost={o["cost"]}')
    half.append(f'cards.{i}.cost={max(2, o["cost"] + 2 * (o["insight"] - c["insight"]))}')
pairs = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = []
for lever, sets in [('', []), ('_half', half), ('_flat', flat)]:
    for p, s in pairs:
        variants.append({'name': p + lever, 'set': s + sets})
print(json.dumps({'name': 'f191-screen', 'note': '191: the credits screen (dev seeds): the pass, credits moved 2 an '
                  'insight, credits unmoved; strategist d1-d3.', 'configs': ['configs/baseline.json'],
                  'variants': variants, 'difficulties': [1, 2, 3], 'agents': ['strategist'], 'games': 32,
                  'records': 0}, indent=2))
