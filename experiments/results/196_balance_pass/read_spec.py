"""196: the fresh ladder's specs (strategist 48 games a pair, greedy 16), the new side farmed, the old (195's r85,
out/base) native on the same seeds. Seeds 44191-44334 (strategist), 44191-44238 (greedy).

    python experiments/results/196_balance_pass/read_spec.py strategist > experiments/farm/f196-strategist.json
    python experiments/results/196_balance_pass/read_spec.py greedy > experiments/farm/f196-greedy.json
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
agent = sys.argv[1]
# `d2`: the d2 confirm after the first read put d2 1.5 under its band (compute 19 -> 17; d0, d1, d3 play unchanged),
# on the next free seeds 44335-44478 (strategist), 44335-44382 (greedy)
DIFFS = [2] if len(sys.argv) > 2 and sys.argv[2] == 'd2' else [0, 1, 2, 3]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
name = f'f196-{agent}' + ('-d2' if DIFFS == [2] else '')
print(json.dumps({'name': name, 'note': '196: the fresh ladder of the balance pass (r86) against r85, on the same seeds',
                  'configs': ['configs/baseline.json'],
                  'variants': [{'name': p, 'set': s} for p, s in PAIRS],
                  'difficulties': DIFFS, 'agents': [agent], 'games': 48, 'records': 0}, indent=2))
