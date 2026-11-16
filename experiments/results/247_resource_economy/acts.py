"""Count action types and building-Action / power uses per seat-game in a games.jsonl."""
import json, sys
from collections import Counter
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
path = sys.argv[1]
limit = int(sys.argv[2]) if len(sys.argv) > 2 else 400
types = Counter()
uses = Counter()
seats = 0
example = {}
with open(path, encoding='utf-8') as f:
    f.readline()
    for i, line in enumerate(f):
        if i >= limit:
            break
        g = json.loads(line)
        boards = [b.split(':')[0] for b in g['build'].split('|')]
        seats += len(boards)
        for a in g['actions']:
            t = a.get('type')
            types[t] += 1
            example.setdefault(t, a)
            if t in ('use', 'power', 'convert', 'standard'):
                if t == 'power':
                    key = f"power:{boards[a['seat']]}:{a['power']}"
                elif t == 'standard':
                    key = f"standard:{a.get('act')}"
                elif t == 'convert':
                    key = f"convert:{boards[a['seat']]}:{a.get('country')}"
                else:
                    key = f"use:{a['card']}"
                uses[key] += 1
print('types per seat-game:')
for t, v in types.most_common():
    print(f'  {t} {v / seats:.2f}  e.g. {json.dumps(example[t])[:200]}')
print('uses per seat-game:')
for k, v in uses.most_common(80):
    print(f'  {k} {v / seats:.3f}')
