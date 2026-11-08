"""151: writes the screen and read specs (python specs151.py [TAG FIRST_SEED]).

Screens run on the dev seeds (36800-), the three pairs, the strategist. Reads are paired against r64's binary
(its own configs) on the same fresh seeds: two 48-seed sets, d0-d3. Screens land beside this file, reads in
experiments/.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# Levers screened beside the base (each a list of --set lines).
LEVERS = {"base": [],
          # The prompt's stated numbers: compute 5 a round, ambassadors 1 a board a round, at every level (the
          # Militarist's Garrisons adds its own +1 a board on top, as a board line may).
          "stated": ["adversary.compute_income=5,5,5,5,5", "adversary.ambassador_income=1,1,1,1,1"],
          # Screen 1, the income dial: one more ambassador a board at every level; more compute.
          "a2": ["adversary.ambassador_income=2,2,2,3,3"],
          "c8": ["adversary.compute_income=8,8,8,8,8"],
          # Screen 2 (baseline now ambassador_income 2,2,2,3,3 from screen 1), d3 only: more income at d3, and the
          # prompt's smaller stage step (one stage-II card removed, one stage-I card added back).
          "d3a4": ["adversary.ambassador_income=2,2,2,4,4"],
          "d3a5": ["adversary.ambassador_income=2,2,2,5,5"],
          "d3c10": ["adversary.compute_income=5,5,5,10,10"],
          "small": ["adversary.stage2_removed=1", "adversary.stage1_added=1"],
          # Screen 3 (after the review's fixes): compute income as the d3 step (d3c10 read -6.2 on screen 2), and
          # a d2 step to centre d2 (63.5 on screen 1, near the band's top).
          "c10d3": ["adversary.compute_income=5,5,5,10,10"],
          "c12d3": ["adversary.compute_income=5,5,5,12,12"],
          "c7d2": ["adversary.compute_income=5,5,7,10,10"],
          # Screen 4 (F1 on compute 5,5,8,10,10: d1 84.4, d2 66.3, a point over): one more step at d1 and d2.
          "c6d1": ["adversary.compute_income=5,6,8,10,10"],
          "c10d2": ["adversary.compute_income=5,5,10,10,10"],
          "c6d1_c10d2": ["adversary.compute_income=5,6,10,10,10"],
          # Screen 5 (on compute 5,5,10,10,10; F2+F3 d1 85.6, d3 50.0): ambassador income 3 from d1 (d2's compute
          # eased or not), the d1 counters' credits step, and one more compute at d3.
          "a3d1": ["adversary.ambassador_income=2,3,3,3,3", "adversary.compute_income=5,5,7,10,10"],
          "a3d1c10": ["adversary.ambassador_income=2,3,3,3,3"],
          "cbc4": ["adversary.counter_bonus_credits=4"],
          "c11d3": ["adversary.compute_income=5,5,10,11,11"],
          # Screen 6 (income moved d1 by nothing on screens 4-5): d1's own dials, bigger steps (they reach d2-d3 too).
          "cba2": ["adversary.counter_bonus_all=2"],
          "cbc6": ["adversary.counter_bonus_credits=6"],
          "cc2": ["adversary.corrupt_cut=2,1,1,0"]}


def spec(name, note, levers, diffs, agents, games, seed, configs=("configs/baseline.json",)):
    variants = []
    for lv in levers:
        sets = LEVERS[lv] if isinstance(lv, str) else lv[1]
        tag = lv if isinstance(lv, str) else lv[0]
        for p, s in PAIRS.items():
            variants.append({"name": p if tag == "base" else f"{p}_{tag}", "set": s + sets})
    return {
        "name": name, "note": note, "configs": list(configs), "variants": variants,
        "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0,
    }


def read_spec(tag, half, seed, configs=("configs/baseline.json",)):
    """A paired ladder read set: the three pairs, d0-d3, the strategist, 48 seeds."""
    return {
        "name": f"read-151-{tag}-{half}",
        "note": f"151 ladder read ({tag}), set {half}: seeds {seed}-{seed + 47}, the three pairs, d0-d3; run the r64 binary on the same spec for the pair.",
        "configs": list(configs),
        "variants": [{"name": p, "set": s} for p, s in PAIRS.items()],
        "difficulties": [0, 1, 2, 3], "agents": ["strategist"], "games": 48, "first_seed": seed, "records": 0,
    }


def dump(path, s):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    # The agents' rally read on and off (151's one new heuristic), with a greedy line: dev seeds, d0 and d2.
    dump(os.path.join(HERE, "smoke151-rally.json"),
         spec("smoke151-rally", "151 smoke: the rally read on / off (strategist, greedy), dev seeds 36800-36815, d0 and d2.",
              ["base"], [0, 2], ["strategist", "strategist:w.rally_read=0", "greedy", "greedy:w.rally_read=0"], 16, 36800))
    dump(os.path.join(HERE, "screen151-s1.json"),
         spec("screen151-s1", "151 screen 1: the income dial (dev seeds 36800-36831, d0-d3, strategist).",
              ["base", "stated", "a2", "c8"], [0, 1, 2, 3], ["strategist"], 32, 36800))
    dump(os.path.join(HERE, "screen151-s2.json"),
         spec("screen151-s2", "151 screen 2: d3 dials on ambassador_income 2,2,2,3,3 (dev seeds 36800-36863, d3, strategist).",
              ["base", "stated", "d3a4", "d3a5", "d3c10", "small"], [3], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen151-s3.json"),
         spec("screen151-s3", "151 screen 3: compute income steps for d2 and d3 (dev seeds 36800-36863, d1-d3, strategist).",
              ["base", "stated", "c10d3", "c12d3", "c7d2"], [1, 2, 3], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen151-s4.json"),
         spec("screen151-s4", "151 screen 4: compute income one step up at d1 and d2 (dev seeds 36800-36863, d1-d2, strategist).",
              ["base", "stated", "c6d1", "c10d2"], [1, 2], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen151-s5.json"),
         spec("screen151-s5", "151 screen 5: a d1 step (ambassador income 3 from d1, or counters' credits) and a d3 step (dev seeds 36800-36863, d1-d3, strategist).",
              ["base", "stated", "a3d1", "a3d1c10", "cbc4", "c11d3"], [1, 2, 3], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen151-s6.json"),
         spec("screen151-s6", "151 screen 6: d1's dials, bigger steps (dev seeds 36800-36863, d1-d3, strategist).",
              ["base", "stated", "cba2", "cbc6", "cc2"], [1, 2, 3], ["strategist"], 64, 36800))
    if len(sys.argv) > 2:
        # A read: python specs151.py TAG FIRST_SEED (two 48-seed sets from FIRST_SEED).
        tag, seed = sys.argv[1], int(sys.argv[2])
        dump(os.path.join(TOP, f"read-151-{tag}-a.json"), read_spec(tag, "a", seed))
        dump(os.path.join(TOP, f"read-151-{tag}-b.json"), read_spec(tag, "b", seed + 48))
