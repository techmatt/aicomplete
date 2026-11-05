"""111: r44 against r43 on the same seeds: wins, rounds, credits each seat left at game end, timeouts, most plays of one
card in a game.

    python experiments/results/111_card_pass/credits_read.py OLD_DIR NEW_DIR

Credits left are `margin.unspent[seat][0]` (what a seat held when its last action phase ended, `experiment.rs`).
"""

import json
import statistics as st
import sys


def load(d):
    lines = open(f"{d}/games.jsonl", encoding="utf-8").read().splitlines()
    head = json.loads(lines[0])
    spec = head["spec"]
    cells = []
    for v in spec["variants"]:
        for diff in spec["difficulties"]:
            for a in spec["agents"]:
                cells.append((v["name"], a))
    games = {}
    for ln in lines[1:]:
        g = json.loads(ln)
        games[(cells[g["cell"]], g["seed"])] = g
    return head["rules_revision"], cells, games


def most_played(g):
    best = (0, "")
    for c, row in g["cards"].items():
        best = max(best, (row.get("played", 0), c))
    return best


def main(old, new):
    ro, cells, go = load(old)
    rn, _, gn = load(new)
    print(f"r{ro} -> r{rn}, paired on the same seeds\n")
    print(f"| cell | games | win % r{ro} | win % r{rn} | diff ± SE | rounds | credits left at end (seat 0, seat 1) r{ro} | r{rn} | timeouts |")
    print("|---|---|---|---|---|---|---|---|---|")
    for cell in cells:
        keys = sorted(k for k in go if k[0] == cell and k in gn)
        if not keys:
            continue
        wo = [go[k]["won"] for k in keys]
        wn = [gn[k]["won"] for k in keys]
        d = [int(b) - int(a) for a, b in zip(wo, wn)]
        se = st.stdev(d) / len(d) ** 0.5 * 100 if len(d) > 1 else 0
        ro_ = st.mean(go[k]["rounds"] for k in keys)
        rn_ = st.mean(gn[k]["rounds"] for k in keys)

        def cr(gs, s):
            return st.mean(gs[k]["margin"]["unspent"][s][0] for k in keys)

        def tout(gs):
            return sum(1 for k in keys if gs[k]["reason"] in ("timeout", "cap", "max_actions"))

        print(
            f"| {cell[0]} {cell[1]} | {len(keys)} | {100 * st.mean(wo):.0f} | {100 * st.mean(wn):.0f} | "
            f"{100 * st.mean(d):+.0f} ± {se:.0f} | {ro_:.1f} → {rn_:.1f} | {cr(go, 0):.1f}, {cr(go, 1):.1f} | "
            f"{cr(gn, 0):.1f}, {cr(gn, 1):.1f} | {tout(go)} / {tout(gn)} |"
        )
    for label, gs in (("old", go), ("new", gn)):
        top = max((most_played(g) + (k,) for k, g in gs.items()), key=lambda t: t[0])
        reasons = {}
        for g in gs.values():
            reasons[g["reason"]] = reasons.get(g["reason"], 0) + 1
        print(f"\n{label}: most plays of one card in a game {top[0]} ({top[1]}, {top[2]}); ended by {reasons}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
