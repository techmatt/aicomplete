"""175's specs, written into out/ (run from the repo root).

    python experiments/results/175_rules_adversary/specs175.py screen [--first 36800] [--games 32]
    python experiments/results/175_rules_adversary/specs175.py fresh FIRST_SEED [--sets abc]

screen: the dev seeds (36800-36831), the three pairs, d0-d4. out/s175-base.json is r74 as ruled (the prompt's
numbers: start_nodes 1, no last-node tool) with the strategist and the strategist with the converge rule off
(the agent on/off for 175's one new decision rule); out/s175-start2.json the same with `board.start_nodes=2`
(Matt's candidate tool, message 7994: start with 2 nodes, screened against no tool). out/s175-old.json is the
strategist alone, for the r73 binary on the same seeds (its configs: out/r73/).

fresh: the final ladder, 48 seeds a set from FIRST_SEED, the three pairs, d0-d4, `strategist` on sets a-c and
`greedy` on set a, at their defaults; out/f175-{agent}-{set}.json. Run each on both binaries (r73 from out/r73/, its configs) and pair
them with scripts/bench/ladder.py; losses by cause with losses175.py.
"""
import argparse
import json
import os

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}


def spec(name, note, agents, first, games, sets=()):
    return {
        "name": name,
        "note": note,
        "configs": ["configs/baseline.json"],
        "variants": [{"name": k, "set": v + list(sets)} for k, v in PAIRS.items()],
        "difficulties": [0, 1, 2, 3, 4],
        "agents": agents,
        "games": games,
        "first_seed": first,
        "records": 0,
    }


def dump(s):
    p = os.path.join(TOP, "out", s["name"] + ".json")
    with open(p, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")
    print(p.replace("\\", "/"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("what", choices=["screen", "fresh"])
    ap.add_argument("first", type=int, nargs="?", default=36800)
    ap.add_argument("--games", type=int, default=32)
    ap.add_argument("--sets", default="abc")
    a = ap.parse_args()
    if a.what == "screen":
        seeds = f"seeds {a.first}-{a.first + a.games - 1}"
        dump(spec("s175-base", f"175 screen, r74 as ruled, {seeds}: strategist, and with the converge rule off.",
                  ["strategist", "strategist:converge_rule=0"], a.first, a.games))
        dump(spec("s175-start2", f"175 screen, r74 with start_nodes 2, {seeds}.",
                  ["strategist"], a.first, a.games, ["board.start_nodes=2"]))
        dump(spec("s175-old", f"175 screen, the r73 binary, {seeds}.", ["strategist"], a.first, a.games))
        return
    for i, half in enumerate(a.sets):
        first = a.first + 48 * "abc".index(half)
        # Greedy on set a only (144 games a level), as 174's fresh read: a flat agent's row is the spot check.
        for agent in ("strategist", "greedy") if half == "a" else ("strategist",):
            dump(spec(f"f175-{agent}-{half}", f"175 fresh ladder, {agent}, set {half}: seeds {first}-{first + 47}, "
                      "the three pairs, d0-d4.", [agent], first, 48))


if __name__ == "__main__":
    main()
