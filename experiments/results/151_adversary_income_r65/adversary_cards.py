"""151: writes the Militarist's board and event deck and the project deck to the income-and-rally direction
(Matt, 2026-11-08, design doc *Adversary system*), and prints each card's give / get.

python experiments/results/151_adversary_income_r65/adversary_cards.py [--check]

Writes drafts/adversary/militarist.json and drafts/adversary/projects.json (then run
scripts/content/load_adversary.py and drafts/adversary/check.py), and cards.md beside this file: every event and
project as name, then each line as printed, with its give / get at configs/agents/values.json.

Every event prints its steps in one order, like phases: Threat (its pulls), Rally X (each board's whole pool lands
on X), Deploy N (printed troops), then at most two other lines, then Corrupt X. Text is generated from the ops, so
the face can't drift from what the engine does. "Each board", never "each country".
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
DRAFTS = os.path.join(ROOT, "drafts", "adversary")
VALUES = json.load(open(os.path.join(ROOT, "configs", "agents", "values.json"), encoding="utf-8"))
RES = VALUES["resources"]
RATES = VALUES["rates"]
OPS = VALUES["ops"]
COUNTRIES = RATES["countries"]  # 2: the baseline's boards
TROOP = 9  # a troop with its strike (146's price)
REMOVAL = 5  # the cheapest answer to one adversary piece: a removal (the drafts' scale)
POOL = 3  # an event's pool a board at d0-d2: ambassador income 2 + the Militarist's Garrisons 1
ROUNDS_LEFT = {1: 8, 2: 5, 3: 2}  # rounds an income rise acts, by the stage that prints it

PLURAL = {"city": "cities", "coastal_city": "coastal cities", "station": "research stations", "factory": "factories",
          "botnet": "botnet hexes", "land": "land"}
TAGNAME = {"military": "Military", "influence": "Influence", "cyber": "Cyber", "economic": "Economic",
           "grid": "Grid"}


# ---------- ops ----------

def rally(site, count=0):
    o = {"op": "rally", "site": site}
    if count:
        o["count"] = count
    return o


def deploy(n):
    return {"op": "troops", "count": n, "each_country": True}


def income(compute=0, ambassadors=0):
    o = {"op": "increase_income"}
    if compute:
        o["compute"] = compute
    if ambassadors:
        o["ambassadors"] = ambassadors
    return o


def corrupt(site, cut=0):
    o = {"op": "corrupt", "site": site}
    if cut:
        o["cut"] = cut
    return o


def sway(n):
    return {"op": "sway", "side": "discord", "amount": n}


def pull_unity(n):
    return {"op": "pull_unity", "amount": n}


def requisition(resource, amount, cards, next_round=False):
    o = {"op": "requisition", "resource": resource, "amount": amount, "cards": cards}
    if next_round:
        o["next_round"] = True
    return o


# ---------- words ----------

def pull_words(pulls):
    parts = []
    for p in pulls:
        tags = p["tags"]
        if tags == ["any"]:
            parts.append(f"any tier-{p['tier']} threat")
        else:
            parts.append(" or ".join(f"{TAGNAME[t]} {p['tier']}" for t in tags))
    return "Threat, each board: " + ", and ".join(parts) + "."


def amount_words(resource, n):
    names = {"credits": "credits", "energy": "energy", "insight": "insight", "chips": "chips",
             "ambassadors": "ambassadors" if n != 1 else "ambassador"}
    return f"{n} {names[resource]}"


def line_words(e):
    op = e["op"]
    if op == "rally":
        if e.get("count"):
            return f"Rally {e['count']}, each board: {PLURAL[e['site']]}."
        return f"Rally: {PLURAL[e['site']]}."
    if op == "troops":
        n = e["count"]
        return f"Deploy {n}: each board, {n} troop{'s' if n > 1 else ''} on its lowest-numbered factory (else city)."
    if op == "increase_income":
        bits = []
        if e.get("ambassadors"):
            bits.append(f"+{e['ambassadors']} ambassador a board")
        if e.get("compute"):
            bits.append(f"+{e['compute']} compute")
        return "Income: " + ", ".join(bits) + " a round."
    if op == "corrupt":
        return f"Corrupt: {PLURAL[e['site']]}" + (" (1 fewer)." if e.get("cut") else ".")
    if op == "sway":
        return f"Sway {e['amount']} toward Discord."
    if op == "pull_unity":
        return f"Remove {e['amount']} Unity from the Unity track."
    if op == "create_site":
        return f"Each board: a neutral {'factory' if e['kind'] == 'factory' else 'research station'} opens on its lowest-numbered free site."
    if op == "each_destroy_node":
        return "Each player with a node on or next to a front's base destroys 1 of those nodes."
    if op == "troop_steps":
        return "Every adversary troop takes 1 more step this round."
    if op == "requisition":
        when = " next round" if e.get("next_round") else ""
        return f"Each player pays {amount_words(e['resource'], e['amount'])}{when} or discards {e['cards']} card{'s' if e['cards'] != 1 else ''}."
    if op == "corp_troops":
        if e.get("fill"):
            return "Every military base fills with corp troops to its cap."
        return "Each round, every military base makes 1 extra corp troop, even past its cap (at most 1 extra a base each production)."
    if op == "extra_war":
        return "Each round, the front with the most corp troops fights twice."
    if op == "deal_threats":
        return "Each board: an extra tier-2 threat, any tag."
    if op == "adversary_build":
        return "Each board rallies its ambassador income now, to this round's Rally."
    if op == "adversary_factory":
        return "The adversary builds a factory on the board with the most free industrial zones (none free: 2 ambassadors on a factory)."
    if op == "team_income":
        return f"Each player's credit income falls by {-e['amount']} for the rest of the game."
    if op == "lose_tokens":
        return f"Each player removes {e['amount']} tokens from their cards (their choice of cards)."
    if op == "flag":
        return "Until the end of next round, cities make no energy."
    if op == "flip_station":
        return "Each board: its lowest-numbered research station not the adversary's flips (a player's holds unless 2 adversary ambassadors are on it)."
    raise KeyError(op)


def card_text(lines, ongoing=False):
    words = [line_words(e) for e in lines]
    return " ".join(words)


# ---------- prices (credits at values.json; the adversary's harm to the players) ----------

def harm(e, stage=2):
    op = e["op"]
    if op == "rally":
        n = e.get("count") or POOL
        return -OPS["rally"]["credits"] * n * COUNTRIES
    if op == "troops":
        return TROOP * e["count"] * COUNTRIES
    if op == "increase_income":
        per = e.get("compute", 0) + 2 * COUNTRIES * e.get("ambassadors", 0)
        return -OPS["increase_income"]["credits"] * per * ROUNDS_LEFT[stage]
    if op == "corrupt":
        return -OPS["corrupt"]["credits"]
    if op == "sway":
        return RATES["sway"] * e["amount"]
    if op == "pull_unity":
        return RATES["sway"] * e["amount"]
    if op == "create_site":
        return -OPS["create_site"]["credits"] * COUNTRIES
    if op == "each_destroy_node":
        return RATES["node"] * 0.5 * 2
    if op == "troop_steps":
        return 5
    if op == "requisition":
        pay = RES[e["resource"]] * e["amount"]
        return 2 * min(pay, RATES["drawn_card"] * e["cards"])
    if op == "corp_troops":
        return -OPS["corp_troops"]["credits"] * (4 if e.get("fill") else 2)
    if op == "extra_war":
        return 16
    if op == "deal_threats":
        return 14 * COUNTRIES
    if op == "adversary_build":
        return -OPS["rally"]["credits"] * POOL * COUNTRIES
    if op == "adversary_factory":
        return -OPS["adversary_factory"]["credits"]
    if op == "team_income":
        return -e["amount"] * 2 * RATES["income_rounds"]
    if op == "lose_tokens":
        return 2 * e["amount"] * 1.5
    if op == "flag":
        return 2 * 2 * RES["energy"]
    if op == "flip_station":
        return 12 * COUNTRIES
    raise KeyError(op)


HARM_OF = {"rally": "adversary pieces placed", "troops": "troops made", "increase_income": "adversary pieces placed",
           "corrupt": "facilities destroyed or seized", "sway": "Discord sway", "pull_unity": "Unity pulled back",
           "create_site": None, "each_destroy_node": "node destroyed", "troop_steps": "war brought forward",
           "corp_troops": "troops made", "extra_war": "war brought forward", "deal_threats": "extra threats",
           "adversary_build": "adversary pieces placed", "adversary_factory": "facilities destroyed or seized",
           "team_income": "credits or income lost", "lose_tokens": "tokens stripped from cards", "flag": "production blacked out", "flip_station": "facilities destroyed or seized"}
REQ_HARM = {"credits": "credits or income lost", "energy": "energy lost", "insight": "insight lost",
            "chips": "credits or income lost", "ambassadors": "ambassadors lost"}


def harms(lines):
    out = []
    for e in lines:
        h = REQ_HARM[e["resource"]] if e["op"] == "requisition" else HARM_OF[e["op"]]
        if e["op"] == "requisition":
            out.append("cards discarded")
        if e["op"] == "increase_income" and e.get("compute"):
            out.append("projects advanced (compute)")
            if not e.get("ambassadors"):
                h = None
        if h and h not in out:
            out.append(h)
    return list(dict.fromkeys(out))


# ---------- the event deck ----------
# (id, name, stage, pulls, rally site, deploy, other lines, corrupt lines, pressure)
P = lambda tier, *tags: {"tier": tier, "tags": list(tags), "per": "country"}  # noqa: E731
EVENTS = [
    ("mil_s1_probing_strikes", "Probing Strikes", 1, [P(1, "military")], "coastal_city", 1, [], [corrupt("city")],
     "Lands the pool on the neutral coastal cities and drops a troop on each board's first factory; Corrupt cities then takes a coastal city that 4 can gather around."),
    ("mil_s1_factory_drive", "Factory Drive", 1, [P(1, "military", "economic")], "factory", 1,
     [{"op": "create_site", "kind": "factory"}], [corrupt("factory")],
     "Contests the factories: the pool and a troop land on them, a new neutral factory opens (a target for both sides), and Corrupt factories takes one 3 gather on."),
    ("mil_s1_long_patrols", "Long Patrols", 1, [P(1, "military", "cyber")], "station", 0, [income(ambassadors=1)],
     [corrupt("station")],
     "Contests the one neutral research station a board: the whole pool lands on it, and Corrupt stations needs only 3; income +1 ambassador a board for the rest of the game."),
    ("mil_s1_war_drums", "War Drums", 1, [P(1, "military", "influence")], "city", 1, [sway(1)], [corrupt("city")],
     "Spreads the pool over the neutral cities, a troop on each board's first factory, and Discord; Corrupt cities takes any city 4 gather around."),
    ("mil_s1_signal_jamming", "Signal Jamming", 1, [P(1, "military", "grid")], "botnet", 0, [income(compute=2)],
     [corrupt("city")],
     "Lands the pool on the botnet hexes (the adversary's own stations at first): it feeds the gather of the cities next to them; compute income +2 for the rest of the game."),
    ("mil_s2_mobilization", "Mobilization", 2, [P(2, "military")], "coastal_city", 1, [income(ambassadors=1)],
     [corrupt("city")],
     "Coastal cities again, a troop a board, and income +1 ambassador a board: the coast fills fastest from stage II."),
    ("mil_s2_station_raids", "Station Raids", 2, [P(2, "military", "cyber")], "station", 1, [], [corrupt("station")],
     "The pool on the neutral stations (else the players'), a troop a board; Corrupt stations takes a station 3 gather on, a player's included."),
    ("mil_s2_fifth_column", "Fifth Column", 2, [P(2, "influence", "cyber")], "city", 0, [income(ambassadors=1)],
     [corrupt("city", 1)],
     "The pool over the neutral cities and Corrupt cities one fewer (3 at d0): the city cascade card; income +1 ambassador a board."),
    ("mil_s2_border_war", "Border War", 2, [P(2, "military")], "factory", 1,
     [{"op": "each_destroy_node", "count": 1, "bases": True}], [corrupt("factory")],
     "Factories and the fronts: the pool and a troop on the factories, nodes at the fronts' bases lost, Corrupt factories."),
    ("mil_s2_supply_lines", "Supply Lines", 2, [P(1, "military", "economic"), P(2, "military", "economic")],
     "factory", 1, [income(compute=2)], [corrupt("factory")],
     "Two pulls, the pool on the factories, a troop a board, compute income +2: the projects fill faster from here."),
    ("mil_s2_forced_march", "Forced March", 2, [P(2, "any")], "city", 1,
     [{"op": "troop_steps", "amount": 1}], [corrupt("city")],
     "Cities and the march: the pool over the neutral cities, a troop a board, every troop a step further this round."),
    ("mil_s3_total_mobilization", "Total Mobilization", 3, [P(2, "military"), P(2, "economic", "grid")], "factory", 2,
     [income(ambassadors=1), pull_unity(2)], [corrupt("factory"), corrupt("city")],
     "Two tier-2 pulls, the pool on the factories, 2 troops a board, Unity pulled back, then both Corrupt lines."),
    ("mil_s3_blitzkrieg", "Blitzkrieg", 3, [P(2, "any"), P(2, "military", "influence")], "city", 2, [sway(1)],
     [corrupt("city"), corrupt("land")],
     "The pool over the cities, 2 troops a board, then Corrupt cities and Corrupt land (empty land 3 gather around)."),
    ("mil_s3_occupation", "Occupation", 3, [P(2, "military", "cyber"), P(2, "influence")], "station", 1,
     [income(ambassadors=1)], [corrupt("station"), corrupt("city", 1)],
     "The pool on the stations, a troop a board, then Corrupt stations and Corrupt cities one fewer."),
    ("mil_s3_naval_blockade", "Naval Blockade", 3, [P(2, "economic", "grid"), P(2, "military", "cyber")],
     "coastal_city", 2, [{"op": "flag", "name": "blackout", "everywhere": True}], [corrupt("city"), corrupt("land")],
     "The coast: the pool on the coastal cities, 2 troops a board, cities dark for two rounds' production, Corrupt cities and Corrupt land."),
]

TWIST_OPS = {"sway", "pull_unity", "each_destroy_node", "troop_steps", "corrupt"}


def event(row):
    eid, name, stage, pulls, site, n_troops, other, corrupts, pressure = row
    standing = [rally(site)] + ([deploy(n_troops)] if n_troops else []) + [e for e in other if e["op"] not in TWIST_OPS]
    twists = [e for e in other if e["op"] in TWIST_OPS] + corrupts
    lines = standing + twists
    text = pull_words(pulls) + " " + card_text(lines)
    priced = [(line_words(e), harm(e, stage)) for e in lines]
    total = sum(v for _, v in priced)
    pieces = (POOL + n_troops) * COUNTRIES
    answer = REMOVAL * pieces
    give_get = (f"harm {total:.0f} at values.json ({'; '.join(f'{w.rstrip(".")}: {v:.0f}' for w, v in priced)}); the players' "
                f"cheapest answer, a removal a piece it lands ({pieces} at 2 boards, {REMOVAL} each): {answer}; "
                f"{total / answer:.2f}x. Pulls priced on their own threat cards.")
    return {
        "id": eid, "name": name, "stage": stage, "pulls": pulls, "standing": standing, "twists": twists,
        "text": text, "harms": harms(lines), "value": give_get,
        "check": pressure, "flags": ["151: rewritten to income and rally (threat, rally, deploy, other lines, corrupt)"],
        "reuses": None,
    }


# ---------- the board ----------

BOARD = {
    "id": "militarist",
    "name": "The Militarist",
    "project_slots": [{"tag": "military"}, {"tag": None}, {"tag": None}],
    "rules": [
        {"name": "Garrisons",
         "text": "Income: +1 ambassador a board a round.",
         "effects": [income(ambassadors=1)]},
        {"name": "Troops where the Protector stands",
         "text": "Each round: on each board where the Protector has a node, 1 adversary troop lands on its lowest-numbered factory (else city), never a hex Wall holds.",
         "effects": [{"rule": "adversary.build", "reads": "wall_troops", "note": "lands on the lowest-numbered factory there (151: deterministic), else a city, never a hex Wall holds"}]},
        {"name": "Warmongering",
         "text": "Each round, a military base with an adversary troop on or next to it makes 1 extra corp troop, even past its cap; at most 1 extra a base in each production, whatever makes it.",
         "effects": [{"op": "corp_troops", "count": 1, "at": "every_base", "near": "adversary_troop",
                      "when": "each_production", "past_cap": True}]},
    ],
    "build": {"corrupt_at": 4, "wall_troops": 1, "botnet_compute": 1},
    "difficulty": {
        "text": "Militarist, harder: each event's Deploy lands 1 more troop a board, and the adversary gains 4 compute for each threat that executes uncountered.",
        "effects": [{"op": "standing_troops_bonus", "amount": 1}, {"op": "threat_compute", "amount": 4}],
        "flags": ["151: unchanged but for the words (Deploy, board)"],
    },
    "harms": ["adversary pieces placed", "troops made", "facilities destroyed or seized"],
    "value": ("Garrisons: +1 ambassador a board a round, 2 at values.json (rally) a board a round, 4 a round at 2 boards, "
              "against one removal (5): 0.8x a round, but it lands every round of the game (about 40 over 10 rounds, "
              "the old board line's 2 on the corrupted city reworded as income). Wall troops: 1 troop (9) a round where "
              "the Protector stands, against one removal (5): 1.8x. Warmongering: 1 extra corp troop a qualifying base "
              "a production step (4), against removing the adversary troop (5)."),
    "check": "151: the board's creation line (2 on the lowest-numbered corrupted city, build_bonus more from d3) is now Garrisons, an income line; its two powers keep their effect, worded for boards, and land deterministically.",
    "flags": ["151: Garrisons replaces the board's creation line; build_bonus is gone (ambassador_income's d3 entry is the ladder's step)"],
    "creates_text": "Garrisons: +1 ambassador a board a round.",
}


# ---------- the project deck ----------
# (id, name, tags, cost, ongoing, effects, resolve)
PROJECTS = [
    ("raiding_parties", "Raiding Parties", ["military"], 4, False, [requisition("chips", 5, 3)], []),
    ("war_footing", "War Footing", ["military"], 7, False,
     [{"op": "corp_troops", "fill": True, "at": "every_base", "when": "now", "past_cap": False}], []),
    ("purge_the_unaligned", "Purge the Unaligned", ["military"], 7, False, [requisition("ambassadors", 2, 2)], []),
    ("armored_columns", "Armored Columns", ["military"], 10, False, [deploy(3)], []),
    ("arms_race", "Arms Race", ["military"], 11, True,
     [{"op": "corp_troops", "count": 1, "at": "every_base", "when": "each_production", "past_cap": True}], []),
    ("declaration_of_war", "Declaration of War", ["military"], 18, True,
     [{"op": "extra_war", "front": "most_corp_troops", "ongoing_use": True}], [deploy(3)]),
    ("disinformation", "Disinformation", ["influence"], 5, False, [sway(5)], []),
    ("recruitment_drive", "Recruitment Drive", ["influence"], 11, False, [income(ambassadors=1)], []),
    ("infiltration_drive", "Infiltration Drive", ["influence"], 18, False, [rally("city", 3), corrupt("city", 1)], []),
    ("data_heist", "Data Heist", ["cyber"], 6, False, [{"op": "lose_tokens", "amount": 3}, requisition("insight", 3, 2, next_round=True)], []),
    ("botnet_swarm", "Botnet Swarm", ["cyber"], 6, False, [rally("botnet", 3)], []),
    ("station_coup", "Station Coup", ["cyber"], 9, False, [rally("station", 2), {"op": "flip_station", "each_country": True}], []),
    ("market_crash", "Market Crash", ["economic"], 5, False, [requisition("credits", 10, 3)], []),
    ("shadow_factories", "Shadow Factories", ["economic"], 7, False,
     [{"op": "adversary_factory", "count": 1, "where": "most_free_zones", "fallback_ambassadors": 2}], []),
    ("capital_flight", "Capital Flight", ["economic"], 8, False,
     [{"op": "team_income", "resource": "credits", "amount": -4}], []),
    ("rolling_blackouts", "Rolling Blackouts", ["grid"], 6, False, [{"op": "flag", "name": "blackout", "everywhere": True}], []),
    ("grid_capture", "Grid Capture", ["grid"], 9, True, [requisition("energy", 1, 1)], []),
    ("false_flag", "False Flag", ["universal"], 6, False,
     [{"op": "deal_threats", "count": 1, "each_country": True, "tier": 2}], []),
    ("grand_offensive", "Grand Offensive", ["universal"], 10, False, [{"op": "adversary_build"}, corrupt("city")], []),
    ("propaganda_network", "Propaganda Network", ["universal"], 14, True, [sway(2)], [income(compute=2)]),
]


def project(row):
    pid, name, tags, cost, ongoing, effects, resolve = row
    if ongoing:
        text = ((card_text(resolve) + " ") if resolve else "") + "Ongoing: " + card_text(effects)
        if not text.startswith("Ongoing") and resolve:
            text = "Now: " + text
    else:
        text = card_text(effects)
    rounds = 4 if ongoing else 1
    priced = [(line_words(e), harm(e) * rounds) for e in effects] + [(line_words(e), harm(e)) for e in resolve]
    total = sum(v for _, v in priced)
    absorbed = cost * COUNTRIES
    give_get = (f"absorbs {cost} compute a player ({absorbed} at 2 players, {absorbed * 1.1:.1f} at values.json "
                f"compute 1.1); harm {total:.0f}{' over about 4 rounds' if ongoing else ''} ({'; '.join(f'{w.rstrip(".")}: {v:.0f}' for w, v in priced)}); "
                f"{total / absorbed:.2f} a compute.")
    out = {"id": pid, "name": name, "tags": tags, "cost": cost, "ongoing": ongoing, "text": text,
           "effects": effects}
    if resolve:
        out["resolve"] = resolve
    out.update({"harms": harms(effects + resolve), "value": give_get,
                "check": "151: one consequence (two at most), deterministic targeting; a player's harm is a fixed amount a seat with a choice of currency.",
                "flags": ["151: rewritten to the standard (one consequence, each board, deterministic)"], "reuses": None})
    return out


def dump(path, obj):
    s = json.dumps(obj, indent=1, ensure_ascii=False) + "\n"
    if "--check" in sys.argv:
        old = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
        if old != s:
            print(f"stale: {path}")
            return False
        return True
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(s)
    return True


def listing(events, projects):
    out = ["# 151: the Militarist's events and the project deck, as printed", "",
           "Generated by [adversary_cards.py](adversary_cards.py). Each card: its name, then each line as printed. "
           "Give / get at `configs/agents/values.json` (an event's pool is 3 a board at d0-d2: ambassador income 2 and Garrisons 1).",
           "", "## The Militarist", ""]
    for r in BOARD["rules"]:
        out.append(f"- **{r['name']}.** {r['text']}")
    out.append(f"- **Difficulty.** {BOARD['difficulty']['text']}")
    out += ["", f"Give / get: {BOARD['value']}", "", "## Events", ""]
    for e in events:
        out.append(f"**{e['name']}** (stage {e['stage']})")
        out.append("")
        out.append(f"- {pull_words(e['pulls'])}")
        for ln in e["standing"] + e["twists"]:
            out.append(f"- {line_words(ln)}")
        out.append(f"- *Give / get:* {e['value']}")
        out.append(f"- *Pressure:* {e['check']}")
        out.append("")
    out += ["## Projects", ""]
    for p in projects:
        tag = "/".join(p["tags"])
        out.append(f"**{p['name']}** ({tag}, {p['cost']} compute a player{', ongoing' if p['ongoing'] else ''})")
        out.append("")
        for ln in p.get("resolve", []):
            out.append(f"- Now: {line_words(ln)}")
        for ln in p["effects"]:
            out.append(f"- {'Ongoing: ' if p['ongoing'] else ''}{line_words(ln)}")
        out.append(f"- *Give / get:* {p['value']}")
        out.append("")
    return "\n".join(out)


def main():
    events = [event(r) for r in EVENTS]
    projects = [project(r) for r in PROJECTS]
    ok = dump(os.path.join(DRAFTS, "militarist.json"), {"board": BOARD, "events": events})
    old = json.load(open(os.path.join(DRAFTS, "projects.json"), encoding="utf-8"))
    note = {k: v for k, v in old.items() if k != "cards"}
    ok &= dump(os.path.join(DRAFTS, "projects.json"), {**note, "cards": projects})
    md = listing(events, projects) + "\n"
    path = os.path.join(HERE, "cards.md")
    if "--check" in sys.argv:
        ok &= open(path, encoding="utf-8").read() == md if os.path.exists(path) else False
    else:
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            f.write(md)
    if not ok:
        sys.exit(1)


if __name__ == "__main__":
    main()
