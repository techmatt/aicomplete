"""148, item 6: leftover resources. In each won game, what each seat held at the end of the turn before the winning
one, before income: `unspent.<seat>.<resource>.rNN` (held as round NN's action phase ends, before production and
upkeep pay), at NN = the winning round less 1. Per difficulty and board, pooled over the three seat pairs.

    python experiments/results/148_card_pass_r62/holdings148.py DIR [DIR ...] [--md]

Columns per resource: the mean held, the share of seat-games holding a "large" stock (over HIGH, a sink's price, and over
two rounds of its own income), and that resource's mean income a round (what production and upkeep paid it, `phase:*` sources, over the rounds) for scale. Then, for the large
stocks, why: the game was won by dominating (won with 3+ adversary cards left, so the last round's income did not
matter), and the sinks open that turn (threats left uncountered that round, which take credits 12 / 17 or a scarce
currency; an Action building skipped or blocked).
"""

import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "147_agents_tempo"))
from read147 import load  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")

RES = ["credits", "chips", "energy", "antimatter", "ambassadors", "research_cubes"]
# `sources` order in games.jsonl (PEAKS): credits, insight, energy, chips, antimatter, research cubes, card tokens,
# ambassadors.
GAINED = {"credits": 0, "energy": 2, "chips": 3, "antimatter": 4, "research_cubes": 5, "ambassadors": 7}
# A large stock: over HIGH (a sink's price: credits a tier-2 counter, 17; chips 7; energy 4; antimatter 2, two
# ultimates; ambassadors 4, a claim and more; research cubes 5) and over two rounds of the seat's own income of it.
HIGH = {"credits": 17, "chips": 7, "energy": 4, "antimatter": 2, "ambassadors": 4, "research_cubes": 5}


def large(h, i, x):
    return h[x] > max(HIGH[x], 2 * i[x])
BOARDS = ["Builder", "Protector", "Researcher"]
PAIR_BOARDS = {"bp": (0, 1), "br": (0, 2), "pr": (1, 2)}


def main():
    a = sys.argv[1:]
    md = "--md" in a
    dirs = [x for x in a if not x.startswith("--")]
    rows = defaultdict(list)
    for d in dirs:
        for g in load(d):
            if not g["won"]:
                continue
            r = g["rounds"] - 1
            if r < 1:
                continue
            c = g["counters"]
            for s, b in enumerate(PAIR_BOARDS[g["_pair"]]):
                held = {x: c.get(f"unspent.{s}.{x}.r{r:02d}", 0) for x in RES}
                # Income: what the automatic phases paid (production, upkeep), a round.
                src = g["sources"][s]
                inc = {x: sum(v[GAINED[x]] for k, v in src.items() if k.startswith("phase:")) / max(1, g["rounds"])
                       for x in RES}
                rows[(g["_diff"], b)].append((held, inc, g, s, r))
    head = "| d | board | games | " + " | ".join(f"{x} held / large / income" for x in RES) + " |"
    print(head)
    print("|" + "---|" * (3 + len(RES)))
    for (d, b), xs in sorted(rows.items()):
        n = len(xs)
        cells = []
        for x in RES:
            m = sum(h[x] for h, *_ in xs) / n
            big = sum(1 for h, i, *_ in xs if large(h, i, x)) / n
            inc = sum(i[x] for _, i, *_ in xs) / n
            cells.append(f"{m:.1f} / {100 * big:.0f}% / {inc:.1f}")
        print(f"| d{d} | {BOARDS[b]} | {n} | " + " | ".join(cells) + " |")
    print()
    print("Large stocks, why (share of the seat-games holding one): won dominating (3+ adversary cards left); a threat")
    print("left uncountered that round (a counter the stock could have paid); an Action building skipped that game.")
    for x in RES:
        big = [(h, i, g, s, r) for xs in rows.values() for (h, i, g, s, r) in xs if large(h, i, x)]
        if not big:
            continue
        dom = sum(1 for _, _, g, _, _ in big if g["margin"]["deck_left"] >= 3) / len(big)
        unc = sum(1 for _, _, g, _, r in big if g["counters"].get(f"threats.uncountered.r{r:02d}", 0) > 0) / len(big)
        skipped = sum(1 for _, _, g, s, _ in big
                      if any(k.startswith(f"actions.skipped.{s}.") for k in g["counters"])) / len(big)
        # Both node powers used that round (each costs energy: 1, then the board's second plus the track space).
        def nodes_used(g, s, r):
            b = BOARDS[PAIR_BOARDS[g["_pair"]][s]].lower()
            return sum(1 for k in (0, 1) if g["counters"].get(f"ai.power.{b}.{k}.r{r:02d}", 0) > 0)
        both = sum(1 for _, _, g, s, r in big if nodes_used(g, s, r) == 2) / len(big)
        none = sum(1 for _, _, g, s, r in big if nodes_used(g, s, r) == 0) / len(big)
        print(f"- {x}: {len(big)} seat-games; won dominating {100 * dom:.0f}%, a threat uncountered that round "
              f"{100 * unc:.0f}%, an Action skipped {100 * skipped:.0f}%; node powers that round: both {100 * both:.0f}%, "
              f"none {100 * none:.0f}%")


if __name__ == "__main__":
    main()
