"""280: two farm outputs of the same spec and seeds (r132 and r133) play the same games: every field but the
counters and timings equal, and the counters that differ are only the ones the cuts re-count.

    python same_play.py OLD.jsonl NEW.jsonl

Prints the games compared, those differing outside the counters, and each counter key (round suffix folded)
that differs, with how many games it differs in. Run from the repo root.
"""
import collections
import json
import re
import sys

TIMING = {'ms', 'agent_ms', 'probe_ms', 'setup_ms', 'legal_ms', 'apply_ms', 'replay_ms', 'engine'}


def games(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'seed' in g and 'cell' in g:
                out[(g['cell'], g['seed'])] = g
    return out


def main():
    a, b = games(sys.argv[1]), games(sys.argv[2])
    other = 0
    keys = collections.Counter()
    for k in sorted(set(a) & set(b)):
        x, y = a[k], b[k]
        for f in set(x) | set(y):
            if f in TIMING or f == 'counters':
                continue
            if x.get(f) != y.get(f):
                other += 1
                print('differs outside the counters:', k, f)
                break
        cx, cy = x.get('counters', {}), y.get('counters', {})
        seen = set()
        for c in set(cx) | set(cy):
            if cx.get(c, 0) != cy.get(c, 0):
                seen.add(re.sub(r'\.r\d\d$', '', c))
        keys.update(seen)
    print(f'{len(set(a) & set(b))} games matched; {other} differ outside the counters')
    for c, n in keys.most_common():
        print(f'  {c}: {n} games')


main()
