"""214: insight measures per lever and board, from farmed reads.

    python experiments/results/214_tier2_pricing_insight/insight_read.py FILE=SPEC [FILE=SPEC ...] [--only bp,br,pr]

A side is `<file label>/<lever>` (the variant name past its pair, `base` for a bare pair). Per board, levels pooled:
slack and bound seat-rounds (ladder.py's 210 definitions), insight to keeps, to plays and unspent a seat-round; per
game: tier-2 keeps in rounds 1-4, tier-2 and tier-1 plays, rounds.
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
argv = sys.argv[1:]
only = None
if '--only' in argv:
    i = argv.index('--only')
    only = set(argv[i + 1].split(','))
    argv = argv[:i] + argv[i + 2:]
args = argv


def per_round(c, key, rounds):
    return [c.get(f'{key}.r{r:02d}', 0) for r in range(1, rounds + 1)]


acc = defaultdict(lambda: defaultdict(float))
game_acc = defaultdict(lambda: defaultdict(float))
order = []
for arg in args:
    path, spec_path = arg.split('=') if '=' in arg else (arg, None)
    label = os.path.basename(path).replace('.jsonl', '').replace('games', os.path.basename(os.path.dirname(path)))
    lines = open(path, encoding='utf-8')
    spec = json.load(open(spec_path, encoding='utf-8')) if spec_path else None
    for ln in lines:
        g = json.loads(ln)
        if 'format' in g:
            spec = g['spec']
            continue
        nd, na = len(spec['difficulties']), len(spec['agents'])
        v = spec['variants'][(g['cell'] // (nd * na)) % len(spec['variants'])]['name']
        pair, _, lever = v.partition('_')
        if only and v not in only:
            continue
        side = f'{label}/{lever or "base"}'
        if side not in order:
            order.append(side)
        c, R = g['counters'], g['rounds']
        ga = game_acc[side]
        ga['games'] += 1
        ga['rounds'] += R
        ga['t2_keep_r1_4'] += sum(per_round(c, 'cards.kept_t2', min(R, 4)))
        ga['t2_played'] += sum(per_round(c, 'cards.played_t2', R))
        ga['t1_played'] += sum(per_round(c, 'cards.played_t1', R))
        ga['t2_kept'] += sum(per_round(c, 'cards.kept_t2', R))
        ga['won'] += g['won']
        for s, b in enumerate(p.split(':')[0] for p in g['build'].split('|')):
            for a in (acc[(side, b)], acc[(side, 'all')]):
                sl = per_round(c, f'cards.slack.{s}', R)
                bo = per_round(c, f'cards.insight_bound.{s}', R)
                a['sr'] += R
                a['slack'] += sum(x > 0 for x in sl)
                a['bound'] += sum(x > 0 for x in bo)
                a['kept'] += sum(per_round(c, f'cards.seat_insight_kept.{s}', R))
                a['played'] += sum(per_round(c, f'cards.seat_insight_played.{s}', R))
                a['unspent'] += sum(per_round(c, f'economy.seat_insight_unspent.{s}', R))

print('| side | board | seat-rounds | slack | bound | insight to keeps | to plays | unspent | (a seat-round) |')
print('|---|---|---|---|---|---|---|---|---|')
for side in order:
    for b in ('builder', 'protector', 'researcher', 'all'):
        a = acc.get((side, b))
        if not a:
            continue
        sr = a['sr'] or 1
        print(f"| {side} | {b} | {int(a['sr'])} | {100 * a['slack'] / sr:.1f}% | {100 * a['bound'] / sr:.1f}% | "
              f"{a['kept'] / sr:.2f} | {a['played'] / sr:.2f} | {a['unspent'] / sr:.2f} | |")
print()
print('| side | games | win % | rounds | tier-2 keeps rounds 1-4 | tier-2 kept | tier-2 played | tier-1 played | (a game) |')
print('|---|---|---|---|---|---|---|---|---|')
for side in order:
    ga = game_acc[side]
    n = ga['games']
    print(f"| {side} | {int(n)} | {100 * ga['won'] / n:.1f} | {ga['rounds'] / n:.2f} | {ga['t2_keep_r1_4'] / n:.2f} | "
          f"{ga['t2_kept'] / n:.2f} | {ga['t2_played'] / n:.2f} | {ga['t1_played'] / n:.2f} | |")
