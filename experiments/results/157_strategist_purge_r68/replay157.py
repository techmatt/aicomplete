"""157: checks two runs of the same spec seeds play the same games at the levels both hold (won, reason, rounds,
decisions per game), when a config change elsewhere moves the config hash and same_results.py would refuse.

python replay157.py DIR_A DIR_B
"""
import json
import sys


def games(d):
    out = {}
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        spec = json.loads(f.readline())["spec"]
        A, D, V = spec["agents"], spec["difficulties"], spec["variants"]
        for line in f:
            g = json.loads(line)
            c = g["cell"]
            a, di, v = c % len(A), (c // len(A)) % len(D), c // (len(A) * len(D))
            out[(V[v]["name"], D[di], g["seed"])] = (g["won"], g["reason"], g["rounds"], g["decisions"])
    return out


a, b = games(sys.argv[1]), games(sys.argv[2])
both = sorted(set(a) & set(b))
diff = [k for k in both if a[k] != b[k]]
print(f"{len(both)} games in both, {len(diff)} differ")
for k in diff[:10]:
    print(k, a[k], b[k])
sys.exit(1 if diff else 0)
