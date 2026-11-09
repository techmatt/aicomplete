"""156: writes the smoke, screen and read specs (python specs156.py [TAG FIRST_SEED [--probe]]).

Screens run on the dev seeds (36800-), the three pairs, the strategist. Reads are paired against r66's binary (its own
configs, out/r66) on the same fresh seeds: three 48-seed sets, d0-d3. Specs land in out/.
--probe adds 154's end-game probe (crates/aic-sim/src/endgame.rs) on every loss, with no replay.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# 155's probe on every loss; no replay agent (the snapshot and the end state only).
PROBE = {"back": 3, "agent": "", "rollouts": 0, "all_losses": True}
LEVERS = {
    "base": [],
    # Adversary income down (d3's entries: compute_income's 4th, ambassador_income's 4th).
    "inc1": ["adversary.compute_income=5,5,8,12,12", "adversary.ambassador_income=2,2,2,2,2"],
    "inc2": ["adversary.compute_income=4,4,6,10,10", "adversary.ambassador_income=1,1,2,2,2"],
    # The counter dials down.
    "cnt": ["adversary.counter_bonus_credits=4", "adversary.counter_more_credits=0"],
    "amb": ["adversary.ambassador_income=1,1,1,2,2"],
}
AGENTS = {
    "on": "strategist",
    "tier0": "engine:strategy=1,w.tier_read=0",
    "deck20": "engine:strategy=1,w.unity_target=20",
}


def spec(name, note, levers, diffs, agents, games, seed, probe=False):
    variants = []
    for lv in levers:
        sets = LEVERS[lv] if isinstance(lv, str) else lv[1]
        tag = lv if isinstance(lv, str) else lv[0]
        for p, s in PAIRS.items():
            variants.append({"name": p if tag == "base" else f"{p}_{tag}", "set": s + sets})
    out = {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
           "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0}
    if probe:
        out["endgame"] = PROBE
    return out


def dump(name, s):
    path = os.path.join(TOP, "out", name)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    os.makedirs(os.path.join(TOP, "out"), exist_ok=True)
    # The agent on/off smoke: 16 dev seeds, d1-d2, each agent change against the strategist; greedy and planner too.
    dump("smoke156-agents.json",
         spec("smoke156-agents", "156 agent smoke: tier read and purge target on/off (dev seeds 36800-36815, d1-d2).",
              ["base"], [1, 2], list(AGENTS.values()) + ["greedy", "greedy:w.tier_read=0"], 16, 36800))
    # The strategist valuing the purge more (154's replay agent read w.blocker=8), beside the screen's base.
    dump("screen156-agents.json",
         spec("screen156-agents", "156 screen: the strategist with the purge weighted up (dev seeds 36800-36831, d0-d3).",
              ["base"], [0, 1, 2, 3], ["engine:strategy=1,w.blocker=8", "engine:strategy=1,w.corrupted=60"], 32, 36800))
    dump("screen156-s1.json",
         spec("screen156-s1", "156 screen 1: r67 base (the prompt's numbers) and the adversary levers (dev seeds 36800-36831, d0-d3, strategist).",
              ["base", "inc1", "inc2", "cnt", "amb"], [0, 1, 2, 3], ["strategist"], 32, 36800, probe=True))
    if len(sys.argv) > 2:
        tag, seed = sys.argv[1], int(sys.argv[2])
        probe = "--probe" in sys.argv
        extra = [a for a in sys.argv[3:] if a != "--probe"]
        for i, half in enumerate("abc"):
            s = spec(f"read-156-{tag}-{half}", f"156 ladder read ({tag}), set {half}: seeds {seed + 48 * i}-{seed + 48 * i + 47}, the three pairs, d0-d3; run r66's binary on the same spec for the pair.",
                     [("base", extra)] if extra else ["base"], [0, 1, 2, 3], ["strategist"], 48, seed + 48 * i, probe)
            dump(f"read-156-{tag}-{half}.json", s)
