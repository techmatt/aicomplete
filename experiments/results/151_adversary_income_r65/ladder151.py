"""151: the ladder read, pooled over its sets and the three pairs, paired on the same pair and seed, and the
adversary measures r64 against r65.

python ladder151.py NEW_DIR[,NEW_DIR...] OLD_DIR[,OLD_DIR...] [--measures] [--deck N]
python ladder151.py --screen DIR [--measures]     a screen: win % per cell (variant suffix = lever), and measures

Each DIR is an experiment output (games.jsonl, results.json). The ladder: NEW's baseline against OLD's (r64's binary)
per difficulty: the new win %, its SE, the old, the paired gap and its SE, each set's new win %, the band.
--measures adds, per difficulty and side, a game's means: nodes lost to corruption, adversary ambassadors placed
(created by any line, rallied, or the board's), and the project deck (amendment 6618): projects revealed and resolved
as a share of the deck (--deck, 20), distinct projects revealed, and reshuffles.
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
BANDS = {0: (90, 100), 1: (70, 85), 2: (55, 65), 3: (40, 50)}
DECK = 20


def load(d):
    res = json.load(open(f"{d}/results.json", encoding="utf-8"))
    cells = {c["cell"]: c for c in res["cells"]}
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        f.readline()
        for line in f:
            g = json.loads(line)
            c = cells[g["cell"]]
            yield c, g


def measures(g):
    c = g["counters"]
    placed = c.get("adversary.created", 0) + c.get("adversary.rallied", 0)
    revealed = c.get("adversary.projects_revealed", 0)
    distinct = sum(1 for k in c if k.startswith("adversary.revealed."))
    return {
        "corruption nodes lost": c.get("board.node_lost.corruption", 0),
        "ambassadors placed": placed,
        "  of them rallied": c.get("adversary.rallied", 0),
        "corruptions": c.get("adversary.corruptions", 0),
        "compute made": c.get("adversary.compute_made", 0),
        "projects revealed": revealed,
        "revealed, share of deck": revealed / DECK,
        "distinct revealed, share": distinct / DECK,
        "projects resolved": c.get("adversary.projects_resolved", 0),
        "resolved, share of deck": c.get("adversary.projects_resolved", 0) / DECK,
        "project reshuffles": c.get("adversary.project_reshuffles", 0),
        "rounds": g["rounds"],
    }


def mean_table(rows_by_side):
    """rows_by_side: {side: {diff: [measure dicts]}} -> markdown."""
    keys = None
    out = []
    for side, by in rows_by_side.items():
        for diff in sorted(by):
            ms = by[diff]
            keys = keys or list(ms[0].keys())
            out.append((side, diff, {k: sum(m[k] for m in ms) / len(ms) for k in keys}, len(ms)))
    head = "| side | d | games | " + " | ".join(keys) + " |"
    lines = [head, "|" + "---|" * (len(keys) + 3)]
    for side, diff, avg, n in out:
        lines.append(f"| {side} | d{diff} | {n} | " + " | ".join(f"{avg[k]:.2f}" for k in keys) + " |")
    return "\n".join(lines)


def paired(n, o):
    common = [k for k in n if k in o]
    diffs = [n[k] - o[k] for k in common]
    if not diffs:
        return float("nan"), float("nan"), float("nan")
    m = sum(diffs) / len(diffs)
    var = sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)
    po = sum(o[k] for k in common) / len(common)
    return po, m, math.sqrt(var / len(diffs))


def screen(dirs):
    wins = defaultdict(dict)
    ms = defaultdict(lambda: defaultdict(list))
    for i, d in enumerate(dirs):
        for c, g in load(d):
            v = c["variant"]
            lever = v.split("_", 1)[1] if "_" in v else "base"
            pair = v.split("_", 1)[0]
            key = (lever, c["agent"], c["difficulty"])
            wins[key][(i, pair, g["seed"])] = 1 if g["won"] else 0
            ms[f"{lever} {c['agent']}"][c["difficulty"]].append(measures(g))
    print("| lever | agent | d | win % | ±SE | gap vs base | ±SE (paired) | games |")
    print("|---|---|---|---|---|---|---|---|")
    for key in sorted(wins):
        lever, agent, diff = key
        n = wins[key]
        p = sum(n.values()) / len(n)
        se = math.sqrt(p * (1 - p) / len(n)) * 100
        base = wins.get(("base", agent, diff), {})
        _, m, gse = paired(n, base) if lever != "base" else (0, float("nan"), float("nan"))
        print(f"| {lever} | {agent} | d{diff} | {100 * p:.1f} | {se:.1f} | {100 * m:+.1f} | {100 * gse:.1f} | {len(n)} |")
    if "--measures" in sys.argv:
        print()
        print(mean_table(ms))


def main():
    global DECK
    if "--deck" in sys.argv:
        DECK = int(sys.argv[sys.argv.index("--deck") + 1])
    if sys.argv[1] == "--screen":
        screen(sys.argv[2].split(","))
        return
    new_dirs, old_dirs = sys.argv[1].split(","), sys.argv[2].split(",")
    new, old = defaultdict(dict), defaultdict(dict)
    per_set = defaultdict(lambda: defaultdict(list))
    ms = {"r65": defaultdict(list), "r64": defaultdict(list)}
    for side, dirs in (("new", new_dirs), ("old", old_dirs)):
        for i, d in enumerate(dirs):
            for c, g in load(d):
                diff, v = c["difficulty"], c["variant"]
                won = 1 if g["won"] else 0
                key = (i, v, g["seed"])
                if side == "new":
                    new[diff][key] = won
                    per_set[diff][i].append(won)
                    ms["r65"][diff].append(measures(g))
                else:
                    old[diff][key] = won
                    ms["r64"][diff].append(measures(g))
    print("| d | r65 win % | ±SE | r64 win % | gap | ±SE (paired) | sets (r65) | band | verdict |")
    print("|---|---|---|---|---|---|---|---|---|")
    for diff in sorted(new):
        n = new[diff]
        p = sum(n.values()) / len(n)
        se = math.sqrt(p * (1 - p) / len(n)) * 100
        po, m, gse = paired(n, old.get(diff, {}))
        sets = " / ".join(f"{100 * sum(x) / len(x):.1f}" for _, x in sorted(per_set[diff].items()))
        lo, hi = BANDS[diff]
        ok = "met" if lo <= 100 * p <= hi else "**unmet**"
        print(f"| d{diff} | **{100 * p:.1f}** | {se:.1f} | {100 * po:.1f} | {100 * m:+.1f} | {100 * gse:.1f} | {sets} | {lo}-{hi} | {ok} | ({len(n)} games)")
    if "--measures" in sys.argv:
        print()
        print(mean_table(ms))


if __name__ == "__main__":
    main()
