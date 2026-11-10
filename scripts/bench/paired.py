"""Win rates of an experiment's cells and paired gaps between them (064).

    python scripts/bench/paired.py DIR [DIR ...] [--pair A B]... [--against-base]

Reads each DIR's games.jsonl. A cell is named VARIANT/AGENT; with several DIRs, or to pick out
one, DIR's folder name goes first: DIR/VARIANT/AGENT. Prints each cell's win %, one SE, mean
rounds, and how its games ended. `--pair A B` prints B minus A on the seeds both played, ± one SE
of the paired difference. A and B are cell names, matched exactly: VARIANT/AGENT, or
DIR/VARIANT/AGENT when two DIRs share it (`bp/strategist` does not name
`bp/strategist:w.packet_keep=0`). Several --pair lines are also pooled, seed by seed, so two
sizes read as one 96-seed gap.

`sweep` follows the engine's current defaults, so a run from before 082 has other weights under
the same name: to compare against one, name the old defaults in the new spec, e.g.
`sweep:plan_table=0,w.table=0,w.packet_keep=1`.

`--against-base` reads a sweep: every variant `cN-x` against variant `cN` of the same DIR, agent
by agent, and each lever `x` pooled over its sizes. The spec names a base per size and a lever per
size, the same overrides under c2-x and c3-x:

    {"name": "c2", "set": ["countries=2"]},
    {"name": "c3", "set": ["countries=3"]},
    {"name": "c2-cost4", "set": ["countries=2", "cards.telemetry_archive.cost=4"]},
    {"name": "c3-cost4", "set": ["countries=3", "cards.telemetry_archive.cost=4"]}

A spec with several configs or difficulties (an on/off config pair) leads each name with the
config's file stem and `dN` where they vary: CONFIG/dN/VARIANT/AGENT (100).

A farmed read's lines name their engine (160). Native and wasm play the same games, so a seed
pairs across engines and each gap counts the shared seeds played on mixed engines (163);
`--same-engine` pairs a seed only within one engine and leaves those out, for a revision whose
engines diverge. When any game is not native, each cell's games by engine are listed.
"""
import argparse
import math
import os
import re
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_labels, engine, gap, mixed_text as _mixed_text, pair_seeds, stream  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(d):
    """{cell name: {seed: game}}, in the spec's order."""
    lines = stream(d)
    names = cell_labels(next(lines)['spec'])
    cells = {n: {} for n in names}
    for g in lines:
        cells[names[g['cell']]][g['seed']] = g
    return cells


def matches(names, text):
    """The cell names `text` names exactly: the whole name, or its end from a `/` on, so
    `bp/strategist` names `DIR/bp/strategist` but not `DIR/bp/strategist:w.packet_keep=0`."""
    return [n for n in names if n == text or n.endswith('/' + text)]


def find(cells, text):
    hits = matches(cells, text)
    if len(hits) != 1:
        sys.exit(f'{text!r} names {len(hits)} cells: {hits[:6]}')
    return cells[hits[0]]


SAME_ENGINE = False  # --same-engine: pair a seed only within one engine (160's rule)


def diffs(base, lever):
    """lever minus base, won or not, on the seeds both played, and how many of them were played
    on mixed engines (paired, or left out under --same-engine)."""
    seeds, mixed = pair_seeds(base, lever, SAME_ENGINE)
    return [lever[s]['won'] - base[s]['won'] for s in seeds], mixed


def mixed_text(mixed):
    return _mixed_text(mixed, SAME_ENGINE)


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
        d, mixed = diffs(base, games)
        levers[f'{prefix}{lever}/{agent}'].append(d)
        m, se, n = gap(d)
        print(f'{name} minus {size}: {m:+.1f} ±{se:.1f} on {n} seeds{mixed_text(mixed)}')
    if not levers:
        sys.exit('no cN-x variant with its cN')
    for lever, sizes in levers.items():
        if len(sizes) > 1:
            m, se, n = gap([x for d in sizes for x in d])
            print(f'{lever} pooled: {m:+.1f} ±{se:.1f} on {n} paired seeds')


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--pair', nargs=2, action='append', default=[], metavar=('A', 'B'),
                    help='B minus A on shared seeds; A and B are exact cell names '
                         '(VARIANT/AGENT or DIR/VARIANT/AGENT). `sweep` follows the engine defaults: '
                         'against a pre-082 run name them, e.g. sweep:plan_table=0,w.table=0,w.packet_keep=1')
    ap.add_argument('--against-base', action='store_true', help='every cN-x variant against cN')
    ap.add_argument('--same-engine', action='store_true',
                    help='pair a seed only when both sides played it on one engine (default: across engines)')
    a = ap.parse_args()
    global SAME_ENGINE
    SAME_ENGINE = a.same_engine
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

    engines = {name: Counter(engine(g) for g in games.values()) for name, games in cells.items()}
    if any(set(e) - {'native'} for e in engines.values()):
        for name, e in engines.items():
            print(f'{name}: ' + ', '.join(f'{k} {v}' for k, v in sorted(e.items())))

    pooled, pooled_mixed = [], 0
    for x, y in a.pair:
        d, mixed = diffs(find(cells, x), find(cells, y))
        pooled += d
        pooled_mixed += mixed
        m, se, n = gap(d)
        print(f'{y} minus {x}: {m:+.1f} ±{se:.1f} on {n} seeds{mixed_text(mixed)}')
    if len(a.pair) > 1:
        m, se, n = gap(pooled)
        print(f'pooled: {m:+.1f} ±{se:.1f} on {n} paired seeds{mixed_text(pooled_mixed)}')
    if a.against_base:
        against_base(cells)


if __name__ == '__main__':
    main()
