"""201: dial screens on the spent fresh seeds (45055-45198, 144 a pair), only the levels a dial moves.

    python experiments/results/201_threat_troop_pass/dial_spec.py NAME LEVELS CELL=SET[;SET...] ... > experiments/farm/d201-NAME.json

LEVELS: comma list (0,2). Each CELL names a variant suffix; its sets apply to every pair.
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
name, levels = sys.argv[1], [int(x) for x in sys.argv[2].split(',')]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = []
for arg in sys.argv[3:]:
    cell, _, sets = arg.partition('=')
    for p, ps in PAIRS:
        variants.append({'name': f'{p}_{cell}', 'set': ps + [s for s in sets.split(';') if s]})
print(json.dumps({'name': f'd201-{name}', 'note': '201: a dial screen on the spent fresh seeds 45055-45198',
                  'configs': ['configs/baseline.json'], 'variants': variants, 'difficulties': levels,
                  'agents': ['strategist'], 'games': 144, 'records': 0}, indent=2))
