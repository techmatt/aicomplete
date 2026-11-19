"""278: the dev-seed screen (s278-screen.json): each flag's win % and rounds against base, paired by seed.

    python screen.py OUT/games.jsonl
"""
import collections
import json
import statistics
import sys


def main():
    # Windows' console default (cp1252) would write "±" as one Latin-1 byte into a redirected file.
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    path = sys.argv[1]
    with open(path, encoding='utf-8') as f:
        lines = [json.loads(x) for x in f]
    spec = None
    games = collections.defaultdict(dict)  # (variant, level) -> seed -> game
    for g in lines:
        if 'spec' in g:
            spec = g['spec']
            continue
        if 'seed' not in g:
            continue
        # Cells in grid order: configs x variants x difficulties x agents (one config, one agent here).
        per_variant = len(spec['difficulties']) * len(spec['agents'])
        name = spec['variants'][g['cell'] // per_variant % len(spec['variants'])]['name']
        games[(name, g['difficulty'])][int(g['seed'])] = g
    print('| Flag | Level | Win % (flag) | Win % (base) | Gap ± SE (points) | Rounds (flag) | Rounds (base) |')
    print('|---|---|---|---|---|---|---|')
    variants = [v['name'] for v in spec['variants']] if spec else sorted({k[0] for k in games})
    for v in variants:
        if v == 'base':
            continue
        for lv in spec['difficulties'] if spec else (2, 4):
            a, b = games.get((v, lv), {}), games.get(('base', lv), {})
            seeds = sorted(set(a) & set(b))
            if not seeds:
                continue
            d = [int(a[s]['won']) - int(b[s]['won']) for s in seeds]
            se = statistics.stdev(d) / len(d) ** 0.5 * 100 if len(d) > 1 else 0
            wa = statistics.mean(int(a[s]['won']) for s in seeds) * 100
            wb = statistics.mean(int(b[s]['won']) for s in seeds) * 100
            ra = statistics.mean(a[s]['rounds'] for s in seeds)
            rb = statistics.mean(b[s]['rounds'] for s in seeds)
            print(f'| {v} | d{lv} | {wa:.0f} | {wb:.0f} | {wa - wb:+.0f} ± {se:.0f} | {ra:.2f} | {rb:.2f} |')


main()
