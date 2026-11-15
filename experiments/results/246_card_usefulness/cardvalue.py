"""246: the good-card measure from a card-value read (mkspec.py's spec: arms `base`, `no-<card>`, redesign arms,
each over three board pairs). Per arm against its reference on the same (seed, pair, level): the win-rate gap
(reference minus arm: what the card adds), its paired SE, and the rounds gap over games both won; with the card's
play rate in the reference arm. Reads a partial stream (seed-major) too.

python cardvalue.py GAMES [--ref base] [--pairs ARM:REF,...] [--levels 2,3]
"""
import argparse
import collections
import json
import math
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
ap = argparse.ArgumentParser()
ap.add_argument("games")
ap.add_argument("--ref", default="base")
ap.add_argument("--pairs", default="", help="extra ARM:REF comparisons (a redesign against its own blank)")
ap.add_argument("--levels", default="")
a = ap.parse_args()
fh = open(a.games, encoding="utf-8")
spec = json.loads(fh.readline())["spec"]
nl, na = len(spec["difficulties"]), len(spec["agents"])
variants = [v["name"] for v in spec["variants"]]
levels = {int(x) for x in a.levels.split(",")} if a.levels else None
res = {}  # (arm, seed, pair, level) -> (won, rounds, played-cards set)
for line in fh:
    if not line.strip():
        continue
    try:
        g = json.loads(line)
    except json.JSONDecodeError:
        break
    if levels is not None and g["difficulty"] not in levels:
        continue
    arm, pair = variants[g["cell"] // (nl * na)].split("|")
    played = {c for c, cc in g["cards"].items() if cc["played"]}
    res[(arm, g["cell"] % na, g["seed"], pair, g["difficulty"])] = (g["won"], g["rounds"], played)
arms = []
for v in variants:
    arm = v.split("|")[0]
    if arm not in arms:
        arms.append(arm)


def compare(arm, ref, card, agent):
    d, rd, n, both, played = [], [], 0, 0, 0
    for (ar, ag, seed, pair, lv), (w, r, pl) in res.items():
        if ar != ref or ag != agent:
            continue
        o = res.get((arm, ag, seed, pair, lv))
        if o is None:
            continue
        n += 1
        d.append(int(w) - int(o[0]))
        # The card's play rate in whichever of the two arms holds it unblanked.
        if card and card in (o[2] if ref.startswith("no-") else pl):
            played += 1
        if w and o[0]:
            both += 1
            rd.append(o[1] - r)
    if not n:
        return None
    m = sum(d) / n
    var = sum((x - m) ** 2 for x in d) / max(n - 1, 1)
    se = math.sqrt(var / n)
    rm = sum(rd) / len(rd) if rd else 0.0
    rse = math.sqrt(sum((x - rm) ** 2 for x in rd) / max(len(rd) - 1, 1) / max(len(rd), 1)) if rd else 0.0
    return n, 100 * m, 100 * se, rm, rse, 100 * played / n


print("adds = ref win% - arm win% (paired, same seed, pair, level); rounds = arm - ref over games both won;")
print("played% = the card's play rate in the arm holding it (ref, or the arm when ref is a blank)")
todo = [(arm, a.ref) for arm in arms if arm != a.ref and a.ref in arms]
todo += [tuple(p.split(":")) for p in a.pairs.split(",") if p]
for ai, agent in enumerate(spec["agents"]):
    base_w = [w for (ar, ag, *_), (w, *_r) in res.items() if ar == a.ref and ag == ai]
    print(f"\n{agent}: {a.ref} {len(base_w)} games, win {100 * sum(base_w) / max(len(base_w), 1):.1f}%")
    print(f"{'arm':36} {'ref':26} {'n':>5} {'adds':>6} {'±SE':>5} {'rounds':>7} {'±SE':>5} {'played%':>8}")
    for arm, ref in todo:
        card = arm[3:] if arm.startswith("no-") else None
        if ref.startswith("no-"):
            card = ref[3:]
        r = compare(arm, ref, card, ai)
        if r:
            n, m, se, rm, rse, pl = r
            print(f"{arm:36} {ref:26} {n:5} {m:+6.1f} {se:5.1f} {rm:+7.2f} {rse:5.2f} {pl:8.1f}")
