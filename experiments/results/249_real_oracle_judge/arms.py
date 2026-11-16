"""249's rank check: 248's arms a, b, c (board.std_off) on the old judge (248's kept probe), the shipped judge
(r249-armsjudge, the same seeds) and the real oracle (r249-arms, 248's oracle spot-check seeds), with 248's full
oracle (its kept p248-oracle read) on those. Each arm's win % by level and its gap to arm a, paired by seed and pair.

    python experiments/results/249_real_oracle_judge/arms.py OLD_PROBE.out NEW_JUDGE_DIR REAL_DIR FULL_DIR [--out F]

OLD_PROBE.out: 248's farm out file (p248.json's cells); NEW_JUDGE_DIR: a games.jsonl with its header; REAL_DIR and
FULL_DIR: experiment outputs.
"""
import json
import math
import os
import sys
from collections import defaultdict

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))


def cells_of(spec):
    return [(v["name"], d, a) for _ in spec["configs"] for v in spec["variants"] for d in spec["difficulties"]
            for a in spec["agents"]]


def load(path, spec=None):
    lines = open(path, encoding="utf-8").read().splitlines()
    if spec is None:
        spec = json.loads(lines[0])["spec"]
        lines = lines[1:]
    cells = cells_of(spec)
    out = {}
    for line in lines:
        g = json.loads(line)
        if "format" in g:
            continue
        v, d, a = cells[g["cell"]]
        arm, pair = v.split("|")
        out[(a, arm, pair, d, g["seed"])] = g["won"]
    return out


def table(name, games, agent, seeds=None):
    rows = []
    keys = [k for k in games if k[0] == agent and (seeds is None or int(k[4]) in seeds)]
    levels = sorted({k[3] for k in keys})
    for arm in ["a", "b", "c"]:
        cells = []
        for lv in levels + ["all"]:
            ks = [k for k in keys if k[1] == arm and (lv == "all" or k[3] == lv)]
            w = sum(games[k] for k in ks)
            cells.append(f"{100 * w / len(ks):.1f}" if ks else "-")
        # the gap to arm a on the same seed, pair and level
        ds = []
        for k in keys:
            if k[1] != arm:
                continue
            ka = (k[0], "a") + k[2:]
            if ka in games:
                ds.append(int(games[k]) - int(games[ka]))
        if arm != "a" and ds:
            m = sum(ds) / len(ds)
            se = math.sqrt(sum((x - m) ** 2 for x in ds) / (len(ds) - 1) / len(ds))
            gap = f"{100 * m:+.1f} ± {100 * se:.1f}"
        else:
            gap = ""
        rows.append(f"| {name} | {arm} | " + " | ".join(cells) + f" | {gap} | {len([k for k in keys if k[1] == arm])} |")
    return rows, levels


def main():
    old_p, new_d, real_d, full_d = sys.argv[1:5]
    p248 = json.load(open(os.path.join(ROOT, "experiments/farm/p248.json"), encoding="utf-8"))
    old = load(old_p, p248)
    new = load(new_d + "/games.jsonl")
    real = load(real_d + "/games.jsonl")
    full = load(full_d + "/games.jsonl")
    spot = set(range(48376, 48380))
    out = ["## On 248's probe seeds (48376-48471, 288 games a level and arm)", "",
           "| agent | arm | d0 | d1 | d2 | d3 | all | gap to a ± SE | games |", "|---|---|---|---|---|---|---|---|---|"]
    oj = next(a for a in {k[0] for k in old})
    nj = next(a for a in {k[0] for k in new})
    out += table("old judge", old, oj)[0] + table("shipped judge", new, nj)[0]
    out += ["", "## On 248's oracle spot-check seeds (48376-48379, 12 games a level and arm)", "",
            "| agent | arm | d0 | d1 | d2 | d3 | all | gap to a ± SE | games |", "|---|---|---|---|---|---|---|---|---|"]
    out += table("old judge", old, oj, spot)[0] + table("shipped judge", new, nj, spot)[0]
    ra = next(a for a in {k[0] for k in real})
    out += table("real oracle", real, ra)[0]
    fa = next(a for a in {k[0] for k in full} if "tries" in a)
    out += table("full oracle", full, fa, spot)[0]
    text = "\n".join(out) + "\n"
    if "--out" in sys.argv:
        open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8", newline="\n").write(text)
    print(text)


if __name__ == "__main__":
    main()
