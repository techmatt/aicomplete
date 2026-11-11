"""176's specs, written into out/ (run from the repo root).

    python experiments/results/176_rules_players/specs176.py screen [FIRST] [--games 32] [--agents a;b] [--tag T] [--set k=v;k=v]
    python experiments/results/176_rules_players/specs176.py fresh FIRST_SEED [--sets abc]

screen: the dev seeds (36800-36831), the three pairs, d0-d4, written as out/s176-TAG.json with AGENTS (';'-separated,
default the strategist) and any `--set` overrides on every variant. Run the r75 build on it, and the r74 binary
(out/r74/, its configs) on the same seeds for the old side.

fresh: the final ladder, 48 seeds a set from FIRST_SEED, the three pairs, d0-d4, `strategist` on sets a-c and
`greedy` on set a, at their defaults; out/f176-{agent}-{set}.json. Run each on both binaries (r74 from out/r74/,
its configs) and pair them with scripts/bench/ladder.py.
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
    ap.add_argument("--agents", default="strategist")
    ap.add_argument("--tag", default="base")
    ap.add_argument("--set", dest="overrides", default="")
    a = ap.parse_args()
    if a.what == "screen":
        seeds = f"seeds {a.first}-{a.first + a.games - 1}"
        sets = [x for x in a.overrides.split(";") if x]
        dump(spec(f"s176-{a.tag}", f"176 screen {a.tag}, {seeds}{': ' + ', '.join(sets) if sets else ''}.",
                  a.agents.split(";"), a.first, a.games, sets))
        return
    for half in a.sets:
        first = a.first + 48 * "abc".index(half)
        # Greedy on set a only (144 games a level), as 174's and 175's fresh reads.
        for agent in ("strategist", "greedy") if half == "a" else ("strategist",):
            dump(spec(f"f176-{agent}-{half}", f"176 fresh ladder, {agent}, set {half}: seeds {first}-{first + 47}, "
                      "the three pairs, d0-d4.", [agent], first, 48))


if __name__ == "__main__":
    main()
