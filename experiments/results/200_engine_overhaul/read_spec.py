"""200: the fresh ladder's specs (d0-d4; d4 has no band, read for the antimatter share), the new side farmed, the old
(r87, out/r87: aab153e) native on the same seeds. Strategist on 44767-44910 (144 seeds a pair), greedy on 44767-44814.

    python experiments/results/200_engine_overhaul/read_spec.py strategist > experiments/farm/f200-strategist.json
    python experiments/results/200_engine_overhaul/read_spec.py greedy > experiments/farm/f200-greedy.json
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
agent = sys.argv[1]
tag = sys.argv[2] if len(sys.argv) > 2 else ''
sets = sys.argv[3:]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
print(json.dumps({'name': f'f200-{agent}' + (f'-{tag}' if tag else ''),
                  'note': '200: the fresh ladder (r88) against r87, on the same seeds',
                  'configs': ['configs/baseline.json'],
                  'variants': [{'name': p, 'set': s + sets} for p, s in PAIRS],
                  'difficulties': [0, 1, 2, 3, 4], 'agents': [agent],
                  'games': 144 if agent == 'strategist' else 48, 'records': 0}, indent=2))
