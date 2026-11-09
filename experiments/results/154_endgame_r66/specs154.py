"""154: writes the screen, probe and read specs (python specs154.py [TAG FIRST_SEED [--probe]]).

Screens run on the dev seeds (36800-), the three pairs, the strategist. Reads are paired against r65's binary (its own
configs) on the same fresh seeds: two 48-seed sets, d0-d3. Specs land beside this file; a read lands in out/.
--probe adds the end-game probe (crates/aic-sim/src/endgame.rs) with the planner replay.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
PROBE = {"back": 3, "agent": "planner:strategy=1,w.blocker=8", "rollouts": 2}
LEVERS = {"base": [],
          # The late liberation off: r65's printed rule (the prompt's stated numbers: nothing else moves).
          "off": ["board.liberate_late_at=5"],
          "l4": ["board.liberate_late_at=4"],
          "l2": ["board.liberate_late_at=2"],
          # From stage II instead of III (round 5 on).
          "s2": ["board.liberate_late_stage=2"],
          # Screen 2 (late liberation 3 taken; on the fresh seeds it should read about d1 +2, d2 +12, d3 +14 over r65):
          # the four dials. counter_bonus_credits reaches d1 and up, counter_more_credits d2 and up, each income entry
          # its own level.
          "cbc8": ["adversary.counter_bonus_credits=8"],
          "cbc10": ["adversary.counter_bonus_credits=10"],
          "x1": ["adversary.counter_bonus_credits=8", "adversary.compute_income=5,5,11,14,14"],
          "x2": ["adversary.counter_bonus_credits=8", "adversary.compute_income=5,5,12,16,16"],
          "x3": ["adversary.counter_bonus_credits=8", "adversary.counter_more_credits=4", "adversary.compute_income=5,5,10,14,14"]}


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


def dump(path, s):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    dump(os.path.join(HERE, "screen154-s1.json"),
         spec("screen154-s1", "154 screen 1: the late liberation at 3 (base), off (5: r65), 4 and 2 (dev seeds 36800-36831, d0-d3, strategist).",
              ["base", "off", "l4", "l2"], [0, 1, 2, 3], ["strategist"], 32, 36800, probe=True))
    dump(os.path.join(HERE, "screen154-s2.json"),
         spec("screen154-s2", "154 screen 2: the dials on the late liberation at 3 (dev seeds 36800-36831, d1-d3, strategist).",
              ["base", "cbc8", "cbc10", "x1", "x2", "x3"], [1, 2, 3], ["strategist"], 32, 36800))
    if len(sys.argv) > 2:
        tag, seed = sys.argv[1], int(sys.argv[2])
        probe = "--probe" in sys.argv
        for i, half in enumerate("ab"):
            s = spec(f"read-154-{tag}-{half}", f"154 ladder read ({tag}), set {half}: seeds {seed + 48 * i}-{seed + 48 * i + 47}, the three pairs, d0-d3; run r65's binary on the same spec for the pair.",
                     ["base"], [0, 1, 2, 3], ["strategist"], 48, seed + 48 * i, probe)
            dump(os.path.join(TOP, "out", f"read-154-{tag}-{half}.json"), s)
