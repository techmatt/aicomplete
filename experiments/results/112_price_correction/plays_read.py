"""112: share of games each repriced card was played (strategist, the three pairs pooled), r44 against r45.

Run from the repo root after `price_read.py out/read-rNN --agent engine:strategy=1 --csv out/pr-rNN.csv` for both.
Prints the repriced cards sorted by the change, and appends nothing: copy the table into the write-up.
"""
import csv
import json
from collections import defaultdict


def load(path):
    acc = defaultdict(list)
    with open(path, encoding='utf-8') as f:
        for r in csv.DictReader(f):
            acc[r['card']].append(float(r['played']))
    return {k: sum(v) / 3 for k, v in acc.items()}


old = load('out/pr-r44.csv')
new = load('out/pr-r45.csv')
with open('out/r45_changes.json', encoding='utf-8') as f:
    changes = json.load(f)
with open('content/cards.json', encoding='utf-8') as f:
    names = {c['id']: c['name'] for c in json.load(f)}
rows = sorted(((new.get(k, 0) - old.get(k, 0), k) for k in changes))
print('| card | played % of games r44 | r45 |')
print('|---|---|---|')
for d, k in rows:
    print(f'| {names[k]} | {old.get(k, 0):.0f} | {new.get(k, 0):.0f} |')
allold = sum(old.get(k, 0) for k in changes) / len(changes)
allnew = sum(new.get(k, 0) for k in changes) / len(changes)
print(f'\nmean over the {len(changes)} repriced cards: {allold:.1f} -> {allnew:.1f}')
