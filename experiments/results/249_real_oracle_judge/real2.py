"""249: the real oracle at twice the worlds and lines beside the old and the shipped judge (r249-real2): wins, rounds
and agent seconds a game by level, and the paired counts against the old judge.

    python experiments/results/249_real_oracle_judge/real2.py READ_DIR
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    lines = open(sys.argv[1] + "/games.jsonl", encoding="utf-8").read().splitlines()
    sp = json.loads(lines[0])["spec"]
    cells = [(v["name"], d, a) for _ in sp["configs"] for v in sp["variants"] for d in sp["difficulties"]
             for a in sp["agents"]]
    res, by = defaultdict(list), defaultdict(dict)
    for line in lines[1:]:
        g = json.loads(line)
        v, d, a = cells[g["cell"]]
        name = "real oracle 2x" if "worlds" in a else ("shipped judge" if a == "strategist" else "old judge")
        res[(name, d)].append(g)
        by[(d, g["seed"])][name] = g["won"]
    print("| agent | level | wins | rounds | s a game |")
    print("|---|---|---|---|---|")
    for (name, d), gs in sorted(res.items()):
        n, w = len(gs), sum(g["won"] for g in gs)
        print(f"| {name} | d{d} | {w}/{n} | {sum(g['rounds'] for g in gs) / n:.2f} | "
              f"{sum(g['agent_ms'] for g in gs) / n / 1000:.0f} |")
    for d in sorted({k[0] for k in by}):
        rows = [v for (dd, _), v in by.items() if dd == d]
        a = sum(1 for v in rows if v["real oracle 2x"] and not v["old judge"])
        b = sum(1 for v in rows if v["old judge"] and not v["real oracle 2x"])
        print(f"d{d}: the real oracle won {a} the old judge lost, lost {b} it won")


if __name__ == "__main__":
    main()
