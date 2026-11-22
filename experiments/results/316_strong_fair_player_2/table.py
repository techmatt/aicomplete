"""316: set C's table: win % per row and level (SE, games), each row's gap to J128 (`real:budget=128`, the
yardstick) paired on (pair, level, seed) at d4 and at d3-d4, the engine rows' doubling steps paired, and native
seconds a game from the broker's shard logs (`farm: cell N seed S ms`; 314's reader).

    python experiments/results/316_strong_fair_player_2/table.py [OUT_DIR] [--jobs JOBS_DIR] [--no-shards]

Reads OUT_DIR/f316-*.jsonl, each with its spec header, and (unless --no-shards) every finished shard of an f316 job
in the broker's jobs folder, so a job cancelled at the cap still counts its finished seeds; a game is keyed on (row,
level, pair, seed), so the two sources never count one twice. A partial row shows '-' where a level did not run.
"""
import glob
import json
import math
import os
import re
import sys
from collections import defaultdict

ENGINE = "oracle:strategy=1,worlds=4,alts=3,answers=1,truth=1"
FULL = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"
NAMES = {
    "greedy": "greedy", "strategist": "judge", FULL: "full oracle",
    "real:budget=128": "J128: judge eval, 128 lines (yardstick)",
    "real:budget=256": "J256: judge eval, 256 lines",
    ENGINE + ",budget=128": "E128: engine eval, 128 lines",
    ENGINE + ",budget=256": "E256: engine eval, 256 lines",
    ENGINE + ",budget=512": "E512: engine eval, 512 lines",
    ENGINE.replace("worlds=4", "worlds=8") + ",budget=512": "E8w: engine eval, 8 worlds, 512 lines",
}
ORDER = ["greedy", "judge", "J128: judge eval, 128 lines (yardstick)", "J256: judge eval, 256 lines",
         "E128: engine eval, 128 lines", "E256: engine eval, 256 lines", "E512: engine eval, 512 lines",
         "E8w: engine eval, 8 worlds, 512 lines", "full oracle"]
BASE = ORDER[2]
STEPS = [(ORDER[4], BASE), (ORDER[5], ORDER[4]), (ORDER[3], BASE), (ORDER[6], ORDER[5]), (ORDER[7], ORDER[5])]


def name(agent):
    return NAMES.get(agent, agent)


def load(path, cells=None):
    """The games in a farmed file; `cells` from its job's header where the file has none (a shard after the first)."""
    games = []
    for line in open(path, encoding="utf-8"):
        g = json.loads(line)
        if "format" in g:
            s = g["spec"]
            cells = [(v["name"], d, a) for v in s["variants"] for d in s["difficulties"] for a in s["agents"]]
            continue
        pair, d, a = cells[g["cell"]]
        assert d == g["difficulty"]
        games.append((name(a), d, pair, int(g["seed"]), bool(g["won"])))
    return games


def costs(jobs_dir, files):
    """Native seconds a game by row, from each job whose out file is one of `files`."""
    secs = defaultdict(list)
    want = {os.path.normcase(os.path.abspath(f)) for f in files}
    for jf in glob.glob(os.path.join(jobs_dir, "*", "job.json")):
        try:
            j = json.load(open(jf, encoding="utf-8"))
        except (OSError, ValueError):
            continue
        if os.path.normcase(os.path.abspath(j.get("out", ""))) not in want:
            continue
        base = os.path.basename(j["args"][0])
        base = base if base.endswith(".json") else base + ".json"
        paths = [os.path.join("experiments", d, base) for d in ("farm", os.path.join("archive", "farm"))]
        found = [p for p in paths if os.path.exists(p)]
        log = os.path.join(os.path.dirname(jf), "main.log")
        if not found or not os.path.exists(log):
            continue
        s = json.load(open(found[0], encoding="utf-8"))
        cells = [(v["name"], d, a) for v in s["variants"] for d in s["difficulties"] for a in s["agents"]]
        for line in open(log, encoding="utf-8", errors="replace"):
            m = re.match(r"farm: cell (\d+) seed (\d+) ([\d.]+) ms", line)
            if m:
                _, d, a = cells[int(m[1])]
                secs[(name(a), d)].append(float(m[3]) / 1000)
    return secs


def paired(by, a, b, levels):
    diffs = [int(v) - int(by[(b,) + k[1:]]) for k, v in by.items()
             if k[0] == a and k[1] in levels and (b,) + k[1:] in by]
    if not diffs:
        return "-"
    n = len(diffs)
    m = sum(diffs) / n
    s = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, n - 1) / n)
    return f"{100 * m:+.1f} ± {100 * s:.1f} ({n})"


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    args = sys.argv[1:]
    jobs = r"C:\Code\aicomplete-control\.conductor\remote\jobs"
    if "--jobs" in args:
        i = args.index("--jobs")
        jobs = args[i + 1]
        del args[i:i + 2]
    pos = [a for a in args if not a.startswith("--")]
    out = pos[0] if pos else os.path.join("out", "316_strong_fair_player_2")
    files = sorted(glob.glob(os.path.join(out, "f316-*.jsonl")))
    shards = []  # (path, cells)
    if "--no-shards" not in args:
        # Finished 2-seed shards of jobs cut short (`farm-seeds --cancel` at the cap): a job's first shard alone holds
        # the spec header, which every shard of that job shares.
        for jd in sorted(glob.glob(os.path.join(jobs, "*", "shards"))):
            outs = sorted(glob.glob(os.path.join(jd, "*.out")))
            cells = None
            for p in outs:
                with open(p, encoding="utf-8") as f:
                    head = json.loads(f.readline() or "{}")
                if "format" in head:
                    s = head["spec"]
                    if s["name"].startswith("f316-"):
                        cells = [(v["name"], d, a) for v in s["variants"] for d in s["difficulties"] for a in s["agents"]]
                    break
            if cells:
                shards += [(p, cells) for p in outs]
    by = {}
    for p, cells in shards + [(f, None) for f in files]:
        for a, d, pair, seed, won in load(p, cells):
            by[(a, d, pair, seed)] = won
    rows = [r for r in ORDER if any(k[0] == r for k in by)]
    secs = costs(jobs, files)
    # A one-agent spec's native shards: the shard's time over its games (who.json), for jobs whose main.log is gone.
    shard_secs = defaultdict(list)
    for p, cells in shards:
        who = p[:-len(".out")] + ".who.json"
        if not os.path.exists(who):
            continue
        w = json.load(open(who, encoding="utf-8"))
        g = load(p, cells)
        if w.get("engine") == "native" and g and len({x[0] for x in g}) == 1:
            shard_secs[(g[0][0], g[0][1])] += [w["ms"] / 1000 / len(g)] * len(g)
    for k, v in shard_secs.items():
        if len(v) > len(secs.get(k, [])):
            secs[k] = v
    levels = sorted({k[1] for k in by})
    print("| row | " + " | ".join(f"d{d}" for d in levels) + " | d4 gap to J128 | d3-d4 gap to J128 | native s a game |")
    print("|---|" + "---|" * len(levels) + "---|---|---|")
    for r in rows:
        cells = []
        for d in levels:
            ws = [v for k, v in by.items() if k[0] == r and k[1] == d]
            if not ws:
                cells.append("-")
                continue
            w = sum(ws) / len(ws)
            cells.append(f"{100 * w:.0f} ± {100 * math.sqrt(w * (1 - w) / len(ws)):.0f} ({len(ws)})")
        g4 = paired(by, r, BASE, [4]) if r != BASE else "-"
        g34 = paired(by, r, BASE, [3, 4]) if r != BASE else "-"
        t = [x for (a, _), v in secs.items() if a == r for x in v]
        cost = f"{sum(t) / len(t):.0f} ({len(t)})" if t else "-"
        print(f"| {r} | " + " | ".join(cells) + f" | {g4} | {g34} | {cost} |")
    print("\nSteps, paired on (pair, level, seed):\n")
    print("| step | d4 | d3-d4 | d0-d4 |")
    print("|---|---|---|---|")
    for a, b in STEPS:
        if any(k[0] == a for k in by) and any(k[0] == b for k in by):
            print(f"| {a.split(':')[0]} − {b.split(':')[0]} | {paired(by, a, b, [4])} | {paired(by, a, b, [3, 4])} "
                  f"| {paired(by, a, b, [0, 1, 2, 3, 4])} |")
    t4 = defaultdict(list)
    for (a, d), v in secs.items():
        t4[(a, "d3-d4" if d >= 3 else "d0-d2")] += v
    print("\nNative s a game by level group: " + "; ".join(
        f"{a.split(':')[0]} {g} {sum(v) / len(v):.0f} ({len(v)})" for (a, g), v in sorted(t4.items())))


if __name__ == "__main__":
    main()
