"""117: where the strategist's credit pile comes from: per round, credits and insight left unspent (team totals,
`economy.credits_unspent.rNN`, `economy.insight_unspent.rNN`), insight carried (the Researcher's Patience), cards
played, and the hand at game end.

    python experiments/results/117_big_cards_and_income/pile.py DIR [--agent strategist]
"""

import argparse
import json
import statistics as st
import sys


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser()
    ap.add_argument("dir")
    ap.add_argument("--agent", default="strategist")
    a = ap.parse_args()
    with open(f"{a.dir}/games.jsonl", encoding="utf-8") as f:
        lines = f.read().splitlines()
    spec = json.loads(lines[0])["spec"]
    cells = [(v["name"], ag) for v in spec["variants"] for _ in spec["difficulties"] for ag in spec["agents"]]
    games = [json.loads(x) for x in lines[1:]]
    for pair in [v["name"] for v in spec["variants"]]:
        gs = [g for g in games if cells[g["cell"]] == (pair, a.agent)]
        print(f"\n{pair} {a.agent}, {len(gs)} games: mean per game, games that reached the round")
        print("| round | games | credits unspent | insight unspent | insight carried | cards played |")
        print("|---|---|---|---|---|---|")
        for r in range(1, 10):
            k = f"r{r:02d}"
            have = [g for g in gs if f"cards.played.{k}" in g["counters"] or f"economy.credits_unspent.{k}" in g["counters"]]
            if not have:
                continue

            def m(name):
                return st.mean(g["counters"].get(f"{name}.{k}", 0) for g in have)

            print(f"| {r} | {len(have)} | {m('economy.credits_unspent'):.1f} | {m('economy.insight_unspent'):.1f} | "
                  f"{m('economy.insight_carried'):.1f} | {m('cards.played'):.1f} |")


if __name__ == "__main__":
    main()
