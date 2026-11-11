"""174: an approximation's strength and speed against base on the same seeds, from games.csv.

    python experiments/results/174_speed/paired174.py BASE_DIR[,DIR...] NEW_DIR[,DIR...] [--bands 90-100,70-80,55-65,40-50]
        [--agent PREFIX]   (only games whose agent string starts with PREFIX, on both sides)

Each side is one or more experiment outputs (a read's sets). Base has one agent a cell; new may have several (the
arms, e.g. `strategist:discards=1` and `=0`): each new agent is paired with base's game on the same pair, level and
seed. Per level and arm: base win %, the arm's win %, its SE, the paired gap and its SE, and the band; then summed
agent ms a decision and a game on each side (the games' own `ms`, not wall time).
"""
import csv
import math
import os
import sys
from collections import defaultdict


def load(dirs):
    rows = []
    for d in dirs.split(","):
        with open(os.path.join(d, "games.csv"), encoding="utf-8") as f:
            rows += list(csv.DictReader(f))
    return rows


def short(agent):
    if agent.startswith("engine:") and "strategy=1" in agent.split(","):
        rest = [p for p in agent[len("engine:"):].split(",") if p.startswith("discards=")]
        return "strategist" + (":" + rest[0] if rest else "")
    return agent


def main():
    args = sys.argv[1:]
    bands = "90-100,70-80,55-65,40-50"
    if "--bands" in args:
        i = args.index("--bands")
        bands = args[i + 1]
        del args[i:i + 2]
    only = None
    if "--agent" in args:
        i = args.index("--agent")
        only = args[i + 1]
        del args[i:i + 2]
    band = [tuple(int(x) for x in b.split("-")) for b in bands.split(",")]
    base, new = load(args[0]), load(args[1])
    if only:
        base = [r for r in base if r["agent"].startswith(only)]
        new = [r for r in new if r["agent"].startswith(only)]
    b = {(r["variant"], r["difficulty"], r["seed"]): r for r in base}
    arms = defaultdict(list)
    for r in new:
        arms[short(r["agent"])].append(r)
    levels = sorted({r["difficulty"] for r in base}, key=int)
    print("| Arm | d | Games | Base win % | Arm win % | Arm SE | Gap | Gap SE | Band | In band |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for arm, rows in sorted(arms.items()):
        for lv in levels:
            pairs = [(int(b[(r["variant"], lv, r["seed"])]["won"]), int(r["won"]))
                     for r in rows if r["difficulty"] == lv and (r["variant"], lv, r["seed"]) in b]
            n = len(pairs)
            if n == 0:
                continue
            pb = sum(x for x, _ in pairs) / n
            pn = sum(y for _, y in pairs) / n
            se = math.sqrt(pn * (1 - pn) / n)
            d = [y - x for x, y in pairs]
            md = sum(d) / n
            sd = math.sqrt(sum((x - md) ** 2 for x in d) / max(1, n - 1) / n)
            bd = band[int(lv)] if int(lv) < len(band) else None
            ok = "-" if bd is None else ("yes" if bd[0] <= 100 * pn <= bd[1] else "no")
            bs = "-" if bd is None else f"{bd[0]}-{bd[1]}"
            print(f"| {arm} | {lv} | {n} | {100 * pb:.1f} | {100 * pn:.1f} | {100 * se:.1f} | {100 * md:+.1f} | {100 * sd:.1f} | {bs} | {ok} |")
    print()
    print("| Side | Games | Decisions | Summed s | ms/decision | ms/game | Rounds |")
    print("|---|---|---|---|---|---|---|")
    sides = [("base " + short(base[0]["agent"]), base)] + sorted(arms.items())
    for name, rows in sides:
        ms = sum(float(r["ms"]) for r in rows)
        dec = sum(int(r["decisions"]) for r in rows)
        rounds = sum(int(r["rounds"]) for r in rows) / len(rows)
        print(f"| {name} | {len(rows)} | {dec} | {ms / 1000:.0f} | {ms / dec:.2f} | {ms / len(rows):.0f} | {rounds:.2f} |")


if __name__ == "__main__":
    main()
