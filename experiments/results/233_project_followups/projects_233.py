"""233: 231's picks on the project deck, as one overlay on drafts/adversary/projects.json.

python experiments/results/233_project_followups/projects_233.py [--write] [--rumor-cost N] [--no-potency] [--no-small]

Every value it sets is absolute (no "+1" on whatever is there), so it reruns from any revision of the draft:
  - big-project potency at the same costs (231's screen-5 overlay);
  - the deck to 30: Rumor Mill (small) and Martial Law (big);
  - each of 231's 17 small projects' cost +2: the prompt's +1, then +1 more after the fresh read f233 resolved 6.5
    small a d2 game (screen s233c; Matt's message 10433); Rumor Mill, drafted at 6, printed at 8 (amendment 10375's
    "+1 applies", and s233c's +1);
  - the `ongoing` key dropped from every card (233: the field is gone from `ProjectCard`).
Prints each touched card's give / get at configs/agents/values.json (231's `worth`, value.rs's arms at 2 countries);
--write rewrites the draft. `--no-potency` / `--no-small` leave that part at 231's numbers (screen cells).
"""
import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
_spec = importlib.util.spec_from_file_location(
    "projects_231", ROOT / "experiments" / "results" / "231_project_overhaul" / "projects_231.py")
P231 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(P231)
row, COUNTRIES, THIRD = P231.row, P231.COUNTRIES, P231.THIRD


def worth(e):
    """231's mirror of value.rs, with 233's fix: a project's Corrupt lands in each country (aic-core's
    `project_ops_reach_as_declared`); value.rs prices it at a threat's one country."""
    w = P231.worth(e)
    return w * COUNTRIES if e["op"] == "corrupt" else w

DRAFT = ROOT / "drafts" / "adversary" / "projects.json"

# 231's printed per-board costs of its 17 small projects; 233 prints each 2 higher.
SMALL_231 = {
    "disinformation": 9, "backdoor_research": 7, "rolling_blackouts": 6, "botnet_swarm": 12, "false_flag": 7,
    "night_raid": 8, "containment_breach": 7, "crackdown": 6, "lab_purge": 9, "asset_seizure": 11,
    "botnet_conscripts": 9, "purge_lists": 5, "dirty_bomb": 10, "lab_seizure": 6, "slander_campaign": 7,
    "coastal_raids": 7, "plant_seizure": 7,
}

FEAR = {"op": "sway", "side": "discord", "narrative": "fear"}

# Big-project potency at 231's costs (231's screen-5 overlay, never read before 233).
POTENCY = {
    "mob_rule": ("Rally 3, each country: cities. Sway 5 toward Discord.",
                 [{"op": "rally", "site": "city", "count": 3}, {**FEAR, "amount": 5}], "Sway 3 to 5"),
    "data_heist": ("Each player removes 4 tokens from their cards (their choice of cards). Each player pays 10 credits "
                   "or discards 3 cards.",
                   [{"op": "lose_tokens", "amount": 4},
                    {"op": "requisition", "resource": "credits", "amount": 10, "cards": 3}],
                   "4 tokens (was 3); 10 credits or 3 cards (was 8 or 2)"),
    "hostile_takeover": ("Rally 3, each country: factories. Corrupt: factories (1 fewer).",
                         [{"op": "rally", "site": "factory", "count": 3}, {"op": "corrupt", "site": "factory", "cut": 1}],
                         "corrupts with 1 fewer"),
    "recruitment_drive": ("Income: +1 Agent a country a round. Rally 3, each country: cities.",
                          [{"op": "increase_income", "ambassadors": 1}, {"op": "rally", "site": "city", "count": 3}],
                          "Rally 2 to 3"),
    "propaganda_network": ("Income: +1 compute a country a round. Sway 8 toward Discord.",
                           [{"op": "increase_income", "compute": 1}, {**FEAR, "amount": 8}], "Sway 6 to 8"),
    "grand_offensive": ("Each country rallies its Agent income now, to this round's Rally. Corrupt: cities (1 fewer). "
                        "Corrupt: factories (1 fewer).",
                        [{"op": "adversary_build"}, {"op": "corrupt", "site": "city", "cut": 1},
                         {"op": "corrupt", "site": "factory", "cut": 1}],
                        "both corrupts with 1 fewer"),
}
P = "233: potency at the same cost (231's screen-5 overlay)"


def new_cards(rumor_cost):
    def card(id, name, tags, cost, text, effects, harms, when, answer):
        return {"id": id, "name": name, "tags": tags, "cost": cost, "text": text, "effects": effects, "harms": harms,
                "when": when, "answer": answer, "flags": ["233: new (231's draft; the deck to 30)"], "reuses": None}
    return [
        card("rumor_mill", "Rumor Mill", ["diplomacy"], rumor_cost, "Sway 4 toward Discord.",
             [{**FEAR, "amount": 4}], ["Discord sway"], "any round", "drain or Audit it"),
        card("martial_law", "Martial Law", ["security"], 15,
             "Rally 3, each country: research stations. Corrupt: research stations.",
             [{"op": "rally", "site": "station", "count": 3}, {"op": "corrupt", "site": "station"}],
             ["adversary pieces placed", "facilities destroyed or corrupted"],
             "any round", "clear station stacks; ambassadors on stations"),
    ]


def lines(c):
    n = len(c["effects"])
    size = "big" if n > 1 else "small"
    assert n <= (3 if c["cost"] >= THIRD else 2), c["id"]
    give = c["cost"] * COUNTRIES * -row("compute")
    get = -sum(worth(e) for e in c["effects"])
    c["value"] = (f"give {c['cost']} compute a board ({c['cost'] * COUNTRIES} at 2 countries, {give:.1f} at values.json "
                  f"compute {-row('compute')}); get {get:.1f}; {get / give:.2f} a compute.")
    if "when" in c:
        c["check"] = (f"233: {n} consequence(s) ({size}; a third only at cost {THIRD} and up), every board, one-shot. "
                      f"When it matters: {c.pop('when')}. Who plays it: automatic for the adversary (compute step); "
                      f"players answer: {c.pop('answer')}.")
    return size, give, get


def main():
    args = sys.argv[1:]
    rumor_cost = int(args[args.index("--rumor-cost") + 1]) if "--rumor-cost" in args else 8
    deck = json.loads(DRAFT.read_text(encoding="utf-8"))
    cards = [c for c in deck["cards"] if c["id"] not in ("rumor_mill", "martial_law")]
    by_id = {c["id"]: c for c in cards}
    touched = []
    for c in cards:
        c.pop("ongoing", None)
    if "--no-small" not in args:
        for id, cost in SMALL_231.items():
            c = by_id[id]
            c["cost"] = cost + 2
            c["flags"] = [f for f in c["flags"] if not f.startswith("233:")] + [
                "233: cost +2 (231's d2 resolved 7.0 small against 5-6; +1 still resolved 6.5 on f233)"]
            touched.append(c)
    if "--no-potency" not in args:
        for id, (text, effects, what) in POTENCY.items():
            c = by_id[id]
            c["text"], c["effects"] = text, effects
            c["flags"] = [f for f in c["flags"] if not f.startswith("233:")] + [f"{P}: {what}"]
            touched.append(c)
    new = new_cards(rumor_cost)
    # Small after the small ones, big after the big ones (id order feeds play; the order here is the printed one).
    last_small = max(i for i, c in enumerate(cards) if len(c["effects"]) == 1)
    cards.insert(last_small + 1, new[0])
    cards.append(new[1])
    touched += new
    print("| Project | per board | in play (2) | effects | give | get | get / give |")
    print("|---|---|---|---|---|---|---|")
    for c in cards:
        if c not in touched:
            lines(c)  # every card's value line at 233's worth (Corrupt in each country)
    for c in touched:
        size, give, get = lines(c)
        print(f"| {c['name']} | {c['cost']} | {c['cost'] * COUNTRIES} | {len(c['effects'])} ({size}) | {give:.1f} | "
              f"{get:.1f} | {get / give:.2f} |")
    small = sum(1 for c in cards if len(c["effects"]) == 1)
    print(f"\n{len(cards)} projects: {small} small, {len(cards) - small} big; costs {sorted(c['cost'] for c in cards)}")
    if "--write" in args:
        deck["cards"] = cards
        DRAFT.write_text(json.dumps(deck, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        print(f"wrote {DRAFT}")


if __name__ == "__main__":
    main()
