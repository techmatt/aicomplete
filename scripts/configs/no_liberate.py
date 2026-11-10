"""080: experiments/configs/no_liberate.json, the baseline with every card that prints `liberate` out of its
deck (the liberation-off arm of the r34 read). The list is read from content/cards.json (165: it had missed
citywide_microgrid). Derived: rerun after editing the baseline or the cards.

    python scripts/configs/no_liberate.py
"""
import json
import os

root = os.path.join(os.path.dirname(__file__), '..', '..')


def prints_liberate(node):
    """Whether any op under `node` is `liberate`."""
    if isinstance(node, dict):
        return node.get('op') == 'liberate' or any(prints_liberate(v) for v in node.values())
    if isinstance(node, list):
        return any(prints_liberate(v) for v in node)
    return False


cards = json.load(open(os.path.join(root, 'content', 'cards.json'), encoding='utf-8'))
LIBERATE = sorted(c['id'] for c in cards if prints_liberate(c))
base = json.load(open(os.path.join(root, 'configs', 'baseline.json'), encoding='utf-8'))
base['name'] = 'no_liberate'
found = 0
for m in base['modules']:
    if 'cards' in m:
        before = len(m['cards'])
        m['cards'] = [c for c in m['cards'] if c not in LIBERATE]
        found += before - len(m['cards'])
assert found == len(LIBERATE), (found, LIBERATE)
out = os.path.join(root, 'experiments', 'configs', 'no_liberate.json')
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(base, indent=2) + '\n')
print('wrote', out, f'({len(LIBERATE)} cards out: {", ".join(LIBERATE)})')
