"""154: each level's end reasons and the round cap's share of its losses, per side, paired on the same seeds.

python ends154.py DIR[,DIR...] [OLD_DIR[,OLD_DIR...]]

Each DIR is an experiment output (games.jsonl, results.json). Per difficulty: games, win %, losses, and each loss
reason's share of the losses (deck_empty is the round cap). With OLD, the same for it, and the cap's share on the
seeds both sides lost.
"""
import json
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def games(dirs):
    for d in dirs.split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                yield c["difficulty"], c["variant"], g


def table(dirs, label):
    by = defaultdict(list)
    for diff, var, g in games(dirs):
        by[diff].append(g)
    print(f"\n### {label}\n")
    print("| Level | Games | Win % | Losses | Round cap | Cap share of losses | Discord deck | Nodes | Other |")
    print("|---|---|---|---|---|---|---|---|---|")
    for diff in sorted(by):
        gs = by[diff]
        n = len(gs)
        wins = sum(g["won"] for g in gs)
        lost = Counter(g["reason"] for g in gs if not g["won"])
        nl = n - wins
        cap = lost.get("deck_empty", 0)
        other = nl - cap - lost.get("discord_deck", 0) - lost.get("node", 0)
        print(f"| d{diff} | {n} | {100 * wins / n:.1f} | {nl} | {cap} | {100 * cap / max(nl, 1):.0f}% | "
              f"{lost.get('discord_deck', 0)} | {lost.get('node', 0)} | {other} |")
    return by


if __name__ == "__main__":
    table(sys.argv[1], "New" if len(sys.argv) > 2 else "Read")
    if len(sys.argv) > 2:
        table(sys.argv[2], "Old")
