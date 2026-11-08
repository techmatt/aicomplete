"""150: per card, the share of games that kept it, played it, and kept it but never played it.

python keep150.py DIR[,DIR...] [--cards a,b] [--by difficulty|variant] [--config SUBSTR]

DIRs are experiment outputs (games.jsonl, results.json). A card counts once a game, whichever seat held it.
Default cards: exascale_training_cluster and market_stabilization_board. Pooled over the DIRs, split by
--by (default difficulty), filtered to configs whose path contains --config.
"""
import argparse
import collections
import json
import math
import os
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dirs")
    ap.add_argument("--cards", default="exascale_training_cluster,market_stabilization_board")
    ap.add_argument("--by", default="difficulty")
    ap.add_argument("--config", default="")
    a = ap.parse_args()
    want = a.cards.split(",")
    n = collections.Counter()
    kept = collections.defaultdict(collections.Counter)
    played = collections.defaultdict(collections.Counter)
    idle = collections.defaultdict(collections.Counter)
    plays = collections.defaultdict(collections.Counter)
    for d in a.dirs.split(","):
        cells = {c["cell"]: c for c in json.load(open(os.path.join(d, "results.json"), encoding="utf-8"))["cells"]}
        with open(os.path.join(d, "games.jsonl"), encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                if a.config not in c["config"]:
                    continue
                key = c[a.by]
                n[key] += 1
                for cid in want:
                    rec = g["cards"].get(cid, {})
                    k, p = rec.get("kept", 0), rec.get("played", 0)
                    kept[key][cid] += k > 0
                    played[key][cid] += p > 0
                    idle[key][cid] += k > 0 and p == 0
                    plays[key][cid] += p
    print(f"| card | {a.by} | games | kept % | played % | kept, unplayed % | plays a game |")
    print("|---|---|---|---|---|---|---|")
    for cid in want:
        for key in sorted(n):
            m = n[key]
            pct = lambda x: f"{100.0 * x / m:.1f}"
            print(f"| {cid} | {key} | {m} | {pct(kept[key][cid])} | {pct(played[key][cid])} | {pct(idle[key][cid])} | {plays[key][cid] / m:.2f} |")
        tot = sum(n.values())
        k = sum(kept[x][cid] for x in n)
        p = sum(played[x][cid] for x in n)
        print(f"| {cid} | all | {tot} | {100.0 * k / tot:.1f} | {100.0 * p / tot:.1f} | {100.0 * sum(idle[x][cid] for x in n) / tot:.1f} | {sum(plays[x][cid] for x in n) / tot:.2f} |")


if __name__ == "__main__":
    main()
