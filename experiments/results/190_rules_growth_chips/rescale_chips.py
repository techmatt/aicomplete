"""190 (r83): halve every chip price and payment in the adversary drafts (rounded up), and fix the hand-written
texts the chip overlay (chips190.overlay) leaves stale. A chip is worth 4 credits at values.json now, so each
price keeps its worth. Run once, on the r82 drafts: a second run halves again.

    python experiments/results/190_rules_growth_chips/rescale_chips.py
then python scripts/content/load_adversary.py and python scripts/regen.py.
"""
import json
import re
import sys

MARK = '190-chips'


def half(n):
    return (n + 1) // 2


def chip_text(s):
    def rep(m):
        n = half(int(m.group(1)))
        return f"{n} chip" + ('s' if n != 1 else '')
    return re.sub(r'\b(\d+) chips?\b', rep, s)


def walk(o):
    if isinstance(o, dict):
        if o.get('resource') == 'chips' and isinstance(o.get('amount'), int):
            o['amount'] = half(o['amount'])
        for k, v in list(o.items()):
            if isinstance(v, str) and k in ('text', 'note', 'value', 'check'):
                o[k] = chip_text(v)
            elif isinstance(v, list) and len(v) == 2 and v[0] == 'chips' and isinstance(v[1], int):
                v[1] = half(v[1])
            else:
                walk(v)
    elif isinstance(o, list):
        for v in o:
            if isinstance(v, list) and len(v) == 2 and v[0] == 'chips' and isinstance(v[1], int):
                v[1] = half(v[1])
            else:
                walk(v)


def save(path, d):
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(d, f, indent=1, ensure_ascii=False)
        f.write('\n')


for path in ('drafts/adversary/threats_t1.json', 'drafts/adversary/threats_t2.json', 'drafts/adversary/projects.json'):
    d = json.load(open(path, encoding='utf-8'))
    walk(d)
    save(path, d)

# Hand-written texts the overlay's numbers leave stale.
TEXTS = {
    'content/ais.json': [('"gain 2 chips"', '"gain 1 chip"'),
                         ('1 chip each round per 2 Industry tags', '1 chip each round per 4 Industry tags')],
    'content/map.json': [('each player cube pays 4 chips and 3 credits', 'each player cube pays 2 chips and 3 credits'),
                         ('1 research cube for 2 chips', '1 research cube for 1 chip')],
}
for path, pairs in TEXTS.items():
    s = open(path, encoding='utf-8').read()
    for a, b in pairs:
        s = s.replace(a, b)
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        f.write(s)
