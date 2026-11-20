"""297 (overhaul 5, the content sweep): the content edits, as one pass over the sources as they stood at r142.

    git checkout 2282b5387d -- drafts/adversary content/alignment.json content/ais.json   # the r142 sources
    python scripts/content/sweep_297.py     # edit drafts/adversary/*.json, content/alignment.json, content/ais.json
    python scripts/cards/apply_overlay.py experiments/results/297_overhaul_5_content_sweep/cards.overlay
    python scripts/regen.py

Items 1-4 made content legal with the smallest rewording; this pass does the design (Matt, 2026-11-19): an event
card is Infiltrate X, its Strike or Redeploy icon and one other line at most; no project picks by "most agents";
a threat's name says what it does (the cube lines 294 put under troop names are renamed or redesigned); troops
come from projects and Discord cards by a condition the table can see (a botnet, a corrupted factory); a Discord
card feeds the research row by the corrupted research stations; four Unity cards remove Discord, each by a board
condition. It writes the old -> new table of the write-up. On sources already passed it changes nothing.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts" / "cards"))
from apply_overlay import dump_alignment  # noqa: E402  (alignment.json's hand layout)

FLAG = "297 (Matt, 2026-11-19): "
LOG = []  # (deck, card, old, new, why)

STRIKE = "Strike: each Troop destroys 1 player Node, research station or factory on or next to it."
GARRISON = ("Each country: {n} garrison{s} its lowest-numbered corrupted factory (with none, its lowest-numbered "
            "corrupted city; with none, its lowest-numbered city).")
BESIEGE = ("Each country: {n} land{s} on its lowest-numbered corrupted city with a Node or an Ambassador on or next "
           "to it (with none, {it} garrison{s}).")
BOTNETS = ("Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one "
           "of its Botnets (a country with none: its lowest-numbered research station).")
FUNDING = "Income: +1 compute a country a round."
FALLOUT = "Each country: 1 Fallout lands next to its lowest-numbered corrupted city."
OWNED = "research station or factory a player owns, or free city with a player's Node on or next to it"


def load(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def dump(path, data):
    (ROOT / path).write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def flag(card, text):
    if FLAG + text not in card.setdefault("flags", []):
        card["flags"].append(FLAG + text)


def log(deck, card, old, new, why):
    LOG.append((deck, card, old, new, why))


def set_harms(card, drop=(), add=()):
    harms = [h for h in card.get("harms", []) if h not in drop]
    card["harms"] = harms + [h for h in add if h not in harms]


# --- event cards -----------------------------------------------------------------------------------------------
# id -> (the line cut, the ops cut from `standing` / `twists`, harms dropped, why)
EVENT_CUTS = {
    "mil_s1_war_drums": ("Sway 1 toward Discord.", ["sway"], ["Discord sway"],
                         "a 1-Sway rider nobody plans around: the card is Infiltrate X alone"),
    "mil_s1_signal_jamming": (BOTNETS, ["spread_botnets"], [],
                              "two other lines became one: it keeps Funding; Long Patrols and Station Raids carry Botnets spread"),
    "mil_s2_fifth_column": (BOTNETS, ["spread_botnets"], [],
                            "its Strike icon is its one extra; Botnets spread stays on Long Patrols and Station Raids"),
    "mil_s3_total_mobilization": ("Remove 2 Unity from the Unity track.", ["pull_unity"], ["Unity pulled back"],
                                  "a stage-III card already reads two Infiltrates and a Strike"),
    "mil_s3_blitzkrieg": ("Sway 1 toward Discord.", ["sway"], ["Discord sway"],
                          "two other lines became one: it keeps its Fallout"),
    "mil_s3_naval_blockade": (FALLOUT, ["fallout"], ["Fallout placed"],
                              "two other lines became one: it keeps the Blackout a blockade is"),
}
BORDER_OLD = "Each player with a Node on or next to a factory destroys 1 of those Nodes."


def events(mil):
    for e in mil["events"]:
        cut = EVENT_CUTS.get(e["id"])
        if cut and cut[0] in e["text"]:
            line, ops, harms, why = cut
            old = e["text"]
            e["text"] = old.replace(" " + line, "")
            e["standing"] = [x for x in e["standing"] if x["op"] not in ops]
            e["twists"] = [x for x in e["twists"] if x["op"] not in ops]
            set_harms(e, drop=harms)
            flag(e, f'"{line}" cut ({why})')
            log("Event cards", e["name"], old, e["text"], why)
        if e["id"] == "mil_s2_border_war" and BORDER_OLD in e["text"]:
            old = e["text"]
            why = ("the one node loss an event printed with no troop behind it becomes a third stage-II Strike icon: "
                   "nodes fall to troops, which the table sees coming")
            e["text"] = old.replace(BORDER_OLD, STRIKE)
            e["twists"] = [x for x in e["twists"] if x["op"] != "each_destroy_node"]
            e["standing"].append({"op": "strike"})
            set_harms(e, add=["facilities destroyed or corrupted", "node destroyed"])
            flag(e, f'"{BORDER_OLD}" became "{STRIKE}" ({why})')
            log("Event cards", e["name"], old, e["text"], why)


# --- threats ---------------------------------------------------------------------------------------------------
CUBE_WHY = "297: a research feeder by name now"


def research(n):
    return {"op": "adversary_research", "amount": n, "why": "297: this threat is a research feeder by name; its cubes bring the row's Garrison and Strike slots nearer"}


def rename(c, name, why, value, check):
    """A display rename (the id stays): the cube line 294 put under a troop's name gets a name that says it."""
    if c["name"] == name:
        return
    old = f'{c["name"]}: {c["text"]}'
    flag(c, f"display name {c['name']} -> {name} (the id stays): {why}")
    c["name"] = name
    c["effects"] = [research(e["amount"]) if e["op"] == "adversary_research" else e for e in c["effects"]]
    c["value"], c["check"] = value, check
    log("Threats", name, old, f'{name}: {c["text"]}', why)


def redo(c, text, effects, harms, why, value, check):
    if c["text"] == text:
        return
    old = c["text"]
    c["text"], c["effects"], c["harms"], c["value"], c["check"] = text, effects, harms, value, check
    flag(c, f'"{old}" became "{text}" ({why})')
    log("Threats", c["name"], old, text, why)


def create_here(n):
    return {"op": "create", "count": n, "at": "location"}


def pay(resource, amount, cards):
    return {"op": "requisition", "resource": resource, "amount": amount, "cards": cards}


def threats(deck):
    for c in deck["cards"]:
        i = c["id"]
        if i == "orbital_strike":
            rename(c, "Weapons Test", "an orbital strike that only moved cubes; the test range is what the cubes are",
                   "297: counter 3 energy (9) or 8 credits; uncountered 2 research cubes, 4 at values.json "
                   "(adversary_research 2 a cube), 0.50x the cheapest counter: it bites through the row, not at once",
                   "bites through the research row: 2 cubes are half a slot at two countries (K 2 a country)")
        elif i == "supply_raid":
            redo(c, "This country's player pays 2 chips or discards 1 card. The adversary gains 4 compute.",
                 [pay("chips", 2, 1), {"op": "compute", "amount": 4}],
                 ["credits or income lost", "projects advanced (compute)"],
                 "a raid takes supplies: its 294 cube line goes, the chips stay and the haul is compute",
                 "297: counter 2 Industry tags (10) or 8 credits; uncountered 6.3 at values.json (2 chips or a card "
                 "3.33, 4 compute 2.93), 0.78x the cheapest counter",
                 "bites chip holders; the compute brings the leftmost project nearer")
        elif i == "call_up":
            redo(c, "1 Agent is created here. 1 Fallout lands next to here.",
                 [create_here(1), {"op": "fallout", "count": 1}],
                 ["adversary pieces placed", "Fallout placed"],
                 "a call-up raises people, not research: an agent on its coastal city (one, so it never takes the "
                 "city with the anchor agent at tier 1) beside the Fallout it had",
                 "297: counter 3 ambassadors (16) or 8 credits; uncountered 9.3 at values.json (an agent 1.33, a "
                 "Fallout 8), 1.17x the cheapest counter",
                 "bites where a coastal city is near corrupting (5 agents, 3 from level 1)")
        elif i == "total_war":
            rename(c, "Weapons Program", "the card is the row's biggest feeder; no war is declared by 4 cubes",
                   "297: counter 4 energy (12) or 11 credits; uncountered 4 research cubes, 8 at values.json, 0.73x "
                   "the cheapest counter: a whole slot at two countries",
                   "bites through the research row: 4 cubes are a slot at two countries (K 2 a country)")
        elif i == "manhunt":
            redo(c, "This country's player pays 3 Ambassadors or discards 3 cards. 2 Agents are created here.",
                 [pay("ambassadors", 3, 3), create_here(2)],
                 ["ambassadors lost", "adversary pieces placed", "facilities destroyed or corrupted"],
                 "a manhunt takes ambassadors and leaves its hunters in the city; its 294 cube line goes",
                 "297: counter 4 energy (12) or 11 credits; uncountered 12.7 at values.json (3 ambassadors or 3 "
                 "cards 10, 2 agents 2.67), 1.15x the cheapest counter; from level 1 the 2 agents and the anchor "
                 "agent take a clean city (3), the harm it names",
                 "bites ambassador holders and a city near corrupting (2 + the anchor agent reach 3 from level 1)")
        elif i == "conscription":
            redo(c, "2 Agents are created here. This country's player pays 2 chips or discards 2 cards.",
                 [create_here(2), pay("chips", 2, 2)],
                 ["adversary pieces placed", "credits or income lost", "facilities destroyed or corrupted"],
                 "conscripts fill the factory: agents on it, not cubes on the track",
                 "297: counter 4 energy (12) or 11 credits; uncountered 9.3 at values.json (2 agents 2.67, 2 chips "
                 "or 2 cards 6.67) and, with the anchor agent, the factory (3 agents corrupt it: take_site's 10), "
                 "1.75x the cheapest counter",
                 "bites the factory under its token: 2 + the anchor agent reach its number (3)")
        elif i == "garrison_state":
            rename(c, "Closed City", "garrison is the troops' word now, and this threat lands none: agents hold a "
                              "city while its labs work",
                   "297: counter 4 insight (10.7) or 11 credits; uncountered 10 at values.json (3 agents 4, 3 cubes "
                   "6), 0.94x the cheapest counter; from level 1 its 3 agents take a clean city, the harm it names",
                   "bites a city near corrupting and feeds the research row (3 cubes)")
        elif i == "iron_curtain":
            rename(c, "Brain Drain", "an iron curtain that moved cubes and Unity: researchers leaving is both lines",
                   "297: counter 4 energy (12) or 11 credits; uncountered 11.3 at values.json (3 cubes 6, 2 Unity "
                   "5.33), 1.03x the cheapest counter",
                   "feeds the research row (3 cubes) and takes Unity off its track")
        elif i == "war_economy" and "294:" in c.get("check", ""):
            c["effects"] = [research(e["amount"]) if e["op"] == "adversary_research" else e for e in c["effects"]]
            c["value"] = ("297: counter 4 energy (12) or 11 credits; uncountered 16 at values.json (an adversary "
                          "factory 10, 3 cubes 6), 1.45x the cheapest counter. Kept as it is: an arsenal is a plant "
                          "and the weapons work it feeds")
            c["check"] = "builds the adversary a factory and feeds the research row (3 cubes)"


# --- projects --------------------------------------------------------------------------------------------------
def project(c, text, effects, harms, why, value, check, cost=None):
    if c["text"] == text and (cost is None or c["cost"] == cost):
        return
    old = f'{c["text"]} ({c["cost"]} a board)'
    c["text"], c["effects"], c["harms"], c["value"], c["check"] = text, effects, harms, value, check
    if cost is not None:
        c["cost"] = cost
    flag(c, f'"{old}" became "{text}" at {c["cost"]} a board ({why})')
    log("Projects", c["name"], old, f'{text} ({c["cost"]} a board)', why)


PICK = "no \"most agents\" pick: the lowest number, as every other line on the adversary's side"


def projects(deck):
    for c in deck["cards"]:
        i = c["id"]
        if i == "night_raid":
            project(c, "Each country: 1 Agent onto each of its two lowest-numbered hexes holding a player Node and "
                       "an Agent. 2 Agents on your Node's hex destroy it.",
                    [{"op": "agents_on_nodes", "count": 2}], ["adversary pieces placed", "node destroyed"],
                    "no \"most agents\" pick, and no agent onto a clean node: the nodes at risk are the ones an "
                    "agent already stands on, which the table can see and clear",
                    "give 10 compute a board (20 at 2 countries); get 0.53 a compute at values.json (agents_on_nodes "
                    "2.67 a landing): each landing now takes a node unless the agent is cleared first.",
                    "297: 1 consequence (small), every board, one-shot. When it matters: mid to late, once agents "
                    "stand on node hexes. Players answer: clear the agent off your node's hex; a firewall.")
        elif i == "asset_seizure":
            project(c, "Each country: its lowest-numbered neutral factory is corrupted; with none neutral, its "
                       "lowest-numbered player-owned factory turns neutral.",
                    [{"op": "take_site", "site": "factory"}], ["facilities destroyed or corrupted"], PICK,
                    c["value"], "297: 1 consequence (small), every board, one-shot. When it matters: any round. "
                                "Players answer: claim the factory it names; drain the project.")
        elif i == "sabotage_teams":
            project(c, "Each country: its lowest-numbered hardened facility loses its hardening. Until the end of "
                       "next round, players' boards and cards make no energy income.",
                    [{"op": "strip_harden"}, {"op": "flag", "name": "sabotage", "everywhere": True}],
                    ["protections stripped", "energy lost"], PICK, c["value"],
                    "297: 2 consequences (big), every board, one-shot. When it matters: mid to late. Players "
                    "answer: bank energy; harden again.")
        elif i == "station_coup":
            project(c, "Each country: its lowest-numbered neutral research station is corrupted; with none neutral, "
                       "its lowest-numbered player-owned station turns neutral. Then each country: 1 Fallout lands "
                       "next to its lowest-numbered corrupted research station, and 2 cubes go onto the adversary's "
                       "research track.",
                    c["effects"], c["harms"], PICK + "; it opens the adversary a research station a country, a "
                    "cube a round for the row", c["value"],
                    "297: 3 consequences (big; its cost allows the third), every board, one-shot. When it matters: "
                    "any round. Players answer: claim the station it names; an ambassador on it stops its cube.")
        elif i == "botnet_conscripts":
            project(c, "Each country holding 2 or more Botnets: its lowest-numbered Botnet becomes 1 Troop on its hex.",
                    [{"op": "botnet_troops", "count": 1, "troops": 1, "least": 2}], ["troops made"],
                    "the troop stands where the botnet stood, beside whatever the players built there, so a Strike "
                    "has something to hit and a Redeploy something to move; one troop, not two, dearer (11 -> 16), "
                    "and only where botnets have spread: on the first screen the starting botnet turned by round 3 "
                    "in every early-troop game",
                    "give 16 compute a board (32 at 2 countries); get 0.21 a compute at values.json (botnet_troops "
                    "3.33 a troop): dear for its number by design: a troop on a hex the players chose to build beside is worth "
                    "more than the row's garrison, and the price is what makes it late.",
                    "297: 1 consequence (small), every board, one-shot. When it matters: once a country holds a "
                    "second botnet (Botnets spread, a Backdoor). Players answer: keep each country to one botnet "
                    "(Take down a botnet); remove the troop.",
                    cost=16)
        elif i == "war_plants":
            project(c, "Every corrupted factory makes 1 Troop on its hex (a player Ambassador on it stops it).",
                    [{"op": "factory_troops", "count": 1}], ["troops made"],
                    "one thing, by a condition the table sees: it builds no factory of its own, so a board with no "
                    "corrupted factory takes no troop, and an ambassador on each one stops it; dearer (19 -> 22), "
                    "so later",
                    "give 22 compute a board (44 at 2 countries); get 0.21-0.64 a compute at values.json (a troop "
                    "4.67 at one to three corrupted factories a country).",
                    "297: 1 consequence, every board, one-shot. When it matters: once factories have fallen "
                    "(Commandeered Plants takes one in two rounds). Players answer: an ambassador on each "
                    "corrupted factory; liberate them.", cost=22)
        elif i == "martial_law":
            project(c, BESIEGE.format(n="2 Troops", s="", it="they"),
                    [{"op": "troops", "count": 2, "each_country": True, "at": "besieged_city"}], ["troops made"],
                    "a third troop project, and the dearest (24): martial law is soldiers in the streets where the "
                    "players are working, so its troops guard the city being liberated and a Strike finds pieces "
                    "beside them; its old line (3 agents on a research station) was Lab Seizure's",
                    "give 24 compute a board (48 at 2 countries); get 0.39 a compute at values.json (2 troops "
                    "9.33): the late, significant one.",
                    "297: 1 consequence, every board, one-shot. When it matters: late (24 a board). Players "
                    "answer: drain it; hold Remove a troop's credits or a troop card for the round it lands.")


# --- Unity and Discord cards -----------------------------------------------------------------------------------
def tiers(card, name, new, why):
    """`new`: three (text, effects). Logs old -> new by strength."""
    old = card["tiers"]
    if [t["text"] for t in old] == [t for t, _ in new] and card["name"] == name:
        return
    side = "Unity cards" if card["side"] == "unity" else "Discord cards"
    was = card["name"]
    said = False
    for k, (t, (text, effects)) in enumerate(zip(old, new)):
        label = f"{name} {'I' * (k + 1)}" if was == name else f"{name} {'I' * (k + 1)} (was {was})"
        if t["text"] != text or was != name:
            log(side, label, t["text"], text, "" if said else why)
            said = True
    card["name"] = name
    card["tiers"] = [{"text": text, "effects": effects} for text, effects in new]


def per_site(where, most, then):
    return [{"op": "per_site", "where": where, "max": most, "then": then}]


def calm(n):
    return [{"op": "pull_discord", "amount": n}]


def alignment(cards):
    by = {c["id"]: c for c in cards}
    tiers(by["volunteer_corps"], "Volunteer Corps",
          [(f"Remove 1 Discord from the Discord track for each {OWNED} (at most {n}).", per_site("owned", n, calm(1)))
           for n in (2, 3, 4)],
          "removes Discord by what the players hold; it was Unity swaying Unity (the same count, the same worth)")
    tiers(by["safe_corridors"], "Safe Corridors",
          [(f"Remove 1 Discord from the Discord track for each hex holding an Ambassador (at most {n}).",
            per_site("ambassador", n, calm(1))) for n in (2, 3, 4)],
          "removes Discord by ambassadors standing on the map; it was a fifth card handing out supply ambassadors")
    tiers(by["reconciliation_panel"], "Reconciliation Panel",
          [(f"Remove 1 Discord from the Discord track for each free city with no Agent on it (at most {n}).",
            per_site("clear_city", n, calm(1))) for n in (2, 3, 4)],
          "removes Discord by the cities the players keep clear of agents; it was the third card removing agents "
          "by corrupted cities (a first cut counted corrupted cities, which a game still running always has)")
    tiers(by["offensive"], "Offensive",
          [(f"Deal {n} with {its} on the Node hex with the most Agents and Troops.", [{"op": "threat_at_node", "tier": 2} for _ in range(k)])
           for k, n, its in ((1, "a tier-2 threat", "its token"), (2, "two tier-2 threats", "their tokens"),
                             (3, "three tier-2 threats", "their tokens"))],
          "one verb: 293's patch line (\"create 2 Agents on its lowest-numbered city\", Surge's heir) and "
          "strength III's second Infiltrate go, and the strengths widen the count")
    tiers(by["town_hall_accord"], "Town Hall Accord",
          [(f"Remove 2 Discord from the Discord track for each city holding an Ambassador (at most {n}).",
            per_site("ambassador_city", n, calm(2))) for n in (1, 2, 3)],
          "removes Discord by ambassadors in cities; it was the second card removing agents by corrupted cities")
    tiers(by["demilitarized_zone"], "Demilitarized Zone",
          [(f"In the next adversary turn, no Troop strikes a hex {words}.", [{"op": "prohibit", "step": "strike", "where": where}])
           for words, where in (("on or next to an Ambassador", "by_ambassador"),
                                ("on or next to an Ambassador or a Node", "by_player"),
                                ("within 2 of an Ambassador or a Node", "near_player"))],
          "strengths II and III were the same line: each strength now widens the one before")
    tiers(by["blight"], "Captured Labs",
          [(f"For each corrupted research station (at most {n}), 2 cubes go onto the adversary's research track.",
            per_site("corrupted_station", n, [{"op": "adversary_research", "amount": 2}])) for n in (2, 3, 4)],
          "the Discord deck's research feeder, by the stations the players left corrupted; Blight (agents on a "
          "city and a neutral factory) was Firestorm with a rider, under the old name of Fallout")
    tiers(by["two_fronts"], "Standing Army",
          [("Each country: 1 Troop lands on its lowest-numbered corrupted factory (a country with none takes no Troop).",
            [{"op": "troops", "count": 1, "each_country": True, "at": "corrupted_factory"}]),
           (BESIEGE.format(n="1 Troop", s="s", it="it"), [{"op": "troops", "count": 1, "each_country": True, "at": "besieged_city"}]),
           (BESIEGE.format(n="2 Troops", s="", it="they"), [{"op": "troops", "count": 2, "each_country": True, "at": "besieged_city"}])],
          "the Discord deck's troop card: fronts went with war, and its extra threats were Incursion's; strength "
          "I lands a troop only where a factory has fallen, so an early tip rarely brings one; from strength II "
          "(the third tip) its troops land on a corrupted city the players stand beside")
    tiers(by["firestorm"], "Firestorm",
          [(f"Each country: create {n} Agents on the lowest-numbered city holding Agents.",
            [{"op": "create", "count": n, "at": "city", "if": "ambassadors"}]) for n in (2, 3, 4)],
          "one verb again: 294's troop rider on strength III moved to Standing Army")


# --- the Protector's board -------------------------------------------------------------------------------------
def ais(data):
    for b in data:
        for p in b.get("powers", []):
            if b["id"] == "protector" and p["name"] == "Outpost" and p["text"] == "place 1 Node; gain 2 credits":
                old = "2 energy: place 1 Node; gain 2 credits"
                p["text"] = "place 1 Node within 2 of yours; gain 2 credits"
                p["effects"] = [{"op": "place_node", "count": 1, "range": 2}, {"op": "gain", "resource": "credits", "amount": 2}]
                log("Board powers", "Outpost (Protector)", old, "2 energy: " + p["text"],
                    "used 0.05 a seat-round beside Accord at the same price (an ambassador beats 2 credits): an "
                    "outpost is now what its name says, a node two hexes out (a shield, the first idea, is the "
                    "cards': the content check refuses one on a board)")


def main():
    mil = load("drafts/adversary/militarist.json")
    events(mil)
    dump("drafts/adversary/militarist.json", mil)
    for tier in (1, 2):
        path = f"drafts/adversary/threats_t{tier}.json"
        deck = load(path)
        threats(deck)
        dump(path, deck)
    deck = load("drafts/adversary/projects.json")
    projects(deck)
    dump("drafts/adversary/projects.json", deck)
    cards = load("content/alignment.json")
    alignment(cards)
    (ROOT / "content/alignment.json").write_text(dump_alignment(cards), encoding="utf-8", newline="\n")
    data = load("content/ais.json")
    ais(data)
    dump("content/ais.json", data)
    if LOG:
        # The two player cards change by `cards.overlay` (the faces print from their ops): listed here for the table.
        log("Player cards", "Breach and Clear (tier 1 event, 2 cr, 1 insight)", "Target: Troops. Remove 1 Troop on it.",
            "Target: Agents or Troops. Remove 3 Agents or 2 Troops on it.",
            "kept 0.21 a game and played 0.03: no card at all with no troop in reach, and under the standard "
            "action's price with one; stronger, not cheaper")
        log("Player cards", "Peacekeeping Mission (tier 2 event, 13 cr, 1 antimatter)",
            "Target: a corrupted city. Pay 1 antimatter. Rescue it. Remove 3 Troops in its country.",
            "Target: a corrupted city. Pay 1 antimatter. Remove 3 Troops on or next to it. Rescue it.",
            "a guarded city was no Rescue target, so its troop half could never open its own target: the troops "
            "go first, and it is the one Rescue that takes a guarded city")
        out = ROOT / "experiments" / "results" / "297_overhaul_5_content_sweep"
        out.mkdir(parents=True, exist_ok=True)
        rows = ["# 297: content, old -> new", "",
                "Written by `scripts/content/sweep_297.py`. Player cards: [README.md](README.md), *Player cards*.", ""]
        deck = None
        for d, card, old, new, why in LOG:
            if d != deck:
                rows += ["", f"## {d}", "", "| Card | Old | New | Why |", "|---|---|---|---|"]
                deck = d
            rows.append(f"| {card} | {old} | {new} | {why} |")
        (out / "content_changes.md").write_text("\n".join(rows) + "\n", encoding="utf-8", newline="\n")
    print(len(LOG), "lines changed")


main()
