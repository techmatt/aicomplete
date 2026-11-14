"""219: each resource gained / spent a seat-game, from a read's games.jsonl (214's table).

    python experiments/results/219_antimatter_sources/resources.py READ [READ...]
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
NAMES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research cubes', 'card tokens', 'ambassadors']
print('| read | ' + ' | '.join(NAMES) + ' |')
print('|---|' + '---|' * len(NAMES))
for p in sys.argv[1:]:
    f = os.path.join(p, 'games.jsonl') if os.path.isdir(p) else p
    gained = [0.0] * len(NAMES)
    spent = [0.0] * len(NAMES)
    seats = 0
    with open(f, encoding='utf-8') as fh:
        for line in fh:
            g = json.loads(line)
            if 'gained' not in g:
                continue
            for gs, ss in zip(g['gained'], g['spent']):
                seats += 1
                for i in range(len(NAMES)):
                    gained[i] += gs[i]
                    spent[i] += ss[i]
    s = seats or 1
    print(f'| {p} | ' + ' | '.join(f'{gained[i] / s:.1f} / {spent[i] / s:.1f}' for i in range(len(NAMES))) + ' |')
