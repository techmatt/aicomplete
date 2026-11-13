"""213: append each board's antimatter power to content/ais.json (apply_overlay can't append to a list).
Idempotent: a power already there by name is replaced in place. Run from the repo root.

    python experiments/results/213_tier2_design/add_powers.py
"""
import json

POWERS = {
    'builder': {"name": "Refine", "tags": [], "cost": [["chips", 2], ["energy", 1]], "text": "gain 1 antimatter",
                "effects": [{"op": "gain", "resource": "antimatter", "amount": 1}]},
    'protector': {"name": "Joint Facility", "tags": [], "cost": [["credits", 12], ["energy", 1]], "text": "gain 1 antimatter",
                  "effects": [{"op": "gain", "resource": "antimatter", "amount": 1}]},
}
path = 'content/ais.json'
ais = json.load(open(path, encoding='utf-8'))
for b in ais:
    p = POWERS.get(b['id'])
    if not p:
        continue
    names = [q['name'] for q in b['powers']]
    if p['name'] in names:
        b['powers'][names.index(p['name'])] = p
    else:
        b['powers'].append(p)
with open(path, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(ais, indent=1, ensure_ascii=False) + '\n')
