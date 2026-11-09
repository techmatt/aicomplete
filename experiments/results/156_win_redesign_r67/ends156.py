"""156: each level's win routes (the purge at each Unity step) and loss kinds, per side, by agent.

python ends156.py DIR[,DIR...] [OLD_DIR[,OLD_DIR...]] [--by-variant] [--by-agent]

Each DIR is an experiment output (games.jsonl, results.json). The Unity step is read from the game's final
`alignment.tips_unity` against `purge_at` (3, 6): step 0 (0-2 tips: all pieces count), 1 (3-5: no ambassadors),
2 (6+: cities only). r66 games can also end `unity_deck` (the Unity-deck win). Losses: deck_empty is the round cap.
Also each level's Unity tips at the end, liberations a game and Unity cards resolved at each tier.
"""
import json
import math
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
PURGE_AT = (3, 6)


def games(dirs):
    for d in dirs.split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                yield c, g


def step(g):
    t = g["counters"].get("alignment.tips_unity", 0)
    return sum(t >= a for a in PURGE_AT)


def table(dirs, label, key):
    by = defaultdict(list)
    for c, g in games(dirs):
        by[key(c)].append(g)
    print(f"\n### {label}\n")
    print("| Cell | Games | Win % | SE | Purge at step 0 / 1 / 2 | Unity-deck wins | Round cap | Discord deck | Nodes | Other | "
          "Unity tips at end | Cities liberated | Unity tiers I / II / III a game |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for k in sorted(by):
        gs = by[k]
        n = len(gs)
        wins = [g for g in gs if g["won"]]
        p = len(wins) / n
        steps = Counter(step(g) for g in wins if g["reason"] == "purge")
        lost = Counter(g["reason"] for g in gs if not g["won"])
        other = n - len(wins) - lost["deck_empty"] - lost["discord_deck"] - lost["node"]
        tips = sum(g["counters"].get("alignment.tips_unity", 0) for g in gs) / n
        lib = sum(g["counters"].get("board.liberated.city", 0) for g in gs) / n
        tiers = [sum(g["counters"].get(f"alignment.tier_unity.{i}", 0) for g in gs) / n for i in (1, 2, 3)]
        print(f"| {k} | {n} | {100 * p:.1f} | {100 * math.sqrt(p * (1 - p) / n):.1f} | "
              f"{steps[0]} / {steps[1]} / {steps[2]} | {sum(g['reason'] == 'unity_deck' for g in wins)} | "
              f"{lost['deck_empty']} | {lost['discord_deck']} | {lost['node']} | {other} | {tips:.1f} | {lib:.2f} | "
              + " / ".join(f"{x:.1f}" for x in tiers) + " |")


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if "--by-variant" in sys.argv:
        key = lambda c: (c["variant"], c["agent"], f"d{c['difficulty']}")
    elif "--by-agent" in sys.argv:
        key = lambda c: (c["agent"], f"d{c['difficulty']}")
    else:
        key = lambda c: f"d{c['difficulty']}"
    table(args[0], "New" if len(args) > 1 else "Read", key)
    if len(args) > 1:
        table(args[1], "Old", key)
