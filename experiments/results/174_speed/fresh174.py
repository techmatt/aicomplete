"""174's fresh-seed read: the strength check of the discard cap and, if adopted, the ladder re-read.

    python experiments/results/174_speed/fresh174.py FIRST_SEED [--sets abc] [--greedy-sets a] [--tag TAG]

Writes out/f174-strat-{set}.json and out/f174-greedy-{set}.json: 48 seeds a set from FIRST_SEED (set a, then b,
c), the three pairs, d0-d4 (the read's d0-d3 and d4, which the ladder places below d3). Agents `strategist` and
`greedy` at their defaults: the base binary plays today's sim players, this item's binary its own. Run each spec on
both binaries (`run174.sh`) and pair them with `scripts/bench/ladder.py` or `paired.py`.
"""
import argparse
import json
import os

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}


def spec(name, agent, first, note):
    return {
        "name": name,
        "note": note,
        "configs": ["configs/baseline.json"],
        "variants": [{"name": k, "set": v} for k, v in PAIRS.items()],
        "difficulties": [0, 1, 2, 3, 4],
        "agents": [agent],
        "games": 48,
        "first_seed": first,
        "records": 0,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("first", type=int)
    ap.add_argument("--sets", default="abc")
    ap.add_argument("--greedy-sets", default="a")
    ap.add_argument("--tag", default="f174")
    ap.add_argument("--suffix", default="", help="added to each agent, ';'-separated arms (`:discards=1;:discards=0`)")
    ap.add_argument("--levels", default="0,1,2,3,4")
    a = ap.parse_args()
    for agent, sets in (("strategist", a.sets), ("greedy", a.greedy_sets)):
        for i, s in enumerate("abc"):
            if s not in sets:
                continue
            first = a.first + 48 * i
            name = f"{a.tag}-{'strat' if agent == 'strategist' else agent}-{s}"
            note = f"174 fresh read, set {s}: {agent}, seeds {first}-{first + 47}, the three pairs, d0-d4."
            p = os.path.join(TOP, "out", name + ".json")
            s = spec(name, agent, first, note)
            s["difficulties"] = [int(x) for x in a.levels.split(",")]
            s["note"] = note.replace("d0-d4", "d" + a.levels.replace(",", ","))
            if a.suffix:
                s["agents"] = [agent + x for x in a.suffix.split(";")]
            with open(p, "w", encoding="utf-8", newline="\n") as f:
                json.dump(s, f, indent=2)
            print(p)


if __name__ == "__main__":
    main()
