"""155: the cost curve (python curve155.py DIR [DIR...]): win rate and wall time a game by agent and level.

DIR holds games.csv files (out/s155-a-bp, ...; a glob of stages works: out/s155-*). Rows are paired on
(difficulty, seed): `+/-` is games the row won that the strategist lost and the reverse. Time is the
agent's own ms a game (`ms`), one thread; `max` the slowest game.
"""
import csv
import glob
import math
import os
import sys
from collections import defaultdict


def rows(dirs):
    out = []
    for d in dirs:
        for path in glob.glob(os.path.join(d, "games.csv")):
            with open(path, encoding="utf-8") as f:
                out.extend(csv.DictReader(f))
    return out


def main(dirs):
    games = rows(dirs)
    by = defaultdict(dict)
    for g in games:
        by[(g["agent"], int(g["difficulty"]))][(g["variant"], int(g["seed"]))] = g
    agents = sorted({a for a, _ in by}, key=lambda a: (a != "engine:strategy=1", a.split(":")[0], len(a), a))
    print("| agent | level | games | win % (SE) | +/- strategist | s a game (mean) | max s | reasons lost |")
    print("|---|---|---|---|---|---|---|---|")
    for d in sorted({d for _, d in by}):
        base = by.get(("engine:strategy=1", d), {})
        for a in agents:
            cell = by.get((a, d))
            if not cell:
                continue
            n = len(cell)
            w = sum(g["won"] == "1" for g in cell.values())
            p = w / n
            se = math.sqrt(p * (1 - p) / n) * 100
            plus = sum(g["won"] == "1" and base.get(k, {}).get("won") == "0" for k, g in cell.items())
            minus = sum(g["won"] == "0" and base.get(k, {}).get("won") == "1" for k, g in cell.items())
            ms = [float(g["ms"]) for g in cell.values()]
            lost = defaultdict(int)
            for g in cell.values():
                if g["won"] != "1":
                    lost[g["reason"]] += 1
            why = ", ".join(f"{r} {c}" for r, c in sorted(lost.items(), key=lambda x: -x[1]))
            print(f"| `{a}` | d{d} | {n} | {100 * p:.1f} ({se:.1f}) | +{plus} / -{minus} | {sum(ms) / n / 1000:.1f} | "
                  f"{max(ms) / 1000:.1f} | {why} |")


if __name__ == "__main__":
    main(sys.argv[1:])
