"""313: split a farmed oracle file by how the judge did on the same pair, level and seed, so `aic-sim agree` ranks the
judge's disagreements where they cost wins (judge lost) against where they didn't (judge won).

    python split.py ORACLE.jsonl JUDGE.jsonl OUT_PREFIX   -> OUT_PREFIX-lost.jsonl, OUT_PREFIX-won.jsonl
"""
import json
import sys


def cells(spec):
    return [(v["name"], d, a) for v in spec["variants"] for d in spec["difficulties"] for a in spec["agents"]]


def main():
    orc, judge, prefix = sys.argv[1:4]
    jw = {}
    cs = None
    # JUDGE=SPEC: a farmed file whose range didn't start at the spec's first seed has no header (283).
    if "=" in judge:
        judge, sp = judge.split("=", 1)
        with open(sp, encoding="utf-8") as f:
            cs = cells(json.load(f))
    for line in open(judge, encoding="utf-8"):
        g = json.loads(line)
        if "format" in g:
            cs = cells(g["spec"])
            continue
        v, d, a = cs[g["cell"]]
        if a == "strategist":
            jw[(v, d, g["seed"])] = g["won"]
    outs = {k: open(f"{prefix}-{k}.jsonl", "w", encoding="utf-8", newline="\n") for k in ("lost", "won")}
    n = {"lost": 0, "won": 0}
    for line in open(orc, encoding="utf-8"):
        g = json.loads(line)
        if "format" in g:
            cs = cells(g["spec"])
            for f in outs.values():
                f.write(line if line.endswith("\n") else line + "\n")
            continue
        v, d, a = cs[g["cell"]]
        k = jw.get((v, d, g["seed"]))
        if k is None:
            continue
        which = "won" if k else "lost"
        outs[which].write(line if line.endswith("\n") else line + "\n")
        n[which] += 1
    print(n)


if __name__ == "__main__":
    main()
