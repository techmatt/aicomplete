"""175: win % per agent and difficulty, new against old on the same seeds (paired by variant, seed), from two sets
of runs' games.jsonl; then each side's agent cost a decision.

    python experiments/results/175_rules_adversary/pairs175.py NEW_DIRS OLD_DIRS

NEW_DIRS and OLD_DIRS are ','-separated. A game pairs when both sides played the same (variant, difficulty, agent,
seed). Prints games, old and new win %, the paired gap and its SE (the SE of the per-seed difference); then, per
agent and side, the games' summed agent ms over their decisions (the games' own clock, not wall time) and mean
decisions and rounds a game.
"""
import argparse
import collections
import json
import math
import os


def read(dirs):
    out, cost = {}, collections.defaultdict(lambda: [0.0, 0, 0, 0])
    for d in dirs.split(","):
        with open(os.path.join(d, "games.jsonl"), encoding="utf-8") as f:
            head = json.loads(f.readline())
            s = head["spec"]
            cells = [(v["name"], diff, a) for _ in s["configs"] for v in s.get("variants", [{"name": "base"}])
                     for diff in s["difficulties"] for a in s["agents"]]
            for line in f:
                g = json.loads(line)
                if "cell" in g:
                    var, diff, agent = cells[g["cell"]]
                    out[(agent, var, diff, g["seed"])] = 1 if g["won"] else 0
                    c = cost[agent]
                    c[0] += g.get("agent_ms", 0.0)
                    c[1] += g.get("decisions", 0)
                    c[2] += 1
                    c[3] += g.get("rounds", 0)
    return out, cost


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("new")
    ap.add_argument("old")
    a = ap.parse_args()
    (new, ncost), (old, ocost) = read(a.new), read(a.old)
    rows = collections.defaultdict(list)
    for key, w in new.items():
        if key in old:
            rows[(key[0], key[2])].append((w, old[key]))
    print("| agent | d | games | old win % | new win % | gap | gap SE |")
    print("|---|---|---|---|---|---|---|")
    for (agent, diff) in sorted(rows):
        p = rows[(agent, diff)]
        n = len(p)
        nw = sum(x for x, _ in p) / n
        ow = sum(y for _, y in p) / n
        d = [x - y for x, y in p]
        mean = sum(d) / n
        var = sum((x - mean) ** 2 for x in d) / max(1, n - 1)
        print(f"| {agent} | {diff} | {n} | {100 * ow:.1f} | {100 * nw:.1f} | {100 * mean:+.1f} | {100 * math.sqrt(var / n):.1f} |")
    print()
    print("| agent | side | games | ms/decision | decisions/game | rounds/game |")
    print("|---|---|---|---|---|---|")
    for agent in sorted(set(ncost) | set(ocost)):
        for side, cost in (("old", ocost), ("new", ncost)):
            ms, dec, games, rounds = cost.get(agent, [0.0, 0, 0, 0])
            if games:
                print(f"| {agent} | {side} | {games} | {ms / max(1, dec):.2f} | {dec / games:.0f} | {rounds / games:.2f} |")


if __name__ == "__main__":
    main()
