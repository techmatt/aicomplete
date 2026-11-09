"""157: win % by agent and level from a run's games.jsonl, finished games only (a run still going reads too).

python partial157.py DIR [--seeds a-b]   (seeds: only games whose seed is in the range, for pairing)
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
d = sys.argv[1]
lo, hi = 0, 1 << 64
if "--seeds" in sys.argv:
    a, b = sys.argv[sys.argv.index("--seeds") + 1].split("-")
    lo, hi = int(a), int(b)
cells = {}
try:
    for c in json.load(open(f"{d}/results.json", encoding="utf-8"))["cells"]:
        cells[c["cell"]] = c
except FileNotFoundError:
    pass
spec = None
out = defaultdict(list)
ms = defaultdict(list)
with open(f"{d}/games.jsonl", encoding="utf-8") as f:
    head = json.loads(f.readline())
    spec = head.get("spec", {})
    if not cells:
        # Cells run variant-major, then difficulty, then agent, in the spec's order.
        A, D = spec["agents"], spec["difficulties"]
        for vi, v in enumerate(spec["variants"]):
            for di, dd in enumerate(D):
                for ai, a in enumerate(A):
                    cells[(vi * len(D) + di) * len(A) + ai] = {"agent": a, "difficulty": dd, "variant": v["name"]}
    for line in f:
        try:
            g = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not lo <= int(g.get("seed", 0)) <= hi:
            continue
        key = (g.get("agent") or cells.get(g["cell"], {}).get("agent"), g.get("difficulty", cells.get(g["cell"], {}).get("difficulty")))
        out[key].append(1 if g.get("won") else 0)
        if "agent_ms" in g:
            ms[key].append(g["agent_ms"])
print("| Agent | Level | Games | Win % | SE | agent s/game |")
print("|---|---|---|---|---|---|")
for k in sorted(out, key=lambda k: (str(k[0]), str(k[1]))):
    v = out[k]
    p = sum(v) / len(v)
    se = math.sqrt(p * (1 - p) / len(v)) * 100
    t = sum(ms[k]) / len(ms[k]) / 1000 if ms[k] else float("nan")
    print(f"| {k[0]} | d{k[1]} | {len(v)} | {100 * p:.1f} | {se:.1f} | {t:.1f} |")
