"""158: writes screen and read specs into out/ (from the repo root).

    python specs158.py offs                      configs out/c158/<id>_off.json: baseline less one new card each, and
                                                 all_off (none of the ten)
    python specs158.py screen NAME AGENTS LEVELS SEEDS [--offs] [--first N] [LEVER...]
    python specs158.py read TAG FIRST_SEED [AGENT] [--probe] [SET...]

Screens run on the dev seeds (36800-), the three pairs. AGENTS is ';'-separated, LEVELS ','-separated. A LEVER is
`tag:a=1;b=2` (its `--set` entries, ';'-separated). --offs adds every card-off config beside the baseline (a card on
and off on the same seeds). Reads are three 48-seed sets, d0-d3, paired against r68's binary (out/r68, its own
configs) on the same fresh seeds; --probe adds 154's end-game probe on every loss.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
PROBE = {"back": 3, "agent": "", "rollouts": 0, "all_losses": True}
NEW = ["mass_defection", "embassy_of_exiles", "double_agents", "kettle_operation", "lure_network", "firewall_array",
       "air_gap", "rally_the_allies", "surgical_strike", "open_source_uprising"]


def offs():
    base = json.load(open(os.path.join(TOP, "configs", "baseline.json"), encoding="utf-8"))
    os.makedirs(os.path.join(TOP, "out", "c158"), exist_ok=True)
    paths = []
    for cid in NEW + ["all"]:
        c = json.loads(json.dumps(base))
        drop = NEW if cid == "all" else [cid]
        for m in c["modules"]:
            if m.get("module") == "cards":
                m["cards"] = [x for x in m["cards"] if x not in drop]
        c["name"] = f"{cid}_off"
        p = os.path.join("out", "c158", f"{cid}_off.json")
        with open(os.path.join(TOP, p), "w", encoding="utf-8", newline="\n") as f:
            json.dump(c, f, indent=2)
            f.write("\n")
        paths.append(p.replace("\\", "/"))
    return paths


def spec(name, note, levers, diffs, agents, games, seed, probe=False, configs=None):
    variants = []
    for tag, sets in levers:
        for p, s in PAIRS.items():
            variants.append({"name": p if tag == "base" else f"{p}_{tag}", "set": s + sets})
    out = {"name": name, "note": note, "configs": configs or ["configs/baseline.json"], "variants": variants,
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
    if mode == "offs":
        print("\n".join(offs()))
    elif mode == "screen":
        name, agents, lv, seeds = sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5])
        rest = sys.argv[6:]
        configs = None
        if "--offs" in rest:
            rest.remove("--offs")
            configs = ["configs/baseline.json"] + offs()
        first = 36800
        if "--first" in rest:
            i = rest.index("--first")
            first = int(rest[i + 1])
            del rest[i:i + 2]
        diffs = [int(x) for x in lv.split(",")]
        dump(name, spec(name, f"158 screen {name} (seeds {first}-{first + seeds - 1}).", levers(rest), diffs,
                        agents.split(";"), seeds, first, configs=configs))
    else:
        tag, seed = sys.argv[2], int(sys.argv[3])
        rest = sys.argv[4:]
        probe = "--probe" in rest
        rest = [a for a in rest if a != "--probe"]
        agent = "strategist"
        if rest and "=" not in rest[0]:
            agent = rest.pop(0)
        for i, half in enumerate("abc"):
            s = spec(f"read-158-{tag}-{half}", f"158 read ({tag}), set {half}: seeds {seed + 48 * i}-{seed + 48 * i + 47}, the three pairs, d0-d3.",
                     [("base", rest)], [0, 1, 2, 3], [agent], 48, seed + 48 * i, probe)
            dump(f"read-158-{tag}-{half}", s)
