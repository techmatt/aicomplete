"""212: every op key on the cards, with the resource or token kind it names, and the tags of the cards printing it.

    python experiments/results/212_tags_node_powers/opkeys.py
"""
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))


def keys(o, acc):
    if isinstance(o, dict):
        if 'op' in o:
            k = o['op']
            for f in ('resource', 'kind', 'side'):
                if isinstance(o.get(f), str):
                    k += ':' + o[f]
            acc.add(k)
        for v in o.values():
            keys(v, acc)
    elif isinstance(o, list):
        for v in o:
            keys(v, acc)
    return acc


cards = json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))
by = collections.defaultdict(collections.Counter)
for c in cards:
    for k in keys(c, set()):
        for t in c['tags']:
            by[k][t] += 1
for k, cn in sorted(by.items()):
    print(f'{k:34s}', dict(cn.most_common()))
