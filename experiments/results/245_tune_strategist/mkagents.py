"""245: agent spec lists for `aic-sim agree --agents FILE`.

python mkagents.py oat OUT [--base K=V,...]   one at a time: each weight below at x0.5 and x2 (or its listed points)
python mkagents.py grid OUT K=a,b,c [K2=...]  every listed value of each key, one key at a time, over --base
"""
import argparse

# The strategist's defaults as it plays (STRATEGIST over Weights::default), the flat-mode weights it reads.
FLAT = {
    "corrupted": 30, "node_danger": 60, "aligned": 8, "adv_ambassador": 3, "pressure": 0.7, "on_piece": 2,
    "on_factory": 6, "troop": 12, "discord_danger": 40, "blocker": 2, "tier_read": 1, "unity_tip": 24,
    "discord_tip": 16, "level": 10, "project": 30, "node": 4, "botnet": 25, "threat_waits": 0.5,
    "threat_harm": 1, "tempo": 0.5, "owned_factory": 10, "chips": 4, "reach": 0.5, "station": 5,
    "project_cube": 2, "research_cube": 0.3, "city_dark": 4, "station_flip": 20, "harden": 1,
    "ambassador": 5, "ambassador_board": 2.5, "freeing": 2, "liberated": 10, "purge": 3.6, "at_risk": 4,
    "income": 3, "track_capstone": 16, "credits": 0.6, "credit_cap": 12, "credit_idle": 0.15, "program": 1,
    "token": 0.5, "energy": 0.8, "antimatter": 2, "insight": 0.5, "hand": 3, "hand_cost": 0.4, "drafted": 0.3,
    "packet_keep": 1, "built": 4, "built_cost": 0.5, "threat": 6, "corrupt_exposure": 0.25, "block": 4,
    "last_node": 120, "insight_cap": 10,
}
EXTRA = {"built_income": [0.5, 1.0, 1.5]}


def fmt(x):
    return f"{x:g}"


ap = argparse.ArgumentParser()
ap.add_argument("mode", choices=["oat", "grid"])
ap.add_argument("out")
ap.add_argument("keys", nargs="*")
ap.add_argument("--base", default="")
a = ap.parse_args()
base = a.base.strip(",")
pre = "strategist" + (":" + base if base else "")
sep = "," if base else ":"
lines = [pre]
if a.mode == "oat":
    for k, d in FLAT.items():
        for m in (0.5, 2.0):
            lines.append(f"{pre}{sep}w.{k}={fmt(d * m)}")
    for k, vs in EXTRA.items():
        for v in vs:
            lines.append(f"{pre}{sep}w.{k}={fmt(v)}")
else:
    for kv in a.keys:
        k, vs = kv.split("=")
        for v in vs.split(","):
            lines.append(f"{pre}{sep}{k}={v}")
with open(a.out, "w", encoding="utf-8", newline="\n") as fh:
    fh.write("\n".join(lines) + "\n")
print(len(lines), "agents ->", a.out)
