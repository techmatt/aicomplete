"""gapsum.py DIR... [--md]: oracle against strategist on the same seeds, per pair x level and pooled by level.

DIR: experiment outputs with both agents. Paired by (variant, difficulty, seed): the gap is the mean
of oracle won minus strategist won over seeds both played, SE from the per-seed differences.
"""
import collections
import json
import math
import os
import sys

dirs = [a for a in sys.argv[1:] if not a.startswith("--")]
md = "--md" in sys.argv
res = collections.defaultdict(dict)  # (variant, diff, seed) -> kind -> (won, agent_ms, reason)
for d in dirs:
    r = json.load(open(os.path.join(d, "results.json"), encoding="utf-8"))
    cell = {c["cell"]: (c["variant"], c["difficulty"], "oracle" if c["agent"].startswith("oracle") else "strategist") for c in r["cells"]}
    for line in open(os.path.join(d, "games.jsonl"), encoding="utf-8"):
        g = json.loads(line)
        if "seed" not in g:
            continue
        v, df, k = cell[g["cell"]]
        res[(v, df, g["seed"])][k] = (g["won"], g["agent_ms"], g["reason"])


def row(keys, label):
    pairs = [res[k] for k in keys if "oracle" in res[k] and "strategist" in res[k]]
    n = len(pairs)
    if not n:
        return None
    o = sum(p["oracle"][0] for p in pairs) / n
    s = sum(p["strategist"][0] for p in pairs) / n
    diffs = [p["oracle"][0] - p["strategist"][0] for p in pairs]
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(n - 1, 1))
    se = sd / math.sqrt(n)
    oms = sum(p["oracle"][1] for p in pairs) / n / 1000
    only_s = sum(1 for p in pairs if p["strategist"][0] and not p["oracle"][0])
    return (label, n, 100 * o, 100 * s, 100 * m, 100 * se, oms, only_s)


rows = []
variants = sorted({k[0] for k in res})
diffs_ = sorted({k[1] for k in res})
for df in diffs_:
    for v in variants:
        r = row([k for k in res if k[0] == v and k[1] == df], f"{v} d{df}")
        if r:
            rows.append(r)
    r = row([k for k in res if k[1] == df], f"all d{df}")
    if r:
        rows.append(r)
r = row(list(res), "all")
if r:
    rows.append(r)
if md:
    print("| cell | n | oracle % | strategist % | gap (pts) | SE | oracle s/game | strategist-only wins |")
    print("|---|---|---|---|---|---|---|---|")
    for x in rows:
        print(f"| {x[0]} | {x[1]} | {x[2]:.1f} | {x[3]:.1f} | {x[4]:+.1f} | {x[5]:.1f} | {x[6]:.1f} | {x[7]} |")
else:
    for x in rows:
        print(f"{x[0]:10s} n {x[1]:4d} oracle {x[2]:5.1f} strategist {x[3]:5.1f} gap {x[4]:+5.1f} ± {x[5]:.1f}  oracle {x[6]:5.1f} s/game  s-only {x[7]}")
