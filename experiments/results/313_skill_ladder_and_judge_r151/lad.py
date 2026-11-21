"""313: the skill ladder table: win %, its SE and mean rounds (won / all) per agent and difficulty, from farmed
games.jsonl files (each with its header). Paired columns: each oracle's gap over the judge on the seeds both played.

    python lad.py FILE [FILE ...] [--seeds LO-HI]
"""
import json
import math
import sys
from collections import defaultdict

SHORT = {"greedy": "greedy", "strategist": "judge"}


def short(agent):
    if agent in SHORT:
        return SHORT[agent]
    if agent.startswith("oracle:") and "worlds=" in agent:
        return "real oracle"
    if agent.startswith("oracle:"):
        return "full oracle"
    return agent


def cells(spec):
    out = []
    variants = spec.get("variants") or [{"name": "base"}]
    for v in variants:
        for d in spec["difficulties"]:
            for a in spec["agents"]:
                out.append((v["name"], d, a))
    return out


def load(paths, lo=None, hi=None):
    games = {}  # (agent, d, pair, seed) -> game
    for p in paths:
        cs = None
        # FILE=SPEC: a farmed file whose range didn't start at the spec's first seed has no header (283).
        if "=" in p:
            p, sp = p.split("=", 1)
            with open(sp, encoding="utf-8") as f:
                cs = cells(json.load(f))
        for line in open(p, encoding="utf-8"):
            g = json.loads(line)
            if "format" in g:
                spec = g["spec"]
                cs = cells(spec)
                continue
            seed = int(g["seed"])
            if lo is not None and not lo <= seed <= hi:
                continue
            pair, d, a = cs[g["cell"]]
            assert d == g["difficulty"], (p, g["cell"], d, g["difficulty"])
            games[(short(a), d, pair, seed)] = g
    return games


def main():
    args = sys.argv[1:]
    lo = hi = None
    if "--seeds" in args:
        i = args.index("--seeds")
        lo, hi = map(int, args[i + 1].split("-"))
        del args[i:i + 2]
    games = load(args, lo, hi)
    agents = ["greedy", "judge", "real oracle", "full oracle"]
    agents = [a for a in agents if any(k[0] == a for k in games)] + sorted({k[0] for k in games} - set(agents))
    print("| level | agent | n | win % | SE | rounds (all) | rounds (won) | gap to judge, paired | SE | n paired |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for d in sorted({k[1] for k in games}):
        for a in agents:
            gs = [(k, g) for k, g in games.items() if k[0] == a and k[1] == d]
            if not gs:
                continue
            n = len(gs)
            w = sum(g["won"] for _, g in gs) / n
            se = math.sqrt(w * (1 - w) / n)
            r = sum(g["rounds"] for _, g in gs) / n
            won = [g["rounds"] for _, g in gs if g["won"]]
            rw = sum(won) / len(won) if won else float("nan")
            diffs = []
            for k, g in gs:
                j = games.get(("judge", d, k[2], k[3]))
                if j is not None:
                    diffs.append(int(g["won"]) - int(j["won"]))
            if diffs and a != "judge":
                m = sum(diffs) / len(diffs)
                sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1) / len(diffs))
                gap = f"{100 * m:+.1f} | {100 * sd:.1f} | {len(diffs)}"
            else:
                gap = "- | - | -"
            print(f"| d{d} | {a} | {n} | {100 * w:.1f} | {100 * se:.1f} | {r:.2f} | {rw:.2f} | {gap} |")


if __name__ == "__main__":
    main()
