"""112: which card names each strategy's `says` names, and whether each is a live deck card.

Run from the repo root: python experiments/results/112_price_correction/strategy_names.py [--deck]
--deck also prints every deck card by tag (name, tier, kind, keywords) to pick replacements from.
"""
import json
import sys

with open('content/cards.json', encoding='utf-8') as f:
    cards = json.load(f)
with open('configs/baseline.json', encoding='utf-8') as f:
    base = json.load(f)
deck = set()


def walk(x):
    if isinstance(x, dict):
        for k, v in x.items():
            if isinstance(v, list) and v and isinstance(v[0], str) and k in ('deck', 'cards', 'tier1', 'tier2'):
                deck.update(v)
            walk(v)
    elif isinstance(x, list):
        for v in x:
            walk(v)


walk(base)
by_name = {c['name']: c for c in cards}
live = {c['name'] for c in cards if c['id'] in deck}
with open('configs/agents/strategies.json', encoding='utf-8') as f:
    strat = json.load(f)
for s in strat['strategies']:
    says = s['says']
    named = sorted(n for n in by_name if n in says)
    dead = [n for n in named if n not in live]
    print(s['id'], 'names', len(named), 'dead:', dead)
if '--deck' in sys.argv:
    for c in sorted((c for c in cards if c['id'] in deck), key=lambda c: (c.get('tags') or ['-'], c['tier'], c['name'])):
        print(' ', '/'.join(c.get('tags') or ['-']), 't%d' % c['tier'], c['kind'], c['name'], c.get('keywords', ''),
              c.get('tokens', {}).get('kind', ''), c.get('cost'))
