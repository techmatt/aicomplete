"""139: Discord cards and adversary content destroy no nodes except by the local routes (Matt, 2026-11-06).

    python experiments/results/139_discord_and_band/discord_139.py      # rewrite content/alignment.json's Discord cards and the drafts

A record of 139's rewrite (moved from scripts/content/ at 144): rerunning it now reverts later items' edits.
    python scripts/content/load_adversary.py   # then write content/ and docs/adversary-cards.md

Rerunnable from any revision: every card it touches is set whole from the tables below. Node loss is local only
(uncountered threats near the node, corruption of the node's own hex, war, Border War at the fronts), so:

- the six Discord cards' "each player destroys N of their nodes" steps become board pressure (more or higher-tier
  threats, creation, Corrupt lines, credits and Unity lost) or stripped player pieces (ambassadors lost);
- Offensive deals a tier-2 threat with its token on the node hex with the most adversary ambassadors and troops
  (tier 3 from the 4th Discord tip) instead of destroying every node there;
- Border War's node loss reaches only nodes on or next to a front's base (Matt's message 5847);
- Raiding Parties (a node or 10 credits) and Purge the Unaligned (an ambassador or a node) lose their node option.
"""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
DRAFTS = ROOT / "drafts" / "adversary"
ALIGNMENT = ROOT / "content" / "alignment.json"


def deal(count, tier=None):
    op = {"op": "deal_threats", "count": count}
    if tier:
        op["tier"] = tier
    return op


def create(count):
    return {"op": "create", "count": count, "at": "city", "if": "ambassadors"}


def corrupt_cities(cut=0):
    op = {"op": "corrupt", "site": "city"}
    if cut:
        op["cut"] = cut
    return op


def threat_at_node(tier):
    return {"op": "threat_at_node", "tier": tier}


def lose(resource, amount):
    return {"op": "players_lose", "resource": resource, "amount": amount}


LOSE_AMBASSADOR = {"op": "lose_ambassador"}
FACTORY = {"op": "create_site", "kind": "factory"}
BUILD = {"op": "adversary_build"}

DISCORD = {
    "incursion": (
        "Deal 1 extra threat. 5+ earlier Discord tips: deal 3 instead, and a tier-3 threat in each country.",
        [deal(1)],
        [(5, [deal(3), deal(1, 3)])],
    ),
    "blight": (
        "Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors, and a "
        "neutral factory opens on its lowest-numbered free industrial zone. 5+ earlier Discord tips: create 3 "
        "instead, then Corrupt cities.",
        [create(1), FACTORY],
        [(5, [create(3), FACTORY, corrupt_cities()])],
    ),
    "propaganda": (
        "Every player loses 4 credits; remove 2 Unity from the Unity track. 5+ earlier Discord tips: lose 8 credits "
        "and remove 4 Unity instead, and every player loses 2 ambassadors.",
        [lose("credits", 4), {"op": "pull_unity", "amount": 2}],
        [(5, [lose("credits", 8), {"op": "pull_unity", "amount": 4}, LOSE_AMBASSADOR, LOSE_AMBASSADOR])],
    ),
    "offensive": (
        "Corrupt cities, then deal a tier-2 threat with its token on the node hex with the most adversary "
        "ambassadors and troops. 3+ earlier Discord tips: the threat is tier 3. 4+ instead: the adversary builds "
        "first, then Corrupt cities and the tier-3 threat.",
        [corrupt_cities(), threat_at_node(2)],
        [(3, [corrupt_cities(), threat_at_node(3)]), (4, [BUILD, corrupt_cities(), threat_at_node(3)])],
    ),
    "two_fronts": (
        "Deal 2 extra threats. 3+ earlier Discord tips: a tier-3 threat in each country instead. 4+ instead: deal 3, "
        "and a tier-3 threat in each country.",
        [deal(2)],
        [(3, [deal(1, 3)]), (4, [deal(3), deal(1, 3)])],
    ),
    "firestorm": (
        "Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. 3+ "
        "earlier Discord tips: then Corrupt cities. 4+ instead: create 3, then Corrupt cities (1 fewer).",
        [create(2)],
        [(3, [create(2), corrupt_cities()]), (4, [create(3), corrupt_cities(1)])],
    ),
}

PROJECTS = {
    "raiding_parties": (
        "Every player loses 8 credits.",
        [lose("credits", 8)],
        ["credits or income lost"],
        "absorbs 4 a player (8 to drain); harm 8 credits a player (139: the node half is gone; it was a node or 10, "
        "about 8); 2.0 a compute",
    ),
    "purge_the_unaligned": (
        "Every player loses an ambassador (from their supply, else one on the board). 1 troop lands on a factory in "
        "each country.",
        [LOSE_AMBASSADOR, {"op": "troops", "count": 1, "each_country": True}],
        ["ambassadors lost", "adversary pieces placed"],
        "absorbs 7 a player (14 to drain); harm (a player's share + shared a country): an ambassador (4; 139: no node "
        "option) + a troop and its strike (about 10) = 14; 2.0 a compute",
    ),
}

BORDER_WAR_TEXT = (
    "Each country: a Military 2 threat. The adversary gains 2 compute. In each country, 1 troop lands on a factory "
    "(else a city). Each player with a node on or next to a front's base destroys 1 of those nodes. Each board: "
    "create 2 adversary ambassadors on the lowest-numbered factory. Corrupt factories."
)


def write(path, data):
    path.write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def main():
    cards = json.loads(ALIGNMENT.read_text(encoding="utf-8"))
    seen = set()
    for c in cards:
        if c["id"] in DISCORD:
            text, effects, steps = DISCORD[c["id"]]
            c["text"], c["effects"] = text, effects
            c["steps"] = [{"at": at, "effects": e} for at, e in steps]
            seen.add(c["id"])
    assert seen == set(DISCORD), set(DISCORD) - seen
    write(ALIGNMENT, cards)

    path = DRAFTS / "projects.json"
    deck = json.loads(path.read_text(encoding="utf-8"))
    seen = set()
    for c in deck["cards"]:
        if c["id"] in PROJECTS:
            c["text"], c["effects"], c["harms"], c["value"] = PROJECTS[c["id"]]
            seen.add(c["id"])
    assert seen == set(PROJECTS), set(PROJECTS) - seen
    write(path, deck)

    path = DRAFTS / "militarist.json"
    board = json.loads(path.read_text(encoding="utf-8"))
    found = False
    for c in board["events"]:
        if c.get("id") == "mil_s2_border_war":
            for t in c["twists"]:
                if t["op"] == "each_destroy_node":
                    t["bases"] = True
                    found = True
            c["text"] = BORDER_WAR_TEXT
    assert found
    write(path, board)
    print("rewrote", len(DISCORD), "Discord cards,", len(PROJECTS), "projects and Border War")


if __name__ == "__main__":
    main()
