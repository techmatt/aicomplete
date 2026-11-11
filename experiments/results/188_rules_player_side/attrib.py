"""188: the attribution screen (s188-attrib.json): win % by difficulty, r81 against r81 with the node powers'
energy put back, pooled over the three pairs and by pair, paired on seed.

    python attrib.py DIR
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

d = sys.argv[1]
by = defaultdict(dict)
with open(f'{d}/games.jsonl', encoding='utf-8') as f:
    spec = json.loads(f.readline())['spec']
    cells = [(v['name'], dd) for v in spec['variants'] for dd in spec['difficulties'] for _ in spec['agents']]
    for line in f:
        g = json.loads(line)
        by[cells[g['cell']]][g['seed']] = 1 if g['won'] else 0


def paired(a, b):
    seeds = sorted(set(a) & set(b))
    ds = [a[s] - b[s] for s in seeds]
    n = len(ds)
    m = sum(ds) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in ds) / (n - 1)) if n > 1 else 0
    return 100 * sum(a[s] for s in seeds) / n, 100 * sum(b[s] for s in seeds) / n, 100 * m, 100 * sd / math.sqrt(n), n


diffs = sorted({k[1] for k in by})
print('| d | pair | r81 win % | powers\' energy back | gap | ±SE | games |')
print('|---|---|---|---|---|---|---|')
for diff in diffs:
    pa, pb = {}, {}
    for pair in ('bp', 'br', 'pr'):
        a, b = by.get((pair, diff), {}), by.get((pair + '-oldpow', diff), {})
        if not a:
            continue
        na, nb, gap, se, n = paired(a, b)
        print(f'| d{diff} | {pair} | {na:.1f} | {nb:.1f} | {gap:+.1f} | {se:.1f} | {n} |')
        pa.update({(pair, s): w for s, w in a.items()})
        pb.update({(pair, s): w for s, w in b.items()})
    na, nb, gap, se, n = paired(pa, pb)
    print(f'| d{diff} | **all** | {na:.1f} | {nb:.1f} | {gap:+.1f} | {se:.1f} | {n} |')
