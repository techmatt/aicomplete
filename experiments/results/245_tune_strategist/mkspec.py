"""245: an experiment spec over the three board pairs: the agents given, at the levels given, on a seed range.

python mkspec.py OUT NAME --levels 2,3 --seed 47488 --games 192 AGENT...
"""
import argparse
import json

ap = argparse.ArgumentParser()
ap.add_argument("out")
ap.add_argument("name")
ap.add_argument("agents", nargs="+")
ap.add_argument("--levels", default="2,3")
ap.add_argument("--seed", type=int, required=True)
ap.add_argument("--games", type=int, required=True)
a = ap.parse_args()
spec = {
    "name": a.name,
    "note": "245 tune the strategist: candidates paired against today's strategist",
    "configs": ["configs/baseline.json"],
    "variants": [
        {"name": "bp", "set": []},
        {"name": "br", "set": ["ai.boards=0,2"]},
        {"name": "pr", "set": ["ai.boards=1,2"]},
    ],
    "difficulties": [int(x) for x in a.levels.split(",")],
    "agents": a.agents,
    "games": a.games,
    "first_seed": a.seed,
    "records": 0,
}
with open(a.out, "w", encoding="utf-8", newline="\n") as fh:
    json.dump(spec, fh, indent=1)
    fh.write("\n")
print("->", a.out)
