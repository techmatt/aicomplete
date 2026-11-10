"""166: a screen's per-lever card rates and win %: for each lever (a variant `PAIR_LEVER`, no suffix: base), each
card's games drafted and played when drafted, and the lever's win %, paired against base on the same pair, level
and seed.

    python levers166.py DIR --cards a,b,c
"""
import json
import pathlib
import sys


def main():
    d = pathlib.Path(sys.argv[1])
    cards = sys.argv[sys.argv.index('--cards') + 1].split(',')
    header = None
    rows = {}
    wins = {}
    for line in open(d / 'games.jsonl', encoding='utf-8'):
        g = json.loads(line)
        if 'seed' not in g:
            header = g
            continue
        spec = header['spec']
        na, nd = len(spec['agents']), len(spec['difficulties'])
        vname = spec['variants'][g['cell'] // (na * nd)]['name']
        pair, _, lever = vname.partition('_')
        lever = lever or 'base'
        diff = spec['difficulties'][g['cell'] // na % nd]
        wins[(lever, pair, diff, g['seed'])] = g['won']
        for c in cards:
            x = g['cards'].get(c, {})
            r = rows.setdefault((lever, c), [0, 0, 0])
            r[0] += x.get('drafted', 0) > 0
            r[1] += x.get('drafted', 0) > 0 and x.get('played', 0) > 0
            r[2] += x.get('played', 0) > 0
    levers = sorted({k[0] for k in rows})
    print('| lever | card | drafted | played when drafted | share | played (any game) |')
    print('|---|---|---|---|---|---|')
    for lv in levers:
        for c in cards:
            dr, dp, pl = rows[(lv, c)]
            print(f'| {lv} | {c} | {dr} | {dp} | {100 * dp / dr if dr else 0:.1f}% | {pl} |')
    print()
    print('| lever | games | win % | paired vs base |')
    print('|---|---|---|---|')
    for lv in levers:
        keys = [k for k in wins if k[0] == lv]
        n = len(keys)
        w = sum(wins[k] for k in keys)
        both = [k for k in keys if ('base',) + k[1:] in wins]
        gap = sum(wins[k] - wins[('base',) + k[1:]] for k in both)
        print(f'| {lv} | {n} | {100 * w / max(n, 1):.1f} | {100 * gap / max(len(both), 1):+.1f} ({len(both)} paired) |')


if __name__ == '__main__':
    main()
