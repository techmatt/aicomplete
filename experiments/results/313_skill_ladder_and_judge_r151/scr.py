"""313: an agent-arm screen: each agent's win % per difficulty and its gap to the base agent (the spec's first, or
--base SPEC), paired on pair, difficulty and seed; pooled d2-d3 and all levels. Several FILEs pool (same spec).

    python scr.py FILE [FILE ...] [--base SPEC]
"""
import json
import math
import sys
from collections import defaultdict


def cells(spec):
    out = []
    for v in spec.get("variants") or [{"name": "base"}]:
        for d in spec["difficulties"]:
            for a in spec["agents"]:
                out.append((v["name"], d, a))
    return out


def paired(xs):
    n = len(xs)
    if n < 2:
        return float("nan"), float("nan")
    m = sum(xs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1) / n)
    return 100 * m, 100 * sd


def main():
    args = sys.argv[1:]
    base = None
    if "--base" in args:
        i = args.index("--base")
        base = args[i + 1]
        del args[i:i + 2]
    won = {}
    order = []
    for p in args:
        cs = None
        # FILE=SPEC: a farmed file whose range didn't start at the spec's first seed has no header (283).
        if "=" in p:
            p, sp = p.split("=", 1)
            with open(sp, encoding="utf-8") as f:
                spec = json.load(f)
            cs = cells(spec)
            order += [a for a in spec["agents"] if a not in order]
        for line in open(p, encoding="utf-8"):
            g = json.loads(line)
            if "format" in g:
                cs = cells(g["spec"])
                for a in g["spec"]["agents"]:
                    if a not in order:
                        order.append(a)
                continue
            pair, d, a = cs[g["cell"]]
            won[(a, d, pair, int(g["seed"]))] = int(g["won"])
    base = base or order[0]
    levels = sorted({k[1] for k in won})
    groups = [(f"d{d}", (d,)) for d in levels] + [("d2-d3", (2, 3)), ("all", tuple(levels))]
    print("agent | " + " | ".join(f"{name} win / gap (SE)" for name, _ in groups))
    for a in order:
        row = []
        for name, ds in groups:
            mine = [(k, v) for k, v in won.items() if k[0] == a and k[1] in ds]
            if not mine:
                row.append("-")
                continue
            w = 100 * sum(v for _, v in mine) / len(mine)
            diffs = [v - won[(base,) + k[1:]] for k, v in mine if (base,) + k[1:] in won]
            m, se = paired(diffs)
            row.append(f"{w:.1f} / {m:+.1f} ({se:.1f})" if a != base else f"{w:.1f} (n {len(mine)})")
        print(f"{a} | " + " | ".join(row))


if __name__ == "__main__":
    main()
