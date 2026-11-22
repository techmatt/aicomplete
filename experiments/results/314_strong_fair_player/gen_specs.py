"""314: writes the item's farm specs into experiments/farm/ (moved to experiments/archive/farm/ when done).

Each decomposition row is farmed per board pair on its own seed range (6 seeds a pair, 18 distinct seeds a cell),
and the costly rows per level group, so a 2-seed shard stays short enough to spread over the workers.

    python experiments/results/314_strong_fair_player/gen_specs.py [--best AGENT]

`--best AGENT` writes the strongest fair player's run instead (fresh set B, `best`).
"""
import json
import os
import sys

PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# Fresh seeds (ledger next free 51888 at 313): the greedy-vs-judge read, then the decomposition's set A.
GJ_FIRST, GJ_SEEDS = 51888, 48
A_FIRST = {"bp": 51936, "br": 51942, "pr": 51948}
A_SEEDS = 6

FULL = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"
ROWS = {
    # name: (agents, level groups, what it is)
    "cheap": (["greedy", "strategist", FULL], [[0, 1, 2, 3, 4]], "greedy, the judge and the full oracle"),
    "r0": (["real"], [[0, 1, 2, 3, 4]], "the real oracle (`real`: the judge's eval, 4 worlds, 32 lines a round)"),
    "r1": (["real:tries=3"], [[0, 1, 2, 3, 4]], "step 1: the real oracle with tries=3 (restarts read the true loss: not fair)"),
    "r2": (["oracle:strategy=1,worlds=4,alts=3,budget=32,answers=1,truth=1"], [[0, 1, 2, 3, 4]],
           "step 2: the real oracle's search on the full oracle's eval (engine weights, strategy=1)"),
    "r3": (["real:budget=128"], [[0, 1, 2], [3, 4]], "step 3: 4x the lines a round (about 8 decisions searched, not 2)"),
    "r4": (["real:worlds=8,budget=64"], [[0, 1, 2], [3, 4]], "step 3: 8 worlds a candidate (2x the lines, the same decisions)"),
}


# The screen of the two new search options, d4 alone, on 313's spent ladder seeds (51552 on, 8 a pair).
SCREEN_FIRST, SCREEN_SEEDS = 51552, 8
SCREEN = {
    "base": "real",
    "lone": "real:lone=1",
    "phases": "real:phases=1,budget=64",
    "both": "real:phases=1,lone=1,budget=64",
}


def spec(name, note, pairs, levels, agents, first, games):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"],
            "variants": [{"name": p, "set": PAIRS[p]} for p in pairs], "difficulties": levels,
            "agents": agents, "games": games, "first_seed": first, "records": 0}


# The strongest fair player's run: fresh set B (51954-51969), one spec per pair and level (a 2-seed shard is 2 games), 6 seeds a
# pair bp, 5 br and pr (16 a cell: Matt, message 14463); greedy, the judge and the full oracle beside it.
B_FIRST = {"bp": 51954, "br": 51960, "pr": 51965}
B_SEEDS = {"bp": 6, "br": 5, "pr": 5}
LEVELS = [0, 1, 2, 3, 4]
GJB_FIRST, GJB_SEEDS = 51970, 96


def best(agent, root):
    out = []
    for p in PAIRS:
        n = B_SEEDS[p]
        for d in LEVELS:
            note = (f"314, the strongest fair player ({agent}); pair {p}, d{d}, fresh set B ({n} seeds from "
                    f"{B_FIRST[p]}).")
            out.append((spec(f"f314-best-{p}-d{d}", note, [p], [d], [agent], B_FIRST[p], n), B_FIRST[p], n))
        note = f"314: greedy, the judge and the full oracle beside the strongest fair player; pair {p}, d0-d4, set B."
        out.append((spec(f"f314-bcheap-{p}", note, [p], LEVELS, ["greedy", "strategist", FULL], B_FIRST[p], n),
                    B_FIRST[p], n))
    # g314's 720 games a side read greedy - judge -1.2 +- 1.7: 96 more seeds beside, to settle the order.
    out.append((spec("g314b", f"314: the greedy-vs-judge read's second set, fresh seeds {GJB_FIRST} on, d0-d4, the "
                     "three pairs.", list(PAIRS), LEVELS, ["greedy", "strategist"], GJB_FIRST, GJB_SEEDS),
                GJB_FIRST, GJB_SEEDS))
    return out


def mid():
    """The compute curve's middle point (the diff review: r0 and r3 alone vary the lines): 64 lines a round, d3-d4,
    set A, a spec per pair and level."""
    out = []
    for p in PAIRS:
        for d in (3, 4):
            note = (f"314, step 3's middle point: real:budget=64 (2x the lines a round); pair {p}, d{d}, fresh set A "
                    f"({A_SEEDS} seeds from {A_FIRST[p]}).")
            out.append((spec(f"f314-r5-{p}-d{d}", note, [p], [d], ["real:budget=64"], A_FIRST[p], A_SEEDS),
                        A_FIRST[p], A_SEEDS))
    return out


def main():
    root = os.path.join(os.path.dirname(__file__), "..", "..", "farm")
    if len(sys.argv) > 1 and sys.argv[1] == "--mid":
        for s, first, n in mid():
            with open(os.path.join(root, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
                json.dump(s, f, indent=2)
                f.write("\n")
            print(s["name"], first, n)
        return
    if len(sys.argv) > 2 and sys.argv[1] == "--best":
        for s, first, n in best(sys.argv[2], root):
            with open(os.path.join(root, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
                json.dump(s, f, indent=2)
                f.write("\n")
            print(s["name"], first, n)
        return
    out = []
    s = spec("g314", f"314: the greedy-vs-judge read at r151, paired, fresh seeds {GJ_FIRST} on, d0-d4, the three pairs.",
             list(PAIRS), [0, 1, 2, 3, 4], ["greedy", "strategist"], GJ_FIRST, GJ_SEEDS)
    out.append((s, GJ_FIRST, GJ_SEEDS))
    for row, (agents, groups, what) in ROWS.items():
        for p in PAIRS:
            for lv in groups:
                suffix = "" if len(groups) == 1 else f"-d{lv[0]}{lv[-1]}"
                name = f"f314-{row}-{p}{suffix}"
                note = (f"314, the strong fair player's decomposition: {what}; pair {p}, d{lv[0]}-d{lv[-1]}, "
                        f"fresh set A ({A_SEEDS} seeds from {A_FIRST[p]}).")
                out.append((spec(name, note, [p], lv, agents, A_FIRST[p], A_SEEDS), A_FIRST[p], A_SEEDS))
    for arm, agent in SCREEN.items():
        for p in PAIRS:
            note = (f"314's screen of the real oracle's new options: {agent}; pair {p}, d4, 313's spent ladder seeds "
                    f"({SCREEN_SEEDS} from {SCREEN_FIRST}).")
            out.append((spec(f"s314-{arm}-{p}", note, [p], [4], [agent], SCREEN_FIRST, SCREEN_SEEDS),
                        SCREEN_FIRST, SCREEN_SEEDS))
    for s, first, n in out:
        with open(os.path.join(root, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
            json.dump(s, f, indent=2)
            f.write("\n")
        print(s["name"], first, n)


if __name__ == "__main__":
    main()
