"""246: oracle-minus-judge play gap per card, on the seeds both played.

python gap.py JUDGE_GAMES ORACLE_GAMES [--agent SUBSTR] [--top N]

Share of games a card is played in, for the judge (the agent whose spec contains SUBSTR) restricted to the oracle's
(seed, variant, level) cells, and for the oracle; also kept a game and offered a game. Sorted by gap.
"""
import argparse
import collections
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
ap = argparse.ArgumentParser()
ap.add_argument("judge")
ap.add_argument("oracle")
ap.add_argument("--agent", default="w.hand_cost=0.1")
ap.add_argument("--top", type=int, default=999)
a = ap.parse_args()


def load(path, want=None):
    fh = open(path, encoding="utf-8")
    spec = json.loads(fh.readline())["spec"]
    agents = spec["agents"]
    out = {}
    for line in fh:
        if not line.strip():
            continue
        g = json.loads(line)
        ag = agents[g["cell"] % len(agents)]
        if want and want not in ag:
            continue
        var = g["cell"] // (len(agents) * len(spec["difficulties"]))
        out[(g["seed"], var, g["difficulty"])] = g
    return out


o = load(a.oracle)
j = load(a.judge, a.agent)
keys = [k for k in o if k in j]
print(f"cells both: {len(keys)} (oracle {len(o)}, judge {len(j)})")
stat = collections.defaultdict(lambda: [0, 0, 0, 0, 0, 0])  # j played, o played, j kept, o kept, j offered, o offered
for k in keys:
    for side, g in ((0, j[k]), (1, o[k])):
        for c, cc in g["cards"].items():
            s = stat[c]
            s[side] += 1 if cc["played"] else 0
            s[2 + side] += cc["kept"]
            s[4 + side] += cc["offered"]
n = len(keys)
ratios = sorted(s[1] / s[0] for s in stat.values() if s[0] >= 0.05 * n and s[1] >= 0.05 * n)
med = ratios[len(ratios) // 2]
print(f"median play-rate ratio oracle / judge (cards both play in 5%+): {med:.2f}")
print("excess = oracle play% - median ratio x judge play%: the gap past the oracle's general extra play")
rows = []
for c, s in stat.items():
    rows.append((100 * (s[1] - med * s[0]) / n, c, s))
rows.sort(reverse=True)
print(f"{'card':30} {'j.play%':>8} {'o.play%':>8} {'gap':>6} {'excess':>7} {'j.kept':>7} {'o.kept':>7} {'offer':>6}")
for ex, c, s in rows[: a.top]:
    gap = 100 * (s[1] - s[0]) / n
    print(f"{c:30} {100 * s[0] / n:8.1f} {100 * s[1] / n:8.1f} {gap:6.1f} {ex:7.1f} {s[2] / n:7.2f} {s[3] / n:7.2f} {s[5] / n:6.2f}")
named = "continental_supergrid,tidal_barrage,multiyear_defense_budget,agency_directorate".split(",")
print("\nnamed in the prompt:")
for c in named:
    s = stat[c]
    print(f"  {c:30} j {100 * s[0] / n:5.1f}  o {100 * s[1] / n:5.1f}  kept j {s[2] / n:.2f} o {s[3] / n:.2f}")
print("\nplayed in under 10% by both:")
for gap, c, s in sorted(rows, key=lambda r: r[2][0] + r[2][1]):
    if s[0] < 0.1 * n and s[1] < 0.1 * n:
        print(f"  {c:30} j {100 * s[0] / n:5.1f}  o {100 * s[1] / n:5.1f}  kept j {s[2] / n:.2f} o {s[3] / n:.2f}")
