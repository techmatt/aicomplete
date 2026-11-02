"""Win rates of an experiment's cells and paired gaps between them (064).

    python scripts/bench/paired.py DIR [DIR ...] [--pair A B]... [--against-base]

Reads each DIR's games.jsonl. A cell is named VARIANT/AGENT; with several DIRs, or to pick out
one, DIR's folder name goes first: DIR/VARIANT/AGENT. Prints each cell's win %, one SE, mean
rounds, and how its games ended. `--pair A B` prints B minus A on the seeds both played, ± one SE
of the paired difference; A and B are any text that names one cell. Several --pair lines are also
pooled, seed by seed, so two sizes read as one 96-seed gap.

`--against-base` reads a sweep: every variant `cN-x` against variant `cN` of the same DIR, agent
by agent, and each lever `x` pooled over its sizes. The spec names a base per size and a lever per
size, the same overrides under c2-x and c3-x:

    {"name": "c2", "set": ["countries=2"]},
    {"name": "c3", "set": ["countries=3"]},
    {"name": "c2-cost4", "set": ["countries=2", "cards.telemetry_archive.cost=4"]},
    {"name": "c3-cost4", "set": ["countries=3", "cards.telemetry_archive.cost=4"]}

Cells must come from specs with one config and one difficulty.
"""
import argparse
import math
import os
import re
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_names, gap, stream  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8')


def load(d):
    """{cell name: {seed: game}}, in the spec's order."""
    lines = stream(d)
    names = [f'{v}/{a}' for v, a in cell_names(next(lines)['spec'])]
    cells = {n: {} for n in names}
    for g in lines:
        cells[names[g['cell']]][g['seed']] = g
    return cells


def find(cells, text):
    hits = [n for n in cells if n == text] or [n for n in cells if text in n]
    if len(hits) != 1:
        sys.exit(f'{text!r} names {len(hits)} cells: {hits[:6]}')
    return cells[hits[0]]


def diffs(base, lever):
    """lever minus base, won or not, on the seeds both played."""
    return [lever[s]['won'] - base[s]['won'] for s in base if s in lever]


def against_base(cells):
    """Each `cN-x` cell less `cN`, then each lever pooled over its sizes."""
    levers = defaultdict(list)
    for name, games in cells.items():
        hit = re.fullmatch(r'(.+/)(c\d+)-([^/]+)/([^/]+)', name)
        if not hit or not games:
            continue
        prefix, size, lever, agent = hit.groups()
        base = cells.get(f'{prefix}{size}/{agent}')
        if not base:
            print(f'{name}: no {prefix}{size}/{agent} to read against')
            continue
        d = diffs(base, games)
        levers[f'{prefix}{lever}/{agent}'].append(d)
        m, se, n = gap(d)
        print(f'{name} minus {size}: {m:+.1f} ±{se:.1f} on {n} seeds')
    if not levers:
        sys.exit('no cN-x variant with its cN')
    for lever, sizes in levers.items():
        if len(sizes) > 1:
            m, se, n = gap([x for d in sizes for x in d])
            print(f'{lever} pooled: {m:+.1f} ±{se:.1f} on {n} paired seeds')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--pair', nargs=2, action='append', default=[])
    ap.add_argument('--against-base', action='store_true', help='every cN-x variant against cN')
    a = ap.parse_args()
    cells = {}
    for d in a.dirs:
        prefix = os.path.basename(os.path.normpath(d)) + '/'
        for name, games in load(d).items():
            cells[prefix + name] = games

    print('| cell | games | win % | ±SE | rounds | ends |')
    print('|---|---|---|---|---|---|')
    for name, games in cells.items():
        gs = list(games.values())
        if not gs:
            continue
        n = len(gs)
        p = sum(g['won'] for g in gs) / n
        rounds = sum(g['rounds'] for g in gs) / n
        ends = Counter(g['reason'] for g in gs)
        text = ', '.join(f'{k} {v}' for k, v in ends.most_common())
        print(f'| {name} | {n} | {100 * p:.0f} | {100 * math.sqrt(p * (1 - p) / n):.1f} | {rounds:.2f} | {text} |')

    pooled = []
    for x, y in a.pair:
        d = diffs(find(cells, x), find(cells, y))
        pooled += d
        m, se, n = gap(d)
        print(f'{y} minus {x}: {m:+.1f} ±{se:.1f} on {n} seeds')
    if len(a.pair) > 1:
        m, se, n = gap(pooled)
        print(f'pooled: {m:+.1f} ±{se:.1f} on {n} paired seeds')
    if a.against_base:
        against_base(cells)


if __name__ == '__main__':
    main()
