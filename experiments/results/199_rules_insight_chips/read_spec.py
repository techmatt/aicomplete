"""199: the fresh ladder's specs (strategist 48 games a pair, greedy 16 a pair), the new side farmed, the old (r86 with
199's stats-only counters patched in, out/r86) native on the same seeds.

    python experiments/results/199_rules_insight_chips/read_spec.py strategist > experiments/farm/f199-strategist.json
    python experiments/results/199_rules_insight_chips/read_spec.py greedy > experiments/farm/f199-greedy.json
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
agent = sys.argv[1]
# `d3`: the d3 confirm after the first read put d3 at 53.5 (compute 18 -> 22; d0-d2 play unchanged), on the next free
# seeds 44623-44766 (strategist), 44623-44670 (greedy)
d3 = len(sys.argv) > 2 and sys.argv[2] == 'd3'
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
print(json.dumps({'name': f'f199-{agent}' + ('-d3' if d3 else ''),
                  'note': '199: the fresh ladder (r87) against r86, on the same seeds',
                  'configs': ['configs/baseline.json'],
                  'variants': [{'name': p, 'set': s} for p, s in PAIRS],
                  'difficulties': [3] if d3 else [0, 1, 2, 3], 'agents': [agent], 'games': 48, 'records': 0}, indent=2))
