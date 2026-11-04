"""106: offered, kept and played per game for every new or changed card, from a read's cards.csv.

    python experiments/results/106_card_overhaul/play_counts.py OUT_DIR > play-counts.md

Per game = the cell's count / its games (48). Kept includes the opening deal and draws (docs/experiments.md).
Status (new / changed / kept) is read against the r41 deck (`git show 97eff1f:content/cards.json`).
"""
import csv
import json
import subprocess
import sys
from collections import defaultdict

out_dir = sys.argv[1]
GAMES = 48
cur = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', '97eff1f:content/cards.json'],
                                                      capture_output=True, encoding='utf-8', check=True).stdout)}
status = {i: ('new' if i not in old else ('changed' if cur[i] != old[i] else 'kept')) for i in cur}

AGENTS = {'engine:strategy=1': 'strategist', 'greedy': 'greedy'}
cells = [(v, a) for v in ['bp', 'br', 'pr'] for a in ['strategist', 'greedy']]
rows = defaultdict(dict)
for r in csv.DictReader(open(f'{out_dir}/cards.csv', encoding='utf-8')):
    a = AGENTS.get(r['agent'])
    if a is None:
        continue
    rows[r['card']][(r['variant'], a)] = tuple(int(r[k]) / GAMES for k in ('offered', 'kept', 'played'))

print('| card | status | tier | kind | cost | ' + ' | '.join(f'{v} {a}' for v, a in cells) + ' | strategist played, mean |')
print('|---|---|---|---|---|' + '---|' * len(cells) + '---|')
order = sorted((i for i in cur if status[i] != 'kept'), key=lambda i: (status[i] != 'new', cur[i]['tags'][0], i))
for i in order:
    c = cur[i]
    cellv = [rows[i].get(cell, (0, 0, 0)) for cell in cells]
    played = sum(cellv[k][2] for k in range(0, len(cells), 2)) / 3
    txt = ' | '.join(f'{o:.2f} / {k:.2f} / {p:.2f}' for o, k, p in cellv)
    print(f"| {c['name']} | {status[i]} | {c['tier']} | {c['kind']} | {c['cost']} | {txt} | {played:.2f} |")
