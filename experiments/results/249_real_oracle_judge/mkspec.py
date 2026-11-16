"""249: writes the item's specs.

    python experiments/results/249_real_oracle_judge/mkspec.py

- experiments/farm/r249-oracles.json: the judge, the full oracle (tries=3) and the real oracle side by side,
  three pairs, d0-d4 (run with --seed on fresh seeds, --games N a pair).
- experiments/farm/r249-arms.json: 248's arms a, b, c (board.std_off) with the judge and the real oracle, d0-d3.
"""
import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
FULL = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"
# The real oracle (249): the oracle's search on 4 sampled worlds, 3 alternatives beside the incumbent, 32 lines a
# round, on the judge's own weights and no strategy (the search on top of the judge: on the engine's weights and
# strategy=1 it read 55% d0-d4 to the judge's 70%, losing on an empty deck); its lines score their own world's
# table (truth=1: the world is the guess).
JUDGE_W = ("w.built=4,w.built_cost=0.5,w.clock=0,w.energy_cap=0,w.hand=1.5,w.hand_cost=0.1,w.horizon=0,"
           "w.insight_income=0,w.movement=0,w.packet_keep=1,w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0,"
           "w.purge=3.6,w.last_node=120")
REAL = "oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1," + JUDGE_W
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
B_OFF = ["board.std_off=1,2,3,5,6,8,9,10,11,12,13,14", "board.sell_credits=0"]
C_OFF = ["board.std_off=1,2,3,5,6,7,8,9,10,11,12,13,14", "board.sell_credits=0"]


def write(path, spec):
    with open(os.path.join(ROOT, path), "w", encoding="utf-8", newline="\n") as f:
        json.dump(spec, f, indent=1)
        f.write("\n")
    print(path)


def spec(name, note, variants, diffs, agents, games=4):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
            "difficulties": diffs, "agents": agents, "games": games, "first_seed": 0, "records": 0}


pairs = [{"name": n, "set": s} for n, s in PAIRS]
write("experiments/farm/r249-oracles.json", spec(
    "r249-oracles", "249: the judge, the full oracle and the real oracle (sampled worlds) on the same seeds, three "
    "pairs, d0-d4.", pairs, [0, 1, 2, 3, 4], ["strategist", FULL, REAL]))
write("experiments/farm/s249-judge.json", spec(
    "s249-judge", "249: the judge's card terms (w.card_income, w.card_action), screened d2-d3 on 248's spent seeds, "
    "three pairs.", pairs, [2, 3],
    ["strategist", "strategist:w.card_income=1", "strategist:w.card_income=0.5", "strategist:w.card_action=2",
     "strategist:w.card_action=4", "strategist:w.card_income=1,w.card_action=2"], games=96))
write("experiments/farm/s249-judge2.json", spec(
    "s249-judge2", "249: the judge's card_action around the first screen's best, on its seeds (its base: s249-judge).",
    pairs, [2, 3],
    ["strategist", "strategist:w.card_action=3", "strategist:w.card_action=6",
     "strategist:w.card_income=0.5,w.card_action=4"], games=96))
write("experiments/farm/s249-judge3.json", spec(
    "s249-judge3", "249: the judge's hand terms back toward greedy's (where the real oracle overrode the judge, greedy "
    "agreed 35%), alone and beside card_action, on the first screen's seeds.", pairs, [2, 3],
    ["strategist", "strategist:w.hand=3,w.hand_cost=0.4", "strategist:w.hand=2.25,w.hand_cost=0.25",
     "strategist:w.card_action=3,w.hand=2.25,w.hand_cost=0.25"], games=96))
write("experiments/farm/r249-fresh.json", spec(
    "r249-fresh", "249: the judge with card_action=3 (the asked lever, the screens' plateau) against the judge on fresh "
    "seeds, three pairs, d0-d3: the ship read and the bands.", pairs, [0, 1, 2, 3],
    ["strategist", "strategist:w.card_action=3"], games=192))
arms = []
for arm, extra in (("a", []), ("b", B_OFF), ("c", C_OFF)):
    for n, s in PAIRS:
        arms.append({"name": f"{arm}|{n}", "set": s + extra})
REAL2 = REAL.replace("worlds=4,alts=3,budget=32", "worlds=8,alts=3,budget=64")
write("experiments/farm/r249-real2.json", spec(
    "r249-real2", "249: the real oracle at twice the worlds and lines (8, 64 a round) beside the old and the shipped "
    "judge, the Builder-Protector pair (a farm shard is one thread: one pair keeps 12 seeds in 6 shards), d3-d4, on "
    "fresh seeds.", pairs[:1], [3, 4],
    ["engine:strategy=0," + JUDGE_W, "strategist", REAL2]))
write("experiments/farm/r249-armsjudge.json", spec(
    "r249-armsjudge", "249's rank check: 248's arms a, b, c (board.std_off) on the shipped judge, d0-d3, three pairs, "
    "on 248's probe seeds (its kept read holds the old judge's games).", arms, [0, 1, 2, 3], ["strategist"], games=96))
write("experiments/farm/r249-arms.json", spec(
    "r249-arms", "249: 248's arms a, b, c (board.std_off) read by the real oracle, d0-d3, three pairs.",
    arms, [0, 1, 2, 3], [REAL]))
