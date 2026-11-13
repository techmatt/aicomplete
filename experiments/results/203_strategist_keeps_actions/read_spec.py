"""203: the fresh read's specs (d0-d4), one per agent: the new side farmed, the old (r89 main, 780bf6c) native on the
same seeds. Strategist and greedy on 45343-45438 (96 seeds a pair; greedy the first 48).

    python experiments/results/203_strategist_keeps_actions/read_spec.py strategist > experiments/farm/f203-strategist.json
    python experiments/results/203_strategist_keeps_actions/read_spec.py greedy > experiments/farm/f203-greedy.json
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
agent = sys.argv[1]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
print(json.dumps({'name': f'f203-{agent}',
                  'note': '203: the fresh read (kept_rule 3) against r89 main, on the same seeds',
                  'configs': ['configs/baseline.json'],
                  'variants': [{'name': p, 'set': s} for p, s in PAIRS],
                  'difficulties': [0, 1, 2, 3, 4], 'agents': [agent],
                  'games': 96 if agent == 'strategist' else 48, 'records': 0}, indent=2))
