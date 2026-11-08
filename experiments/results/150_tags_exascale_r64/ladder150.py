"""150: the ladder read, pooled over its sets and the three pairs, paired on the same pair and seed.

python ladder150.py NEW_DIR[,NEW_DIR...] OLD_DIR[,OLD_DIR...] [--msb]

Each DIR is an experiment output (games.jsonl, results.json). The ladder: NEW's baseline config against OLD's
(r63's binary) per difficulty: the new win %, its SE, the old, the paired gap and its SE, each set's new win %,
and the band. --msb adds item 3's read from NEW alone: the deck with Market Stabilization Board
(`msb_deck.json`) against the baseline deck on the same games, and the card's plays.
"""
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
BANDS = {0: (90, 100), 1: (70, 85), 2: (55, 65), 3: (40, 50)}


def load(d):
    res = json.load(open(f"{d}/results.json", encoding="utf-8"))
    cells = {c["cell"]: c for c in res["cells"]}
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        f.readline()
        for line in f:
            g = json.loads(line)
            c = cells[g["cell"]]
            yield c["config"], c["difficulty"], c["variant"], g


def paired(n, o):
    common = [k for k in n if k in o]
    diffs = [n[k] - o[k] for k in common]
    if not diffs:
        return float("nan"), float("nan"), float("nan")
    m = sum(diffs) / len(diffs)
    var = sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)
    po = sum(o[k] for k in common) / len(common)
    return po, m, math.sqrt(var / len(diffs))


def main():
    new_dirs, old_dirs = sys.argv[1].split(","), sys.argv[2].split(",")
    new, old, msb = defaultdict(dict), defaultdict(dict), defaultdict(dict)
    per_set = defaultdict(lambda: defaultdict(list))
    msb_cards = defaultdict(lambda: [0, 0, 0, 0])  # games, kept, played, plays
    for side, dirs in (("new", new_dirs), ("old", old_dirs)):
        for i, d in enumerate(dirs):
            for cfg, diff, v, g in load(d):
                won = 1 if g["won"] else 0
                key = (i, v, g["seed"])
                if "msb_deck" in cfg:
                    msb[diff][key] = won
                    c = g["cards"].get("market_stabilization_board", {})
                    m = msb_cards[diff]
                    m[0] += 1
                    m[1] += c.get("kept", 0) > 0
                    m[2] += c.get("played", 0) > 0
                    m[3] += c.get("played", 0)
                elif side == "new":
                    new[diff][key] = won
                    per_set[diff][i].append(won)
                else:
                    old[diff][key] = won
    print("| d | new win % | ±SE | r63 win % | gap | ±SE (paired) | sets (new) | band | verdict |")
    print("|---|---|---|---|---|---|---|---|---|")
    for diff in sorted(new):
        n = new[diff]
        p = sum(n.values()) / len(n)
        se = math.sqrt(p * (1 - p) / len(n)) * 100
        po, m, gse = paired(n, old.get(diff, {}))
        sets = " / ".join(f"{100 * sum(x) / len(x):.1f}" for _, x in sorted(per_set[diff].items()))
        lo, hi = BANDS[diff]
        ok = "met" if lo <= 100 * p <= hi else "**unmet**"
        print(f"| d{diff} | **{100 * p:.1f}** | {se:.1f} | {100 * po:.1f} | {100 * m:+.1f} | {100 * gse:.1f} | {sets} | {lo}-{hi} | {ok} | ({len(n)} games)")
    if "--msb" in sys.argv:
        print()
        print("| d | baseline deck win % | with Market Stabilization Board | gap | ±SE (paired) | games kept % | games played % | plays a game |")
        print("|---|---|---|---|---|---|---|---|")
        for diff in sorted(msb):
            pb, m, gse = paired(msb[diff], new[diff])
            pm = sum(msb[diff].values()) / len(msb[diff])
            g, k, pl, plays = msb_cards[diff]
            print(f"| d{diff} | {100 * pb:.1f} | {100 * pm:.1f} | {100 * m:+.1f} | {100 * gse:.1f} | {100 * k / g:.1f} | {100 * pl / g:.1f} | {plays / g:.2f} |")


if __name__ == "__main__":
    main()
