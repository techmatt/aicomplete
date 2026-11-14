"""231: a screen's arms against a base on the same seeds: win %, the paired gap and its SE, rounds, project resolutions
(small / big by effects at content/projects.json), per agent and difficulty.

python arms.py SCREEN BASE [BASE2]
SCREEN, BASE: a games.jsonl with its header, or OUT.jsonl=SPEC.json (a farm's out file). Variants are PAIR_ARM
(bp_P) or PAIR alone (the base arm). A game pairs on (pair, difficulty, agent, seed).
"""
import json
import math
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BIG = {p["id"] for p in json.loads((ROOT / "content" / "projects.json").read_text(encoding="utf-8"))
       if len(p["effects"]) > 1}


def load(arg):
    spec = None
    if "=" in arg:
        arg, sp = arg.split("=", 1)
        spec = json.loads(open(sp, encoding="utf-8").read())
    out = []
    with open(arg, encoding="utf-8") as f:
        for line in f:
            g = json.loads(line)
            if "spec" in g:
                spec = g["spec"]
                continue
            if "cell" not in g:
                continue
            cells = [(v["name"], d, a) for v in spec["variants"] for d in spec["difficulties"] for a in spec["agents"]]
            v, d, a = cells[g["cell"]]
            pair, _, arm = v.partition("_")
            c = g["counters"]
            big = sum(n for k, n in c.items() if k.startswith("adversary.project.") and k[18:] in BIG)
            res = c.get("adversary.projects_resolved", 0)
            out.append({"arm": arm, "pair": pair, "d": d, "agent": a.split(":")[0], "seed": g["seed"],
                        "won": 1 if g["won"] else 0, "rounds": g["rounds"], "res": res, "big": big})
    return out


def main():
    screen = load(sys.argv[1])
    base = {(g["pair"], g["d"], g["agent"], g["seed"]): g for g in load(sys.argv[2])}
    by = defaultdict(list)
    for g in screen:
        by[(g["arm"] or "base", g["agent"], g["d"])].append(g)
    print("| arm | agent | d | games | win % | gap | ±SE | rounds | resolved | small | big |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for key in sorted(by):
        gs = by[key]
        n = len(gs)
        diffs = [g["won"] - base[(g["pair"], g["d"], g["agent"], g["seed"])]["won"] for g in gs
                 if (g["pair"], g["d"], g["agent"], g["seed"]) in base]
        m = sum(diffs) / len(diffs) if diffs else 0
        var = sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)
        se = math.sqrt(var / max(1, len(diffs)))
        res = sum(g["res"] for g in gs) / n
        big = sum(g["big"] for g in gs) / n
        print(f"| {key[0]} | {key[1]} | {key[2]} | {n} | {100 * sum(g['won'] for g in gs) / n:.1f} | {100 * m:+.1f} | "
              f"{100 * se:.1f} | {sum(g['rounds'] for g in gs) / n:.2f} | {res:.2f} | {res - big:.2f} | {big:.2f} |")


if __name__ == "__main__":
    main()
