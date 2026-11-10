"""Compares two farm outputs of the same spec and seeds, as played on two engines or builds (160).

    python scripts/farm/compare.py A B [--ms A_ERR B_ERR]

A and B: farm shard outputs (game lines, no header), or games.jsonl files (a header line is
skipped). Games are matched by (cell, seed). Prints how many games differ once the engine field and
every timing are set aside, the first few that do, and each side's win % with its SE and the
paired gap (B minus A) with its SE. --ms reads each side's per-game times from the stderr logs
the farm commands wrote (`aic-sim farm`'s `farm: cell C seed S X ms` lines, or scripts/farm/run.mjs's
`farm: seed S N games X ms` lines) and prints the mean ms a game and the ratio B / A.
"""
import argparse
import json
import math
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

TIMING = ('ms', 'agent_ms', 'probe_ms', 'setup_ms', 'legal_ms', 'apply_ms', 'replay_ms')


def games(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            g = json.loads(line)
            if 'spec' in g and 'format' in g:
                continue
            out[(g['cell'], str(g['seed']))] = g
    return out


def bare(g):
    g = dict(g)
    g.pop('engine', None)
    for k in TIMING:
        g.pop(k, None)
    return g


def mean_ms(path):
    """Mean ms a game in a farm stderr log."""
    total, n = 0.0, 0
    with open(path, encoding='utf-8') as f:
        for line in f:
            m = re.match(r'farm: cell \d+ seed \d+ ([\d.]+) ms', line)
            if m:
                total += float(m.group(1))
                n += 1
                continue
            m = re.match(r'farm: seed \d+ (\d+) games ([\d.]+) ms', line)
            if m:
                total += float(m.group(2))
                n += int(m.group(1))
    return total / n if n else float('nan'), n


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('a')
    ap.add_argument('b')
    ap.add_argument('--ms', nargs=2, metavar=('A_ERR', 'B_ERR'))
    x = ap.parse_args()
    a, b = games(x.a), games(x.b)
    keys = sorted(set(a) & set(b), key=lambda k: (int(k[1]), k[0]))
    differ = [k for k in keys if bare(a[k]) != bare(b[k])]
    engines = lambda gs: sorted({g.get('engine') or 'native' for g in gs.values()})  # noqa: E731
    print(f'{len(keys)} games on both ({len(a)} in A, {len(b)} in B); engines A {engines(a)}, B {engines(b)}')
    print(f'games that differ (engine and timings aside): {len(differ)}')
    for k in differ[:5]:
        ga, gb = bare(a[k]), bare(b[k])
        fields = [f for f in sorted(set(ga) | set(gb)) if ga.get(f) != gb.get(f)]
        print(f'  cell {k[0]} seed {k[1]}: {", ".join(fields[:8])}')
    n = len(keys)
    if n:
        for name, gs in (('A', a), ('B', b)):
            p = sum(gs[k]['won'] for k in keys) / n
            print(f'win % {name}: {100 * p:.1f} ±{100 * math.sqrt(p * (1 - p) / n):.1f} ({n} games)')
        d = [b[k]['won'] - a[k]['won'] for k in keys]
        m = sum(d) / n
        se = math.sqrt(sum((v - m) ** 2 for v in d) / max(1, n - 1) / n)
        print(f'paired gap B - A: {100 * m:+.1f} ±{100 * se:.1f}')
    if x.ms:
        (ma, na), (mb, nb) = mean_ms(x.ms[0]), mean_ms(x.ms[1])
        print(f'ms a game: A {ma:.1f} ({na} games), B {mb:.1f} ({nb} games), B / A {mb / ma:.3f}')


if __name__ == '__main__':
    main()
