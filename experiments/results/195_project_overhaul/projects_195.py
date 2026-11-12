"""195: the project deck rewritten as the adversary's small, visible plans on the board (Matt, 2026-11-11).

    python experiments/results/195_project_overhaul/projects_195.py          # write drafts/adversary/projects.json
    python experiments/results/195_project_overhaul/projects_195.py --table  # print the design check (give / get)

Each card's give / get is priced at configs/agents/values.json: the compute it absorbs (its cost a player, at
`ops.compute` 1.1 a point) against the harm its ops do at `rates.countries` (2). A record of 195's day: later items
edit drafts/adversary/projects.json by hand.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
VALUES = json.loads((ROOT / "configs" / "agents" / "values.json").read_text(encoding="utf-8"))
OPS = VALUES["ops"]
COUNTRIES = VALUES["rates"]["countries"]
COMPUTE = -OPS["compute"]["credits"]
ROUNDS_ONGOING = 4  # an ongoing project acts about 4 rounds (151's reckoning)

RAID = "Each country: the node hex holding the most Agents (at least 1; ties: lowest-numbered) loses its node."


def row(name):
    return -OPS[name]["credits"]


def harm(op):
    """The harm one op does at the table's rates (credits), and its words."""
    k = op["op"]
    if k == "factory_troops":
        return row("factory_troops") * COUNTRIES * op.get("count", 1)
    if k == "botnet_troops":
        return row("botnet_troops") * COUNTRIES * op.get("count", 1) * op["troops"]
    if k == "spread_botnets":
        return row("spread_botnets") * COUNTRIES * op["count"]
    if k in ("take_site", "raid_node", "strip_harden"):
        return row(k) * COUNTRIES
    if k == "clock":
        return row("clock") * op.get("count", 1)
    if k == "adversary_cubes":
        return row("adversary_cubes") * COUNTRIES * op["count"]
    if k == "adversary_factory":
        return row("adversary_factory") * op.get("count", 1)
    if k == "rally":
        return row("rally") * COUNTRIES * op["count"]
    if k == "corrupt":
        return row("corrupt")
    if k == "corp_troops":
        return 16  # War Footing: 151's reckoning, kept
    if k == "increase_income":
        return 22  # Recruitment Drive: 151's reckoning, kept
    if k == "sway":
        return VALUES["rates"]["sway"] * op["amount"]
    if k == "flag":
        return 18  # Rolling Blackouts: 151's reckoning, kept
    if k == "deal_threats":
        return 14 * COUNTRIES  # False Flag: a tier-2 threat a country (151: 28)
    if k == "lose_tokens":
        return 9  # Data Heist: 151's reckoning, kept
    if k == "requisition":
        return 8  # Data Heist: 189's reckoning of 8 credits or 2 cards
    if k == "adversary_build":
        return 12
    raise SystemExit(f"no price for {k}")


META = {}


def card(id, name, tag, cost, text, effects, harms, when, play, resolve=None, ongoing=False, flags=None):
    once = sum(harm(e) for e in (resolve or []))
    each = sum(harm(e) for e in effects)
    total = once + each * (ROUNDS_ONGOING if ongoing else 1)
    give = cost * COUNTRIES * COMPUTE
    value = (f"give {cost} compute a player ({cost * COUNTRIES} at 2 players, {give:.1f} at values.json compute "
             f"{COMPUTE}); get {total:.0f}{' over about 4 rounds' if ongoing else ''}; {total / (cost * COUNTRIES):.2f} "
             f"a compute.")
    META[id] = (give, total, when, play)
    c = {"id": id, "name": name, "tags": [tag], "cost": cost, "ongoing": ongoing, "text": text, "effects": effects}
    if resolve:
        c["resolve"] = resolve
    c.update({
        "harms": harms,
        "value": value,
        "check": f"195: {len(effects) + len(resolve or [])} consequence(s) (two at most), targets read off the board "
                 f"at completion. When it matters: {when} Who plays it: {play}",
        "flags": flags or ["195: board verb (Matt, 2026-11-11)"],
        "reuses": None,
    })
    return c


TROOP_HARM = "troops made"

CARDS = [
    # ---- security: the troops, tied to adversary factories ----
    card("factory_garrisons", "Factory Garrisons", "security", 12,
         "Ongoing: each round, every adversary factory musters 1 Troop on its hex (a player Ambassador on it stops it).",
         [{"op": "factory_troops", "count": 1}], [TROOP_HARM],
         "late: adversary factories come from stage-II corruption and Shadow Factories / War Plants; worth little "
         "before round 5.",
         "automatic for the adversary (Income beat); players answer with an Ambassador on the factory, liberating "
         "it, or draining.", ongoing=True),
    card("night_raid", "Night Raid", "security", 9,
         "Corrupt: nodes. " + RAID,
         [{"op": "corrupt", "site": "node"}, {"op": "raid_node"}], ["node destroyed", "adversary pieces placed"],
         "mid to late: needs agents beside node hexes (from round 3 on).",
         "automatic; players clear agents off and beside their node hexes, armor them, or drain."),
    card("war_footing", "War Footing", "security", 7,
         "Every military base fills with Drones to its cap.",
         [{"op": "corp_troops", "fill": True, "at": "every_base", "when": "now", "past_cap": False}],
         [TROOP_HARM],
         "any round with active fronts (2+ countries); kept from 151.",
         "automatic; players stand Ambassadors on bases.", flags=["151 card, kept by 195 (board pieces)"]),
    # ---- industry: factories, the adversary's engine ----
    card("shadow_factories", "Shadow Factories", "industry", 7,
         "The adversary builds a factory in the country with the most free industrial zones (none free: 2 Agents on "
         "a factory).",
         [{"op": "adversary_factory", "count": 1, "where": "most_free_zones", "fallback_ambassadors": 2}],
         ["facilities destroyed or seized", "troops made"],
         "any round; its troop a round makes it worth more early.",
         "automatic; players liberate the factory or stand an Ambassador on it.",
         flags=["151 card, kept by 195 (factories are the engine)"]),
    card("hostile_takeover", "Hostile Takeover", "industry", 8,
         "Rally 2, each country: factories. Corrupt: factories.",
         [{"op": "rally", "site": "factory", "count": 2}, {"op": "corrupt", "site": "factory"}],
         ["adversary pieces placed", "facilities destroyed or seized"],
         "any round: factories with agents beside them corrupt; strongest once agents gather (round 3 on).",
         "automatic; players remove agents near factories, firewall them, or drain."),
    card("asset_seizure", "Asset Seizure", "industry", 9,
         "Each country: its neutral factory with the most Agents on or next to it (ties: lowest-numbered) goes to "
         "the adversary; with none neutral, its player-owned factory with the most turns neutral.",
         [{"op": "take_site", "site": "factory"}], ["facilities destroyed or seized"],
         "any round: early it takes a neutral factory (a troop a round from then on); late, a claimed one.",
         "automatic; players claim neutral factories first, or firewall the target."),
    card("war_plants", "War Plants", "industry", 11,
         "The adversary builds a factory in the country with the most free industrial zones (none free: 2 Agents on "
         "a factory). Every adversary factory musters 1 Troop on its hex.",
         [{"op": "adversary_factory", "count": 1, "where": "most_free_zones", "fallback_ambassadors": 2},
          {"op": "factory_troops", "count": 1}],
         ["facilities destroyed or seized", TROOP_HARM],
         "mid to late: its troops scale with the adversary factories standing.",
         "automatic; players block or liberate adversary factories."),
    # ---- cyber: botnets ----
    card("botnet_swarm", "Botnet Swarm", "cyber", 9,
         "Each country: a botnet spreads to its lowest-numbered city, research station or factory next to one of its "
         "botnets (no botnet there: its lowest-numbered research station).",
         [{"op": "spread_botnets", "count": 1}], ["adversary pieces placed", "projects advanced (compute)"],
         "early to mid: a botnet makes compute every round and blocks the purge until removed.",
         "automatic; players take botnets down or stand on them."),
    card("station_coup", "Station Coup", "cyber", 9,
         "Each country: its neutral research station with the most Agents on or next to it (ties: lowest-numbered) "
         "goes to the adversary; with none neutral, its player-owned station with the most turns neutral.",
         [{"op": "take_site", "site": "station"}], ["facilities destroyed or seized"],
         "any round; an adversary station puts cubes on its megaproject until it completes, then compute.",
         "automatic; players claim stations or firewall them."),
    card("backdoor_research", "Backdoor Research", "cyber", 5,
         "2 adversary research cubes onto each open megaproject.",
         [{"op": "adversary_cubes", "count": 2}], ["adversary cubes on megaprojects", "Discord sway"],
         "early to mid: before the megaprojects complete (each cube pays its adversary payout at completion).",
         "automatic; players finish megaprojects with their own cubes."),
    card("data_heist", "Data Heist", "cyber", 6,
         "Each player removes 3 tokens from their cards (their choice of cards). Each player pays 8 credits or "
         "discards 2 cards.",
         [{"op": "lose_tokens", "amount": 3}, {"op": "requisition", "resource": "credits", "amount": 8, "cards": 2}],
         ["tokens stripped from cards", "cards discarded", "credits or income lost"],
         "mid: once cards hold tokens.", "automatic; each player chooses which tokens, and pays or discards.",
         flags=["151 card, kept by 195 as the deck's one tax ('cut or nearly cut'): the only project source of "
                "tokens stripped and credits lost, which would otherwise open two ledger gaps"]),
    card("botnet_conscripts", "Botnet Conscripts", "cyber", 7,
         "Each country: its lowest-numbered botnet becomes 2 Troops on its hex.",
         [{"op": "botnet_troops", "count": 1, "troops": 2}], [TROOP_HARM],
         "mid to late: the troops strike a station or factory near the botnet within a round or two.",
         "automatic; players remove the troops, harden or armor the target."),
    # ---- diplomacy: agents and cities ----
    card("recruitment_drive", "Recruitment Drive", "diplomacy", 11,
         "Income: +1 Agent a country a round.",
         [{"op": "increase_income", "ambassadors": 1}], ["adversary pieces placed"],
         "early: worth more the more rounds are left.", "automatic.",
         flags=["151 card, kept by 195 (creation)"]),
    card("infiltration_drive", "Infiltration Drive", "diplomacy", 14,
         "Rally 3, each country: cities. Corrupt: cities (1 fewer).",
         [{"op": "rally", "site": "city", "count": 3}, {"op": "corrupt", "site": "city", "cut": 1}],
         ["adversary pieces placed", "facilities destroyed or seized"],
         "mid: cities with 2 agents beside them corrupt.", "automatic.",
         flags=["151 card, kept by 195 (board verb); 18 to 14 a player"]),
    card("mob_rule", "Mob Rule", "diplomacy", 8,
         "Rally 2, each country: cities. " + RAID,
         [{"op": "rally", "site": "city", "count": 2}, {"op": "raid_node"}],
         ["adversary pieces placed", "node destroyed"],
         "mid to late: a node on a city hex takes the rally's agents and is raided.",
         "automatic; players keep agents off their node hexes."),
    card("disinformation", "Disinformation", "diplomacy", 5,
         "Sway 5 toward Discord.",
         [{"op": "sway", "side": "discord", "amount": 5}], ["Discord sway"],
         "any round: brings a Discord tip nearer.", "automatic.",
         flags=["151 card, kept by 195 (one of the few off-board cards)"]),
    # ---- power ----
    card("rolling_blackouts", "Rolling Blackouts", "power", 6,
         "Until the end of next round, cities make no energy.",
         [{"op": "flag", "name": "blackout", "everywhere": True}], ["production blacked out"],
         "early to mid, while energy is short.", "automatic.",
         flags=["151 card, kept by 195"]),
    card("sabotage_teams", "Sabotage Teams", "power", 6,
         "Each country: the hardened facility with the most Agents on or next to it (ties: lowest-numbered) loses its "
         "hardening. " + RAID,
         [{"op": "strip_harden"}, {"op": "raid_node"}], ["protections stripped", "node destroyed"],
         "mid to late: hardening comes from cards; a raid needs an agent on a node hex.",
         "automatic; players clear agents off node hexes."),
    # ---- universal ----
    card("false_flag", "False Flag", "universal", 6,
         "Each country: an extra tier-2 threat, any tag.",
         [{"op": "deal_threats", "count": 1, "each_country": True, "tier": 2}], ["extra threats"],
         "any round.", "automatic; players counter.", flags=["151 card, kept by 195"]),
    card("grand_offensive", "Grand Offensive", "universal", 10,
         "Each country rallies its Agent income now, to this round's Rally. Corrupt: cities.",
         [{"op": "adversary_build"}, {"op": "corrupt", "site": "city"}],
         ["adversary pieces placed", "facilities destroyed or seized"],
         "mid.", "automatic.", flags=["151 card, kept by 195 (board verb)"]),
    card("propaganda_network", "Propaganda Network", "universal", 14,
         "Now: Income: +2 compute a round. Ongoing: Sway 2 toward Discord.",
         [{"op": "sway", "side": "discord", "amount": 2}], ["Discord sway", "projects advanced (compute)"],
         "early: the income and the sway both run for the rest of the game.", "automatic.",
         resolve=[{"op": "increase_income", "compute": 2}], ongoing=True, flags=["151 card, kept by 195"]),
    card("accelerated_timetable", "Accelerated Timetable", "universal", 14,
         "Discard the top card of the adversary's event deck: the round cap comes a round sooner.",
         [{"op": "clock", "count": 1}], ["the clock sped up"],
         "late: a round less to finish the purge (at the cap, a loss).",
         "automatic; players drain it."),
]


def main():
    if "--table" in sys.argv:
        print("| Project | Tag | Cost a player | Give at 2 players (credits) | Get (credits) | Get a compute | What the nearest "
              "player sees | When it matters (8-10 rounds) | Who plays it |")
        print("|---|---|---|---|---|---|---|---|---|")
        for c in CARDS:
            give, total, when, play = META[c["id"]]
            ongoing = " (ongoing, 4 rounds)" if c["ongoing"] else ""
            print(f"| {c['name']} | {c['tags'][0]} | {c['cost']} | {give:.1f} | {total:.0f}{ongoing} | "
                  f"{total / give:.2f} | {c['text']} | {when} | {play} |")
        return
    out = {"deck": "projects", "cards": CARDS}
    path = ROOT / "drafts" / "adversary" / "projects.json"
    path.write_text(json.dumps(out, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {len(CARDS)} projects to {path}")


if __name__ == "__main__":
    main()
