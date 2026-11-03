"""080: experiments/configs/no_liberate.json, the baseline with the seven liberate capstones out of the
tier-2 deck (the liberation-off arm of the r34 read). Derived: rerun after editing the baseline.

    python scripts/configs/no_liberate.py
"""
import json
import os

LIBERATE = ['nuclear_option', 'power_to_the_people', 'hostile_buyout', 'forensic_audit', 'root_access',
            'peacekeeping_mission', 'popular_uprising']

root = os.path.join(os.path.dirname(__file__), '..', '..')
base = json.load(open(os.path.join(root, 'configs', 'baseline.json'), encoding='utf-8'))
base['name'] = 'no_liberate'
found = 0
for m in base['modules']:
    if 'cards' in m:
        before = len(m['cards'])
        m['cards'] = [c for c in m['cards'] if c not in LIBERATE]
        found += before - len(m['cards'])
assert found == len(LIBERATE), found
out = os.path.join(root, 'experiments', 'configs', 'no_liberate.json')
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(base, indent=2) + '\n')
print('wrote', out)
