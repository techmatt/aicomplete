"""273: the diagnostic screen's spec (experiments/farm/f273-diag.json): r128 with its rounded-up prices at the nearest
whole number instead, to name what moved play (not a lever to ship: the write-up, *Read*).

    python experiments/results/273_smaller_numbers/diag_spec.py

Variants, each paired on 271's spent seeds (49740-49787) with r127's kept read and r128's screen:
- `energy`: Buy energy at 3 (r127's 5 ÷ 1.5 = 3.33; r128 prints 4).
- `nearest`: every price at the nearest whole number of r127's ÷ 1.5 (card costs floored at 2, as printed; Buy
  energy 3, Gain an ambassador and Take down a botnet 13; threat counters stay: they are in drafts, not overlays).
"""
import json
import math
import os
import subprocess

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))


def base(path):
    return subprocess.run(['git', 'show', f'aea5535805:{path}'], cwd=ROOT, capture_output=True, check=True,
                          encoding='utf-8').stdout


sets = []
for c in json.loads(base('content/cards.json')):
    x = c['cost']
    if x <= 2:
        continue
    ceil = max(2, math.ceil(x / 1.5 - 1e-9))
    near = max(2, int(math.floor(x / 1.5 + 0.5)))
    if near != ceil:
        sets.append(f"cards.{c['id']}.cost={near}")
std = ['board.city_energy_credits=3', 'board.ambassador_credits=13', 'board.botnet_credits=13']
spec = {
    'name': 'f273-diag',
    'note': "273's diagnostic screen (diag_spec.py): r128 with Buy energy at 3, and every price at the nearest of "
            "r127's ÷ 1.5, on 271's spent seeds 49740-49787 (a screen, not a fresh read).",
    'configs': ['configs/baseline.json'],
    'variants': [
        {'name': 'bp_energy', 'set': ['board.city_energy_credits=3']},
        {'name': 'br_energy', 'set': ['ai.boards=0,2', 'board.city_energy_credits=3']},
        {'name': 'pr_energy', 'set': ['ai.boards=1,2', 'board.city_energy_credits=3']},
        {'name': 'bp_nearest', 'set': std + sets},
        {'name': 'br_nearest', 'set': ['ai.boards=0,2'] + std + sets},
        {'name': 'pr_nearest', 'set': ['ai.boards=1,2'] + std + sets},
    ],
    'difficulties': [0, 1, 2, 3, 4],
    'agents': ['strategist', 'greedy'],
    'games': 48,
    'first_seed': 49740,
    'records': 0,
}
open(os.path.join(ROOT, 'experiments/farm/f273-diag.json'), 'w', encoding='utf-8', newline='\n').write(
    json.dumps(spec) + '\n')
print(len(sets), 'card costs moved')
