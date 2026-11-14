"""223: plays a game of named cards (and fx of Unity cards), new beside base.

    python plays.py NEW.jsonl BASE.jsonl ID [ID...]
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def mean(path, key):
    n = t = 0
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            n += 1
            t += sum(v for k, v in g.get('counters', {}).items() if k == key or k.startswith(key + '.'))
    return t / max(1, n)


new, base = sys.argv[1], sys.argv[2]
print('| card | key | base a game | new a game |')
print('|---|---|---|---|')
for cid in sys.argv[3:]:
    for key in (f'cards.played.{cid}', f'alignment.resolved.{cid}', f'fx.{cid}.land'):
        b, n = mean(base, key), mean(new, key)
        if b or n:
            print(f'| {cid} | {key} | {b:.3f} | {n:.3f} |')
