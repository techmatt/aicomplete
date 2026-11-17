"""256: writes experiments/farm/s256-probe.json: what every group arm shares (the fresh group arms all lost 7-12
points to r115 at d3-d4). Arms on the dev seeds, the judge, d3-d4: `all`; `core` (every group restored: left are
the claim cards' widening to factories, Hostile Buyout's agent value and the four new cards drafted but blanked);
`core-noclaim` (and Power Purchase Agreement and Microgrid Lab Compact blanked). r115's native screen run covers
these seeds and levels.
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import arms  # noqa: E402  (writes s256-screen.json and f256-groups.json again, unchanged)

core = [s for g in arms.GROUPS.values() for s in g]
claim = arms.blank('power_purchase_agreement', 'microgrid_lab_compact')
variants = []
for pair, pset in arms.PAIRS:
    variants += [{'name': f'{pair}-all', 'set': pset}, {'name': f'{pair}-core', 'set': pset + core},
                 {'name': f'{pair}-core-noclaim', 'set': pset + core + claim}]
spec = {'name': 's256-probe', 'note': __doc__.split('\n\n')[0].replace('\n', ' '), 'configs': ['configs/baseline.json'],
        'variants': variants, 'difficulties': [3, 4], 'agents': ['strategist'], 'games': 32, 'first_seed': 36800,
        'records': 0}
with open(os.path.join(arms.ROOT, 'experiments', 'farm', 's256-probe.json'), 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(spec, indent=1) + '\n')
print(len(variants), 'variants')
