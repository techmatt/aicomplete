"""mkspec.py NAME FIRST_SEED GAMES DIFFS AGENT [AGENT...]: writes out/243_oracle_overnight/NAME.json, three pairs."""
import json
import sys

name, first, games, diffs = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), [int(x) for x in sys.argv[4].split(",")]
agents = sys.argv[5:]
spec = {
    "name": name,
    "note": "243 oracle overnight",
    "configs": ["configs/baseline.json"],
    "variants": [
        {"name": "bp", "set": []},
        {"name": "br", "set": ["ai.boards=0,2"]},
        {"name": "pr", "set": ["ai.boards=1,2"]},
    ],
    "difficulties": diffs,
    "agents": agents,
    "games": games,
    "first_seed": first,
    "records": 0,
}
with open(f"out/243_oracle_overnight/{name}.json", "w", encoding="utf-8", newline="\n") as f:
    json.dump(spec, f, indent=1)
    f.write("\n")
