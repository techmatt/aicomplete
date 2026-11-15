"""245: whether agent A in one read played the same games as agent B in another (same variant, level and seed:
action for action), over the games both have.

python same_games.py READ_A/games.jsonl AGENT_A READ_B/games.jsonl AGENT_B
"""
import json
import sys


def load(path, agent):
    fh = open(path, encoding="utf-8")
    spec = json.loads(fh.readline())["spec"]
    ags, nv, nd = spec["agents"], len(spec["variants"]), len(spec["difficulties"])
    out = {}
    for line in fh:
        g = json.loads(line)
        c = g["cell"]
        if ags[c % len(ags)] != agent:
            continue
        v = spec["variants"][c // (len(ags) * nd)]["name"]
        out[(v, g["difficulty"], g["seed"])] = (g["won"], g["actions"])
    return out


a = load(sys.argv[1], sys.argv[2])
b = load(sys.argv[3], sys.argv[4])
both = sorted(set(a) & set(b))
diff = [k for k in both if a[k] != b[k]]
print(f"{len(both)} games in both, {len(diff)} differ" + (f": {diff[:5]}" if diff else ""))
sys.exit(1 if diff or not both else 0)
