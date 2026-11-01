"""Win rates of an experiment's cells and paired gaps between them (064).

    python scripts/bench/paired.py DIR [DIR ...] [--pair A B]...

Reads each DIR's games.jsonl. A cell is named VARIANT/AGENT; with several DIRs, or to pick out
one, DIR's folder name goes first: DIR/VARIANT/AGENT. Prints each cell's win %, one SE, mean
rounds, and how its games ended. `--pair A B` prints B minus A on the seeds both played, ± one SE
of the paired difference; A and B are any text that names one cell. Several --pair lines are also
pooled, seed by seed, so two sizes read as one 96-seed gap.

Cells must come from specs with one config and one difficulty.
"""
import argparse
import json
import math
import os
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8')


def load(d):
    """{cell name: {seed: game}}, in the spec's order."""
    with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        games = [json.loads(line) for line in f if line.strip()]
    assert len(spec['configs']) == 1 and len(spec['difficulties']) == 1, d
    names = [f"{v['name']}/{a}" for v in spec['variants'] for a in spec['agents']]
    cells = {n: {} for n in names}
    for g in games:
        if 'cell' in g:
            cells[names[g['cell']]][g['seed']] = g
    return cells


def find(cells, text):
    hits = [n for n in cells if n == text] or [n for n in cells if text in n]
    if len(hits) != 1:
        sys.exit(f'{text!r} names {len(hits)} cells: {hits[:6]}')
    return cells[hits[0]]


def gap(pairs):
    """Mean of paired differences and its SE, in points."""
    n = len(pairs)
    if n < 2:
        return 0.0, 0.0, n
    mean = sum(pairs) / n
    var = sum((x - mean) ** 2 for x in pairs) / (n - 1)
    return 100 * mean, 100 * math.sqrt(var / n), n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--pair', nargs=2, action='append', default=[])
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
        ca, cb = find(cells, x), find(cells, y)
        diffs = [cb[s]['won'] - ca[s]['won'] for s in ca if s in cb]
        pooled += diffs
        m, se, n = gap(diffs)
        print(f'{y} minus {x}: {m:+.1f} ±{se:.1f} on {n} seeds')
    if len(a.pair) > 1:
        m, se, n = gap(pooled)
        print(f'pooled: {m:+.1f} ±{se:.1f} on {n} paired seeds')


if __name__ == '__main__':
    main()
