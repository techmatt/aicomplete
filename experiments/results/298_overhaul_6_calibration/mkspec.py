"""298: writes a screen spec, the three board pairs crossed with named lever arms.

    python experiments/results/298_overhaul_6_calibration/mkspec.py NAME OUT.json AGENT LEVELS ARM=set1;set2 ...

An arm's sets are `;`-separated `key=value` entries (params or content overlays); `base` with nothing after `=`
is the build as committed. Every variant is `<pair>-<arm>`.
"""
import json
import sys

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]


def main():
    name, out, agent, levels = sys.argv[1:5]
    arms = []
    for a in sys.argv[5:]:
        k, _, v = a.partition("=")
        arms.append((k, [s for s in v.split(";") if s]))
    spec = {
        "name": name,
        "note": "298: a lever screen (mkspec.py), the three board pairs crossed with the arms.",
        "configs": ["configs/baseline.json"],
        "variants": [{"name": f"{p}-{k}", "set": ps + sets} for k, sets in arms for p, ps in PAIRS],
        "difficulties": [int(x) for x in levels.split(",")],
        "agents": agent.split(","),
        "games": 32,
        "records": 0,
    }
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        json.dump(spec, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    main()
