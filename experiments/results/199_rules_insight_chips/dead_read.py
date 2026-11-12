"""199: dead keeps by how many rounds before the game's end they were kept, and by the card's credit price.

    python dead_read.py FILE_OR_DIR [...]

A dead keep is a drafted keep never played (`cards.keep_at.<s>.<id>` above 0 at the end: plan_read.py). "Rounds
before the end" is the game's last round less the keep's round (0: kept in the last round). The card's price is its
printed credits (content/cards.json).
"""
import json
import os
import re
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
    CARDS = json.load(f)
COST = {c['id']: c.get('cost', 0) for c in CARDS}
NEEDS = {c['id'] for c in CARDS if c.get('needs')}
kept_needs = Counter()
KEEP_AT = re.compile(r'^cards\.keep_at\.(\d+)\.(.+)$')
before, never, price, n = Counter(), Counter(), Counter(), 0
for p in sys.argv[1:]:
    path = p if os.path.isfile(p) else os.path.join(p, 'games.jsonl')
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'counters' not in g:
                continue
            n += 1
            c = g['counters']
            for k, v in c.items():
                m = KEEP_AT.match(k)
                if m and v > 0:
                    b = min(g['rounds'] - v, 5)
                    before[b] += 1
                    ok = c.get(f'cards.keep_ok.{m.group(1)}.{m.group(2)}', 0) > 0
                    if not ok:
                        never[b] += 1
                    kept_needs['needs' if m.group(2) in NEEDS else 'no needs'] += 1
                    if not ok:
                        kept_needs[('needs' if m.group(2) in NEEDS else 'no needs') + ', never had'] += 1
                    cost = COST.get(m.group(2), 0)
                    price['0-9' if cost < 10 else '10-19' if cost < 20 else '20-29' if cost < 30 else '30+'] += 1
print(f'{n} games; dead keeps a game by rounds kept before the end (5: 5 or more), never-had in brackets\n')
print('| rounds before the end | ' + ' | '.join(str(b) for b in range(6)) + ' |')
print('|---|' + '---|' * 6)
print('| dead keeps a game | ' + ' | '.join(f'{before[b] / n:.2f} ({never[b] / n:.2f})' for b in range(6)) + ' |')
print('\n| printed credits | ' + ' | '.join(price) + ' |')
print('|---|' + '---|' * len(price))
print('| dead keeps a game | ' + ' | '.join(f'{v / n:.2f}' for v in price.values()) + ' |')
print('\n| by tag requirement | ' + ' | '.join(sorted(kept_needs)) + ' |')
print('|---|' + '---|' * len(kept_needs))
print('| dead keeps a game | ' + ' | '.join(f'{kept_needs[k] / n:.2f}' for k in sorted(kept_needs)) + ' |')
print(f'\n(deck: {len(NEEDS)} of {len(CARDS)} cards print `needs`)')
