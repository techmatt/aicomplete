"""149: the ladder read, pooled over its sets and the three pairs, paired against the old revision.

python ladder149.py NEW_DIR[,NEW_DIR...] OLD_DIR[,OLD_DIR...] [--moves]

Each DIR is an experiment output (games.jsonl, results.json) of the same spec on its own
revision. Per difficulty: the new win %, its SE, the old, the paired gap (same pair and seed)
and its SE, each set's new win %, and the band. --moves adds ambassador moves a game per board.
"""
import json
import math
import sys
from collections import defaultdict

BANDS = {0: (90, 100), 1: (70, 85), 2: (55, 65), 3: (40, 50)}
BOARDS = ["Builder", "Protector", "Researcher"]


def seat_boards(variant_set):
    for s in variant_set:
        if s.startswith("ai.boards="):
            return [int(x) for x in s.split("=", 1)[1].split(",")]
    return [0, 1]


def load(d):
    res = json.load(open(f"{d}/results.json", encoding="utf-8"))
    cells = {c["cell"]: c for c in res["cells"]}
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        sets = {v["name"]: v["set"] for v in json.loads(f.readline())["spec"]["variants"]}
        for line in f:
            g = json.loads(line)
            c = cells[g["cell"]]
            yield c["difficulty"], c["variant"], g, seat_boards(sets[c["variant"]])


def main():
    new_dirs = sys.argv[1].split(",")
    old_dirs = sys.argv[2].split(",")
    new, old = defaultdict(dict), defaultdict(dict)
    per_set = defaultdict(lambda: defaultdict(list))
    moves = {"new": defaultdict(lambda: defaultdict(list)), "old": defaultdict(lambda: defaultdict(list))}
    for side, dirs, store in (("new", new_dirs, new), ("old", old_dirs, old)):
        for i, d in enumerate(dirs):
            for diff, v, g, boards in load(d):
                store[diff][(v, g["seed"])] = 1 if g["won"] else 0
                if side == "new":
                    per_set[diff][i].append(1 if g["won"] else 0)
                for s, b in enumerate(boards):
                    moves[side][diff][b].append(g["counters"].get(f"board.amb_moves.{s}", 0))
    print("| d | new win % | ±SE | old win % | gap | ±SE (paired) | sets (new) | band | verdict |")
    print("|---|---|---|---|---|---|---|---|---|")
    for diff in sorted(new):
        n = new[diff]
        p = sum(n.values()) / len(n)
        se = math.sqrt(p * (1 - p) / len(n)) * 100
        o = old.get(diff, {})
        common = [k for k in n if k in o]
        po = sum(o[k] for k in common) / len(common) if common else float("nan")
        diffs = [n[k] - o[k] for k in common]
        m = sum(diffs) / len(diffs) if diffs else float("nan")
        var = sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1) if diffs else float("nan")
        gse = math.sqrt(var / len(diffs)) * 100 if diffs else float("nan")
        sets = " / ".join(f"{100 * sum(x) / len(x):.1f}" for _, x in sorted(per_set[diff].items()))
        lo, hi = BANDS[diff]
        ok = "met" if lo <= 100 * p <= hi else "**unmet**"
        print(f"| d{diff} | **{100 * p:.1f}** | {se:.1f} | {100 * po:.1f} | {100 * m:+.1f} | {gse:.1f} | {sets} | {lo}-{hi} | {ok} |")
    if "--moves" in sys.argv:
        print()
        print("| d | " + " | ".join(f"{b} moves old → new" for b in BOARDS) + " |")
        print("|---|---|---|---|")
        for diff in sorted(new):
            row = []
            for b in range(3):
                o, nn = moves["old"][diff][b], moves["new"][diff][b]
                row.append(f"{sum(o) / len(o):.1f} → {sum(nn) / len(nn):.1f}" if o and nn else "")
            print(f"| d{diff} | " + " | ".join(row) + " |")


if __name__ == "__main__":
    main()
