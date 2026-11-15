"""245: the share of games each agent plays a card in, and its kept per game, from games.jsonl files (header first).

python cardplay.py FILE... [--cards a,b,c] [--levels 2,3]
"""
import argparse
import collections
import json

CARDS = "dyson_swarm_tranche,continental_supergrid,power_to_the_people,tidal_barrage,smart_grid_backbone"
ap = argparse.ArgumentParser()
ap.add_argument("files", nargs="+")
ap.add_argument("--cards", default=CARDS)
ap.add_argument("--levels", default="")
a = ap.parse_args()
cards = a.cards.split(",")
levels = {int(x) for x in a.levels.split(",")} if a.levels else None
games = collections.Counter()
played = collections.defaultdict(collections.Counter)
kept = collections.defaultdict(collections.Counter)
for f in a.files:
    fh = open(f, encoding="utf-8")
    spec = json.loads(fh.readline())["spec"]
    agents = spec["agents"]
    for line in fh:
        if not line.strip():
            continue
        g = json.loads(line)
        if levels is not None and g["difficulty"] not in levels:
            continue
        ag = agents[g["cell"] % len(agents)]
        games[ag] += 1
        for c in cards:
            cc = g["cards"].get(c)
            if cc:
                played[ag][c] += 1 if cc["played"] else 0
                kept[ag][c] += cc["kept"]
print(f"{'agent':70} {'games':>6} " + " ".join(f"{c[:14]:>15}" for c in cards))
for ag in games:
    n = games[ag]
    print(f"{ag[:70]:70} {n:6} " + " ".join(f"{100 * played[ag][c] / n:6.1f}% ({kept[ag][c] / n:4.2f})" for c in cards))
