"""149: win % pooled over the three board pairs, per lever and difficulty, with each
lever's paired gap from base on the same seeds.

python pool149.py DIR [--base NAME]

Variant names are `<pair>` (base) or `<pair>_<lever>`; agents are kept apart.
"""
import json
import math
import sys
from collections import defaultdict


def main():
    d = sys.argv[1]
    res = json.load(open(f"{d}/results.json", encoding="utf-8"))
    cells = {c["cell"]: c for c in res["cells"]}
    games = defaultdict(dict)  # (lever, diff, agent) -> {(pair, seed): won}
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        f.readline()
        for line in f:
            g = json.loads(line)
            c = cells[g["cell"]]
            pair, _, lever = c["variant"].partition("_")
            games[(lever or "base", c["difficulty"], c["agent"])][(pair, g["seed"])] = 1 if g["won"] else 0
    print("| lever | d | agent | games | win % | ±SE | gap vs base | ±SE |")
    print("|---|---|---|---|---|---|---|---|")
    for (lever, diff, agent), w in sorted(games.items()):
        n = len(w)
        p = sum(w.values()) / n
        se = math.sqrt(p * (1 - p) / n) * 100
        base = games.get(("base", diff, agent))
        gap = ""
        if base and lever != "base":
            common = [k for k in w if k in base]
            diffs = [w[k] - base[k] for k in common]
            m = sum(diffs) / len(diffs)
            var = sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)
            gap = f"{100 * m:+.1f} | {100 * math.sqrt(var / len(diffs)):.1f}"
        else:
            gap = " | "
        print(f"| {lever} | {diff} | {agent} | {n} | {100 * p:.1f} | {se:.1f} | {gap} |")


if __name__ == "__main__":
    main()
