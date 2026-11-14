"""226: board balance reader. Variants are named PAIR or PAIR-LEVER (bp, br-A, rb-AG ...).
For each agent and lever: win % by pair at d1-d2 and at d0-d3, each pair's gap to the pair mean, by board
(the pairs holding it), and the lever's paired gap to the base (no suffix) on the same pair, level and seed.
With --base FILE=SPEC, the base cells come from another read (same seeds, older binary).

    python read.py OUT.jsonl SPEC.json [--base OUT.jsonl=SPEC.json] [--levels 1,2]
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(path, spec_path, tag=None):
    spec = json.load(open(spec_path, encoding='utf-8'))
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    out = {}
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if 'cell' not in g:
            continue
        v, d, a = cells[g['cell']]
        pair, _, lever = v.partition('-')
        if tag is not None:
            lever = tag
        out[(lever, pair, d, a, g['seed'])] = g
    return out, spec['agents']


args = sys.argv[1:]
levels = None
base_file = None
if '--levels' in args:
    i = args.index('--levels')
    levels = [int(x) for x in args[i + 1].split(',')]
    del args[i:i + 2]
if '--base' in args:
    i = args.index('--base')
    base_file = args[i + 1]
    del args[i:i + 2]
games, agents = load(args[0], args[1])
if base_file:
    f, s = base_file.split('=')
    bg, _ = load(f, s, tag='')
    games = {k: v for k, v in games.items() if k[0] != ''}
    games.update(bg)
levers = sorted({k[0] for k in games}, key=lambda x: (len(x), x))
pairs = sorted({k[1] for k in games})
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}


def se(xs):
    if len(xs) < 2:
        return 0.0
    m = sum(xs) / len(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


for lv_name, lv in (('d1-d2', levels or [1, 2]), ('d0-d3', [0, 1, 2, 3])):
    print(f'\n## {lv_name}\n')
    print('| agent | lever | ' + ' | '.join(pairs) + ' | mean | max gap | ' + ' | '.join(BOARD.values())
          + ' | rounds | vs base ± SE |')
    print('|---|---|' + '---|' * (len(pairs) + 2 + len(BOARD)) + '---|---|')
    for a in agents:
        for lever in levers:
            win = {}
            for p in pairs:
                ks = [k for k in games if k[0] == lever and k[1] == p and k[3] == a and k[2] in lv]
                win[p] = 100 * sum(games[k]['won'] for k in ks) / len(ks) if ks else float('nan')
            mean = sum(win.values()) / len(win)
            gaps = {p: win[p] - mean for p in pairs}
            boards = []
            for b in BOARD:
                ws = [win[p] for p in pairs if b in p]
                boards.append(sum(ws) / len(ws) if ws else float('nan'))
            ks = [k for k in games if k[0] == lever and k[3] == a and k[2] in lv]
            rounds = sum(games[k]['rounds'] for k in ks) / max(1, len(ks))
            vs = ''
            if lever != '':
                d = [100 * (games[k]['won'] - games[('',) + k[1:]]['won']) for k in ks if ('',) + k[1:] in games]
                if d:
                    vs = f'{sum(d) / len(d):+.1f} ± {se(d):.1f}'
            worst = max(gaps.values(), key=abs)
            print(f'| {a} | {lever or "base"} | ' + ' | '.join(f'{win[p]:.1f} ({gaps[p]:+.1f})' for p in pairs)
                  + f' | {mean:.1f} | {worst:+.1f} | ' + ' | '.join(f'{x:.1f}' for x in boards)
                  + f' | {rounds:.2f} | {vs} |')

# uses a seat-game of the powers and actions this item touches
print('\n## Uses (pooled over levels; a seat-game of the board\'s seats)\n')
KEYS = [('ai.power.builder.mint', 'b'), ('ai.power.researcher.grant', 'r'), ('ai.power.protector.recovery', 'p'),
        ('ai.power.protector.debrief', 'p'), ('board.std.remove', None), ('board.std.liberate', None)]
print('| agent | lever | ' + ' | '.join(k for k, _ in KEYS) + ' |')
print('|---|---|' + '---|' * len(KEYS))
for a in agents:
    for lever in levers:
        row = []
        for key, board in KEYS:
            ks = [k for k in games if k[0] == lever and k[3] == a and (board is None or board in k[1])]
            tot = sum(games[k]['counters'].get(key, 0) for k in ks)
            seats = len(ks) * (2 if board is None else 1)
            row.append(f'{tot / max(1, seats):.2f}')
        print(f'| {a} | {lever or "base"} | ' + ' | '.join(row) + ' |')
