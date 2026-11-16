"""248: the probe's specs. Arms: a (baseline r112), b (every standard action off but Liberate and Dispatch),
bd (b with Dispatch off too), c (every standard action off, Liberate included; Dispatch kept). `board.std_off` takes
the actions out by place in STANDARD from 1: 1 station, 2 remove, 3 lift, 4 dispatch, 5 ambassador, 6 claim,
7 liberate, 8 insight, 9 envoy, 10 botnet, 11 buy energy, 12 antimatter, 13 node, 14 troop, 15 factory (off at
baseline); `board.sell_credits=0` takes Sell a card out.

python experiments/results/248_standard_actions_probe/mkspec.py  -> experiments/farm/p248.json, smoke248.json,
p248-oracle.json
"""
import json

B_OFF = [1, 2, 3, 5, 6, 8, 9, 10, 11, 12, 13, 14]
ARMS = {
    "a": [],
    "b": ["board.std_off=" + ",".join(map(str, B_OFF)), "board.sell_credits=0"],
    "bd": ["board.std_off=" + ",".join(map(str, sorted(B_OFF + [4]))), "board.sell_credits=0"],
    "c": ["board.std_off=" + ",".join(map(str, sorted(B_OFF + [7]))), "board.sell_credits=0"],
}
PLACE = {"station": 1, "remove": 2, "lift": 3, "dispatch": 4, "ambassador": 5, "claim": 6, "insight": 8,
         "botnet": 10, "buy_energy": 11, "antimatter": 12, "node": 13, "troop": 14}
# The job screen (spent seeds, paired with p248's a and b): each action off alone from a, and a few back alone into b.
LOO = ["node", "station", "remove", "troop", "lift", "dispatch", "ambassador", "claim", "insight", "botnet",
       "buy_energy", "antimatter", "sell"]
LOI = ["node", "station", "insight", "buy_energy", "botnet", "claim"]
for j in LOO:
    ARMS["-" + j] = ["board.sell_credits=0"] if j == "sell" else [f"board.std_off={PLACE[j]},"]
for j in LOI:
    ARMS["b+" + j] = ["board.std_off=" + ",".join(str(i) for i in B_OFF if i != PLACE[j]), "board.sell_credits=0"]
JOB_ARMS = ["-" + j for j in LOO] + ["b+" + j for j in LOI]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
ORACLE = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"


def variants(arms):
    return [{"name": f"{a}|{p}", "set": PAIRS[p] + ARMS[a]} for a in arms for p in PAIRS]


def spec(name, note, arms, agents, games=48, first=0):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants(arms),
            "difficulties": [0, 1, 2, 3], "agents": agents, "games": games, "first_seed": first, "records": 0}


def write(path, s):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=1)
        f.write("\n")


write("experiments/farm/p248.json", spec(
    "p248", "248's standard-actions probe on r112, the strategist, d0-d3, three pairs, paired across arms: a "
    "baseline; b every standard action off but Liberate and Dispatch (and Sell); bd b with Dispatch off; c every "
    "standard action off, Liberate included (Dispatch kept). Farm a seed range: farm-seeds F N OUT p248.",
    ["a", "b", "bd", "c"], ["strategist"]))
write("experiments/farm/p248j.json", spec(
    "p248j", "248's job screen on p248's spent seeds (48376-48423), paired with p248's a and b: each standard action "
    "off alone from a (-X), and six back alone into b (b+X). The strategist, d0-d3, three pairs.",
    JOB_ARMS, ["strategist"]))
write("experiments/archive/farm/smoke248.json", spec(
    "smoke248", "248: the probe's arms on the dev seeds (36800-36815), the strategist: crash-finding and the "
    "16-seed smoke of the reverted purge term.", ["a", "b", "bd", "c"], ["strategist"], games=16, first=36800))
write("experiments/archive/farm/p248-oracle.json", spec(
    "p248-oracle", "248's oracle spot-check: the oracle (tries=3) beside the strategist on the probe's first 4 seeds a "
    "pair (12 a level), arms a, b, c, d0-d3.", ["a", "b", "c"], ["strategist", ORACLE], games=4))
print("ok")
