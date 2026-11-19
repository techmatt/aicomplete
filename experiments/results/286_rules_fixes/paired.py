"""286: the paired win gap, new - old, per agent pooled over d0-d4, on the same games (cell, seed, level), with its SE.

python paired.py OLD.jsonl NEW.jsonl SPEC.json
"""
import json
import math
import sys
from collections import defaultdict


def load(path, agents):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            out[(agents[g['cell'] % len(agents)], g['cell'], g['seed'], g['difficulty'])] = 1 if g['won'] else 0
    return out


def main():
    agents = json.load(open(sys.argv[3], encoding='utf-8'))['agents']
    old, new = load(sys.argv[1], agents), load(sys.argv[2], agents)
    by = defaultdict(list)
    for k in old.keys() & new.keys():
        by[k[0]].append(new[k] - old[k])
    for a, d in sorted(by.items()):
        n = len(d)
        m = sum(d) / n
        sd = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1))
        print(f'{a}: {100 * m:+.1f} +- {100 * sd / math.sqrt(n):.1f} points over {n} paired games')


if __name__ == '__main__':
    main()
