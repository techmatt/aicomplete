"""Paired win and rounds gaps NEW - OLD on the same (pair, difficulty, seed), pairs bp / br / pr by variant prefix.

    python experiments/results/138_corruption_and_arrivals/paired.py NEW_DIR OLD_DIR [--new-lever L] [--old-lever L]

A variant `<pair>_<lever>` is matched by `--new-lever` / `--old-lever` (none: the bare pair). Prints, per difficulty,
games, the win gap in points and the rounds gap, each with its standard error.
"""
import json
import math
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def games(d, lever):
    out, cells = {}, None
    with open(Path(d) / 'games.jsonl', encoding='utf-8') as f:
        for i, line in enumerate(f):
            g = json.loads(line)
            if i == 0:
                sp = g['spec']
                cells = [(v['name'], df) for v in sp['variants'] for df in sp['difficulties'] for _ in sp['agents']]
                continue
            var, df = cells[g['cell']]
            pair, _, lev = var.partition('_')
            if lev == (lever or ''):
                out[(df, pair, g['seed'])] = g
    return out


def se(xs):
    m = sum(xs) / len(xs)
    return m, math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def main():
    args = sys.argv[1:]
    opt = lambda k: args[args.index(k) + 1] if k in args else None
    new, old = games(args[0], opt('--new-lever')), games(args[1], opt('--old-lever'))
    print('| Difficulty | Games | Win gap (points) | Rounds gap |\n|---|---|---|---|')
    for df in sorted({k[0] for k in new}):
        keys = [k for k in new if k[0] == df and k in old]
        w = [100.0 * (new[k]['won'] - old[k]['won']) for k in keys]
        r = [new[k]['rounds'] - old[k]['rounds'] for k in keys]
        (mw, sw), (mr, sr) = se(w), se(r)
        print(f'| d{df} | {len(keys)} | {mw:+.1f} ±{sw:.1f} | {mr:+.2f} ±{sr:.2f} |')


if __name__ == '__main__':
    main()
