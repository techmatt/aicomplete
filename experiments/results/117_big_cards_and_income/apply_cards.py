"""117: apply r46's card list and printed incomes (run once, from the repo root, on r45's content).

    python experiments/results/117_big_cards_and_income/apply_cards.py

Edits content/cards.json (new cards appended, changed cards in place), configs/baseline.json (new ids appended to
the deck, cut ids removed: they stay in the content, "not in deck") and content/ais.json (printed credit income).
The card list with its reasoning and anchor checks is CARDS below; `card_list.py` prints it for the write-up.
"""

import json

INCOME = {"builder": 18, "protector": 16, "researcher": 14}

# New cards. Anchors (docs/card-guidelines.md): pay = credits + 4 x (keep + play insight) + other costs;
# get = effects at the anchors (+1 credit income 4, +1 insight income 12, +1 energy income 14, +1 chip income 10,
# a draw 5, an ambassador removed 5, an ambassador gained 4, a Sway 4, a node placed free 6 (Relay Mast Rollout's
# read), a research cube 6 (Moonshot Grant's read), a botnet removed 16 (111's Botnet Takedown read)).
NEW = [
    {"id": "fabrication_megaplex", "name": "Fabrication Megaplex", "tier": 1, "kind": "building", "cost": 34,
     "insight": 1, "tags": ["industry"],
     "passives": [{"op": "income", "resource": "credits", "amount": 7},
                  {"op": "income", "resource": "chips", "amount": 1}],
     "effects": [{"op": "draw", "count": 3}]},
    {"id": "frontier_lab_charter", "name": "Frontier Lab Charter", "tier": 1, "kind": "program", "cost": 26,
     "insight": 1, "tags": ["science"],
     "effects": [{"op": "production", "resource": "insight", "amount": 2},
                 {"op": "production", "resource": "credits", "amount": 3}]},
    {"id": "national_grid_buildout", "name": "National Grid Buildout", "tier": 1, "kind": "program", "cost": 34,
     "insight": 1, "tags": ["power"],
     "effects": [{"op": "production", "resource": "energy", "amount": 2},
                 {"op": "place_node", "count": 3, "free": True}]},
    {"id": "decapitation_strike", "name": "Decapitation Strike", "tier": 2, "kind": "event", "cost": 30,
     "insight": 1, "needs": [["security", 2]], "tags": ["security"],
     "effects": [{"op": "remove_botnet"},
                 {"op": "remove_ambassadors_country", "unity_per": 0, "max": 6, "reach": True}]},
]

# Changed cards: id -> the fields that change.
CHANGED = {
    # The megaproject late objective, raised to fill a project in one play (Matt: "we have some already, but they
    # aren't expensive enough"). Peace Conference and Headquarters Raid were drafted raised and put back: Sway past
    # one tip is lost (alignment.rs tip() empties the bucket), and Raid's 6 duplicated Decapitation Strike.
    "moonshot_grant": {"cost": 46, "effects": [{"op": "project_cube", "count": 8}, {"op": "draw", "count": 2}]},
    # The eight tier-2 cards at 1.55-1.96x their anchors, toward 1.5x (price only).
    "deep_state_ledger": {"cost": 18},
    "dyson_swarm_tranche": {"cost": 29},
    "recursive_self_improvement": {"cost": 29},
    "kinetic_interdiction": {"cost": 6},
    "killswitch_directive": {"cost": 14},
    "zero_day_auction": {"cost": 14},
    "smart_grid_backbone": {"cost": 25},
    "exodus_corridors": {"cost": 14},
}

CUT = ["plant_overdrive", "grid_census", "shadow_broker_feed"]


def load(p):
    with open(p, encoding="utf-8") as f:
        return json.load(f)


def save(p, obj, indent):
    with open(p, "w", encoding="utf-8", newline="\n") as f:
        json.dump(obj, f, indent=indent, ensure_ascii=False)
        f.write("\n")


def main():
    cards = load("content/cards.json")
    ids = {c["id"] for c in cards}
    for c in cards:
        if c["id"] in CHANGED:
            c.update(CHANGED[c["id"]])
    for n in NEW:
        assert n["id"] not in ids, n["id"]
        cards.append(n)
    save("content/cards.json", cards, 1)

    base = load("configs/baseline.json")
    mod = next(m for m in base["modules"] if m.get("module") == "cards")
    deck = [i for i in mod["cards"] if i not in CUT]
    assert len(deck) == len(mod["cards"]) - len(CUT)
    deck += [n["id"] for n in NEW]
    mod["cards"] = deck
    save("configs/baseline.json", base, 2)

    ais = load("content/ais.json")
    for a in ais:
        a["credits"] = INCOME[a["id"]]
    save("content/ais.json", ais, 1)


if __name__ == "__main__":
    main()
