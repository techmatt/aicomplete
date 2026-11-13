"""203: which cards die held back (kept, playable at some action phase's end, never played), a game, with kind and cost.

    python dead_cards.py RUN [--top N]
"""
import json
import os
import re
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
    CARDS = {c['id']: c for c in json.load(f)}
KEEP_AT = re.compile(r'^cards\.keep_at\.(\d+)\.(.+)$')
path = sys.argv[1] if os.path.isfile(sys.argv[1]) else os.path.join(sys.argv[1], 'games.jsonl')
top = int(sys.argv[sys.argv.index('--top') + 1]) if '--top' in sys.argv else 30
held, kind, n = Counter(), Counter(), 0
with open(path, encoding='utf-8') as f:
    for line in f:
        g = json.loads(line)
        if 'counters' not in g:
            continue
        n += 1
        c = g['counters']
        for k, v in c.items():
            x = KEEP_AT.match(k)
            if x and 0 < v < g['rounds'] and c.get(f'cards.keep_ok.{x.group(1)}.{x.group(2)}', 0) > 0:
                held[x.group(2)] += 1
                d = CARDS.get(x.group(2), {})
                kind[(d.get('kind'), 'action' if d.get('ability') else 'income' if d.get('passives') else 'other')] += 1
print(f'{n} games; held-back dead keeps a game by kind:')
for k, v in kind.most_common():
    print(f'  {k}: {v / n:.2f}')
print('| card | kind | cost | insight | held back a game |')
print('|---|---|---|---|---|')
for card, v in held.most_common(top):
    d = CARDS.get(card, {})
    print(f"| {card} | {d.get('kind')}{' (Action)' if d.get('ability') else ''} | {d.get('cost', 0)} | {d.get('insight', 0)} | {v / n:.3f} |")
