"""156: a screen or read by lever and agent: win % by level, cities liberated, d3's loss kinds.

python screen156.py DIR[,DIR...]

The lever is the variant name past its pair (bp_inc1 -> inc1; a bare pair is base).
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def rows(dirs):
    out = defaultdict(list)
    for d in dirs.split(","):
        r = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in r["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                lever = c["variant"].split("_", 1)[1] if "_" in c["variant"] else "base"
                out[(lever, c["agent"], c["difficulty"])].append(g)
    return out


if __name__ == "__main__":
    o = rows(sys.argv[1])
    keys = sorted({k[:2] for k in o})
    diffs = sorted({k[2] for k in o})
    print("| Lever | Agent | " + " | ".join(f"d{d} (SE)" for d in diffs) + " | Cities liberated (each level) | Losses at the top level: cap / Discord / nodes |")
    print("|---|---|" + "---|" * len(diffs) + "---|---|")
    for k in keys:
        cs, libs = [], []
        for d in diffs:
            gs = o.get(k + (d,), [])
            n = max(len(gs), 1)
            p = sum(g["won"] for g in gs) / n
            cs.append(f"{100 * p:.1f} ({100 * math.sqrt(p * (1 - p) / n):.1f})")
            libs.append(f"{sum(g['counters'].get('board.liberated.city', 0) for g in gs) / n:.2f}")
        gs = o.get(k + (diffs[-1],), [])
        loss = [sum(g["reason"] == r for g in gs) for r in ("deck_empty", "discord_deck", "node")]
        print(f"| {k[0]} | {k[1]} | " + " | ".join(cs) + f" | {' / '.join(libs)} | {' / '.join(map(str, loss))} |")
