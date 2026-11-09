"""Whether two agents rank the cards alike: Spearman's rank correlation of each card's keep
and play rates between agent A and agent B in one experiment (103).

    python scripts/bench/card_ranks.py DIR AGENT_A AGENT_B [--variant V]...

Agents go as cards.csv names them, resolved: the strategist is `engine:strategy=1`, and an
override follows it (`engine:strategy=1,w.packet_keep=0`).
Reads DIR/cards.csv, pooled over its variants, configs and difficulties (or only `--variant`s).
A card's keep rate is kept per offer, its play rate played per kept; a card neither agent
kept is left out of the play ranks. The bar an agent approximation is held to is `sweep`'s
against the planner's: 0.985-0.991 (docs/speed.md). Ties take their mean rank.
"""
import argparse
import collections
import csv
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def ranks(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    r = [0.0] * len(xs)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and xs[order[j + 1]] == xs[order[i]]:
            j += 1
        for k in range(i, j + 1):
            r[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def spearman(a, b):
    ra, rb = ranks(a), ranks(b)
    n = len(a)
    ma, mb = sum(ra) / n, sum(rb) / n
    cov = sum((x - ma) * (y - mb) for x, y in zip(ra, rb))
    va = sum((x - ma) ** 2 for x in ra)
    vb = sum((y - mb) ** 2 for y in rb)
    return cov / (va * vb) ** 0.5 if va and vb else float("nan")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("dir")
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("--variant", action="append")
    a = ap.parse_args()
    tally = {a.a: collections.defaultdict(lambda: [0, 0, 0]), a.b: collections.defaultdict(lambda: [0, 0, 0])}
    with open(os.path.join(a.dir, "cards.csv"), encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f):
            if row["agent"] not in tally or (a.variant and row["variant"] not in a.variant):
                continue
            t = tally[row["agent"]][row["card"]]
            t[0] += int(row["offered"])
            t[1] += int(row["kept"])
            t[2] += int(row["played"])
    cards = sorted(set(tally[a.a]) & set(tally[a.b]))
    keep_a = [tally[a.a][c][1] / max(1, tally[a.a][c][0]) for c in cards]
    keep_b = [tally[a.b][c][1] / max(1, tally[a.b][c][0]) for c in cards]
    played = [c for c in cards if tally[a.a][c][1] and tally[a.b][c][1]]
    play_a = [tally[a.a][c][2] / tally[a.a][c][1] for c in played]
    play_b = [tally[a.b][c][2] / tally[a.b][c][1] for c in played]
    print(f"{a.b} against {a.a}, {len(cards)} cards ({len(played)} kept by both)")
    print(f"keep rate rank correlation: {spearman(keep_a, keep_b):.3f}")
    print(f"play rate rank correlation: {spearman(play_a, play_b):.3f}")


if __name__ == "__main__":
    main()
