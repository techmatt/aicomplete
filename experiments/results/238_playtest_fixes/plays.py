"""238: plays (and Action uses) per game of the four retargeted cards, base against new, from farm outputs.

    python experiments/results/238_playtest_fixes/plays.py BASE.jsonl NEW.jsonl

Reads every counter whose key names one of the cards (`...<card id>...`) and prints its mean a game, each side.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
CARDS = ('summit_caucus', 'exodus_corridors', 'rally_the_allies', 'firewall_array')


def means(path):
    tot, n = defaultdict(float), 0
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            n += 1
            for k, v in (g.get('counters') or {}).items():
                if any(c in k for c in CARDS) and isinstance(v, (int, float)):
                    tot[k] += v
    return {k: v / n for k, v in tot.items()}, n


(a, na), (b, nb) = means(sys.argv[1]), means(sys.argv[2])
print(f'{na} base games, {nb} new')
for k in sorted(set(a) | set(b)):
    print(f'  {k:52s} {a.get(k, 0):7.3f} {b.get(k, 0):7.3f} {b.get(k, 0) - a.get(k, 0):+7.3f}')
