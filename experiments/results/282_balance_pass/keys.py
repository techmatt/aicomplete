"""List counter keys matching a pattern, with their mean per game by difficulty (judge cells only by default).

python keys.py FILE PATTERN [--agent judge|greedy|all]
"""
import json
import re
import sys
from collections import defaultdict


def main():
    path, pat = sys.argv[1], re.compile(sys.argv[2])
    agent = sys.argv[sys.argv.index('--agent') + 1] if '--agent' in sys.argv else 'all'
    sums = defaultdict(lambda: defaultdict(float))
    n = defaultdict(int)
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            # grid: agent fastest; agents [judge, greedy] in the read specs
            if '--cells' in sys.argv:
                lo, hi = map(int, sys.argv[sys.argv.index('--cells') + 1].split('-'))
                if not lo <= g['cell'] <= hi:
                    continue
            else:
                a = 'judge' if g['cell'] % 2 == 0 else 'greedy'
                if agent != 'all' and a != agent:
                    continue
            d = g['difficulty']
            n[d] += 1
            for k, v in g['counters'].items():
                if pat.search(k) and isinstance(v, (int, float)):
                    sums[k][d] += v
    keys = sorted(sums)
    ds = sorted(n)
    print('key', *[f'd{d}' for d in ds], sep='\t')
    for k in keys:
        print(k, *[f'{sums[k][d] / n[d]:.2f}' for d in ds], sep='\t')


if __name__ == '__main__':
    main()
