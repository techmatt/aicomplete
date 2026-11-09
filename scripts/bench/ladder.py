"""The ladder read: a read's win % per difficulty against the old binary's on the same seeds, pooled over its sets and
the three pairs, paired on the same set, pair and seed, with the adversary's standing measures (157's ladder157.py,
promoted in 159).

    python scripts/bench/ladder.py NEW_DIR[,NEW_DIR...] OLD_DIR[,OLD_DIR...] [--measures] [--deck N] [--bands B]
    python scripts/bench/ladder.py --screen DIR[,DIR...] [--measures]   a screen: win % per cell, and measures

Each DIR is an experiment output (games.jsonl, results.json); several DIRs on a side are a read's sets, in the same
order on both sides (set a, b, c of `bench/read_specs.py read`). Each side is labeled by its games' rules revision
(`r69 new` / `r69 old` when both share one). The table, per difficulty: the new win % and its SE, the old, the paired
gap and its SE, each set's new win %, the band and whether the new rate is in it. `--bands` gives the band of each
difficulty from d0 up, `lo-hi` ','-separated (default `90-100,70-80,55-65,40-50`, 157's prompt); a difficulty with no
band prints `-`. `--screen` reads a screen's cells: a variant `PAIR_LEVER` is lever LEVER (no suffix: `base`), with
its gap to base paired on the same pair and seed. `--measures` adds, per difficulty and side, a game's means: nodes
lost to corruption, adversary ambassadors placed (created or rallied), corruptions, compute made, and the project deck
(amendment 6618): projects revealed and resolved as a share of the deck (`--deck`, 20), distinct projects revealed,
and reshuffles.
"""
import argparse
import json
import math
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import gap, stream  # noqa: E402

BANDS = "90-100,70-80,55-65,40-50"  # 157's prompt
DECK = 20


def parse_bands(text):
    """`90-100,70-80` -> {0: (90, 100), 1: (70, 80)}."""
    out = {}
    for d, b in enumerate(text.split(",")):
        lo, hi = b.split("-")
        out[d] = (int(lo), int(hi))
    return out


def load(d):
    """(cell, game) for each game of DIR, in file order."""
    with open(f"{d}/results.json", encoding="utf-8") as f:
        cells = {c["cell"]: c for c in json.load(f)["cells"]}
    lines = stream(d)
    next(lines)
    for g in lines:
        yield cells[g["cell"]], g


def rev(d):
    """`rNN`: the rules revision in DIR's games.jsonl header."""
    return f"r{next(stream(d)).get('rules_revision', '?')}"


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
    """(old win share over the common keys, mean gap in points, its paired SE in points); NaN with no common key."""
    common = [k for k in n if k in o]
    diffs = [n[k] - o[k] for k in common]
    if not diffs:
        return float("nan"), float("nan"), float("nan")
    po = sum(o[k] for k in common) / len(common)
    if len(diffs) == 1:
        return po, 100 * diffs[0], 0.0
    m, se, _ = gap(diffs)
    return po, m, se


def screen(dirs, with_measures):
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
        print(f"| {lever} | {agent} | d{diff} | {100 * p:.1f} | {se:.1f} | {m:+.1f} | {gse:.1f} | {len(n)} |")
    if with_measures:
        print()
        print(mean_table(ms))


def ladder(new_dirs, old_dirs, bands, with_measures):
    new, old = defaultdict(dict), defaultdict(dict)
    per_set = defaultdict(lambda: defaultdict(list))
    nl, ol = rev(new_dirs[0]), rev(old_dirs[0])
    if nl == ol:
        nl, ol = nl + " new", ol + " old"
    ms = {nl: defaultdict(list), ol: defaultdict(list)}
    for side, dirs in (("new", new_dirs), ("old", old_dirs)):
        for i, d in enumerate(dirs):
            for c, g in load(d):
                diff, v = c["difficulty"], c["variant"]
                won = 1 if g["won"] else 0
                key = (i, v, g["seed"])
                if side == "new":
                    new[diff][key] = won
                    per_set[diff][i].append(won)
                    ms[nl][diff].append(measures(g))
                else:
                    old[diff][key] = won
                    ms[ol][diff].append(measures(g))
    print(f"| d | {nl} win % | ±SE | {ol} win % | gap | ±SE (paired) | sets ({nl}) | band | verdict |")
    print("|---|---|---|---|---|---|---|---|---|")
    for diff in sorted(new):
        n = new[diff]
        p = sum(n.values()) / len(n)
        se = math.sqrt(p * (1 - p) / len(n)) * 100
        po, m, gse = paired(n, old.get(diff, {}))
        sets = " / ".join(f"{100 * sum(x) / len(x):.1f}" for _, x in sorted(per_set[diff].items()))
        if diff in bands:
            lo, hi = bands[diff]
            band, ok = f"{lo}-{hi}", "met" if lo <= 100 * p <= hi else "**unmet**"
        else:
            band, ok = "-", "-"
        print(f"| d{diff} | **{100 * p:.1f}** | {se:.1f} | {100 * po:.1f} | {m:+.1f} | {gse:.1f} | {sets} | {band} | {ok} | ({len(n)} games)")
    if with_measures:
        print()
        print(mean_table(ms))


def main():
    global DECK
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("dirs", nargs="*", help="NEW_DIRS OLD_DIRS, each ','-separated")
    ap.add_argument("--screen", metavar="DIRS", help="read a screen instead: its output dirs, ','-separated")
    ap.add_argument("--measures", action="store_true", help="add the adversary's measures, per difficulty and side")
    ap.add_argument("--deck", type=int, default=DECK, help="the project deck's size (default 20)")
    ap.add_argument("--bands", default=BANDS, help=f"each difficulty's band from d0, lo-hi ','-separated (default {BANDS})")
    a = ap.parse_args()
    DECK = a.deck
    if a.screen:
        screen(a.screen.split(","), a.measures)
        return
    if len(a.dirs) != 2:
        ap.error("give NEW_DIRS OLD_DIRS, or --screen DIRS")
    ladder(a.dirs[0].split(","), a.dirs[1].split(","), parse_bands(a.bands), a.measures)


if __name__ == "__main__":
    main()
