"""117: r46's card list: every new, changed and cut card with its reasoning and anchor check.

    python experiments/results/117_big_cards_and_income/card_list.py          # print the table (cards.md)
    python experiments/results/117_big_cards_and_income/card_list.py --notes  # write docs/card-pricing-notes.json lines

Anchor check (docs/card-guidelines.md): pay = credits + 4 x (keep + play insight) + other costs (antimatter 7,
energy 4.5); keep 1 insight at tier 1, 2 at tier 2. get = the effects at the anchors: +1 credit income 4, +1 insight
income 12, +1 energy income 14, +1 chip income 10, a draw 5, an ambassador removed 5, an ambassador gained 4, a Sway 4,
a node placed free 6 (Relay Mast Rollout's read at r45), a research cube 6 (Moonshot Grant's read at r45), a botnet
removed 16 (111's Botnet Takedown read: values.json's 12 plus its 2 Sway). The calibrated tier-2 cards keep 112's get.
"""

import json
import sys

# (id, kind of change, r45 text, r46 text, reasoning, pay, get)
ROWS = [
    # New: early jump-starts. The anchors price income at mid-game (4 rounds); bought in round 1-3 of a 7-round game
    # it runs about 5, so the income part reads about 1.25x: the "early" figure in each reason.
    ("fabrication_megaplex", "new, early", "-", "t1 building, 34 cr, 1 ins: Each round, 7 credits and 1 chip. Draw 3 cards.",
     "Industry's jump-start: a round-1-3 building that pays from the next upkeep, with cards to feed it; no needs, so "
     "a starting board plays it from its 40 credits. Under Gigafactory Charter per credit, but 6 cheaper, so it is "
     "the one a round-1 seat can afford beside its keeps. Payback 1 round. Early: 1.49.",
     "34 + 4 x 2 = 42", "7 x 4 = 28 + chip income 10 + draw 3 15 = 53"),
    ("frontier_lab_charter", "new, early", "-", "t1 program, 26 cr, 1 ins: Raise your insight income by 2. Raise your credit income by 3.",
     "Science's jump-start: insight income is keeps and plays every round, the scarcest thing early, with credits to "
     "spend it on. No draw, so it is not a cut-down Recursive Self-Improvement (+4 insight, draw 4). Early: 1.32.",
     "26 + 4 x 2 = 34", "2 x 12 = 24 + 3 x 4 = 12 = 36"),
    ("national_grid_buildout", "new, early", "-", "t1 program, 34 cr, 1 ins: Raise your energy income by 2. Place 3 nodes free.",
     "Power's jump-start: energy is every board's shortage and nodes are reach; 3 free nodes in round 1-3 open the map "
     "and uncover 3 track spaces with no energy paid. A node at 6 (Relay Mast Rollout's read; values.json 4: 1.00). "
     "For the Builder the 3 spaces are about +8 credit income, about 1.4. Early: 1.26.",
     "34 + 4 x 2 = 42", "2 x 14 = 28 + 3 nodes 18 = 46"),
    # New: late objective
    ("decapitation_strike", "new, late", "-", "t2 event, 30 cr, 1 ins, needs 2 security: Remove a botnet in reach. Remove 6 adversary ambassadors in reach.",
     "A late objective: the botnet that blocks claims and feeds a project, and the ambassadors around it, in one play. "
     "6 is the median in reach, not a remove-all. It is offered only with a botnet in reach (feasibility). A botnet "
     "at 16 (111's read: values.json 12 + its 2 Sway); at 12, 1.00.",
     "30 + 4 x 3 = 42", "botnet 16 + 6 x 5 = 30 = 46"),
    # Changed: the megaproject late objective raised (Peace Conference and Headquarters Raid stay at r45: see r46.md)
    ("moonshot_grant", "raised, late", "23 cr, 1 ins: 6 research cubes on a megaproject, draw 2",
     "46 cr, 1 ins: 8 research cubes, draw 2",
     "A late objective: 8 cubes fill a megaproject (8 a card) alone, so on an empty project one play completes it "
     "and every cube pays (9, not the 6 of a cube that may not complete). On a part-filled one the rest is lost: "
     "at 6 a cube it reads 1.07.",
     "46 + 4 x 2 = 54", "8 x 9 = 72 + draw 2 10 = 82"),
    # Changed: tier-2 calibration, price only (112's get)
    ("deep_state_ledger", "calibrated", "10 cr, 2 ins (1.96)", "18 cr, 2 ins", "Read 1.96 at r45: toward 1.5.", "18 + 16 = 34", "51"),
    ("dyson_swarm_tranche", "calibrated", "22 cr, 2 ins (1.79)", "29 cr, 2 ins", "Read 1.79: toward 1.5.", "29 + 16 = 45", "68"),
    ("recursive_self_improvement", "calibrated", "24 cr, 2 ins (1.70)", "29 cr, 2 ins", "Read 1.70: toward 1.5.", "29 + 16 = 45", "68"),
    ("kinetic_interdiction", "calibrated", "3 cr, 1 am, 1 ins (1.73)", "6 cr, 1 am, 1 ins", "Read 1.73: toward 1.5.", "6 + 12 + 7 = 25", "38"),
    ("killswitch_directive", "calibrated", "10 cr, 2 ins (1.69)", "14 cr, 2 ins", "Read 1.69: toward 1.5.", "14 + 16 = 30", "44"),
    ("zero_day_auction", "calibrated", "10 cr, 2 ins (1.69)", "14 cr, 2 ins", "Read 1.69: toward 1.5.", "14 + 16 = 30", "44"),
    ("smart_grid_backbone", "calibrated", "20 cr, 2 ins (1.69)", "25 cr, 2 ins", "Read 1.69: toward 1.5.", "25 + 16 = 41", "61"),
    ("exodus_corridors", "calibrated", "13 cr, 2 ins (1.55)", "14 cr, 2 ins", "Read 1.55: toward 1.5.", "14 + 16 = 30", "45"),
    # Cut (they stay in the content, out of the deck)
    ("plant_overdrive", "cut", "9 cr: 1 energy per Industry tag", "-",
     "Redundant: Capacitor Discharge is the same one-off per Power tag, and Industry has Scrap for energy-free "
     "chips; played in 6% of games.", "-", "-"),
    ("grid_census", "cut", "14 cr: 1 energy per node on the board", "-",
     "Redundant: Beamed Power Constellation does the energy-per-node payoff with its nodes; National Grid Buildout "
     "takes its place in Power; played in 8% of games.", "-", "-"),
    ("shadow_broker_feed", "cut", "6 cr (Shadow Broker Scrub): clear 2 corruption markers, draw 2", "-",
     "A near-duplicate of Field Sensor Grid (clear 3, draw 2, 10 cr); played in 11% of games.", "-", "-"),
]


def ratio(pay, get):
    p = float(pay.split("=")[-1])
    g = float(get.split("=")[-1].split()[-1]) if "=" in get else float(get)
    return g / p


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    with open("content/cards.json", encoding="utf-8") as f:
        names = {c["id"]: c["name"] for c in json.load(f)}
    if "--notes" in sys.argv:
        path = "docs/card-pricing-notes.json"
        with open(path, encoding="utf-8") as f:
            notes = json.load(f)
        for cid, kind, old, new, why, pay, get in ROWS:
            if kind == "cut":
                notes.pop(cid, None)
                continue
            r = ratio(pay, get)
            head = {"new, early": "r46 (117): new, an early jump-start",
                    "new, late": "r46 (117): new, a late objective",
                    "raised, late": f"r46 (117): raised as a late objective (r45: {old})",
                    "calibrated": f"r46 (117): raised toward 1.5x (r45: {old})"}[kind]
            notes[cid] = f"{head}; anchors: pay {pay}; get {get} = {r:.2f}."
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(notes, f, indent=2, ensure_ascii=False)
            f.write("\n")
        return
    print("| card | change | r45 | r46 | why | anchor check |")
    print("|---|---|---|---|---|---|")
    for cid, kind, old, new, why, pay, get in ROWS:
        chk = "-" if kind == "cut" else f"pay {pay}; get {get} = {ratio(pay, get):.2f}"
        print(f"| {names[cid]} | {kind} | {old} | {new} | {why} | {chk} |")


if __name__ == "__main__":
    main()
