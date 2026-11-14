"""231: writes the farm specs: each named dial setting x the three pairs (bp, br, pr), d0-d3, greedy and strategist.

python experiments/results/231_project_overhaul/specs.py NAME GAMES NOTE LABEL=SET[;SET...] [LABEL=...]
A LABEL with an empty SET is the baseline as committed. Writes experiments/farm/NAME.json.
"""
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]


def main():
    name, games, note = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    variants = []
    for arg in sys.argv[4:]:
        label, _, sets = arg.partition("=")
        extra = [s for s in sets.split(";") if s]
        for pair, base in PAIRS:
            v = f"{pair}_{label}" if label else pair
            variants.append({"name": v, "set": base + extra})
    spec = {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
            "difficulties": [int(d) for d in os.environ.get("DIFFS", "0,1,2,3").split(",")], "agents": ["greedy", "strategist"], "games": games, "records": 0}
    path = ROOT / "experiments" / "farm" / f"{name}.json"
    path.write_text(json.dumps(spec, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {path}: {len(variants)} variants x 4 levels x 2 agents x {games} seeds")


if __name__ == "__main__":
    main()
