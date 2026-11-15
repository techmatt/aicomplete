"""246: the done-when card table: per card, the judge's and the oracle's play rate (245's kept test read, the
oracle's 576 cells), the engine's in the card-value read's base arm, and the good-card measure (cardvalue.py).

python table.py JUDGE_GAMES ORACLE_GAMES VALUE_GAMES
"""
import json
import math
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
judge, oracle, value = sys.argv[1:4]
CARDS = ("power_to_the_people positron_storage_ring forensic_audit antiproton_foundry nuclear_option wafer_fab_line "
         "dyson_swarm_tranche citywide_microgrid catalyzed_fab_run treaty_isotope_bank smart_grid_backbone "
         "baseload_contract field_grant annihilation_lance pumped_hydro_storage deep_state_ledger "
         "multiyear_defense_budget").split()


def load(path, want=None):
    fh = open(path, encoding="utf-8")
    spec = json.loads(fh.readline())["spec"]
    agents, nl = spec["agents"], len(spec["difficulties"])
    out = {}
    for line in fh:
        if not line.strip():
            continue
        g = json.loads(line)
        ag = agents[g["cell"] % len(agents)]
        if want and want not in ag:
            continue
        out[(g["seed"], g["cell"] // (len(agents) * nl), g["difficulty"])] = g
    return out


o = load(oracle)
j = load(judge, "w.hand_cost=0.1")
keys = [k for k in o if k in j]
rate = lambda games, c: 100 * sum(1 for g in games if g["cards"].get(c, {}).get("played")) / len(games)
jg, og = [j[k] for k in keys], [o[k] for k in keys]
# The engine's base arm and each card's blank arm, paired.
fh = open(value, encoding="utf-8")
spec = json.loads(fh.readline())["spec"]
nl = len(spec["difficulties"])
names = [v["name"] for v in spec["variants"]]
res = {}
for line in fh:
    if line.strip():
        g = json.loads(line)
        arm, pair = names[g["cell"] // nl].split("|")
        res[(arm, g["seed"], pair, g["difficulty"])] = g
base = [(k, g) for k, g in res.items() if k[0] == "base"]
print("| card | judge play % | oracle play % | engine play % | adds (engine, win pts) | ±SE |")
print("|---|---|---|---|---|---|")
for c in CARDS:
    d = [int(g["won"]) - int(res[("no-" + c,) + k[1:]]["won"]) for k, g in base if ("no-" + c,) + k[1:] in res]
    m = sum(d) / len(d)
    se = math.sqrt(sum((x - m) ** 2 for x in d) / (len(d) - 1) / len(d))
    eng = 100 * sum(1 for _, g in base if g["cards"].get(c, {}).get("played")) / len(base)
    print(f"| {c} | {rate(jg, c):.1f} | {rate(og, c):.1f} | {eng:.1f} | {100 * m:+.1f} | {100 * se:.1f} |")
