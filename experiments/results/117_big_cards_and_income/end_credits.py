"""117: is the strategist's "credits left at game end" a pile, or the last round's income caught mid-round?

    python experiments/results/117_big_cards_and_income/end_credits.py DIR [DIR...] [--agent strategist]

For each game: the team's credits left at the end (`margin.unspent[s][0]` summed over seats), and the team's credits
held at the end of each completed action phase (`economy.credits_unspent.rNN`, recorded in upkeep, so only for rounds
whose upkeep ran). Splits games by end reason and by whether the last round's upkeep ran (the game ended after its
action phase) or not (it ended inside a round: the seats still held that round's income).
"""

import argparse
import json
import statistics as st
import sys


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser()
    ap.add_argument("dirs", nargs="+")
    ap.add_argument("--agent", default="strategist")
    a = ap.parse_args()
    for d in a.dirs:
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            lines = f.read().splitlines()
        spec = json.loads(lines[0])["spec"]
        agents = [ag for v in spec["variants"] for _ in spec["difficulties"] for ag in spec["agents"]]
        rows = {}
        for ln in lines[1:]:
            g = json.loads(ln)
            if agents[g["cell"]] != a.agent:
                continue
            end = sum(u[0] for u in g["margin"]["unspent"])
            c = g["counters"]
            rr = g["rounds"]
            last = c.get(f"economy.credits_unspent.r{rr:02d}")
            prev = c.get(f"economy.credits_unspent.r{rr - 1:02d}")
            mid = last is None
            key = (g["reason"], "inside a round" if mid else "after upkeep")
            rows.setdefault(key, []).append((end, prev if mid else last, len(g["margin"]["unspent"])))
        print(f"\n{d}, {a.agent}: team credits (per seat in brackets)\n")
        print("| end reason | when | games | left at end | held after the previous completed action phase |")
        print("|---|---|---|---|---|")
        for (reason, when), v in sorted(rows.items(), key=lambda kv: -len(kv[1])):
            n = v[0][2]
            e = st.mean(x[0] for x in v)
            p = [x[1] for x in v if x[1] is not None]
            ps = f"{st.mean(p):.0f} ({st.mean(p) / n:.0f})" if p else "-"
            print(f"| {reason} | {when} | {len(v)} | {e:.0f} ({e / n:.0f}) | {ps} |")
        allv = [x for v in rows.values() for x in v]
        n = allv[0][2]
        p = [x[1] for x in allv if x[1] is not None]
        print(f"| all | | {len(allv)} | {st.mean(x[0] for x in allv) / n:.1f} a seat | {st.mean(p) / n:.1f} a seat |")


if __name__ == "__main__":
    main()
