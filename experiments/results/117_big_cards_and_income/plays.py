"""117: per-card play counts from an experiment's games.jsonl.

    python experiments/results/117_big_cards_and_income/plays.py DIR [--cells REGEX] [--agent NAME] [--cards a,b] [--low N]

For each card: offered (dealt into a draft packet, or the opening deal, share of games), kept (kept or drawn, share of
games), played (share of games it was played in at least once) and plays a game. `--cards` lists only those ids;
`--low N` lists the N cards played in the fewest games (deck cards only, the cards seen in any game).
"""

import argparse
import json
import re
import sys


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser()
    ap.add_argument("dir")
    ap.add_argument("--cells", default=".")
    ap.add_argument("--agent", default=None)
    ap.add_argument("--cards", default=None)
    ap.add_argument("--low", type=int, default=0)
    a = ap.parse_args()
    with open(f"{a.dir}/games.jsonl", encoding="utf-8") as f:
        lines = f.read().splitlines()
    spec = json.loads(lines[0])["spec"]
    cells = [(v["name"], ag) for v in spec["variants"] for _ in spec["difficulties"] for ag in spec["agents"]]
    with open("content/cards.json", encoding="utf-8") as f:
        names = {c["id"]: (c["name"], c["tier"], c["cost"]) for c in json.load(f)}
    acc = {}
    n = 0
    for ln in lines[1:]:
        g = json.loads(ln)
        v, ag = cells[g["cell"]]
        if not re.search(a.cells, v) or (a.agent and ag != a.agent):
            continue
        n += 1
        for cid, r in g["cards"].items():
            e = acc.setdefault(cid, [0, 0, 0, 0])
            e[0] += r.get("offered", 0) > 0
            e[1] += (r.get("kept", 0) + r.get("drawn", 0)) > 0
            e[2] += r.get("played", 0) > 0
            e[3] += r.get("played", 0)
    ids = a.cards.split(",") if a.cards else sorted(acc, key=lambda k: (acc[k][2], acc[k][1]))
    if a.low:
        ids = ids[: a.low]
    print(f"{n} games\n")
    print("| card | tier | cost | offered % | kept % | played % | plays a game |")
    print("|---|---|---|---|---|---|---|")
    for cid in ids:
        e = acc.get(cid, [0, 0, 0, 0])
        nm, t, c = names.get(cid, (cid, "?", "?"))
        print(f"| {nm} | {t} | {c} | {100 * e[0] / n:.0f} | {100 * e[1] / n:.0f} | {100 * e[2] / n:.0f} | {e[3] / n:.2f} |")


if __name__ == "__main__":
    main()
