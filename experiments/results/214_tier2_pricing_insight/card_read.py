"""214: kept and played a game for chosen cards, per lever, from farmed reads.

    python experiments/results/214_tier2_pricing_insight/card_read.py FILE=SPEC [FILE=SPEC ...] [--cards a,b] [--tier 2]

With no --cards: the cards overlay.txt reprices. Also antimatter made and spent a seat-game (`gained` / `spent` index 4, read213.py's RES).
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
argv = sys.argv[1:]
cards = None
tier = None
if '--cards' in argv:
    i = argv.index('--cards'); cards = argv[i + 1].split(','); argv = argv[:i] + argv[i + 2:]
if '--tier' in argv:
    i = argv.index('--tier'); tier = int(argv[i + 1]); argv = argv[:i] + argv[i + 2:]
content = json.load(open(os.path.join(HERE, '..', '..', '..', 'content', 'cards.json'), encoding='utf-8'))
if tier is not None:
    cards = [c['id'] for c in content if c['tier'] == tier]
if cards is None:
    cards = []
    for ln in open(os.path.join(HERE, 'overlay.txt'), encoding='utf-8'):
        if ln.startswith('cards.'):
            cid = ln.split('.')[1]
            if cid not in cards:
                cards.append(cid)
st = defaultdict(lambda: defaultdict(float))
n = defaultdict(int)
order = []
for arg in argv:
    path, spec_path = arg.split('=') if '=' in arg else (arg, None)
    spec = json.load(open(spec_path, encoding='utf-8')) if spec_path else None
    label = os.path.basename(path).replace('.jsonl', '')
    if label == 'games':
        label = os.path.basename(os.path.dirname(os.path.abspath(path)))
    for ln in open(path, encoding='utf-8'):
        g = json.loads(ln)
        if 'format' in g:
            spec = g['spec']
            continue
        nd, na = len(spec['difficulties']), len(spec['agents'])
        v = spec['variants'][(g['cell'] // (nd * na)) % len(spec['variants'])]['name']
        side = f'{label}/{v.partition("_")[2] or "base"}'
        if side not in order:
            order.append(side)
        n[side] += 1
        for cid in cards:
            s = g['cards'].get(cid)
            if s:
                st[side][cid + ' kept'] += s['kept']
                st[side][cid + ' played'] += s['played']
        for seat_g, seat_s in zip(g['gained'], g['spent']):
            st[side]['am made'] += seat_g[4] / len(g['gained'])
            st[side]['am spent'] += seat_s[4] / len(g['gained'])
print('| card | ' + ' | '.join(order) + ' |')
print('|---|' + '---|' * len(order))
for cid in cards:
    print(f'| {cid} | ' + ' | '.join(f"{st[s][cid + ' kept'] / n[s]:.2f} / {st[s][cid + ' played'] / n[s]:.2f}" for s in order) + ' |')
for k in ('am made', 'am spent'):
    print(f'| {k} (a seat-game) | ' + ' | '.join(f'{st[s][k] / n[s]:.2f}' for s in order) + ' |')
