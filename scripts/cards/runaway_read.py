"""Loops and runaways in finished runs (118): the games where a counter a loop would
inflate stands far above its mean, and the games whose team insight held grows round on round.

    python scripts/cards/runaway_read.py DIR... [--times 5] [--floor 20] [--held 150] [--top 15]

Reads each DIR's `games.jsonl` (an experiment's output). For each loop-prone counter
(every trigger and threshold by card, reuses, discards from hand, discounts, insight
given, cubes converted, chips made, cards drawn) it prints the per-game maximum, the
mean over the games that used it and the game holding the maximum, when the maximum is
at least `--floor` and over `--times`
times the mean. Then the games whose insight after the keep (summed over the seats) grew in each of three
rounds running and ended over `--held` (the Researcher's Patience banking a team's insight
reads this way: D43). A loop shows as a counter in the hundreds where the mean is a
handful; a long game alone is about 2x. Seconds.
"""
import argparse
import collections
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

LOOPY = re.compile(r'^(cards\.(trigger\.|threshold\.|triggers$|thresholds$|reused$|hand_discarded$|discount_credits$|drawn$|tokens_gained$)'
                   r'|economy\.insight_given$|board\.(cubes_converted|chips_made)$)')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--times', type=float, default=5.0, help='flag a maximum over this many times the mean')
    ap.add_argument('--top', type=int, default=15)
    ap.add_argument('--floor', type=int, default=20, help='flag only a maximum at least this big')
    ap.add_argument('--held', type=int, default=150, help='flag held insight growing to over this')
    a = ap.parse_args()
    total = collections.Counter()
    used = collections.Counter()
    peak = {}
    n = 0
    growing = []
    for d in a.dirs:
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            f.readline()
            for line in f:
                g = json.loads(line)
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


if __name__ == '__main__':
    main()
