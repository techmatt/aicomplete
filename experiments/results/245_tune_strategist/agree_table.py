"""245: `aic-sim agree` output as a table: each agent's agreement (same pick / same or later this round) over all
decisions and by kind, and its change from the first agent (the base), in points. Sorted by the change in `score`.

python agree_table.py AGREE.jsonl [--group all|d2|d3] [--top N] [--kinds play,power,...]

`score`: same picks over the decisions the search decides (every kind but keep, tier and draft, which the keep
plan and the draft rule mostly decide), plus same-or-later picks over the same, halved: one number to rank by.
"""
import argparse
import json

KINDS = ["choose", "play", "power", "use", "standard", "threat", "sell", "pass", "convert", "draft", "tier", "keep"]
SEARCH = {"choose", "play", "power", "use", "standard", "threat", "sell", "pass", "convert"}

ap = argparse.ArgumentParser()
ap.add_argument("file")
ap.add_argument("--group", default="all")
ap.add_argument("--top", type=int, default=0)
ap.add_argument("--kinds", default="play,power,use,standard,choose,threat,sell,pass")
a = ap.parse_args()
rows = []
for line in open(a.file, encoding="utf-8"):
    j = json.loads(line)
    t = j["by"][a.group]
    same = sum(v[0] for k, v in t.items() if k in SEARCH)
    later = sum(v[1] for k, v in t.items() if k in SEARCH)
    n = sum(v[2] for k, v in t.items() if k in SEARCH)
    score = 100 * (same + 0.5 * later) / n
    rows.append((j["agent"], score, t))
base = rows[0]
kinds = a.kinds.split(",")
print(f"{'agent':60} {'score':>6} {'d':>6} " + " ".join(f"{k[:6]:>7}" for k in kinds))


def cell(t, k):
    v = t.get(k, [0, 0, 0])
    return 100 * v[0] / v[2] if v[2] else 0.0


out = sorted(rows[1:], key=lambda r: -r[1])
if a.top:
    out = out[: a.top] + [None] + out[-a.top:]
for r in [base, None] + out:
    if r is None:
        print("-" * 40)
        continue
    name = r[0].replace("strategist", "S", 1)
    d = r[1] - base[1]
    print(f"{name[:60]:60} {r[1]:6.2f} {d:+6.2f} " + " ".join(f"{cell(r[2], k) - (0 if r is base else cell(base[2], k)):+7.2f}" if r is not base else f"{cell(r[2], k):7.2f}" for k in kinds))
