"""201: the fresh ladder's specs (d0-d4; d4 has no band), the new side farmed, the old (r88: out/r88, b99ed8f's
content and configs with 201's stats-only measures) native on the same seeds. Strategist on 45055-45198 (144 seeds a
pair), greedy on 45055-45102 (48).

    python experiments/results/201_threat_troop_pass/read_spec.py strategist > experiments/farm/f201-strategist.json
    python experiments/results/201_threat_troop_pass/read_spec.py greedy > experiments/farm/f201-greedy.json

Extra arguments after the agent: a tag, then overlay sets for every variant (a dial screen on the spent seeds).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
agent = sys.argv[1]
tag = sys.argv[2] if len(sys.argv) > 2 else ''
sets = sys.argv[3:]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
print(json.dumps({'name': f'f201-{agent}' + (f'-{tag}' if tag else ''),
                  'note': '201: the fresh ladder (r89) against r88, on the same seeds',
                  'configs': ['configs/baseline.json'],
                  'variants': [{'name': p, 'set': s + sets} for p, s in PAIRS],
                  'difficulties': [0, 1, 2, 3, 4], 'agents': [agent],
                  'games': 144 if agent == 'strategist' else 48, 'records': 0}, indent=2))
