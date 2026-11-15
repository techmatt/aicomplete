"""246: write a card-value spec: the base deck and each card blanked (cost 99: never affordable, so never kept or
played; the deck order is unchanged, so every game pairs with base's on its seed), three board pairs.

python mkspec.py NAME AGENT[+AGENT...] LEVELS CARD[,CARD...] [--extra NAME=SET;SET ...] [--no-base]
  --extra adds named arms (a redesign overlay, say), each with its own blank arm unless --no-blank.
"""
import argparse
import json
import sys

ap = argparse.ArgumentParser()
ap.add_argument("name")
ap.add_argument("agent")
ap.add_argument("levels")
ap.add_argument("cards")
ap.add_argument("--extra", action="append", default=[], help="ARM=set1;set2: an arm with these overlays")
ap.add_argument("--no-base", action="store_true")
a = ap.parse_args()
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
arms = [] if a.no_base else [("base", [])]
for c in [c for c in a.cards.split(",") if c]:
    arms.append((f"no-{c}", [f"cards.{c}.cost=99"]))
for e in a.extra:
    name, sets = e.split("=", 1)
    arms.append((name, [s for s in sets.split(";") if s]))
variants = []
for arm, sets in arms:
    for p, ps in PAIRS:
        variants.append({"name": f"{arm}|{p}", "set": ps + sets})
spec = {
    "name": a.name,
    "note": "246 card usefulness: each card blanked (cost 99) against the base deck, same seeds",
    "configs": ["configs/baseline.json"],
    "variants": variants,
    "difficulties": [int(x) for x in a.levels.split(",")],
    "agents": a.agent.split("+"),
    "games": 96,
    "records": 0,
}
with open(f"experiments/farm/{a.name}.json", "w", encoding="utf-8", newline="\n") as fh:
    json.dump(spec, fh, indent=1)
    fh.write("\n")
print(f"experiments/farm/{a.name}.json: {len(variants)} variants x {len(spec['difficulties'])} levels", file=sys.stderr)
