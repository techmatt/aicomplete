"""206's tables: per lever (the variant's suffix, pairs pooled) and level, from a read's games.jsonl.

    python experiments/results/206_pricing_powers_insight/read206.py LABEL=DIR [LABEL=DIR ...] [--std] [--powers] [--insight]

Rows: win %, rounds, troops removed a game, nodes lost a game, games lost to node loss (%), and with --std each
standard action's uses a seat-game (levels pooled), --powers each power's uses a seat-game of its board, --insight
keeps and plays (insight spent) a seat-round. Paired gaps: a lever against its own read's base on the same seeds.
"""
import json
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
INSIGHT = 1  # experiment.rs PEAKS order: credits, insight, energy, ...


def load(d):
    path = d if d.endswith('.jsonl') else os.path.join(d, 'games.jsonl')
    with open(path, encoding='utf-8') as f:
        h = json.loads(f.readline())['spec']
        names = [v['name'] for v in h['variants']]
        diffs = h['difficulties']
        out = defaultdict(lambda: defaultdict(dict))  # lever -> diff -> {(pair, seed): game}
        for line in f:
            g = json.loads(line)
            v = names[g['cell'] // len(diffs)]
            pair, _, lever = v.partition('_')
            out[lever or 'base'][g['difficulty']][(pair, g['seed'])] = g
    return out


def seats(g):
    return 3 if g.get('build', '').count('|') >= 2 else 2


def row(games):
    n = len(games) or 1
    w = sum(g['won'] for g in games) / n
    se = math.sqrt(w * (1 - w) / n)
    c = lambda k: sum(g['counters'].get(k, 0) for g in games) / n
    lost_node = sum((not g['won']) and g['reason'] == 'node' for g in games) / n
    return (f"{100 * w:5.1f} ±{100 * se:3.1f} | {sum(g['rounds'] for g in games) / n:4.2f} | "
            f"{c('adversary.troops_removed'):4.2f} | {c('board.node_lost'):4.2f} | {100 * lost_node:4.1f}")


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    reads = [(a.split('=', 1)[0], load(a.split('=', 1)[1])) for a in args]
    print('| read | lever | d | win % | rounds | troops removed | nodes lost | lost to nodes % | gap vs base (paired) |')
    print('|---|---|---|---|---|---|---|---|---|')
    for label, r in reads:
        for lever in r:
            for d in sorted(r[lever]):
                cur = r[lever][d]
                gap = ''
                if lever != 'base' and 'base' in r:
                    base = r['base'].get(d, {})
                    ks = [k for k in cur if k in base]
                    diffs = [cur[k]['won'] - base[k]['won'] for k in ks]
                    if diffs:
                        m = sum(diffs) / len(diffs)
                        sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1) / len(diffs))
                        gap = f'{100 * m:+.1f} ± {100 * sd:.1f}'
                print(f'| {label} | {lever} | d{d} | {row(list(cur.values()))} | {gap} |')
    if '--std' in sys.argv:
        print('\n| read | lever | standard action | uses a seat-game |')
        print('|---|---|---|---|')
        for label, r in reads:
            for lever in r:
                gs = [g for d in r[lever] for g in r[lever][d].values()]
                ns = sum(seats(g) for g in gs) or 1
                tot = defaultdict(float)
                for g in gs:
                    for k, v in g['counters'].items():
                        if k.startswith('board.std.'):
                            tot[k[10:]] += v
                print(f"| {label} | {lever} | all | " + ', '.join(f'{k} {v / ns:.2f}' for k, v in sorted(tot.items())) + ' |')
    if '--powers' in sys.argv:
        print('\n| read | lever | power uses a seat-game of its board |')
        print('|---|---|---|')
        for label, r in reads:
            for lever in r:
                gs = [g for d in r[lever] for g in r[lever][d].values()]
                boards = defaultdict(int)
                tot = defaultdict(float)
                for g in gs:
                    for part in g.get('build', '').split('|'):
                        boards[part.split(':')[0]] += 1
                    for k, v in g['counters'].items():
                        if k.startswith('ai.power.') and k.count('.') == 3:
                            tot[k[9:]] += v
                print(f'| {label} | {lever} | ' + ', '.join(
                    f'{k} {v / max(1, boards[k.split(".")[0]]):.2f}' for k, v in sorted(tot.items())) + ' |')
    if '--insight' in sys.argv:
        print('\n| read | lever | insight spent a seat-round | insight left unspent share |')
        print('|---|---|---|---|')
        for label, r in reads:
            for lever in r:
                gs = [g for d in r[lever] for g in r[lever][d].values()]
                sr = sum(seats(g) * g['rounds'] for g in gs) or 1
                spent = sum(g['spent'][INSIGHT] if isinstance(g['spent'][INSIGHT], (int, float)) else sum(g['spent'][INSIGHT]) for g in gs)
                gained = sum(g['gained'][INSIGHT] if isinstance(g['gained'][INSIGHT], (int, float)) else sum(g['gained'][INSIGHT]) for g in gs)
                print(f'| {label} | {lever} | {spent / sr:.2f} | {1 - spent / gained if gained else 0:.2f} |')


main()
