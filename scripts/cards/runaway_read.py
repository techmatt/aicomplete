"""Loops and runaways in finished runs (118, peaks 121): the games where a counter a loop would
inflate stands far above its mean, the games whose team insight held grows round on round, and
the games where a seat's holding of a resource peaks far above its income a round.

    python scripts/cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--top 15]
        [--peak-times 3] [--peak-floor 10]

Reads each DIR's `games.jsonl` (an experiment's output). For each loop-prone counter
(every trigger and threshold by card, reuses, discards from hand, discounts, cubes converted,
chips made, cards drawn, tokens gained) it prints the per-game maximum, the mean over the
games that used it and the game holding the maximum, when the maximum is at least `--floor`
and over `--times` times the mean. Then the games whose insight after the keep (summed over
the seats) grew in each of three rounds running and ended over `--held` (the Researcher's
Patience banking a team's insight reads this way: D43). A loop shows as a counter in the
hundreds where the mean is a handful; a long game alone is about 2x.

Then the peak check, from each game's `peaks` (a seat's highest holding after any action) and
`gained` (its rises summed over the game; over `rounds`, a round's income), per cell, seat and
resource: the mean and maximum peak, the mean income a round, the worst peak / income and its
seed, and the games flagged for a peak over `--peak-times` x the income and over
`--peak-floor`; and each seat's mean peak per resource per cell, which every read reports.
A resource a seat never gains (income 0: only what setup gave) is never flagged. Runs from before 121 have
no `peaks`: the section is skipped with a note. Seconds.
"""
import argparse
import collections
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_labels, stream  # noqa: E402

LOOPY = re.compile(r'^(cards\.(trigger\.|threshold\.|triggers$|thresholds$|reused$|hand_discarded$|discount_credits$|drawn$|tokens_gained$)'
                   r'|board\.(cubes_converted|chips_made)$)')

# The order of `peaks` and `gained`: `PEAKS` in crates/aic-sim/src/experiment.rs.
PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']


def peak_section(cells, a):
    """cells: (dir order, cell id, label) -> list of (seed, peaks, gained, rounds)."""
    print(f'\n## Peaks: a seat\'s highest holding against its income a round '
          f'(flag over {a.peak_times:g}x and over {a.peak_floor})\n')
    seats = max((len(p) for rows in cells.values() for _, p, _, _ in rows), default=0)
    for (_, _, label), rows in sorted(cells.items()):
        print(f'### {label} ({len(rows)} games)\n')
        print('| resource | ' + ' | '.join(f'seat {s} mean peak' for s in range(seats)) + ' |')
        print('|---|' + '---|' * seats)
        for i, res in enumerate(PEAKS):
            means = []
            for s in range(seats):
                vals = [p[s][i] for _, p, _, _ in rows if s < len(p)]
                means.append(f'{sum(vals) / len(vals):.1f}' if vals else '-')
            print(f'| {res} | ' + ' | '.join(means) + ' |')
        table = []
        for s in range(seats):
            for i, res in enumerate(PEAKS):
                peaks, incomes, flagged, worst = [], [], 0, (0.0, None)
                for seed, p, gained, rounds in rows:
                    if s >= len(p):
                        continue
                    pk = p[s][i]
                    inc = gained[s][i] / rounds if rounds else 0.0
                    peaks.append(pk)
                    incomes.append(inc)
                    # A holding never gained (what setup gave) is no engine: it isn't flagged.
                    ratio = pk / inc if inc > 0 else 0.0
                    if ratio > worst[0] or worst[1] is None:
                        worst = (ratio, seed)
                    if inc > 0 and pk > a.peak_floor and pk > a.peak_times * inc:
                        flagged += 1
                if peaks:
                    table.append((res, s, sum(peaks) / len(peaks), max(peaks), sum(incomes) / len(incomes),
                                  worst, flagged))
        print('\n| resource | seat | mean peak | max peak | mean income / round | worst peak / income (seed) | flagged |')
        print('|---|---|---|---|---|---|---|')
        for res, s, mean, top, inc, (ratio, seed), flagged in table:
            r = f'{ratio:.1f}'
            print(f'| {res} | {s} | {mean:.1f} | {top} | {inc:.2f} | {r} ({seed}) | {flagged} |')
        print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--times', type=float, default=5.0, help='flag a maximum over this many times the mean')
    ap.add_argument('--top', type=int, default=15)
    ap.add_argument('--floor', type=int, default=20, help='flag only a maximum at least this big')
    ap.add_argument('--held', type=int, default=150, help='flag held insight growing to over this')
    ap.add_argument('--peak-times', type=float, default=3.0, help='flag a peak over this many times a round\'s income')
    ap.add_argument('--peak-floor', type=int, default=10, help='flag only a peak over this')
    a = ap.parse_args()
    total = collections.Counter()
    used = collections.Counter()
    peak = {}
    n = 0
    growing = []
    cells = collections.defaultdict(list)
    no_peaks = []
    for di, d in enumerate(a.dirs):
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        lead = f'{d}: ' if len(a.dirs) > 1 else ''
        missing = 0
        for g in lines:
            n += 1
            where = (d, g['cell'], g['seed'], g['rounds'])
            for k, v in g['counters'].items():
                if LOOPY.match(k) and v > 0:
                    total[k] += v
                    used[k] += 1
                    if v > peak.get(k, (0, None))[0]:
                        peak[k] = (v, where)
            held = [g['counters'].get(f'cards.insight_after_keep.r{r:02d}') for r in range(1, g['rounds'] + 1)]
            held = [h for h in held if h is not None]
            runs = sum(1 for i in range(1, len(held)) if held[i] > held[i - 1])
            if len(held) >= 4 and all(held[i] > held[i - 1] for i in range(len(held) - 3, len(held))) and held[-1] > a.held:
                growing.append((held[-1], where, runs))
            if g.get('peaks') and g.get('gained'):
                label = labels[g['cell']] if g['cell'] < len(labels) else f"cell {g['cell']}"
                cells[(di, g['cell'], lead + label)].append((g['seed'], g['peaks'], g['gained'], g['rounds']))
            else:
                missing += 1
        if missing:
            no_peaks.append(f'{d} ({missing} games)')
    if not n:
        print('no games')
        return
    print(f'{n} games\n')
    rows = []
    for k, (v, where) in peak.items():
        # The mean over the games that used it: a rare card's one use is no outlier.
        mean = total[k] / used[k]
        if v >= a.floor and v > a.times * mean:
            rows.append((v / mean, k, v, mean, where))
    rows.sort(reverse=True)
    print(f'Counters at {a.floor} or more and over {a.times:g}x their mean where used: {len(rows)}\n')
    if rows:
        print('| counter | max | mean where used | max / mean | game (dir, cell, seed, rounds) |')
        print('|---|---|---|---|---|')
        for ratio, k, v, mean, where in rows[:a.top]:
            print(f'| {k} | {v} | {mean:.2f} | {ratio:.0f} | {where} |')
    growing.sort(reverse=True)
    print(f'\nGames whose insight after the keep grew three rounds running to over {a.held}: {len(growing)}\n')
    for held, where, _ in growing[:a.top]:
        print(f'- {held} insight in the last round: {where}')
    if no_peaks:
        print(f'\nNo `peaks` (a run from before 121): {", ".join(no_peaks)}; '
              f'{"the peak check reads the rest" if cells else "peak check skipped"}.')
    if cells:
        peak_section(cells, a)


if __name__ == '__main__':
    main()
