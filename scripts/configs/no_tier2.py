"""213: experiments/configs/no_tier2.json, the baseline with every tier-2 card out of its deck and the opening's
tier-2 cards dealt from tier 1 instead (the tier-2-off arm: what the majors add). The draft then offers tier 1 only (an empty tier-2 deck
is not open, `cards::deck::tier_open`). Derived: rerun after editing the baseline or the cards (regen.py does).

    python scripts/configs/no_tier2.py
"""
import json
import os

root = os.path.join(os.path.dirname(__file__), '..', '..')
cards = json.load(open(os.path.join(root, 'content', 'cards.json'), encoding='utf-8'))
TIER2 = {c['id'] for c in cards if c['tier'] == 2}
base = json.load(open(os.path.join(root, 'configs', 'baseline.json'), encoding='utf-8'))
base['name'] = 'no_tier2'
out_n = 0
for m in base['modules']:
    if 'cards' in m:
        before = len(m['cards'])
        m['cards'] = [c for c in m['cards'] if c not in TIER2]
        out_n += before - len(m['cards'])
    if m['module'] == 'cards' and 'opening_deal_tier2' in m.get('params', {}):
        # the opening keeps its size: its tier-2 cards are tier-1 ones
        m['params']['opening_deal'] += m['params']['opening_deal_tier2']
        m['params']['opening_deal_tier2'] = 0
assert out_n > 0
out = os.path.join(root, 'experiments', 'configs', 'no_tier2.json')
with open(out, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(base, indent=2) + '\n')
print('wrote', out, f'({out_n} tier-2 cards out)')
