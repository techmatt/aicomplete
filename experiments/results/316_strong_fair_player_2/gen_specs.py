"""316: writes the item's farm specs into experiments/farm/ (moved to experiments/archive/farm/ when done) and
appends to the jobs file farm_all.py reads (`name first count`, one line a spec), d4 first.

Fresh set C (52210-52245, 12 seeds a pair: bp 52210, br 52222, pr 52234): 36 a cell at d3-d4, 24 at d0-d2 (each
pair's first 8). One spec per row, pair and level (a 2-seed shard is 2 games), so the costly rows spread over the
workers. Every row plays the same seeds, paired on (pair, level, seed).

    python experiments/results/316_strong_fair_player_2/gen_specs.py JOBS_FILE [--rows ROW,...] [--levels 4,3,...]
"""
import json
import os
import sys

PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# Matt, amendment 14593: 36 seeds a cell at d3-d4 (12 a pair), 24 at d0-d2 (the first 8 of each pair's 12).
C_FIRST = {"bp": 52210, "br": 52222, "pr": 52234}
HIGH, LOW = 12, 8

FULL = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3"
ENGINE = "oracle:strategy=1,worlds=4,alts=3,answers=1,truth=1"
ROWS = {
    # name: (agents, what it is)
    "cheap": (["greedy", "strategist", FULL], "greedy, the judge and the full oracle"),
    "j128": (["real:budget=128"], "the judge's eval with 128 lines a round (314's strongest, the yardstick)"),
    "e128": ([ENGINE + ",budget=128"], "row 1: the engine's eval with 128 lines a round"),
    "e256": ([ENGINE + ",budget=256"], "row 2: the engine's eval with 256 lines a round"),
    "j256": (["real:budget=256"], "row 2: the judge's eval with 256 lines a round"),
    "e512": ([ENGINE + ",budget=512"], "row 3: the engine's eval with 512 lines a round"),
    "e8w": ([ENGINE.replace("worlds=4", "worlds=8") + ",budget=512"],
            "row 3: the engine's eval on 8 worlds, 512 lines a round (row 2's decisions, twice the worlds)"),
}


def seeds(d):
    return HIGH if d >= 3 else LOW


def spec(name, note, pair, levels, agents, n):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"],
            "variants": [{"name": pair, "set": PAIRS[pair]}], "difficulties": levels,
            "agents": agents, "games": n, "first_seed": C_FIRST[pair], "records": 0}


def main():
    jobs = sys.argv[1]
    rows = sys.argv[sys.argv.index("--rows") + 1].split(",") if "--rows" in sys.argv else ["cheap", "j128", "e128", "e256"]
    levels = ([int(x) for x in sys.argv[sys.argv.index("--levels") + 1].split(",")] if "--levels" in sys.argv
              else [4, 3, 2, 1, 0])
    root = os.path.join(os.path.dirname(__file__), "..", "..", "farm")
    out = []
    for row in rows:
        agents, what = ROWS[row]
        for p in PAIRS:
            if row == "cheap":
                note = f"316: {what}; pair {p}, d0-d4, fresh set C ({HIGH} seeds from {C_FIRST[p]})."
                out.append((-1, spec(f"f316-cheap-{p}", note, p, [0, 1, 2, 3, 4], agents, HIGH)))
                continue
            for d in levels:
                n = seeds(d)
                note = f"316: {what}; pair {p}, d{d}, fresh set C ({n} seeds from {C_FIRST[p]})."
                out.append((levels.index(d), spec(f"f316-{row}-{p}-d{d}", note, p, [d], agents, n)))
    out.sort(key=lambda t: t[0])
    with open(jobs, "a", encoding="utf-8", newline="\n") as jf:
        for _, s in out:
            with open(os.path.join(root, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
                json.dump(s, f, indent=2)
                f.write("\n")
            jf.write(f"{s['name']} {s['first_seed']} {s['games']}\n")
            print(s["name"], s["first_seed"], s["games"])


if __name__ == "__main__":
    main()
