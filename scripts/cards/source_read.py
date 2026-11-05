"""Where each seat's gains come from, and how it keeps (124): per cell, board and resource, the top
sources of a seat's gains, and per board the keep counters split by runaway_read's insight flag.

    python scripts/cards/source_read.py DIR... [--top 5] [--pool] [--resource credits,insight]

Reads each DIR's `games.jsonl` (a run from 124 on: `sources`, `keeps`, `start`). The seat's board
is the head of each `build` entry. Per cell (`--pool`: per agent, the variants pooled), board and
resource, the `--top` sources by mean gain a game (a game counts a seat of that board; a source it
never used counts 0), with the mean total gained. A source is `own:<action>`, the seat's own action
(`play:<card>`, `use:<card>`, `power:<name>`, `standard:<act>`, `pass`, ...; a choice counts as the
action that opened it), `other:<action>`, a teammate's, or `phase:<name>`, the automatic phase
that gave most of a rise during an action (production, upkeep and the next round's opening show
as `phase:board.produce`, `phase:economy.upkeep`, ...) or that asked a choice.

Then per cell and board: seats, the insight peak and rise (peak less `start`), draft picks after
the opening (`offered`), keeps after the opening, the keep rate, the tier-2 share of tier choices,
and the mean insight at the keep, split into seats runaway_read flags on insight (rise over 3x a
round's insight income and over 10) and the rest. Seconds.
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from screens import cell_labels, stream  # noqa: E402
from runaway_read import PEAKS, flagged_peak, rise_of  # noqa: E402

INSIGHT = PEAKS.index('insight')


def board_of(g, s):
    seats = g.get('build', '').split('|')
    return seats[s].split(':')[0].capitalize() if s < len(seats) and seats[s] else f'seat {s}'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--top', type=int, default=5, help='sources per board and resource')
    ap.add_argument('--pool', action='store_true', help='pool the variants: one row set per agent')
    ap.add_argument('--resource', default='credits,insight,energy,chips',
                    help='resources to list (PEAKS names, comma-separated; "all" for every one)')
    a = ap.parse_args()
    wanted = PEAKS if a.resource == 'all' else a.resource.split(',')
    for r in wanted:
        if r not in PEAKS:
            sys.exit(f'unknown resource {r}: one of {", ".join(PEAKS)}')
    # (cell, board) -> seat-games; source sums per resource; keep rows by flag.
    seats = collections.Counter()
    sums = collections.defaultdict(lambda: collections.defaultdict(lambda: [0] * len(PEAKS)))
    gained = collections.defaultdict(lambda: [0] * len(PEAKS))
    keeps = collections.defaultdict(list)
    missing = 0
    for d in a.dirs:
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        lead = f'{d}: ' if len(a.dirs) > 1 and not a.pool else ''
        for g in lines:
            if 'sources' not in g or not g.get('keeps'):
                missing += 1
                continue
            label = labels[g['cell']] if g['cell'] < len(labels) else f"cell {g['cell']}"
            cell = label.split('/')[-1] if a.pool else lead + label
            rounds = g['rounds'] or 1
            for s, src in enumerate(g['sources']):
                key = (cell, board_of(g, s))
                seats[key] += 1
                for name, row in src.items():
                    acc = sums[key][name]
                    for i, v in enumerate(row):
                        acc[i] += v
                for i, v in enumerate(g['gained'][s]):
                    gained[key][i] += v
                k = g['keeps'][s]
                peak = g['peaks'][s][INSIGHT]
                inc = g['gained'][s][INSIGHT] / rounds
                flag = flagged_peak(peak, g.get('start'), s, INSIGHT, inc)
                ins = k.get('insight', [])
                keeps[key + (flag,)].append((peak, rise_of(peak, g.get('start'), s, INSIGHT), k['offered'],
                                              k['kept'], k['tiers'], k['tier2'],
                                              sum(ins) / len(ins) if ins else None, g['rounds']))
    if missing:
        print(f'{missing} games without `sources` or `keeps` (a run from before 124): skipped\n')
    if not seats:
        print('no games')
        return
    print(f'## Gains by source (mean a game, per seat of the board; top {a.top})\n')
    for key in sorted(seats):
        n = seats[key]
        print(f'### {key[0]}, {key[1]} ({n} seat-games)\n')
        print('| resource | gained | ' + ' | '.join(f'#{j + 1}' for j in range(a.top)) + ' |')
        print('|---|---|' + '---|' * a.top)
        for res in wanted:
            i = PEAKS.index(res)
            ranked = sorted(((row[i], name) for name, row in sums[key].items() if row[i] > 0), reverse=True)
            cells = [f'{name} {v / n:.1f}' for v, name in ranked[:a.top]]
            cells += [''] * (a.top - len(cells))
            print(f'| {res} | {gained[key][i] / n:.1f} | ' + ' | '.join(cells) + ' |')
        print()
    print('## Keeps by the insight flag (rise over 3x income and over 10)\n')
    print('| cell | board | seats | n | insight peak | rise | offered | kept | keep rate | tier-2 share '
          '| insight at keep | rounds |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|')
    for (cell, board, flag), v in sorted(keeps.items()):
        n = len(v)
        off = sum(x[2] for x in v)
        kept = sum(x[3] for x in v)
        tiers = sum(x[4] for x in v)
        t2 = sum(x[5] for x in v)
        at = [x[6] for x in v if x[6] is not None]
        print(f'| {cell} | {board} | {"flagged" if flag else "not flagged"} | {n} | '
              f'{sum(x[0] for x in v) / n:.1f} | {sum(x[1] for x in v) / n:.1f} | {off / n:.1f} | {kept / n:.1f} | '
              f'{kept / off if off else 0:.0%} | {t2 / tiers if tiers else 0:.0%} | '
              f'{sum(at) / len(at) if at else 0:.1f} | {sum(x[7] for x in v) / n:.2f} |')


if __name__ == '__main__':
    main()
