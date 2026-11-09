"""157: writes screen and read specs into out/ (python specs157.py screen NAME AGENTS LEVELS SEEDS [LEVER...]
or python specs157.py read TAG FIRST_SEED [AGENT] [--probe] [SET...]).

Screens run on the dev seeds (36800-), the three pairs. AGENTS is ';'-separated, LEVELS ','-separated. A LEVER is
a ';'-separated list of `--set` entries named `name=a;b` (written `name:a;b`). Reads are three 48-seed sets, d0-d3,
paired against r67's binary (out/r67, its own configs) on the same fresh seeds. --probe adds 154's end-game probe
on every loss (no replay agent).
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
PROBE = {"back": 3, "agent": "", "rollouts": 0, "all_losses": True}
# Greedy's flat weights on the engine (156's judge read: 70.8 at d2 against the strategist's 25).
FLAT = ("w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=3,w.hand_cost=0.4,w.horizon=0,"
        "w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0")


def spec(name, note, levers, diffs, agents, games, seed, probe=False):
    variants = []
    for tag, sets in levers:
        for p, s in PAIRS.items():
            variants.append({"name": p if tag == "base" else f"{p}_{tag}", "set": s + sets})
    out = {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
           "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0}
    if probe:
        out["endgame"] = PROBE
    return out


def dump(name, s):
    path = os.path.join(TOP, "out", name + ".json")
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")
    print(path)


def levers(args):
    out = [("base", [])]
    for a in args:
        tag, sets = a.split(":", 1)
        out.append((tag, sets.split(";")))
    return out


if __name__ == "__main__":
    os.makedirs(os.path.join(TOP, "out"), exist_ok=True)
    mode = sys.argv[1]
    if mode == "screen":
        name, agents, lv, seeds = sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5])
        agents = [a.replace("FLAT", FLAT) for a in agents.split(";")]
        diffs = [int(x) for x in lv.split(",")]
        dump(name, spec(name, f"157 screen {name} (dev seeds 36800-{36800 + seeds - 1}).", levers(sys.argv[6:]),
                        diffs, agents, seeds, 36800))
    else:
        tag, seed = sys.argv[2], int(sys.argv[3])
        rest = sys.argv[4:]
        probe = "--probe" in rest
        rest = [a for a in rest if a != "--probe"]
        agent = "strategist"
        if rest and not "=" in rest[0].split(",")[0].split(":")[0] and not rest[0].startswith("adversary."):
            agent = rest.pop(0).replace("FLAT", FLAT)
        for i, half in enumerate("abc"):
            s = spec(f"read-157-{tag}-{half}", f"157 read ({tag}), set {half}: seeds {seed + 48 * i}-{seed + 48 * i + 47}, the three pairs, d0-d3.",
                     [("base", rest)], [0, 1, 2, 3], [agent], 48, seed + 48 * i, probe)
            dump(f"read-157-{tag}-{half}", s)
