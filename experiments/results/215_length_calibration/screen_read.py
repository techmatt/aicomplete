"""A 215 screen: each lever against a base, per level, pooled over the three pairs and paired on seed.

    python experiments/results/215_length_calibration/screen_read.py READ [READ ...] [--base base] [--wins]

READ is a run dir, a games file or FILE=SPEC (scripts/screens.py `stream`). Variants are named `<pair>_<lever>`
(`bp` alone is the base lever). Prints per lever and level: win %, paired win delta ± SE, mean rounds, paired
rounds delta ± SE, win rounds, and the end mix. Several READs pool (their levers must not clash).
"""
import collections
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
import screens  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8')
PAIRS = ('bp', 'br', 'pr')


def split(variant):
    pair, _, lever = variant.partition('_')
    return pair, lever or 'base'


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    base = sys.argv[sys.argv.index('--base') + 1] if '--base' in sys.argv else 'base'
    if '--base' in sys.argv:
        args.remove(base)
    games = {}  # (lever, d) -> {(pair, seed): game}
    for read in args:
        it = screens.stream(read)
        cells = screens.cells_of(next(it)['spec'])
        for g in it:
            c = cells[g['cell']]
            pair, lever = split(c['variant'])
            games.setdefault((lever, g['difficulty']), {})[(pair, g['seed'])] = g
    levers = sorted({k[0] for k in games}, key=lambda x: (x != base, x))
    diffs = sorted({k[1] for k in games})
    print('lever | d | n | win % | Δwin ± SE | rounds | Δrounds ± SE | win rds | loss rds | ends')
    print('---|---|---|---|---|---|---|---|---|---')
    for lever in levers:
        for d in diffs:
            gs = games.get((lever, d))
            if not gs:
                continue
            vals = list(gs.values())
            n = len(vals)
            win = 100 * sum(g['won'] for g in vals) / n
            rds = sum(g['rounds'] for g in vals) / n
            wr = [g['rounds'] for g in vals if g['won']]
            lr = [g['rounds'] for g in vals if not g['won']]
            dw = dr = ''
            b = games.get((base, d))
            if lever != base and b:
                keys = [k for k in gs if k in b]
                if keys:
                    xs = [100 * (gs[k]['won'] - b[k]['won']) for k in keys]
                    ys = [gs[k]['rounds'] - b[k]['rounds'] for k in keys]
                    dw = fmt(xs)
                    dr = fmt(ys, 2)
            ends = collections.Counter(g['reason'] for g in vals)
            mix = ', '.join(f'{r} {100 * c / n:.0f}' for r, c in ends.most_common())
            print(f'{lever} | {d} | {n} | {win:.1f} | {dw} | {rds:.2f} | {dr} | {avg(wr):.2f} | {avg(lr):.2f} | {mix}')


def avg(xs):
    return sum(xs) / len(xs) if xs else float('nan')


def fmt(xs, nd=1):
    n = len(xs)
    m = sum(xs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1)) if n > 1 else 0.0
    return f'{m:+.{nd}f} ± {sd / math.sqrt(n):.{nd}f}'


if __name__ == '__main__':
    main()
