"""175: win % and the share of losses by cause, per agent and difficulty, from runs' games.jsonl.

    python experiments/results/175_rules_adversary/losses175.py DIR [DIR ...] [--by-variant]

Causes are the engine's end reasons: `deck_empty` (the round cap: no event card to turn up), `discord_deck`
(a Discord tip finding the deck empty), `node` (a player with no node left), anything else by its name.
Also each level's mean rounds, last-node losses a game, nodes lost to corruption a game and node corruptions a game.
"""
import argparse
import collections
import json
import os

NAMES = {"deck_empty": "round cap", "node": "last node"}


def read(dirs):
    for d in dirs:
        p = os.path.join(d, "games.jsonl")
        with open(p, encoding="utf-8") as f:
            head = json.loads(f.readline())
            cells = []
            s = head["spec"]
            for cfg in s["configs"]:
                for v in s.get("variants", [{"name": "base"}]):
                    for diff in s["difficulties"]:
                        for a in s["agents"]:
                            cells.append((cfg, v["name"], diff, a))
            for line in f:
                g = json.loads(line)
                if "cell" not in g:
                    continue
                yield cells[g["cell"]], g


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dirs", nargs="+")
    ap.add_argument("--by-variant", action="store_true")
    a = ap.parse_args()
    rows = collections.defaultdict(list)
    for (cfg, var, diff, agent), g in read(a.dirs):
        key = (agent, var if a.by_variant else "", diff)
        rows[key].append(g)
    causes = sorted({g.get("reason", "?") for gs in rows.values() for g in gs if not g["won"]})
    head = ["agent", "variant", "d", "games", "win %", "rounds"] + [f"{NAMES.get(c, c)} % of losses" for c in causes] + [
        "node corruptions/game", "nodes lost/game"]
    print("| " + " | ".join(head) + " |")
    print("|" + "---|" * len(head))
    for key in sorted(rows):
        gs = rows[key]
        n = len(gs)
        lost = [g for g in gs if not g["won"]]
        wins = n - len(lost)
        cnt = collections.Counter(g.get("reason", "?") for g in lost)
        share = [f"{100 * cnt[c] / max(1, len(lost)):.0f}" for c in causes]
        rounds = sum(g.get("rounds", 0) for g in gs) / max(1, n)
        nc = sum(g["counters"].get("adversary.corruptions.node", 0) for g in gs) / max(1, n)
        nl = sum(g["counters"].get("board.node_lost", 0) for g in gs) / max(1, n)
        print("| " + " | ".join([key[0], key[1] or "all", str(key[2]), str(n), f"{100 * wins / n:.1f}", f"{rounds:.2f}"] + share
                                + [f"{nc:.2f}", f"{nl:.2f}"]) + " |")


if __name__ == "__main__":
    main()
