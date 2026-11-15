"""armsum.py NAME...: per arm (out/243_oracle_overnight/NAME/games.jsonl + NAME.trace): wins, agent s, lines; and per game vs the first arm."""
import json
import os
import re
import sys

D = os.path.dirname(os.path.abspath(__file__))
pat = re.compile(r"OTRACE (\S+) seed=(\d+) round=(\d+) actions=(\d+) spent=\d+ reseeded=\w+ lines=(\d+) won=(\w+)")


def lines_by_game(trace):
    """(seed, order within the seed) -> lines; games on one thread in turn: a new game when actions drop."""
    games = []
    cur = {}
    if not os.path.exists(trace):
        return games
    for line in open(trace, encoding="utf-8", errors="replace"):
        m = pat.search(line)
        if not m:
            continue
        th, seed, rnd, act, lines, won = m.groups()
        g = cur.get(th)
        if g is None or g["seed"] != seed or int(act) < g["act"]:
            g = {"seed": seed, "act": 0, "lines": 0, "win_round": None}
            cur[th] = g
            games.append(g)
        g["act"] = int(act)
        g["lines"] = int(lines)
        if won == "true" and g["win_round"] is None:
            g["win_round"] = int(rnd)
    return games


def load(name):
    games = {}
    with open(os.path.join(D, name, "games.jsonl"), encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            if "seed" in d:
                games[(d["cell"], d["difficulty"], d["seed"])] = d
    return games


arms = sys.argv[1:]
base = load(arms[0])
for a in arms:
    g = load(a)
    tr = lines_by_game(os.path.join(D, a + ".trace"))
    won = sum(x["won"] for x in g.values())
    ms = sum(x["agent_ms"] for x in g.values()) / 1000
    lines = sum(t["lines"] for t in tr)
    common = [k for k in g if k in base]
    wb = sum(base[k]["won"] for k in common)
    wa = sum(g[k]["won"] for k in common)
    gain = [k for k in common if g[k]["won"] and not base[k]["won"]]
    lost = [k for k in common if base[k]["won"] and not g[k]["won"]]
    print(f"{a:40s} n {len(g):3d} won {won:3d} (paired {wa} vs {wb}; +{len(gain)} -{len(lost)}) agent {ms:7.0f} s ({ms/len(g):5.1f}/game) lines {lines:6d}")
