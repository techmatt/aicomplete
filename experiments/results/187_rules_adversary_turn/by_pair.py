"""187: win % by difficulty and board pair, new against old on the same seeds, with each pair's paired gap.

    python by_pair.py NEW_DIR OLD_DIR

Each DIR holds a games.jsonl whose header names the cells (variant x difficulty x agent, in spec order).
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(d):
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        head = json.loads(f.readline())
        spec = head['spec']
        cells = [(v['name'], dd, a) for v in spec['variants'] for dd in spec['difficulties'] for a in spec['agents']]
        out = {}
        for line in f:
            g = json.loads(line)
            v, dd, a = cells[g['cell']]
            out[(v, dd, a, g['seed'])] = 1 if g['won'] else 0
    return out


new, old = load(sys.argv[1]), load(sys.argv[2])
rows = defaultdict(list)
for k, w in new.items():
    if k in old:
        rows[(k[1], k[0])].append((w, old[k]))
print('| d | pair | games | new win % | old win % | gap | ±SE |')
print('|---|---|---|---|---|---|---|')
for (dd, v) in sorted(rows):
    xs = rows[(dd, v)]
    n = len(xs)
    nw = sum(a for a, _ in xs) / n
    ow = sum(b for _, b in xs) / n
    diffs = [a - b for a, b in xs]
    m = sum(diffs) / n
    se = math.sqrt(sum((x - m) ** 2 for x in diffs) / (n - 1) / n) if n > 1 else 0
    print(f'| d{dd} | {v} | {n} | {100 * nw:.1f} | {100 * ow:.1f} | {100 * m:+.1f} | {100 * se:.1f} |')
