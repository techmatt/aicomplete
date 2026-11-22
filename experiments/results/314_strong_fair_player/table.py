"""314: the decomposition table: win % per row and level (SE), its gap to the real oracle paired on (pair, seed),
and each row's native seconds a game (from the broker's shard logs, `farm: cell N seed S ms`).

    python experiments/results/314_strong_fair_player/table.py [OUT_DIR] [--set A|B] [--jobs JOBS_DIR]

Reads OUT_DIR/f314-<row>-<pair>[-dLH].jsonl (each with its spec header; `--set B`: the strongest player's run,
f314-best-* and f314-bcheap-*) and, for the greedy-vs-judge read,
OUT_DIR/g314.jsonl (paired greedy - judge by level, pooled, and by pair).
"""
import glob
import json
import math
import os
import re
import sys
from collections import defaultdict

NAMES = {
    "greedy": "greedy", "strategist": "judge", "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3": "full oracle",
    "real": "real oracle (r0)", "real:tries=3": "r1: tries=3 (not fair)",
    "oracle:strategy=1,worlds=4,alts=3,budget=32,answers=1,truth=1": "r2: full oracle's eval",
    "real:budget=128": "r3: 4x lines (128 a round)", "real:worlds=8,budget=64": "r4: 8 worlds (64 a round)",
    "real:budget=64": "r5: 2x lines (64 a round)",
}
ORDER = ["greedy", "judge", "real oracle (r0)", "r1: tries=3 (not fair)", "r2: full oracle's eval",
         "r4: 8 worlds (64 a round)", "r5: 2x lines (64 a round)", "r3: 4x lines (128 a round)"]


def name(agent):
    return NAMES.get(agent, agent)


def load(path):
    games, cells = [], None
    for line in open(path, encoding="utf-8"):
        g = json.loads(line)
        if "format" in g:
            s = g["spec"]
            cells = [(v["name"], d, a) for v in s["variants"] for d in s["difficulties"] for a in s["agents"]]
            continue
        pair, d, a = cells[g["cell"]]
        assert d == g["difficulty"]
        games.append((name(a), d, pair, int(g["seed"]), bool(g["won"]), g["rounds"]))
    return games


def costs(jobs_dir, out_dir, files):
    """Native seconds a game by row, from each job of this item whose out file is one of `files`."""
    secs = defaultdict(list)
    want = {os.path.normcase(os.path.abspath(f)) for f in files}
    for jf in glob.glob(os.path.join(jobs_dir, "*", "job.json")):
        try:
            j = json.load(open(jf, encoding="utf-8"))
        except (OSError, ValueError):
            continue
        if os.path.normcase(j.get("out", "")) not in want:
            continue
        base = os.path.basename(j["args"][0])
        base = base if base.endswith(".json") else base + ".json"
        paths = [os.path.join("experiments", d, base) for d in ("farm", os.path.join("archive", "farm"))]
        found = [p for p in paths if os.path.exists(p)]
        if not found:
            continue
        s = json.load(open(found[0], encoding="utf-8"))
        cells = [(v["name"], d, a) for v in s["variants"] for d in s["difficulties"] for a in s["agents"]]
        log = os.path.join(os.path.dirname(jf), "main.log")
        if not os.path.exists(log):
            continue
        for line in open(log, encoding="utf-8", errors="replace"):
            m = re.match(r"farm: cell (\d+) seed (\d+) ([\d.]+) ms", line)
            if m:
                _, d, a = cells[int(m[1])]
                secs[(name(a), d)].append(float(m[3]) / 1000)
    return secs


def se_paired(diffs):
    n = len(diffs)
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, n - 1) / n)
    return m, sd


def main():
    args = sys.argv[1:]
    jobs = r"C:\Code\aicomplete-control\.conductor\remote\jobs"
    if "--jobs" in args:
        i = args.index("--jobs")
        jobs = args[i + 1]
        del args[i:i + 2]
    which = "A"
    if "--set" in args:
        i = args.index("--set")
        which = args[i + 1]
        del args[i:i + 2]
    out = args[0] if args else os.path.join("out", "314_strong_fair_player")
    games, files = [], []
    for p in sorted(glob.glob(os.path.join(out, "f314-*.jsonl"))):
        b = os.path.basename(p).startswith(("f314-best-", "f314-bcheap-"))
        if b == (which == "B"):
            games += load(p)
            files.append(p)
    by = {(a, d, pair, seed): (won, r) for a, d, pair, seed, won, r in games}
    rows = [r for r in ORDER if any(k[0] == r for k in by)] + sorted({k[0] for k in by} - set(ORDER) - {"full oracle"})
    rows += ["full oracle"] if any(k[0] == "full oracle" for k in by) else []
    secs = costs(jobs, out, files)
    levels = sorted({k[1] for k in by})
    print("| row | " + " | ".join(f"d{d}" for d in levels) + " | d3-d4 gap to r0, paired | native s a game |")
    print("|---|" + "---|" * len(levels) + "---|---|")
    for r in rows:
        cells = []
        for d in levels:
            ws = [v[0] for k, v in by.items() if k[0] == r and k[1] == d]
            if not ws:
                cells.append("-")
                continue
            w = sum(ws) / len(ws)
            cells.append(f"{100 * w:.0f} ± {100 * math.sqrt(w * (1 - w) / len(ws)):.0f} ({len(ws)})")
        diffs = [int(v[0]) - int(by[("real oracle (r0)",) + k[1:]][0]) for k, v in by.items()
                 if k[0] == r and k[1] >= 3 and ("real oracle (r0)",) + k[1:] in by]
        gap = "-"
        if diffs and r != "real oracle (r0)":
            m, s = se_paired(diffs)
            gap = f"{100 * m:+.1f} ± {100 * s:.1f} ({len(diffs)})"
        t = [x for (a, _), v in secs.items() if a == r for x in v]
        cost = f"{sum(t) / len(t):.1f} ({len(t)})" if t else "-"
        print(f"| {r} | " + " | ".join(cells) + f" | {gap} | {cost} |")
    gfiles = [p for p in (os.path.join(out, "g314.jsonl"), os.path.join(out, "g314b.jsonl")) if os.path.exists(p)]
    if gfiles and which == "A":
        gj = {}
        for g in gfiles:
            for a, d, pair, seed, won, _ in load(g):
                gj[(a, d, pair, seed)] = won
        print(f"\nGreedy against the judge, paired on (pair, seed), {' + '.join(map(os.path.basename, gfiles))}:\n")
        print("| level | greedy | judge | greedy − judge | SE | n |")
        print("|---|---|---|---|---|---|")
        groups = [(f"d{d}", [d]) for d in sorted({k[1] for k in gj})] + [("all", [0, 1, 2, 3, 4]), ("d1-d4", [1, 2, 3, 4])]
        for label, ds in groups:
            diffs, gw, jw = [], [], []
            for (a, d, pair, seed), won in gj.items():
                if a == "greedy" and d in ds and ("judge", d, pair, seed) in gj:
                    j = gj[("judge", d, pair, seed)]
                    diffs.append(int(won) - int(j))
                    gw.append(won)
                    jw.append(j)
            if not diffs:
                continue
            m, s = se_paired(diffs)
            print(f"| {label} | {100 * sum(gw) / len(gw):.1f} | {100 * sum(jw) / len(jw):.1f} | {100 * m:+.1f} | {100 * s:.1f} | {len(diffs)} |")


if __name__ == "__main__":
    main()
