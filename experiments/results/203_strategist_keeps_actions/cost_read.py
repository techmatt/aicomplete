"""203: agent ms a decision per agent (cell), from games.jsonl's `agent_ms` and `decisions`: two agents in one run
interleave, so load falls on both alike.

    python cost_read.py RUN [RUN...]
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
for p in sys.argv[1:]:
    path = p if os.path.isfile(p) else os.path.join(p, 'games.jsonl')
    cells, ms, dec = [], defaultdict(float), defaultdict(float)
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'spec' in g:
                s = g['spec']
                cells = [a for _ in s['configs'] for _ in s['variants'] for _ in s['difficulties'] for a in s['agents']]
            elif 'agent_ms' in g:
                a = cells[g['cell']]
                ms[a] += g['agent_ms']
                dec[a] += g['decisions']
    for a in ms:
        print(f'{p} | {a} | {ms[a] / max(1, dec[a]):.3f} ms a decision | {dec[a]:.0f} decisions')
