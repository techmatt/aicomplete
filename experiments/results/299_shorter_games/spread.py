"""299: the adversary's spread a game by level (Matt's message 13811: rulings' "the adversary must spread at higher
difficulty"), for question 2's trade.

    python spread.py LABEL=FILE [LABEL=FILE ...]

A row a label and level: games, wins, cities the adversary corrupted during the game (`adversary.corruptions.city`,
setup's corrupted cities not counted), the share of games with at least one, the same in the first five rounds
(`map.cities_corrupted.rNN`), and every site it corrupted (`adversary.corruptions`: cities, stations, factories,
nodes).
"""
import collections
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")  # make_tables.sh redirects into committed Markdown


def main():
    print("| Read | Level | Games | Wins | Cities corrupted a game | Games with one | Cities by round 5 | Sites corrupted a game |")
    print("|---|---|---|---|---|---|---|---|")
    for a in sys.argv[1:]:
        label, _, path = a.partition("=")
        lv = collections.defaultdict(list)
        for line in open(path, encoding="utf-8"):
            g = json.loads(line)
            if "cell" in g:
                lv[g["difficulty"]].append(g)
        for d in sorted(lv):
            gs = lv[d]
            n = len(gs)
            c = [g["counters"].get("adversary.corruptions.city", 0) for g in gs]
            early = [sum(g["counters"].get(f"map.cities_corrupted.r{r:02d}", 0) for r in range(1, 6)) for g in gs]
            sites = [g["counters"].get("adversary.corruptions", 0) for g in gs]
            print(f"| {label} | d{d} | {n} | {100 * sum(g['won'] for g in gs) / n:.0f}% | {sum(c) / n:.2f} | "
                  f"{100 * sum(1 for x in c if x) / n:.0f}% | {sum(early) / n:.2f} | {sum(sites) / n:.2f} |")


if __name__ == "__main__":
    main()
