"""167: which tiers of r70's Unity deck carry the gap: a dev screen spec on the final deck (H), d1, `PAIR` (H),
`PAIR_old3` (r70's tier III restored on every Unity card r70 printed) and `PAIR_old12` (r70's tiers I and II).
Information only.

    python experiments/results/167_unity_discord_r71/tiers167.py OLD_ALIGNMENT_JSON   # writes out/r71h/.../tiers167.json
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]

with open(sys.argv[1], encoding='utf-8') as f:
    old = [c for c in json.load(f) if c['side'] == 'unity']


def restore(tiers):
    return [f"alignment.{c['id']}.tiers.{t}={json.dumps(c['tiers'][t])}" for c in old for t in tiers]


variants = []
for pair, sets in PAIRS:
    variants += [{'name': pair, 'set': sets},
                 {'name': f'{pair}_old3', 'set': sets + restore([2])},
                 {'name': f'{pair}_old12', 'set': sets + restore([0, 1])}]
spec = {'name': 'tiers167', 'note': '167: H against r70 Unity tiers restored by tier (information only), d1',
        'configs': ['configs/baseline.json'], 'variants': variants, 'difficulties': [1], 'agents': ['strategist'],
        'games': 8, 'records': 0}
with open(os.path.join(ROOT, 'out', 'r71h', 'experiments', 'farm', 'tiers167.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec, f, indent=2)
print('wrote tiers167.json')
