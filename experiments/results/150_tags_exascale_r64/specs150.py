"""150: writes the screen and read specs (python specs150.py; screens land beside this file, reads in experiments/).

Screens run on the dev seeds (36800-), the three pairs, the strategist. Reads are paired against r63's binary
(its own configs) on the same fresh seeds: two 48-seed sets, d0-d3. The Market Stabilization Board read is a
second config (the baseline deck plus the card, `experiments/configs/msb_deck.json`) on the same seeds.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# The printed-tag rule has no config switch: a before/after screen pairs binaries (r63 against r64).
LEVERS = {"base": [],
          # Screen 4, the d2 dial (F1 d2 53.8, band 55-65; d3 49.7 at its top): counters' second step from d3,
          # not d2, which eases d2 alone (each level includes the ones below).
          "cma3": ["adversary.counter_more_at=3"],
          # Screen 5 (cma3 read +23.4 at d2): a smaller step, the credits item's second step cut (d2 and d3).
          "cmc2": ["adversary.counter_more_credits=2"], "cmc1": ["adversary.counter_more_credits=1"],
          # Screen 6: cmc2 (d2 +6.2, d3 +4.2) with a d3-only dial back (d3 sat at 49.7 on F1).
          "cmc2_bb4": ["adversary.counter_more_credits=2", "adversary.build_bonus=4"],
          "cmc2_s2r2": ["adversary.counter_more_credits=2", "adversary.stage2_removed=2"],
          # Screen 7 (on the dialed baseline, F2 d3 51.0): larger builds again.
          "bb5": ["adversary.build_bonus=5"]}


def spec(name, note, levers, diffs, agents, games, seed, configs=("configs/baseline.json",)):
    variants = []
    for lv in levers:
        for p, s in PAIRS.items():
            variants.append({"name": p if lv == "base" else f"{p}_{lv}", "set": s + LEVERS[lv]})
    return {
        "name": name, "note": note, "configs": list(configs), "variants": variants,
        "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0,
    }


def read_spec(tag, half, seed, configs=("configs/baseline.json",)):
    """A paired ladder read set: the three pairs, d0-d3, the strategist, 48 seeds."""
    return {
        "name": f"read-150-{tag}-{half}",
        "note": f"150 ladder read ({tag}), set {half}: seeds {seed}-{seed + 47}, the three pairs, d0-d3; run the r63 binary on the same spec (baseline config only) for the pair.",
        "configs": list(configs),
        "variants": [{"name": p, "set": s} for p, s in PAIRS.items()],
        "difficulties": [0, 1, 2, 3], "agents": ["strategist"], "games": 48, "first_seed": seed, "records": 0,
    }


def dump(path, s):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


def msb_config():
    """The baseline deck with Market Stabilization Board added after the deck's last card (148 kept it out)."""
    with open(os.path.join(TOP, "..", "configs", "baseline.json"), encoding="utf-8") as f:
        base = json.load(f)
    for m in base["modules"]:
        if "cards" in m:
            assert "market_stabilization_board" not in m["cards"]
            m["cards"].append("market_stabilization_board")
    base["name"] = base.get("name", "baseline") + "+msb"
    return base


if __name__ == "__main__":
    dump(os.path.join(TOP, "configs", "msb_deck.json"), msb_config())
    dump(os.path.join(HERE, "keep150.json"),
         spec("keep150", "150: Exascale keep and play rates (dev seeds 36800-36863, d0 and d2, the strategist).",
              ["base"], [0, 2], ["strategist"], 64, 36800))
    # Screen 1 (r64 build): the first keep fix (`plan_eval`, a keep filter by the eval's one-ply read of each
    # pick; taken out, patch in the write-up) on and off; r63's binary on keep150 is the third cell.
    dump(os.path.join(HERE, "screen150-s1.json"),
         spec("screen150-s1", "150 screen 1: plan_eval on / off at r64's tags (dev seeds 36800-36863, d0 and d2).",
              ["base"], [0, 2], ["strategist", "strategist:plan_eval=0"], 64, 36800))
    # Screen 2: the keep plan on the eval's clock (`plan_clock`, the deck cut short by the projected win) on and
    # off: the same games to the action (the clock never cut), so taken out.
    dump(os.path.join(HERE, "screen150-s2.json"),
         spec("screen150-s2", "150 screen 2: plan_clock on / off at r64's tags (dev seeds 36800-36863, d0 and d2).",
              ["base"], [0, 2], ["strategist", "strategist:plan_clock=0"], 64, 36800))
    # Screen 3: spare Patience insight prices an insight income at the idle rate in the plan (`plan_spare`).
    dump(os.path.join(HERE, "screen150-s3.json"),
         spec("screen150-s3", "150 screen 3: plan_spare on / off at r64's tags (dev seeds 36800-36863, d0 and d2).",
              ["base"], [0, 2], ["strategist", "strategist:plan_spare=0"], 64, 36800))
    dump(os.path.join(HERE, "screen150-s4.json"),
         spec("screen150-s4", "150 screen 4: the d2 dial counter_more_at 2 -> 3 (dev seeds 36800-36863, d2).",
              ["base", "cma3"], [2], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen150-s5.json"),
         spec("screen150-s5", "150 screen 5: counter_more_credits 3 -> 2 / 1 (dev seeds 36800-36863, d2 and d3).",
              ["base", "cmc2", "cmc1"], [2, 3], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen150-s6.json"),
         spec("screen150-s6", "150 screen 6: cmc2 with build_bonus 4 or stage2_removed 2 (dev seeds 36800-36863, d2 and d3).",
              ["base", "cmc2", "cmc2_bb4", "cmc2_s2r2"], [3], ["strategist"], 64, 36800))
    dump(os.path.join(HERE, "screen150-s7.json"),
         spec("screen150-s7", "150 screen 7: build_bonus 4 -> 5 on the dialed baseline (dev seeds 36800-36863, d3).",
              ["base", "bb5"], [3], ["strategist"], 64, 36800))
    # F1: the first fresh paired read, seeds 37912-38007 (the ledger's next free at 149); the MSB deck rides as a
    # second config on the same seeds (item 3).
    dump(os.path.join(TOP, "read-150-f1-a.json"), read_spec("f1", "a", 37912, ("configs/baseline.json", "experiments/configs/msb_deck.json")))
    dump(os.path.join(TOP, "read-150-f1-b.json"), read_spec("f1", "b", 37960, ("configs/baseline.json", "experiments/configs/msb_deck.json")))
    if len(sys.argv) > 1:
        # A later read: python specs150.py TAG FIRST_SEED (two 48-seed sets from FIRST_SEED).
        tag, seed = sys.argv[1], int(sys.argv[2])
        both = ("configs/baseline.json", "experiments/configs/msb_deck.json")
        dump(os.path.join(TOP, f"read-150-{tag}-a.json"), read_spec(tag, "a", seed, both))
        dump(os.path.join(TOP, f"read-150-{tag}-b.json"), read_spec(tag, "b", seed + 48, both))
