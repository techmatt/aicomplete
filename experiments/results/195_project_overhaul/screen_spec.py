"""195: the driver screen: r85's deck with r84's cut cards put back by part, as content overlays on r85's ids (the
cut ids are gone, so each old card's cost and effects go onto a new card's id). Writes experiments/farm/f195-screen.json.

    python experiments/results/195_project_overhaul/screen_spec.py

Levers: `taxes` (Raiding Parties, Purge the Unaligned, Market Crash, Capital Flight, Grid Capture onto Backdoor
Research, Sabotage Teams, Mob Rule, Accelerated Timetable, Botnet Conscripts) and `troops` (Arms Race, Armored Columns,
Declaration of War onto Factory Garrisons, War Plants, Night Raid). Each replaced card loses its own effect, so a lever
is "the old part instead of these new cards".
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]


def card(pid, cost, effects, ongoing=False, resolve=None):
    out = [f"projects.{pid}.cost={cost}", f"projects.{pid}.effects={json.dumps(effects)}",
           f"projects.{pid}.ongoing={'true' if ongoing else 'false'}"]
    if resolve:
        out.append(f"projects.{pid}.resolve={json.dumps(resolve)}")
    return out


TAXES = (card("backdoor_research", 4, [{"op": "requisition", "resource": "chips", "amount": 3, "cards": 3}])
         + card("sabotage_teams", 7, [{"op": "requisition", "resource": "ambassadors", "amount": 2, "cards": 2}])
         + card("mob_rule", 5, [{"op": "requisition", "resource": "credits", "amount": 10, "cards": 3}])
         + card("accelerated_timetable", 8, [{"op": "team_income", "resource": "credits", "amount": -4}])
         + card("botnet_conscripts", 9, [{"op": "requisition", "resource": "energy", "amount": 1, "cards": 1}],
                ongoing=True))
TROOPS = (card("factory_garrisons", 11, [{"op": "corp_troops", "count": 1, "at": "every_base",
                                          "when": "each_production", "past_cap": True}], ongoing=True)
          + card("war_plants", 10, [{"op": "troops", "count": 3, "each_country": True}])
          + card("night_raid", 18, [{"op": "extra_war", "front": "most_corp_troops", "ongoing_use": True}],
                 ongoing=True, resolve=[{"op": "troops", "count": 3, "each_country": True}]))
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]

variants = []
for lever, sets in [("", []), ("_taxes", TAXES), ("_troops", TROOPS)]:
    for pair, base in PAIRS:
        variants.append({"name": pair + lever, "set": base + sets})
spec = {
    "name": "f195-screen",
    "note": "195: the driver screen: r84's cut taxes and untied troops put back onto r85 ids, by part (screen_spec.py)",
    "configs": ["configs/baseline.json"],
    "variants": variants,
    "difficulties": [2, 3],
    "agents": ["strategist"],
    "games": 32,
    "records": 0,
}
(ROOT / "experiments" / "farm" / "f195-screen.json").write_text(json.dumps(spec, indent=2) + "\n", encoding="utf-8",
                                                                 newline="\n")
print("wrote experiments/farm/f195-screen.json")
