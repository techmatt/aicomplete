"""Win rates of an experiment's cells and paired gaps between them (064).

    python scripts/bench/paired.py DIR [DIR ...] [--pair A B]... [--against-base] [--pool] [--other DIR]...
                                   [--against-agent AGENT] [--lever-suffix LEVER]...

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

229 promoted two item readers. `--against-agent AGENT` (222's read.py) pairs every other agent against AGENT in
the same config, difficulty and pair, and `--lever-suffix LEVER` (226; 223's r6read.py) every `PAIR-LEVER` (or
`PAIR_LEVER`) cell against its `PAIR` cell; each prints a row per agent (or lever and agent) for each difficulty,
each pair and all, both sides' win % and rounds on the paired seeds and the gap. `--pool` reads the DIRs as one read
farmed in seed ranges (cells merge, no DIR in their names); `--other DIR` adds another read on the same seeds (a kept
read, another build) whose agents join the first DIR's cells as `other:AGENT`, so `--against-agent` pairs them too.
A DIR is also a games file or FILE=SPEC (a farm's bare out file), named by its file stem.

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
        if 'cell' in g:  # a FILE=SPEC whose file has a header after all: the header isn't a game
            cells[names[g['cell']]][g['seed']] = g
    return cells


def matches(names, text):
    """The cell names `text` names exactly: the whole name, or its end from a `/` on, so
    `bp/strategist` names `DIR/bp/strategist` but not `DIR/bp/strategist:w.packet_keep=0`."""
    return [n for n in names if n == text or n.endswith('/' + text)]


def dir_name(d):
    """A DIR's name in cell names: its folder, or a games file's (FILE=SPEC's) stem."""
    path = os.path.normpath(d.partition('=')[0])
    return os.path.splitext(os.path.basename(path))[0] if os.path.isfile(path) else os.path.basename(path)


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


def parts(name):
    """(lead, variant, agent, difficulty) of a cell name: [DIR/][CONFIG/][dN/]VARIANT/AGENT; difficulty `dN` or None."""
    *lead, variant, agent = name.split('/')
    d = next((p for p in lead if re.fullmatch(r'd\d+', p)), None)
    return lead, variant, agent, d


def side_by_side(rows, what):
    """229: ROWS {label: [(base games, games, dN or None, pair)]} as one table: per label, each difficulty, each
    pair and all, both sides' win % and rounds on the paired seeds and the paired gap, games minus base."""
    print(f'\n{what}: win % and rounds on the seeds both played, gap ± one paired SE')
    print('| cell | slice | n | base win | win | gap ± SE | base rounds | rounds |')
    print('|---|---|---|---|---|---|---|---|')
    for label, members in rows.items():
        diffs_ = sorted({d for _, _, d, _ in members if d}, key=lambda d: int(d[1:]))
        pairs_ = list(dict.fromkeys(p for _, _, _, p in members))
        slices = [(d, lambda m, d=d: m[2] == d) for d in diffs_] + \
                 ([(p, lambda m, p=p: m[3] == p) for p in pairs_] if len(pairs_) > 1 else []) + [('all', None)]
        for name, keep in slices:
            got, mixed = [], 0
            for m in members:
                if keep and not keep(m):
                    continue
                seeds, mx = pair_seeds(m[0], m[1], SAME_ENGINE)
                got += [(m[0][s], m[1][s]) for s in seeds]
                mixed += mx
            if not got:
                continue
            n = len(got)
            g, se, _ = gap([b['won'] - a['won'] for a, b in got])
            w0, w1 = (100 * sum(x[i]['won'] for x in got) / n for i in (0, 1))
            r0, r1 = (sum(x[i]['rounds'] for x in got) / n for i in (0, 1))
            print(f'| {label} | {name} | {n} | {w0:.1f} | {w1:.1f} | {g:+.1f} ± {se:.1f} | {r0:.2f} | {r1:.2f} |'
                  + mixed_text(mixed))


def against_agent(cells, base):
    """229 (222's read.py): every agent against BASE in the same cell otherwise (config, difficulty, pair)."""
    rows = defaultdict(list)
    for name, games in cells.items():
        lead, variant, agent, d = parts(name)
        other = '/'.join(lead + [variant, base])
        if agent != base and games and other in cells:
            rows[agent].append((cells[other], games, d, variant))
    if not rows:
        sys.exit(f'--against-agent {base}: no cell of another agent beside one of {base}')
    side_by_side(rows, f'each agent against {base}')


def against_lever(cells, levers):
    """229 (226; 223's r6read.py): every `PAIR-LEVER` (or `PAIR_LEVER`) cell against its `PAIR` cell."""
    rows = defaultdict(list)
    for name, games in cells.items():
        lead, variant, agent, d = parts(name)
        for lever in levers:
            if games and len(variant) > len(lever) + 1 and variant[-len(lever) - 1:] in (f'-{lever}', f'_{lever}'):
                pair = variant[:-len(lever) - 1]
                base = cells.get('/'.join(lead + [pair, agent]))
                if base:
                    rows[f'{lever}/{agent}'].append((base, games, d, pair))
    if not rows:
        sys.exit(f'--lever-suffix {",".join(levers)}: no PAIR-LEVER variant beside its PAIR')
    side_by_side(rows, 'each lever against its pair (lever minus base)')


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
    ap.add_argument('--pool', action='store_true',
                    help='the DIRs are one read on other seed ranges: their cells merge, no DIR in the names')
    ap.add_argument('--other', action='append', default=[], metavar='DIR',
                    help="another read on the same seeds (a kept read, another build): its agents join the first "
                         "DIR's cells as other:AGENT; repeatable")
    ap.add_argument('--against-agent', metavar='AGENT',
                    help='every other agent against AGENT, per difficulty, pair and all')
    ap.add_argument('--lever-suffix', action='append', default=[], metavar='LEVER',
                    help='every PAIR-LEVER (or PAIR_LEVER) cell against its PAIR cell, per difficulty, pair and all; '
                         'repeatable')
    a = ap.parse_args()
    global SAME_ENGINE
    SAME_ENGINE = a.same_engine
    cells = {}
    for d in a.dirs:
        prefix = '' if a.pool else dir_name(d) + '/'
        for name, games in load(d).items():
            cells.setdefault(prefix + name, {}).update(games)
    first = '' if a.pool else dir_name(a.dirs[0]) + '/'
    for d in a.other:
        for name, games in load(d).items():
            lead, variant, agent, _ = parts(name)
            cells.setdefault(first + '/'.join(lead + [variant, 'other:' + agent]), {}).update(games)

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
    if a.against_agent:
        against_agent(cells, a.against_agent)
    if a.lever_suffix:
        against_lever(cells, a.lever_suffix)


if __name__ == '__main__':
    main()
