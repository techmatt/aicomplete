"""162: the farm specs for the judge screens and the final read (writes experiments/farm/*.json).

python experiments/results/162_judge_purge/specs162.py
"""
import json
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
FARM = os.path.join(ROOT, "experiments", "farm")

PAIRS = [{"name": "bp", "set": []}, {"name": "br", "set": ["ai.boards=0,2"]}, {"name": "pr", "set": ["ai.boards=1,2"]}]
# Greedy's flat weights on the engine (157's FLAT; w.clock=0 is the strategist's own).
FLAT = ("w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=3,w.hand_cost=0.4,w.horizon=0,"
        "w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0")
OLD = "engine:strategy=1,w.clock=0"  # r69's strategist (157)
PURGE = "w.purge=3.6"  # (a): an ambassador toward the bill at 2.4 + 3.6 = 6, a city's 30 over its 5-ambassador bill
CELLS = {
    "old": OLD,
    "a": f"{OLD},{PURGE}",
    "b": f"engine:strategy=1,{FLAT}",
    "ab": f"engine:strategy=1,{FLAT},{PURGE}",
}


def spec(name, note, diffs, agents, games=32):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": PAIRS,
            "difficulties": diffs, "agents": agents, "games": games, "records": 0}


def write(s):
    with open(os.path.join(FARM, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    write(spec("s162-screen", "162: the judge factorial, (a) the purge term and (b) greedy's flat weights with "
               "strategy, each on and off, beside greedy; three pairs, d1-d3 (dev seeds 36800-36831).",
               [1, 2, 3], [CELLS["old"], CELLS["a"], CELLS["b"], CELLS["ab"], "greedy"]))
    write(spec("s162-read", "162: the final read, the old judge, the new (`strategist`: b + a) and greedy, paired on the "
               "same seeds; three pairs, d0-d3; two 48-seed sets (40264-40311, 40312-40359).",
               [0, 1, 2, 3], [OLD, "strategist", "greedy"], 48))
    # The oracle row beside the read (not farmable: run locally with `experiment`), d3, 8 seeds a pair.
    oracle = spec("s162-oracle", "162: the oracle row beside the final read, d3, its first 8 seeds a pair.", [3],
                  ["oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1"], 8)
    oracle["first_seed"] = 40264
    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "s162-oracle.json"), "w", encoding="utf-8",
              newline="\n") as f:
        json.dump(oracle, f, indent=2)
        f.write("\n")
