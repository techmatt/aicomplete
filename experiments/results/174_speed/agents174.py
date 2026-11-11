"""174: win % and agent ms by agent and level, base against after, from games.csv (the oracle row, same.json).

    python experiments/results/174_speed/agents174.py BASE_DIR NEW_DIR

Per agent (short name) and level: games, win % on each side, and summed agent ms a game and a decision.
"""
import csv
import os
import sys
from collections import defaultdict


def short(a):
    k, _, rest = a.partition(":")
    parts = rest.split(",") if rest else []
    if k == "engine" and "strategy=1" in parts:
        return "strategist"
    if k == "oracle":
        return "oracle"
    return a


def load(d):
    out = defaultdict(list)
    with open(os.path.join(d, "games.csv"), encoding="utf-8") as f:
        for r in csv.DictReader(f):
            out[(short(r["agent"]), r["difficulty"])].append(r)
    return out


def main():
    b, n = load(sys.argv[1]), load(sys.argv[2])
    print("| Agent | d | Games | Win % base | Win % after | ms a game base | after | ms a decision base | after | ratio |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for key in sorted(set(b) | set(n)):
        rb, rn = b.get(key, []), n.get(key, [])
        if not rb or not rn:
            continue

        def stats(rows):
            ms = sum(float(r["ms"]) for r in rows)
            dec = sum(int(r["decisions"]) for r in rows)
            win = 100 * sum(int(r["won"]) for r in rows) / len(rows)
            return win, ms / len(rows), ms / dec

        wb, gb, db = stats(rb)
        wn, gn, dn = stats(rn)
        print(f"| {key[0]} | {key[1]} | {len(rn)} | {wb:.1f} | {wn:.1f} | {gb:.0f} | {gn:.0f} | {db:.2f} | {dn:.2f} | {db / dn:.2f}x |")


if __name__ == "__main__":
    main()
