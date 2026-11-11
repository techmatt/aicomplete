"""A timing read's cost from its games.csv: ms a decision, and the slowest games (174).

    python experiments/results/174_speed/tail174.py DIR [DIR...] [--top N]

For each DIR: games, decisions, summed ms, ms a decision and a game, by pair and level, and the top N
games by `ms` (seed, pair, level, rounds, decisions, ms, ms a decision).
"""
import csv
import os
import sys
from collections import defaultdict


def read(d):
    with open(os.path.join(d, "games.csv"), encoding="utf-8") as f:
        return list(csv.DictReader(f))


def main():
    args = sys.argv[1:]
    top = 8
    if "--top" in args:
        i = args.index("--top")
        top = int(args[i + 1])
        del args[i:i + 2]
    for d in args:
        rows = read(d)
        ms = sum(float(r["ms"]) for r in rows)
        dec = sum(int(r["decisions"]) for r in rows)
        rounds = sum(int(r["rounds"]) for r in rows) / max(1, len(rows))
        print(f"{d}: {len(rows)} games, {dec} decisions, {ms / 1000:.1f} s summed, "
              f"{ms / dec:.3f} ms/decision, {ms / len(rows):.0f} ms/game, {rounds:.2f} rounds")
        cells = defaultdict(lambda: [0, 0.0, 0])
        for r in rows:
            c = cells[(r["variant"], r["difficulty"])]
            c[0] += 1
            c[1] += float(r["ms"])
            c[2] += int(r["decisions"])
        print("  " + "  ".join(f"{v}d{l} {c[1] / c[2]:.2f}" for (v, l), c in sorted(cells.items())))
        rows.sort(key=lambda r: -float(r["ms"]))
        for r in rows[:top]:
            print(f"  seed {r['seed']} {r['variant']} d{r['difficulty']} rounds {r['rounds']} "
                  f"decisions {r['decisions']} ms {float(r['ms']):.0f} ({float(r['ms']) / int(r['decisions']):.2f}/dec)")


if __name__ == "__main__":
    main()
