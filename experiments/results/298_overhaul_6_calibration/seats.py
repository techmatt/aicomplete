"""298: two seats against three, by round: liberations, cities corrupted, ambassador moves and supply, at one level.

    python seats.py LEVEL TWO.jsonl THREE.jsonl [--arm base]

Reads farmed games (one a line); per-round counters `<key>.rNN` summed over games, over the games still running
that round, a seat. Says where a three-seat game loses its pace.
"""
import collections
import json
import sys

KEYS = [("map.agents", "agents on the map"), ("map.ambassadors", "ambassadors on the map"),
        ("map.ambassadors_held", "ambassadors in supply"), ("threats.uncountered", "threats uncountered"),
        ("threats.countered", "threats countered"), ("cards.played", "cards played"), ("map.cities_paying", "cities paying")]


def read(path, level, arm, names):
    games = []
    for line in open(path, encoding="utf-8"):
        g = json.loads(line)
        if "cell" not in g or g["difficulty"] != level:
            continue
        if names and not names[g["cell"] // 4].endswith(arm):
            continue
        games.append(g)
    return games


def by_round(games, key):
    seats = len(games[0]["margin"]["unspent"])
    tot, live = collections.Counter(), collections.Counter()
    for g in games:
        for r in range(1, g["rounds"] + 1):
            live[r] += 1
            tot[r] += g["counters"].get(f"{key}.r{r:02d}", 0)
    return {r: tot[r] / live[r] / seats for r in sorted(live) if live[r] >= 8}


def main():
    level, two, three = int(sys.argv[1]), sys.argv[2], sys.argv[3]
    arm = sys.argv[sys.argv.index("--arm") + 1] if "--arm" in sys.argv else "base"
    spec2 = json.load(open("experiments/archive/farm/s298r.json", encoding="utf-8"))
    spec3 = json.load(open("experiments/archive/farm/s298t.json", encoding="utf-8"))
    g2 = read(two, level, arm, [v["name"] for v in spec2["variants"]])
    g3 = read(three, level, arm, [v["name"] for v in spec3["variants"]])
    print(f"d{level}: {len(g2)} two-seat games, {len(g3)} three-seat games; a seat a round, games still running")
    for key, name in KEYS:
        a, b = by_round(g2, key), by_round(g3, key)
        rounds = sorted(set(a) | set(b))
        print(f"| {name} | " + " | ".join(f"r{r}: {a.get(r, 0):.2f} / {b.get(r, 0):.2f}" for r in rounds[:12]) + " |")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    main()
