"""176: a heuristic on and off on the same seeds: win % per agent spec and difficulty, and each "off" spec's paired
gap against its base agent (the spec before the first ':' of an option list, e.g. `strategist`).

    python experiments/results/176_rules_players/onoff176.py DIR [DIR...]
"""
import collections
import json
import math
import os
import sys


def main():
    wins = {}
    for d in sys.argv[1:]:
        with open(os.path.join(d, "games.jsonl"), encoding="utf-8") as f:
            s = json.loads(f.readline())["spec"]
            cells = [(v["name"], diff, a) for _ in s["configs"] for v in s.get("variants", [{"name": "base"}])
                     for diff in s["difficulties"] for a in s["agents"]]
            for line in f:
                g = json.loads(line)
                if "cell" in g:
                    var, diff, agent = cells[g["cell"]]
                    wins[(agent, var, diff, g["seed"])] = 1 if g["won"] else 0
    agents = sorted({k[0] for k in wins}, key=lambda a: (a.split(":")[0], a))
    diffs = sorted({k[2] for k in wins})
    print("| agent | " + " | ".join(f"d{d}" for d in diffs) + " | all |")
    print("|---|" + "---|" * (len(diffs) + 1))
    for a in agents:
        row = []
        for d in diffs + [None]:
            v = [w for k, w in wins.items() if k[0] == a and (d is None or k[2] == d)]
            row.append(f"{100 * sum(v) / max(1, len(v)):.1f}")
        print(f"| {a} | " + " | ".join(row) + " |")
    print()
    print("| off spec | d | games | gap (off - base) | SE |")
    print("|---|---|---|---|---|")
    for a in agents:
        base = a.split(":")[0]
        if a == base or base not in agents:
            continue
        for d in diffs + [None]:
            diff = [w - wins[(base,) + k[1:]] for k, w in wins.items()
                    if k[0] == a and (d is None or k[2] == d) and (base,) + k[1:] in wins]
            n = len(diff)
            if n < 2:
                continue
            m = sum(diff) / n
            sd = math.sqrt(sum((x - m) ** 2 for x in diff) / (n - 1))
            print(f"| {a} | {'all' if d is None else d} | {n} | {100 * m:+.1f} | {100 * sd / math.sqrt(n):.1f} |")


if __name__ == "__main__":
    main()
