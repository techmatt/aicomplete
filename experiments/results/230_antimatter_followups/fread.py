"""230: the fresh read, base against new on the same seeds and spec (f230.json: three pairs, d0-d3, greedy and the
strategist). Win % by level and pair per side, the paired gap new - base with its SE (per seed and cell), rounds.

    python fread.py BASE.jsonl NEW.jsonl SPEC.json
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(path, spec):
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' in g:
                v, d, a = cells[g['cell']]
                out[(a, v, d, g['seed'])] = (1.0 if g.get('won') else 0.0, g['rounds'])
    return out


def gap(keys, base, new):
    diffs = [new[k][0] - base[k][0] for k in keys if k in base and k in new]
    n = len(diffs)
    if n < 2:
        return 'n/a'
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / (n - 1))
    return f'{100 * m:+.1f} ± {100 * sd / math.sqrt(n):.1f}'


def main():
    spec = json.load(open(sys.argv[3], encoding='utf-8'))
    base, new = load(sys.argv[1], spec), load(sys.argv[2], spec)
    pct = lambda side, ks: f'{100 * sum(side[k][0] for k in ks) / max(len(ks), 1):.1f}'
    rnd = lambda side, ks: f'{sum(side[k][1] for k in ks) / max(len(ks), 1):.2f}'
    for a in spec['agents']:
        ks = [k for k in base if k[0] == a]
        print(f'## {a}: {len(ks)} games a side')
        print('| slice | base | new | new - base | rounds base → new |')
        print('|---|---|---|---|---|')
        for d in spec['difficulties']:
            kd = [k for k in ks if k[2] == d]
            print(f'| d{d} | {pct(base, kd)} | {pct(new, kd)} | {gap(kd, base, new)} | {rnd(base, kd)} → {rnd(new, kd)} |')
        for v in [x['name'] for x in spec['variants']]:
            for lab, ds in ((f'{v} d1-d2', (1, 2)), (f'{v} all', tuple(spec['difficulties']))):
                kv = [k for k in ks if k[1] == v and k[2] in ds]
                print(f'| {lab} | {pct(base, kv)} | {pct(new, kv)} | {gap(kv, base, new)} | {rnd(base, kv)} → {rnd(new, kv)} |')
        print(f'| all | {pct(base, ks)} | {pct(new, ks)} | {gap(ks, base, new)} | {rnd(base, ks)} → {rnd(new, ks)} |')
        print()


if __name__ == '__main__':
    main()
