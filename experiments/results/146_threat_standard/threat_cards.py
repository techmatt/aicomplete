"""146: every threat, rewritten to Matt's threat card standard (2026-11-07), from one table.

    python experiments/results/146_threat_standard/threat_cards.py          # writes the drafts and cards.md
    python experiments/results/146_threat_standard/threat_cards.py --check  # exit 1 if a draft differs

Writes drafts/adversary/threats_t1.json and threats_t2.json (tier 3 is gone: its deck file is deleted by hand) and
this folder's cards.md: every threat as name, tags, location, counter, uncountered, and its give / get at
configs/agents/values.json. Then run `python scripts/content/load_adversary.py` and `python drafts/adversary/check.py`.

The standard: one consequence (rarely two), hitting the nearest player; one counter option (two at most), never in
a currency its harm takes; no special pieces; tier 2 about two tier-1s. Prices: values.json, a troop 9 (5 placed plus
its strike, 134's convention), an extra threat its op row. Calibration target: harm about 1-1.5x the cheapest counter.
`SCALE` holds the screens' harm scaling (1 = as printed below).
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
VALUES = json.loads((ROOT / "configs" / "agents" / "values.json").read_text(encoding="utf-8"))
RES = VALUES["resources"]
OPS = VALUES["ops"]
TAG = 5  # a sacrificed tag (134's spec)
TROOP = 9  # 5 placed + its strike

# Counter shorthands: E energy, A ambassadors, I insight, R research cubes, C chips, $ credits, T:<tag> tags.
# Tier 1: 3 E, 3 A, 6 I, 3 R, 6 C, 2 tags, 12 $; tier 2: 4 E, 4 A, 7 I, 5 R, 7 C, 3 tags, 17 $ (146: each scarce
# option about 10-20 at values.json; the screens' 85% counter dial, folded in, rounded up as the engine does).
CODES = {"E": "energy", "A": "ambassadors", "I": "insight", "R": "research_cubes", "C": "chips", "$": "credits"}
AMOUNT = {1: {"E": 3, "A": 3, "I": 6, "R": 3, "C": 6, "$": 12, "T": 2},
          2: {"E": 4, "A": 4, "I": 7, "R": 5, "C": 7, "$": 17, "T": 3}}
SAC = {"military": "security", "influence": "diplomacy", "cyber": "cyber", "economic": "industry", "grid": "power"}


def counters(spec, tier):
    out = []
    for code in spec.split("|"):
        if code.startswith("T:"):
            out.append([{"sacrifice": code[2:], "amount": AMOUNT[tier]["T"]}])
        else:
            out.append([{"resource": CODES[code], "amount": AMOUNT[tier][code]}])
    return out


def counter_value(opt):
    item = opt[0]
    if "sacrifice" in item:
        return item["amount"] * TAG
    return item["amount"] * RES[item["resource"]]


def counter_words(opt):
    item = opt[0]
    if "sacrifice" in item:
        return f"{item['amount']} {item['sacrifice'].capitalize()} tags"
    r = item["resource"].replace("research_cubes", "research cubes")
    return f"{item['amount']} {r}"


# ---- effects, priced and worded ----
def troops(n, location=None):
    e = {"op": "troops", "count": n, "at": "location"}
    if location:
        e["location"] = location
    return e


def price(e):
    op = e["op"]
    if op == "troops":
        return TROOP * e["count"]
    if op == "requisition":
        return min(RES[e["resource"]] * e["amount"], VALUES["rates"]["drawn_card"] * e["cards"])
    if op == "theft":
        paid = min(RES[e["resource"]] * e["amount"], VALUES["rates"]["drawn_card"] * e.get("cards", 2))
        return paid - OPS["compute"]["credits"] * e["compute"]
    if op == "compute":
        return -OPS["compute"]["credits"] * e["amount"]
    if op == "sway":
        return VALUES["rates"]["sway"] * e["amount"]
    if op in ("pull_unity", "lose_insight", "destroy_node"):
        return -OPS[op]["credits"] * e.get("amount", e.get("count", 1))
    if op == "discard":
        return VALUES["rates"]["drawn_card"] * e["count"]
    if op == "disable_building":
        return -OPS[op]["credits"] * e["count"]
    if op == "create":
        return -OPS[op]["credits"] * e["count"]
    if op == "deal_threats":
        return -OPS[op]["credits"] * e["count"] * e["tier"]
    if op in ("flip_station", "seize_factory", "destroy_facility", "adversary_factory", "march_now", "extra_war",
              "corrupt_city"):
        return -OPS[op]["credits"]
    if op == "lose_tokens":
        return -OPS[op]["credits"] * e["amount"]
    if op == "place_botnet":
        return -OPS[op]["credits"] * e.get("count", 1)
    raise KeyError(op)


HARM_OF = {
    "troops": "adversary pieces placed", "create": "adversary pieces placed", "compute": "projects advanced (compute)",
    "sway": "Discord sway", "pull_unity": "Unity pulled back", "destroy_node": "node destroyed",
    "discard": "cards discarded", "lose_insight": "insight lost", "disable_building": "actions or claims blocked",
    "flip_station": "facilities destroyed or seized", "seize_factory": "facilities destroyed or seized",
    "destroy_facility": "facilities destroyed or seized", "adversary_factory": "facilities destroyed or seized",
    "corrupt_city": "facilities destroyed or seized", "place_botnet": "adversary pieces placed",
    "deal_threats": "extra threats", "lose_tokens": "tokens stripped from cards", "march_now": "war brought forward", "extra_war": "war brought forward",
}


def harms(effects):
    out = []
    for e in effects:
        if e["op"] in ("requisition", "theft"):
            h = {"energy": "energy lost", "ambassadors": "ambassadors lost", "insight": "insight lost"}.get(
                e["resource"], "credits or income lost")
            if e["op"] == "theft":
                out.append("projects advanced (compute)")
        else:
            h = HARM_OF[e["op"]]
        out.append(h)
    return list(dict.fromkeys(out))


def cards_for(r, n):
    """D62 (amendment 6227): a resource harm is "pays N or discards M cards" (`requisition`), so holding none
    dodges nothing. M: about the payment's worth at 4.5 a card, 1 to 3."""
    return min(3, max(1, round(RES[r] * n / 4.5)))


def lose(r, n):
    return {"op": "requisition", "resource": r, "amount": n, "cards": cards_for(r, n)}


def theft(r, n, compute):
    return {"op": "theft", "who": "nearest", "resource": r, "amount": n, "compute": compute, "cards": cards_for(r, n)}


def compute(n):
    return {"op": "compute", "amount": n}


def discord(n):
    return {"op": "sway", "side": "discord", "amount": n}


def unity(n):
    return {"op": "pull_unity", "amount": n}


def create(n):
    return {"op": "create", "count": n, "at": "location"}


def node(n=1):
    return {"op": "destroy_node", "count": n}


def discard(n):
    return {"op": "discard", "count": n}


def face_down(n):
    return {"op": "disable_building", "count": n, "until": "end_of_next_round"}


def insight(n):
    return {"op": "lose_insight", "amount": n}


def deal(tier):
    return {"op": "deal_threats", "count": 1, "tier": tier, "scope": "country"}


N = "The nearest player"

# (id, name, tags, locations, counter spec, effects, text, check)
T1 = [
    ("orbital_strike", "Orbital Strike", ["military"], ["base"], "E|$", [troops(2)],
     "2 adversary troops land here.", "bites from round 2 where a base touches a player's factory or station: the troops strike there next war"),
    ("commandeer", "Commandeer", ["military"], ["base", "factory"], "A|$", [lose("energy", 3), compute(2)],
     f"{N} loses 3 energy. The adversary gains 2 compute.", "bites any seat holding energy; the counter asks ambassadors, never energy"),
    ("supply_raid", "Supply Raid", ["military", "economic"], ["factory"], "T:industry|$", [troops(1), lose("chips", 3)],
     f"1 adversary troop lands here. {N} loses 3 chips.", "bites chip holders near factories; Industry tags or credits pay it"),
    ("eviction", "Eviction", ["military"], ["city"], "T:security|$", [lose("ambassadors", 2), compute(4)],
     f"{N} loses 2 ambassadors from their supply. The adversary gains 4 compute.", "bites a seat saving ambassadors for a claim; the counter asks Security tags, never ambassadors"),
    ("call_up", "Call-Up", ["military"], ["base"], "A|$", [troops(1), discord(2)],
     "1 adversary troop lands here. Sway 2 toward Discord.", "bites all game; Discord rides with Military here"),
    ("provocation", "Provocation", ["influence", "military"], ["base"], "A|$", [deal(1), discord(2)],
     "A tier-1 threat is dealt into this country (it executes next round). Sway 2 toward Discord.", "bites by tempo: the next threat comes a round early; no loop (the dealt threat is any tag, never a chain past one)"),
    ("agitators", "Agitators", ["influence"], ["city"], "T:diplomacy|$", [create(3), discord(2)],
     "3 adversary ambassadors are created here. Sway 2 toward Discord.", "bites where a city is near corrupting (Corrupt city at 4)"),
    ("defection", "Defection", ["influence"], ["city"], "T:diplomacy|$", [lose("ambassadors", 2), discord(1)],
     f"{N} loses 2 ambassadors from their supply. Sway 1 toward Discord.", "bites diplomats; the counter asks Diplomacy tags, never ambassadors"),
    ("smear_campaign", "Smear Campaign", ["influence"], ["city", "station"], "A|$", [unity(2), insight(4)],
     f"2 Unity comes off the Unity track. {N} gets 4 less insight next round.", "bites the Unity race and next round's keeps"),
    ("propaganda_blitz", "Propaganda Blitz", ["influence"], ["coastal_city"], "A|$", [discord(3), compute(3)],
     "Sway 3 toward Discord. The adversary gains 3 compute.", "bites the Discord race; Influence's home harm"),
    ("sabotage", "Sabotage", ["cyber"], ["station"], "T:cyber|$", [{"op": "lose_tokens", "amount": 4}, compute(6)],
     f"{N} loses 4 tokens from their cards. The adversary gains 6 compute.", "bites seats building toward an At N Effect"),
    ("insider_access", "Insider Access", ["cyber", "influence"], ["station"], "A|$", [create(2), lose("research_cubes", 3)],
     f"2 adversary ambassadors are created here. {N} loses 3 research cubes.", "bites station holders: cubes lost, the station nearer corrupting (at 3)"),
    ("wiper_malware", "Wiper Malware", ["cyber"], ["station"], "R|$", [discard(3)],
     f"{N} discards 3 cards.", "bites a hand held for next round; the counter asks research cubes"),
    ("cryptojacking", "Cryptojacking", ["cyber"], ["station", "factory"], "T:cyber|$", [compute(6), lose("research_cubes", 2)],
     f"The adversary gains 6 compute. {N} loses 2 research cubes.", "feeds the projects; bites cube holders"),
    ("backdoor", "Backdoor", ["cyber"], ["station"], "I|$", [{"op": "place_botnet", "count": 1}],
     "A botnet lands on the station here.", "a lasting piece at tier 1: the botnet makes compute each round and blocks the claim until removed"),
    ("requisition", "Requisition", ["economic"], ["factory", "city"], "T:industry|$", [lose("chips", 5), compute(3)],
     f"{N} loses 5 chips. The adversary gains 3 compute.", "bites chip holders; Industry tags or credits pay it"),
    ("shakedown", "Shakedown", ["economic", "influence"], ["city"], "A|$", [theft("chips", 5, 4)],
     f"Theft: {N.lower()} loses 5 chips; the adversary gains 4 compute.", "bites chip holders; theft hits the nearest player like every harm"),
    ("embargo", "Embargo", ["economic"], ["factory"], "C|$", [create(2), lose("energy", 2)],
     f"2 adversary ambassadors are created here. {N} loses 2 energy.", "bites near factories (Corrupt factory at 3)"),
    ("credit_crunch", "Credit Crunch", ["economic"], ["coastal_city"], "C|$", [discard(2), compute(4)],
     f"{N} discards 2 cards. The adversary gains 4 compute.", "bites a hand held for next round"),
    ("chip_shortage", "Chip Shortage", ["economic"], ["factory"], "C|$", [lose("energy", 3)],
     f"{N} loses 3 energy.", "bites energy holders; chips pay it"),
    ("blackout", "Blackout", ["grid", "cyber"], ["city"], "T:power|$", [lose("energy", 3)],
     f"{N} loses 3 energy.", "bites energy holders; Power tags or credits pay it"),
    ("load_shedding", "Load Shedding", ["grid"], ["city", "station"], "R|$", [face_down(1), compute(3)],
     f"{N} turns 1 building face down until the end of next round. The adversary gains 3 compute.", "bites engine seats from round 3"),
    ("power_surcharge", "Power Surcharge", ["grid"], ["station"], "R|$", [lose("energy", 2), compute(4)],
     f"{N} loses 2 energy. The adversary gains 4 compute.", "bites energy holders; research cubes pay it"),
    ("cooling_failure", "Cooling Failure", ["grid"], ["station"], "T:power|$", [lose("research_cubes", 3), compute(3)],
     f"{N} loses 3 research cubes. The adversary gains 3 compute.", "bites cube holders"),
    ("grid_tap", "Grid Tap", ["grid"], ["city"], "R|$", [theft("energy", 2, 4)],
     f"Theft: {N.lower()} loses 2 energy; the adversary gains 4 compute.", "bites energy holders; theft hits the nearest player"),
]

T2 = [
    ("total_war", "Total War", ["military"], ["base"], "E|$", [troops(3), {"op": "march_now"}],
     "3 adversary troops land here and march at once.", "bites from stage II where a base touches player sites: the strike comes before the players can answer"),
    ("siege", "Siege", ["military"], ["factory"], "A|$", [{"op": "destroy_facility", "at": "factory"}, compute(4)],
     "The factory here is destroyed. The adversary gains 4 compute.", "bites the factory's owner (a node on it goes with it, never a last node)"),
    ("manhunt", "Manhunt", ["military"], ["base", "city"], "E|$", [lose("ambassadors", 3), troops(1, "base")],
     f"{N} loses 3 ambassadors from their supply. 1 adversary troop lands on the base here.", "bites diplomats; energy or credits pay it"),
    ("conscription", "Conscription", ["military", "economic"], ["base"], "E|$", [troops(2), lose("chips", 4)],
     f"2 adversary troops land here. {N} loses 4 chips.", "bites near bases; a normal troop marches and strikes as any"),
    ("casus_belli", "Casus Belli", ["military"], ["base"], "A|$", [{"op": "extra_war"}, discord(3)],
     "This base's front has a second war step next round. Sway 3 toward Discord.", "bites where corp troops can win a front; Discord rides with Military here"),
    ("overrun", "Overrun", ["influence"], ["city"], "I|$", [{"op": "corrupt_city", "at": "city"}],
     "The city here is corrupted.", "a lasting harm: the city corrupts as by the Corrupt line (a node on it is destroyed, an ambassador there is armor)"),
    ("insurrection", "Insurrection", ["influence"], ["city"], "T:diplomacy|$", [create(4), unity(3)],
     "4 adversary ambassadors are created here. 3 Unity comes off the Unity track.", "bites the Unity race and the city's next Corrupt line"),
    ("deepfake_wave", "Deepfake Wave", ["influence", "cyber"], ["city", "station"], "A|$", [discord(3), unity(2)],
     "Sway 3 toward Discord. 2 Unity comes off the Unity track.", "bites both tracks"),
    ("turncoats", "Turncoats", ["influence"], ["city"], "T:diplomacy|$", [lose("ambassadors", 4), compute(3)],
     f"{N} loses 4 ambassadors from their supply. The adversary gains 3 compute.", "bites diplomats; Diplomacy tags or credits pay it"),
    ("propaganda_ministry", "Propaganda Ministry", ["influence"], ["city", "coastal_city"], "T:diplomacy|$", [discord(4), compute(4)],
     "Sway 4 toward Discord. The adversary gains 4 compute.", "bites the Discord race"),
    ("skimming_operation", "Skimming Operation", ["economic", "influence"], ["coastal_city"], "A|$", [theft("chips", 8, 6)],
     f"Theft: {N.lower()} loses 8 chips; the adversary gains 6 compute.", "bites chip holders"),
    ("dark_cities", "Dark Cities", ["grid", "influence"], ["city", "coastal_city"], "T:power|$", [node(1), unity(3)],
     f"{N} destroys 1 of their nodes closest to it. 3 Unity comes off the Unity track.", "bites a seat's network and the Unity race; never a last node"),
    ("station_seizure", "Station Seizure", ["cyber"], ["station"], "T:cyber|$", [{"op": "flip_station", "force": True}, compute(4)],
     "The station here is corrupted, even a player's. The adversary gains 4 compute.", "a lasting harm on the station's owner"),
    ("sleeper_cells", "Sleeper Cells", ["cyber"], ["station"], "T:cyber|$", [{"op": "place_botnet", "count": 1}, create(2)],
     "A botnet lands on the station here. 2 adversary ambassadors are created here.", "bites the station: compute each round, nearer corrupting"),
    ("purge_protocols", "Purge Protocols", ["cyber"], ["station"], "R|$", [node(1), discard(3)],
     f"{N} destroys 1 of their nodes closest to it and discards 3 cards.", "bites a seat's network and hand; never a last node"),
    ("data_breach", "Data Breach", ["cyber", "economic"], ["station", "factory"], "C|$", [lose("research_cubes", 5), compute(5)],
     f"{N} loses 5 research cubes. The adversary gains 5 compute.", "bites cube holders before a megaproject"),
    ("zero_day", "Zero Day", ["cyber"], ["station"], "R|$", [compute(10), face_down(1)],
     f"The adversary gains 10 compute. {N} turns 1 building face down until the end of next round.", "feeds the projects hard"),
    ("asset_freeze", "Asset Freeze", ["economic"], ["factory"], "E|$", [face_down(2), discard(2)],
     f"{N} turns 2 buildings face down until the end of next round and discards 2 cards.", "bites engine seats"),
    ("mass_production", "Mass Production", ["economic"], ["factory"], "C|$", [{"op": "adversary_factory"}, compute(5)],
     "An adversary factory is built here. The adversary gains 5 compute.", "a lasting site for the adversary"),
    ("factory_occupation", "Factory Occupation", ["economic", "military"], ["factory"], "E|$", [{"op": "seize_factory"}, compute(5)],
     "The adversary seizes the factory here, even a player's. The adversary gains 5 compute.", "a lasting harm on the factory's owner"),
    ("trade_blockade", "Trade Blockade", ["economic"], ["coastal_city", "factory"], "E|$", [create(4), lose("chips", 6)],
     f"4 adversary ambassadors are created here. {N} loses 6 chips.", "bites chip holders and the site's next Corrupt line"),
    ("grid_collapse", "Grid Collapse", ["grid"], ["city", "coastal_city"], "R|$", [node(1), lose("energy", 4)],
     f"{N} destroys 1 of their nodes closest to it and loses 4 energy.", "bites a seat's network and energy; never a last node"),
    ("power_rationing", "Power Rationing", ["grid"], ["city"], "A|$", [lose("energy", 4)],
     f"{N} loses 4 energy.", "bites energy holders; ambassadors or credits pay it"),
    ("thermal_runaway", "Thermal Runaway", ["grid"], ["station"], "T:power|$", [lose("research_cubes", 6), compute(4)],
     f"{N} loses 6 research cubes. The adversary gains 4 compute.", "bites cube holders"),
    ("overload", "Overload", ["grid"], ["factory", "station"], "T:power|$", [node(2), compute(6)],
     f"{N} destroys 2 of their nodes closest to it (never their last). The adversary gains 6 compute.", "bites a seat's network hardest"),
    ("garrison_state", "Garrison State", ["universal"], ["city", "base"], "I|$", [create(3), troops(2, "base")],
     "3 adversary ambassadors are created on the city here. 2 adversary troops land on the base here.", "bites both sites"),
    ("shock_doctrine", "Shock Doctrine", ["universal"], ["city", "factory"], "E|$", [lose("ambassadors", 2), unity(3)],
     f"{N} loses 2 ambassadors from their supply. 3 Unity comes off the Unity track.", "bites diplomats and the Unity race"),
    ("full_spectrum_assault", "Full Spectrum Assault", ["universal"], ["station"], "A|$", [{"op": "place_botnet", "count": 1}, deal(1)],
     "A botnet lands on the station here. A tier-1 threat is dealt into this country (it executes next round).", "bites by tempo and a lasting piece"),
    ("last_ultimatum", "Last Ultimatum", ["universal"], ["city", "base"], "R|$", [deal(2), node(1)],
     f"A tier-2 threat is dealt into this country (it executes next round). {N} destroys 1 of their nodes closest to it.", "146: re-cut from tier 3; the dealt threat is any tag, never a chain past one"),
    ("iron_curtain", "Iron Curtain", ["universal"], ["factory", "city"], "E|$", [troops(2), unity(2)],
     "2 adversary troops land on the factory here. 2 Unity comes off the Unity track.", "146: re-cut from tier 3; bites the factory and the Unity race"),
    ("war_economy", "War Economy", ["military", "economic"], ["factory", "base"], "E|$", [{"op": "adversary_factory"}, troops(1, "factory")],
     "An adversary factory is built here. 1 adversary troop lands on the factory here.", "146: re-cut from tier 3; a lasting adversary site"),
    ("cascade_failure", "Cascade Failure", ["grid", "cyber"], ["station", "city"], "I|$", [{"op": "place_botnet", "count": 1}, lose("energy", 3)],
     f"A botnet lands on the station here. {N} loses 3 energy.", "146: re-cut from tier 3; a lasting piece and an energy hit"),
]

OLD_IDS = {"last_ultimatum": "t3_last_ultimatum", "iron_curtain": "t3_iron_curtain", "war_economy": "t3_war_economy",
           "cascade_failure": "t3_cascade_failure"}


WORDS = {"energy": "energy", "chips": "chips", "research_cubes": "research cubes", "ambassadors": "ambassadors"}


def pay_or_discard(text, effects):
    """The face's "loses N X" for a requisition: "pays N X or discards M cards"."""
    for e in effects:
        if e["op"] not in ("requisition", "theft"):
            continue
        word = WORDS[e["resource"]]
        m = e.get("cards", 2)
        for old in (f"loses {e['amount']} {word} from their supply", f"loses {e['amount']} {word}"):
            if old in text:
                text = text.replace(old, f"pays {e['amount']} {word} or discards {m} cards", 1)
                break
        else:
            raise ValueError(f"{text!r}: no loss of {e['amount']} {word}")
    return text


def build(tier, rows):
    out = []
    for cid, name, tags, locs, spec, effects, text, check in rows:
        text = pay_or_discard(text, effects)
        menu = counters(spec, tier)
        give = [counter_value(o) for o in menu]
        harm = sum(price(e) for e in effects)
        cheap = min(give)
        words = " or ".join(f"{counter_words(o)} ({v:g})" for o, v in zip(menu, give))
        value = (f"counter {words}; uncountered {harm:g} at values.json (a troop 9 with its strike), "
                 f"{harm / cheap:.2f}x the cheapest counter")
        out.append({
            "id": cid, "name": name, "tier": tier, "tags": tags, "locations": locs, "text": text,
            "counters": menu, "effects": effects, "harms": harms(effects), "value": value, "check": check,
            "flags": ["146: rewritten to Matt's threat card standard (2026-11-07): one consequence or two, the nearest "
                      "player hit, one counter option or two, no shared currency, no special pieces"],
            "reuses": OLD_IDS.get(cid, cid),
            "_give": give, "_harm": harm,
        })
    return out


def strip(cards):
    return [{k: v for k, v in c.items() if not k.startswith("_")} for c in cards]


NOTE = {1: "Tier-1 threat deck, 146 (Matt's threat card standard, 2026-11-07): each card has one consequence, rarely "
           "two, on the nearest player (the one whose node is closest to its token) or the board; one counter option or "
           "two, never in a currency its harm takes. Counters: 3 energy, 3 ambassadors, 6 insight, 3 research cubes, "
           "6 chips, 2 tags or 12 credits. Value lines price at configs/agents/values.json (a troop 9: placed 5 plus its "
           "strike). Source: experiments/results/146_threat_standard/threat_cards.py.",
        2: "Tier-2 threat deck, 146 (Matt's threat card standard, 2026-11-07): bigger consequences, about two tier-1s; "
           "four cards re-cut from the old tier 3 (Last Ultimatum, Iron Curtain, War Economy, Cascade Failure). Counters: "
           "4 energy, 4 ambassadors, 7 insight, 5 research cubes, 7 chips, 3 tags or 17 credits. Value lines price at "
           "configs/agents/values.json. Source: experiments/results/146_threat_standard/threat_cards.py."}


def table(cards):
    lines = ["| Threat | Tags | Location | Counter | Uncountered | Give (cheapest) | Get | Get / give |",
             "|---|---|---|---|---|---|---|---|"]
    for c in cards:
        counter = " or ".join(counter_words(o) for o in c["counters"])
        lines.append(f"| {c['name']} | {', '.join(t.capitalize() for t in c['tags'])} | "
                     f"{', '.join(l.replace('_', ' ') for l in c['locations'])} | {counter} | {c['text']} | "
                     f"{min(c['_give']):g} | {c['_harm']:g} | {c['_harm'] / min(c['_give']):.2f} |")
    return "\n".join(lines)


def main():
    decks = {1: build(1, T1), 2: build(2, T2)}
    check = "--check" in sys.argv
    stale = False
    for tier, cards in decks.items():
        path = ROOT / "drafts" / "adversary" / f"threats_t{tier}.json"
        doc = {"deck": f"threats_t{tier}", "note": NOTE[tier], "cards": strip(cards)}
        text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
        if check:
            stale |= path.read_text(encoding="utf-8") != text
        else:
            path.write_text(text, encoding="utf-8", newline="\n")
    md = ["# 146: every threat, at the standard", "",
          "Generated by `threat_cards.py`. Give: the cheapest counter at `configs/agents/values.json` (a tag 5). Get: the",
          "uncountered harm at the same table (a troop 9, placed 5 plus its strike; compute 1.1, a node 4, Sway and Unity 4,",
          "a building face down 6, a card 4; \"pays N or discards M cards\" at the cheaper), on the one nearest player or the board. Target 1-1.5.", ""]
    for tier, cards in decks.items():
        ratios = [c["_harm"] / min(c["_give"]) for c in cards]
        md += [f"## Tier {tier} ({len(cards)} cards; get / give median {sorted(ratios)[len(ratios) // 2]:.2f}, "
               f"range {min(ratios):.2f}-{max(ratios):.2f})", "", table(cards), ""]
    out = Path(__file__).with_name("cards.md")
    if check:
        stale |= out.read_text(encoding="utf-8") != "\n".join(md)
        sys.exit(1 if stale else 0)
    out.write_text("\n".join(md), encoding="utf-8", newline="\n")
    print({t: len(c) for t, c in decks.items()})


if __name__ == "__main__":
    main()
