"""223: ruling 6's screen (s223-r6): each pair with `cards.tag_income_once` on against off, paired on agent,
difficulty and seed; the three per-tag buildings' plays a game.

    python r6read.py OUT.jsonl experiments/farm/s223-r6.json
"""
import json
import math
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
BUILDINGS = ['foundation_model_campus', 'vertical_monopoly', 'power_to_the_people']
spec = json.load(open(sys.argv[2], encoding='utf-8'))
cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
         for a in spec['agents']]
games = {}
for line in open(sys.argv[1], encoding='utf-8'):
    g = json.loads(line)
    if 'cell' in g:
        v, d, a = cells[g['cell']]
        games[(v, d, a, g['seed'])] = g
print('| agent | slice | n | off win | once win | gap ± SE | off rounds | once rounds |')
print('|---|---|---|---|---|---|---|---|')
for a in spec['agents']:
    for name, keep in [(f'd{d}', lambda k, d=d: k[1] == d) for d in spec['difficulties']] + \
            [(p, lambda k, p=p: k[0] == p) for p in ('bp', 'br', 'pr')] + [('all', lambda k: True)]:
        ks = [k for k in games if k[2] == a and '-once' not in k[0] and keep(k)]
        pairs = [(games[k], games[(k[0] + '-once',) + k[1:]]) for k in ks if (k[0] + '-once',) + k[1:] in games]
        if not pairs:
            continue
        diff = [100 * (b['won'] - o['won']) for o, b in pairs]
        m = sum(diff) / len(diff)
        sd = math.sqrt(sum((x - m) ** 2 for x in diff) / max(1, len(diff) - 1))
        w0 = 100 * sum(o['won'] for o, _ in pairs) / len(pairs)
        w1 = 100 * sum(b['won'] for _, b in pairs) / len(pairs)
        r0 = sum(o['rounds'] for o, _ in pairs) / len(pairs)
        r1 = sum(b['rounds'] for _, b in pairs) / len(pairs)
        print(f'| {a} | {name} | {len(pairs)} | {w0:.1f} | {w1:.1f} | {m:+.1f} ± {sd / math.sqrt(len(pairs)):.1f} | {r0:.2f} | {r1:.2f} |')
print('\n| building | off plays a game | once plays a game |')
print('|---|---|---|')
for b in BUILDINGS:
    key = f'cards.played.{b}'
    off = [g for k, g in games.items() if '-once' not in k[0]]
    on = [g for k, g in games.items() if '-once' in k[0]]
    f = lambda gs: sum(g.get('counters', {}).get(key, 0) for g in gs) / max(1, len(gs))
    print(f'| {b} | {f(off):.3f} | {f(on):.3f} |')
