"""155: the end states of one agent's losses (python losses155.py AGENT DIR [DIR...]).

Reads games.jsonl in each DIR (the end-game probe with `all_losses`, crates/aic-sim/src/endgame.rs; cells in the
spec's order: config, variant, difficulty, agent) and prints a row a loss: level, pair, seed, end reason and round,
Unity cards drawn (9 win), the Alignment bar, adversary cities / ambassadors / troops on the board at the end, the
purge count still needed, three rounds before the end the same, and which other agents won that seed.
"""
import json
import os
import sys
from collections import defaultdict


def label(a):
    return "strategist" if a == "strategist" else a


def main(agent, dirs):
    print("| level | pair | seed | reason | round | Unity drawn | bar | cities | adv. amb. | troops | purge | 3 rounds back: cities, amb., troops, purge | team credits then | won by |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    counts = defaultdict(int)
    for d in dirs:
        lines = [json.loads(x) for x in open(os.path.join(d, "games.jsonl"), encoding="utf-8")]
        spec = lines[0]["spec"]
        cells = [(c, v["name"], lv, a) for c in spec["configs"] for v in spec["variants"]
                 for lv in spec["difficulties"] for a in spec["agents"]]
        won = defaultdict(list)
        for g in lines[1:]:
            if g["won"]:
                _, _, lv, a = cells[g["cell"]]
                won[(lv, g["seed"])].append(a.split(",")[0])
        for g in sorted(lines[1:], key=lambda g: (cells[g["cell"]][2], int(g["seed"]))):
            _, pair, lv, a = cells[g["cell"]]
            if a != agent or g["won"]:
                continue
            e = g.get("endgame") or {}
            end, snap, team = e.get("end", {}), e.get("snap", {}), e.get("team", {})
            counts[(lv, g["reason"])] += 1
            others = ", ".join(sorted(set(won[(lv, g["seed"])]))) or "none"
            back = f"{snap.get('cities', '?')}, {snap.get('ambassadors', '?')}, {snap.get('troops', '?')}, {snap.get('purge', '?')}"
            print(f"| d{lv} | {pair} | {g['seed']} | {g['reason']} | {g['rounds']} | {e.get('unity_drawn', '?')} | "
                  f"{e.get('level', '?')} | {end.get('cities', '?')} | {end.get('ambassadors', '?')} | {end.get('troops', '?')} | "
                  f"{end.get('purge', '?')} | {back} | {team.get('credits', '?')} | {others} |")
    print()
    print("Losses by level and reason: " + ", ".join(f"d{lv} {why} {n}" for (lv, why), n in sorted(counts.items())))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2:])
