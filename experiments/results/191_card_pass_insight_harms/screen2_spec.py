"""191: the second screen's spec (dev seeds), attributing what `flat` (screen_spec.py) leaves of the drop.

    python experiments/results/191_card_pass_insight_harms/screen2_spec.py > experiments/farm/f191-screen2.json

Levers, each on the built pass: `flat` (as in the first screen: every spread card at its pre-pass credits); `flat_no2`
(flat, and the 12 tier-1 cards moved to 2 back at 1 insight); `old` (every card the pass touched back at its pre-pass
row: credits, insight and effects, so it should read as the pre-pass cards).
"""
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
HAND = {'national_grid_buildout', 'beamed_power_constellation', 'relay_mast_rollout', 'aegis_grid_lockdown',
        'rally_the_allies', 'mass_defection', 'surgical_strike', 'double_agents', 'air_gap'}

old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', 'de6b0aa:content/cards.json'], cwd=ROOT,
                                                     capture_output=True, check=True).stdout.decode('utf-8'))}
new = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
flat, no2, back = [], [], []
for i, c in new.items():
    o = old[i]
    if c == o:
        continue
    back += [f'cards.{i}.cost={o["cost"]}', f'cards.{i}.insight={o["insight"]}']
    if o.get('effects') != c.get('effects'):
        back.append(f'cards.{i}.effects={json.dumps(o["effects"], separators=(",", ":"))}')
    if i in HAND or c['insight'] == o['insight']:
        continue
    flat.append(f'cards.{i}.cost={o["cost"]}')
    if c['tier'] == 1 and c['insight'] == 2:
        no2.append(f'cards.{i}.insight={o["insight"]}')
pairs = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = []
for lever, sets in [('', []), ('_flat', flat), ('_flatno2', flat + no2), ('_old', back)]:
    for p, s in pairs:
        variants.append({'name': p + lever, 'set': s + sets})
print(json.dumps({'name': 'f191-screen2', 'note': '191: the second screen (dev seeds): flat, flat with the tier-1 2s '
                  'back at 1, and every touched card back at its pre-pass row; strategist d1-d3.',
                  'configs': ['configs/baseline.json'], 'variants': variants, 'difficulties': [1, 2, 3],
                  'agents': ['strategist'], 'games': 32, 'records': 0}, indent=2))
