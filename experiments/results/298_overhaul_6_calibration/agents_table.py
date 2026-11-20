"""298: wins and rounds by agent and level from a farmed multi-agent spec, the variants pooled, with each agent's
paired gap to the first agent (same seed, variant and level).

    python agents_table.py SPEC.json FILE
"""
import collections
import json
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "scripts"))
from screens import cells_of  # noqa: E402


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    cells = cells_of(spec)
    agents = spec["agents"]
    res = {}
    for line in open(sys.argv[2], encoding="utf-8"):
        g = json.loads(line)
        if "cell" not in g:
            continue
        c = cells[g["cell"]]
        res[(c["agent"], c["variant"], c["difficulty"], g["seed"])] = (g["won"], g["rounds"])
    levels = spec["difficulties"]
    print("| Agent | " + " | ".join(f"d{d}" for d in levels) + " | pooled | paired gap to the judge |")
    print("|---|" + "---|" * (len(levels) + 2))
    base = agents[0]
    for a in agents:
        cols, allw, diffs = [], [], []
        for d in levels:
            ws = [v for k, v in res.items() if k[0] == a and k[2] == d]
            cols.append(f"{100 * sum(w for w, _ in ws) / len(ws):.0f}% ({sum(r for _, r in ws) / len(ws):.1f})")
            allw += [w for w, _ in ws]
        for k, (w, _) in res.items():
            if k[0] == a and (base,) + k[1:] in res:
                diffs.append(int(w) - int(res[(base,) + k[1:]][0]))
        n = len(diffs)
        mean = sum(diffs) / n
        se = math.sqrt(sum((x - mean) ** 2 for x in diffs) / (n - 1) / n) if n > 1 else 0
        print(f"| `{a}` | " + " | ".join(cols) + f" | {100 * sum(allw) / len(allw):.1f}% | {100 * mean:+.1f} ± {100 * se:.1f} |")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    main()
