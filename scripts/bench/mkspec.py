"""Writes a screen spec, the three board pairs crossed with named lever arms (298's mkspec.py, promoted in 304).

    python scripts/bench/mkspec.py NAME OUT.json AGENT LEVELS ARM=set1;set2 ... [--note TEXT]

An arm's sets are `;`-separated `key=value` entries (params or content overlays); `base` with nothing after `=`
is the build as committed. Every variant is `<pair>-<arm>`. `--note` sets the spec's note (default: 298's).
"""
import json
import sys

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
NOTE = "298: a lever screen (mkspec.py), the three board pairs crossed with the arms."


def main():
    argv = sys.argv[1:]
    note = NOTE
    if "--note" in argv:
        i = argv.index("--note")
        note = argv[i + 1]
        del argv[i:i + 2]
    name, out, agent, levels = argv[:4]
    arms = []
    for a in argv[4:]:
        k, _, v = a.partition("=")
        arms.append((k, [s for s in v.split(";") if s]))
    spec = {
        "name": name,
        "note": note,
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
