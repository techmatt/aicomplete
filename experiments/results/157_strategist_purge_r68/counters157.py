"""157: per-game counter means by agent (one level), for counters matching a pattern; a diagnostic for screens.

python counters157.py DIR LEVEL PATTERN [--min 0.2]
"""
import json
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
d, level, pat = sys.argv[1], int(sys.argv[2]), re.compile(sys.argv[3])
lo = float(sys.argv[sys.argv.index("--min") + 1]) if "--min" in sys.argv else 0.2
sums = defaultdict(lambda: defaultdict(float))
n = defaultdict(int)
with open(f"{d}/games.jsonl", encoding="utf-8") as f:
    spec = json.loads(f.readline())["spec"]
    A, D = spec["agents"], spec["difficulties"]
    for line in f:
        g = json.loads(line)
        c = g["cell"]
        a, di = A[c % len(A)], D[(c // len(A)) % len(D)]
        if di != level:
            continue
        n[a] += 1
        for k, v in g.get("counters", {}).items():
            if pat.search(k):
                sums[a][k] += v
        for k, v in g.get("margin", {}).items():
            if isinstance(v, (int, float)) and pat.search("margin." + k):
                sums[a]["margin." + k] += v
agents = list(n)
keys = sorted({k for a in agents for k in sums[a] if sums[a][k] / n[a] >= lo})
short = [a[:40] for a in agents]
print("| counter | " + " | ".join(short) + " |")
print("|---" * (len(agents) + 1) + "|")
for k in keys:
    print(f"| {k} | " + " | ".join(f"{sums[a][k] / n[a]:.2f}" for a in agents) + " |")
