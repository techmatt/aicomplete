"""231: writes drafts/adversary/projects.json, the project deck (fewer, bigger, one-shot, every board, per-board costs).

python experiments/results/231_project_overhaul/projects_231.py [--write]

Each card's give / get at configs/agents/values.json: give is its per-board cost x 2 countries x the compute row;
get is its ops at their rows (value.rs's arms, at rates.countries 2). Prints the table; --write writes the draft.
A record of its day: later items edit the draft by hand.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
VALUES = json.loads((ROOT / "configs" / "agents" / "values.json").read_text(encoding="utf-8"))
OPS = VALUES["ops"]
RATES = VALUES["rates"]
COUNTRIES = RATES["countries"]
THIRD = 18  # content::THIRD_EFFECT_COST


def row(name):
    return OPS[name]["credits"]


def worth(e):
    """value.rs's arm for each op the deck prints (credits, the players' side: a harm is negative)."""
    op = e["op"]
    c = COUNTRIES
    if op == "sway":
        return -e["amount"] * RATES.get("sway", 3)
    if op == "adversary_cubes":
        return row("adversary_cubes") * c * e["count"]
    if op == "flag":
        # Rows read against the players: the flag row (3) times the countries' energy at stake, about 4 a country.
        return -row("flag") * c * 2
    if op == "spread_botnets":
        return row("spread_botnets") * c * e["count"]
    if op == "deal_threats":
        return row("deal_threats") * e["count"] * (c if e.get("each_country") else 1)
    if op == "agents_on_nodes":
        return row("agents_on_nodes") * c * e["count"]
    if op == "requisition":
        pay = e["amount"] * VALUES["resources"][e["resource"]]
        return row("requisition") * min(pay, e["cards"] * RATES["drawn_card"]) * c
    if op == "recall_ambassadors":
        return row("recall_ambassadors") * c
    if op == "strip_cubes":
        return row("strip_cubes") * c * e["count"]
    if op == "take_site":
        return row("take_site") * c
    if op == "botnet_troops":
        return row("botnet_troops") * c * e["count"] * e["troops"]
    if op == "strip_harden":
        return row("strip_harden") * c
    if op == "rally":
        return -3.3 * e["count"] * c
    if op == "lose_tokens":
        return row("lose_tokens") * e["amount"] * c
    if op == "corrupt":
        return row("corrupt") * (1.5 if e.get("cut") else 1)
    if op == "fallout":
        return row("fallout") * c * RATES.get("target_share", 0.5) * e.get("count", 1)
    if op == "adversary_factory":
        return row("take_site") * c
    if op == "factory_troops":
        return row("factory_troops") * c * e.get("count", 1)
    if op == "increase_income":
        rounds = 4  # about half a game left when a big project lands
        return -(e.get("compute", 0) * c * 1.1 + e.get("ambassadors", 0) * c * 3.3) * rounds
    if op == "pull_unity":
        return row("pull_unity") * e["amount"]
    if op == "lose_ambassador":
        return row("lose_ambassador") * c
    if op == "clock":
        return row("clock") * e["count"]
    if op == "adversary_build":
        return -3.3 * 2 * c
    raise SystemExit(f"no worth for {op}")


def card(id, name, tags, cost, text, effects, harms, when, answer, flags):
    return {"id": id, "name": name, "tags": tags, "cost": cost, "ongoing": False, "text": text, "effects": effects,
            "harms": harms, "when": when, "answer": answer, "flags": flags}


R = "231: reworked (Matt, 2026-11-14: fewer, bigger, one-shot, every board)"
N = "231: new (a ledger gap)"
K = "231: kept, repriced per board"
DECK = [
    # Small: one thing.
    card("disinformation", "Disinformation", ["diplomacy"], 7, "Sway 8 toward Discord.",
         [{"op": "sway", "side": "discord", "amount": 8, "narrative": "fear"}], ["Discord sway"],
         "any round", "drain or Audit it", [R + ": Sway 5 to 8"]),
    card("backdoor_research", "Backdoor Research", ["cyber"], 7,
         "4 adversary research cubes onto each open megaproject.",
         [{"op": "adversary_cubes", "count": 4}], ["adversary cubes on megaprojects", "Discord sway"],
         "while megaprojects are open (early to mid)", "finish megaprojects first; drain", [R + ": 2 to 4 cubes"]),
    card("rolling_blackouts", "Rolling Blackouts", ["power"], 8, "Until the end of next round, cities make no energy.",
         [{"op": "flag", "name": "blackout", "everywhere": True}], ["production blacked out"],
         "any round; worst when energy is short", "bank energy a round ahead (the compute beside it is public)", [K]),
    card("botnet_swarm", "Botnet Swarm", ["cyber"], 8,
         "Each country, twice: a botnet spreads to its lowest-numbered city, research station or factory next to one "
         "of its botnets (no botnet there: its lowest-numbered research station).",
         [{"op": "spread_botnets", "count": 2}], ["adversary pieces placed", "projects advanced (compute)"],
         "any round", "take botnets down; stand ambassadors on them", [R + ": 2 botnets a country"]),
    card("false_flag", "False Flag", ["universal"], 9, "Each country: an extra tier-2 threat, any tag.",
         [{"op": "deal_threats", "count": 1, "each_country": True, "tier": 2}], ["extra threats"],
         "any round", "keep counter resources; drain", [K]),
    card("night_raid", "Night Raid", ["security"], 8,
         "Each country: 1 Agent onto each of two different hexes holding a player node (those holding the most Agents "
         "first; ties: lowest-numbered). 2 Agents on your node's hex destroy your node in the next Corrupt beat.",
         [{"op": "agents_on_nodes", "count": 2}], ["adversary pieces placed", "node destroyed"],
         "mid to late, once agents stand by nodes", "remove agents from node hexes in the action phase; firewall",
         [R + ": agents land, the 2-agent rule destroys (players answer first)"]),
    card("containment_breach", "Containment Breach", ["power"], 7, "Each player pays 1 antimatter or discards 3 cards.",
         [{"op": "requisition", "resource": "antimatter", "amount": 1, "cards": 3}],
         ["antimatter lost", "cards discarded"],
         "mid to late, once antimatter is made", "spend antimatter before it lands, or keep spare cards", [N]),
    card("crackdown", "Crackdown", ["security"], 8,
         "Each country: every player Ambassador on its hex holding the most (ties: lowest-numbered) goes back to its "
         "owner's supply.",
         [{"op": "recall_ambassadors"}], ["pieces pushed or reach cut"],
         "mid to late, when ambassadors guard stacks", "spread ambassadors; Dispatch them back", [N]),
    card("lab_purge", "Lab Purge", ["cyber"], 8,
         "Each open megaproject loses 4 player research cubes, from the player with the most on it first.",
         [{"op": "strip_cubes", "count": 4}], ["a megaproject or converter denied"],
         "while megaprojects are open", "finish a megaproject before it fills", [N]),
    card("asset_seizure", "Asset Seizure", ["industry"], 9,
         "Each country: its neutral factory with the most Agents on or next to it (ties: lowest-numbered) is "
         "corrupted; with none neutral, its player-owned factory picked the same way turns neutral.",
         [{"op": "take_site", "site": "factory"}], ["facilities destroyed or corrupted"],
         "any round", "claim factories; stand ambassadors on them", [K]),
    card("botnet_conscripts", "Botnet Conscripts", ["cyber"], 9,
         "Each country: its lowest-numbered botnet becomes 3 Troops on its hex.",
         [{"op": "botnet_troops", "count": 1, "troops": 3}], ["troops made"],
         "any round with botnets", "take botnets down", [R + ": 2 to 3 troops"]),
    # 231: four more small ones (deck margin, a small-heavy mix).
    card("purge_lists", "Purge Lists", ["diplomacy"], 7, "Each player loses 1 Ambassador (from their supply, else one "
         "of theirs on the board).",
         [{"op": "lose_ambassador"}], ["ambassadors lost"],
         "any round", "keep a spare ambassador in supply", [N.replace("a ledger gap", "small, a deck margin")]),
    card("dirty_bomb", "Dirty Bomb", ["power"], 8, "Each country: 1 Fallout lands next to each of its two "
         "lowest-numbered corrupted cities.",
         [{"op": "fallout", "count": 2, "at": "corrupted_city"}], ["Fallout placed"],
         "mid to late, once cities are corrupted", "liberate cities; Lift Fallout",
         [N.replace("a ledger gap", "small, a deck margin")]),
    card("lab_seizure", "Lab Seizure", ["cyber"], 8, "Corrupt: research stations.",
         [{"op": "corrupt", "site": "station"}], ["adversary pieces placed", "facilities destroyed or corrupted"],
         "any round with agents by stations", "clear agents next to stations",
         [N.replace("a ledger gap", "small, a deck margin")]),
    card("slander_campaign", "Slander Campaign", ["diplomacy"], 7, "Pull 4 Sway back off the Unity track.",
         [{"op": "pull_unity", "amount": 4}], ["Unity pulled back"],
         "mid to late, once Unity leads", "drain it", [N.replace("a ledger gap", "small, a deck margin")]),
    # 231, screen 4: two more, so a d3 game at the frozen dial never reshuffles (it used all 26 once).
    card("coastal_raids", "Coastal Raids", ["security"], 7, "Rally 3, each country: coastal cities.",
         [{"op": "rally", "site": "coastal_city", "count": 3}], ["adversary pieces placed"],
         "any round", "clear coastal stacks; ambassadors on coastal cities",
         [N.replace("a ledger gap", "small, a deck margin")]),
    card("plant_seizure", "Plant Seizure", ["industry"], 7, "Corrupt: factories.",
         [{"op": "corrupt", "site": "factory"}], ["adversary pieces placed", "facilities destroyed or corrupted"],
         "any round with agents by factories", "clear agents next to factories",
         [N.replace("a ledger gap", "small, a deck margin")]),
    # Big: a main effect and a side harm; the costliest (THIRD and up) up to three.
    card("sabotage_teams", "Sabotage Teams", ["power"], 13,
         "Each country: the hardened facility with the most Agents on or next to it (ties: lowest-numbered) loses its "
         "hardening. Until the end of next round, players make no energy from their nodes in upkeep.",
         [{"op": "strip_harden"}, {"op": "flag", "name": "sabotage", "everywhere": True}],
         ["protections stripped", "energy lost"],
         "mid to late", "bank energy; harden again", [R + ": the node raid out, a sabotage round in"]),
    card("mob_rule", "Mob Rule", ["diplomacy"], 14, "Rally 3, each country: cities. Sway 3 toward Discord.",
         [{"op": "rally", "site": "city", "count": 3}, {"op": "sway", "side": "discord", "amount": 3, "narrative": "fear"}],
         ["adversary pieces placed", "Discord sway"],
         "any round", "clear city stacks; drain", [R + ": the node raid out, Discord in"]),
    card("data_heist", "Data Heist", ["cyber"], 14,
         "Each player removes 3 tokens from their cards (their choice of cards). Each player pays 8 credits or "
         "discards 2 cards.",
         [{"op": "lose_tokens", "amount": 3}, {"op": "requisition", "resource": "credits", "amount": 8, "cards": 2}],
         ["tokens stripped from cards", "cards discarded", "credits or income lost"],
         "mid to late", "spend tokens and credits ahead", [K]),
    card("hostile_takeover", "Hostile Takeover", ["industry"], 15, "Rally 3, each country: factories. Corrupt: factories.",
         [{"op": "rally", "site": "factory", "count": 3}, {"op": "corrupt", "site": "factory"}],
         ["adversary pieces placed", "facilities destroyed or corrupted"],
         "any round", "clear factory stacks; ambassadors on factories", [K]),
    card("station_coup", "Station Coup", ["cyber"], 15,
         "Each country: its neutral research station with the most Agents on or next to it (ties: lowest-numbered) is "
         "corrupted; with none neutral, its player-owned station picked the same way turns neutral. Then each "
         "country: 1 Fallout lands next to its lowest-numbered corrupted research station.",
         [{"op": "take_site", "site": "station"}, {"op": "fallout", "at": "corrupted_station"}],
         ["facilities destroyed or corrupted", "Fallout placed"],
         "any round", "claim stations; ambassadors on them", [K]),
    card("war_plants", "War Plants", ["industry"], 17,
         "Each country: the adversary builds a factory on its free industrial zone next to an Agent, else its "
         "lowest-numbered (none free: 2 Agents on one of its factories). Every adversary factory makes 2 Troops on its "
         "hex (a player Ambassador on it stops them).",
         [{"op": "adversary_factory", "count": 1, "where": "each_country", "fallback_ambassadors": 2},
          {"op": "factory_troops", "count": 2}],
         ["facilities destroyed or corrupted", "troops made"],
         "any round", "ambassadors on adversary factories; liberate them", [R + ": every country, not the one with most free zones (Shadow Factories folded in)"]),
    card("recruitment_drive", "Recruitment Drive", ["diplomacy"], 18,
         "Income: +1 Agent a country a round. Rally 2, each country: cities.",
         [{"op": "increase_income", "ambassadors": 1}, {"op": "rally", "site": "city", "count": 2}],
         ["adversary pieces placed"],
         "early is worst", "drain it; clear city stacks", [R + ": a rally now beside the income"]),
    card("propaganda_network", "Propaganda Network", ["universal"], 18,
         "Income: +1 compute a country a round. Sway 6 toward Discord.",
         [{"op": "increase_income", "compute": 1}, {"op": "sway", "side": "discord", "amount": 6, "narrative": "fear"}],
         ["projects advanced (compute)", "Discord sway"],
         "early is worst", "drain it", [R + ": one-shot (was ongoing Sway 2 a round)"]),
    card("infiltration_drive", "Infiltration Drive", ["diplomacy"], 20,
         "Rally 3, each country: cities. Corrupt: cities (1 fewer). Then each country: 1 Fallout lands next to its "
         "lowest-numbered corrupted city.",
         [{"op": "rally", "site": "city", "count": 3}, {"op": "corrupt", "site": "city", "cut": 1},
          {"op": "fallout", "at": "corrupted_city"}],
         ["adversary pieces placed", "facilities destroyed or corrupted", "Fallout placed"],
         "mid to late", "clear city stacks; ambassadors on cities", [R + ": Fallout added"]),
    card("accelerated_timetable", "Accelerated Timetable", ["universal"], 20,
         "Discard the top face-down card of the adversary's event deck, if any: the round cap comes a round sooner. "
         "Each country: an extra tier-2 threat, any tag.",
         [{"op": "clock", "count": 1}, {"op": "deal_threats", "count": 1, "each_country": True, "tier": 2}],
         ["the clock sped up", "extra threats"],
         "any round", "drain it", [R + ": a threat a country added"]),
    card("grand_offensive", "Grand Offensive", ["universal"], 22,
         "Each country rallies its Agent income now, to this round's Rally. Corrupt: cities. Corrupt: factories.",
         [{"op": "adversary_build"}, {"op": "corrupt", "site": "city"}, {"op": "corrupt", "site": "factory"}],
         ["adversary pieces placed", "facilities destroyed or corrupted"],
         "mid to late", "clear stacks by cities and factories", [R + ": Corrupt factories added"]),
]


# The per-board costs, set from the first give / get pass (get / give near 1, a big one's a little under: its
# bundle is the point).
COSTS = {
    "disinformation": 9, "backdoor_research": 7, "rolling_blackouts": 6, "botnet_swarm": 12, "false_flag": 7,
    "night_raid": 8, "containment_breach": 7, "crackdown": 6, "lab_purge": 9, "asset_seizure": 11,
    "botnet_conscripts": 9, "purge_lists": 5, "dirty_bomb": 10, "lab_seizure": 6, "slander_campaign": 7, "coastal_raids": 7, "plant_seizure": 7,
    # Big, +2 each after screen 2 (s231b: potency P adopted, the throughput held near 10 at d2).
    "sabotage_teams": 10, "mob_rule": 14, "data_heist": 12, "hostile_takeover": 15,
    "station_coup": 19, "war_plants": 19, "recruitment_drive": 20, "propaganda_network": 18,
    "infiltration_drive": 22, "accelerated_timetable": 20, "grand_offensive": 22,
}
for _c in DECK:
    _c["cost"] = COSTS[_c["id"]]


def main():
    write = "--write" in sys.argv
    compute = row("compute")
    out = []
    print("| Project | per board | in play (2) | effects | give | get | get / give |")
    print("|---|---|---|---|---|---|---|")
    for c in DECK:
        n = len(c["effects"])
        assert n <= (3 if c["cost"] >= THIRD else 2), c["id"]
        give = c["cost"] * COUNTRIES * -compute
        get = -sum(worth(e) for e in c["effects"])
        size = "big" if n > 1 else "small"
        print(f"| {c['name']} | {c['cost']} | {c['cost'] * COUNTRIES} | {n} ({size}) | {give:.1f} | {get:.1f} | {get / give:.2f} |")
        d = {k: c[k] for k in ("id", "name", "tags", "cost", "ongoing", "text", "effects", "harms")}
        d["value"] = (f"give {c['cost']} compute a board ({c['cost'] * COUNTRIES} at 2 countries, {give:.1f} at values.json "
                      f"compute {-compute}); get {get:.1f}; {get / give:.2f} a compute.")
        d["check"] = (f"231: {n} consequence(s) ({size}; a third only at cost {THIRD} and up), every board, one-shot. "
                      f"When it matters: {c['when']}. Who plays it: automatic for the adversary (compute step); "
                      f"players answer: {c['answer']}.")
        d["flags"] = c["flags"]
        d["reuses"] = None
        out.append(d)
    small = sum(1 for c in DECK if len(c["effects"]) == 1)
    print(f"\n{len(DECK)} projects: {small} small, {len(DECK) - small} big; costs {sorted(c['cost'] for c in DECK)}")
    if write:
        path = ROOT / "drafts" / "adversary" / "projects.json"
        path.write_text(json.dumps({"deck": "projects", "cards": out}, indent=1, ensure_ascii=False) + "\n",
                        encoding="utf-8", newline="\n")
        print(f"wrote {path}")


if __name__ == "__main__":
    main()
