"""249: the judge, the full oracle and the real oracle side by side: win rate, rounds (all and won) and cost (agent
seconds a game, one thread) by level and agent, from a read whose agents are those three (any order), plus the
paired counts: real-oracle wins where the judge lost and the reverse, and full-oracle wins the real one lost.

    python experiments/results/249_real_oracle_judge/oracles.py READ_DIR [--out FILE]
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def name(spec):
    if spec.startswith("oracle") and "worlds=" in spec:
        return "real oracle"
    if spec.startswith("oracle"):
        return "full oracle"
    if spec.startswith("engine") and "w.purge" in spec or spec.startswith("strategist"):
        return "judge"
    return spec


def cells_of(spec):
    return [(v["name"], d, a) for _ in spec["configs"] for v in spec["variants"] for d in spec["difficulties"]
            for a in spec["agents"]]


def main():
    d = sys.argv[1]
    lines = open(d + "/games.jsonl", encoding="utf-8").read().splitlines()
    head = json.loads(lines[0])
    cells = cells_of(head["spec"])
    res = defaultdict(list)  # (agent, level) -> games
    by_seed = defaultdict(dict)  # (variant, level, seed) -> agent -> won
    for line in lines[1:]:
        g = json.loads(line)
        v, lvl, a = cells[g["cell"]]
        who = name(a)
        res[(who, lvl)].append(g)
        by_seed[(v, lvl, g["seed"])][who] = g["won"]
    agents = ["judge", "real oracle", "full oracle"]
    levels = sorted({k[1] for k in res})
    out = ["| agent | level | wins | win % | rounds (all / won) | s a game |", "|---|---|---|---|---|---|"]
    for a in agents:
        for lvl in levels + ["all"]:
            gs = [g for (x, l), v in res.items() if x == a and (l == lvl or lvl == "all") for g in v]
            if not gs:
                continue
            n, w = len(gs), sum(g["won"] for g in gs)
            p = w / n
            se = math.sqrt(p * (1 - p) / n) * 100
            won = [g["rounds"] for g in gs if g["won"]]
            rw = f"{sum(won) / len(won):.2f}" if won else "-"
            secs = sum(g["agent_ms"] for g in gs) / n / 1000
            out.append(f"| {a} | {('d' + str(lvl)) if lvl != 'all' else 'all'} | {w}/{n} | {100 * p:.1f} ± {se:.1f} | "
                       f"{sum(g['rounds'] for g in gs) / n:.2f} / {rw} | {secs:.1f} |")
    out.append("")
    out.append("| level | real wins, judge lost | judge wins, real lost | full wins, real lost | real wins, full lost |")
    out.append("|---|---|---|---|---|")
    for lvl in levels:
        rows = [v for (_, l, _), v in by_seed.items() if l == lvl]
        c = lambda x, y: sum(1 for r in rows if r.get(x) and r.get(y) is False)  # noqa: E731
        out.append(f"| d{lvl} | {c('real oracle', 'judge')} | {c('judge', 'real oracle')} | "
                   f"{c('full oracle', 'real oracle')} | {c('real oracle', 'full oracle')} |")
    text = "\n".join(out) + "\n"
    if "--out" in sys.argv:
        open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8", newline="\n").write(text)
    print(text)


if __name__ == "__main__":
    main()
