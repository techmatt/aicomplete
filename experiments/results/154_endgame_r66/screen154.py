"""154: a screen or read, by lever (variant suffix after the pair) and level: win %, its SE, the round cap's share
of the losses, Liberate uses a game (all, cities, at the late price) and Unity cards drawn at the end.

python screen154.py DIR[,DIR...] [--paired BASE_LEVER]

--paired L: each lever's win % minus lever L's on the same pair and seed, with the paired SE.
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
BANDS = {0: (90, 100), 1: (70, 80), 2: (55, 65), 3: (40, 50)}


def load(dirs):
    for d in dirs.split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                pair, _, lever = c["variant"].partition("_")
                yield lever or "base", pair, c["difficulty"], g


def main():
    dirs = sys.argv[1]
    base = sys.argv[sys.argv.index("--paired") + 1] if "--paired" in sys.argv else None
    by = defaultdict(list)
    won = {}
    for lever, pair, diff, g in load(dirs):
        by[(lever, diff)].append(g)
        won[(lever, pair, diff, g["seed"])] = g["won"]
    levers = sorted({k[0] for k in by}, key=lambda x: (x != "base", x))
    print("| Lever | Level | Games | Win % | SE | Band | Cap share of losses | Liberate a game (cities; late) | Unity drawn at end |"
          + (f" vs {base} (paired) |" if base else ""))
    print("|---|---|---|---|---|---|---|---|---|" + ("---|" if base else ""))
    for lv in levers:
        for d in sorted({k[1] for k in by}):
            gs = by.get((lv, d))
            if not gs:
                continue
            n = len(gs)
            p = sum(g["won"] for g in gs) / n
            se = math.sqrt(p * (1 - p) / n) * 100
            lo, hi = BANDS[d]
            band = "in" if lo <= 100 * p <= hi else "OUT"
            losses = [g for g in gs if not g["won"]]
            cap = sum(g["reason"] == "deck_empty" for g in losses)
            lib = sum(g["counters"].get("board.std.liberate", 0) for g in gs) / n
            libc = sum(g["counters"].get("board.liberated.city", 0) for g in gs) / n
            late = sum(g["counters"].get("board.liberated_late", 0) for g in gs) / n
            unity = sum(g["margin"]["unity_tips"] for g in gs) / n
            row = (f"| {lv} | d{d} | {n} | {100 * p:.1f} | {se:.1f} | {band} | {100 * cap / max(len(losses), 1):.0f}% ({cap}) | "
                   f"{lib:.2f} ({libc:.2f}; {late:.2f}) | {unity:.1f} |")
            if base and lv != base:
                diffs = [won[k] - won[(base,) + k[1:]] for k in won if k[0] == lv and k[2] == d and (base,) + k[1:] in won]
                if diffs:
                    m = sum(diffs) / len(diffs)
                    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(len(diffs) - 1, 1) / len(diffs))
                    row += f" {100 * m:+.1f} ± {100 * sd:.1f} |"
            elif base:
                row += " |"
            print(row)


if __name__ == "__main__":
    main()
