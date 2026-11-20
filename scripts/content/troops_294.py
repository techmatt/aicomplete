"""294 (overhaul 3, troops and winning): the content edits, as one pass over the sources as they stood at r140.

    git checkout 5666e85482 -- drafts/adversary content/alignment.json content/ais.json   # the r140 sources
    python scripts/content/troops_294.py     # edit drafts/adversary/*.json, content/alignment.json, content/ais.json
    python scripts/regen.py

Troops come from the research row, projects and Discord cards (Matt, 2026-11-19): no event card musters and no
threat lands one. A stage-III card's Muster 1 becomes a Strike icon, two stage-II cards gain one, and Forced
March's extra step becomes a Redeploy icon. A threat's troop line becomes a research-cube line (the row is where
its troops now come from). Research-cube lines are rescaled to K 2 a country (293 had halved them for K 1).
The Militarist's row takes its Garrison and Strike icons. The Protector's Wall and Vigil are reworded for troops
that garrison and strike; Demilitarized Zone forbids a Strike in place of a Muster. On sources already passed it
changes nothing.
"""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
FLAG = "294 (Matt, 2026-11-19): "
LOG = []

STRIKE = "Strike: each Troop destroys 1 player Node, research station or factory on or next to it."
REDEPLOY = "Redeploy: each Troop not on or next to a corrupted city moves onto the nearest corrupted city."
GARRISON = ("Each country: 1 Troop garrisons its lowest-numbered corrupted factory (with none, its lowest-numbered "
            "corrupted city; with none, its lowest-numbered city).")
MUSTER = "Muster 1: each country, 1 Troop on its lowest-numbered factory (else city)."


def load(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def dump(path, data):
    (ROOT / path).write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def flag(card, text):
    if FLAG + text not in card.setdefault("flags", []):
        card["flags"].append(FLAG + text)


def reword(card, old, new, why):
    """One printed line of `card` changes from `old` to `new`; the old -> new pair is logged for the write-up."""
    if old not in card["text"]:
        return False
    card["text"] = card["text"].replace(old, new)
    LOG.append((card.get("name", card["id"]), old, new))
    flag(card, f'"{old}" became "{new}" ({why})')
    return True


def swap_harm(card, old, new):
    harms = card.get("harms", [])
    if old in harms:
        harms.remove(old)
    for h in new:
        if h not in harms:
            harms.append(h)


# --- event cards -----------------------------------------------------------------------------------------------
STRIKE_HARMS = ["facilities destroyed or corrupted", "node destroyed"]
STAGE2_STRIKES = ("mil_s2_mobilization", "mil_s2_fifth_column")


def events(mil):
    for e in mil["events"]:
        muster = [x for x in e["standing"] if x["op"] == "troops"]
        if muster:
            i = e["standing"].index(muster[0])
            e["standing"][i] = {"op": "strike"}
            reword(e, MUSTER, STRIKE, "no event card musters: troops come from the research row, projects and "
                                      "Discord cards, and strike on this icon, shown a round ahead")
            swap_harm(e, "troops made", STRIKE_HARMS)
        if e["id"] in STAGE2_STRIKES and {"op": "strike"} not in e["standing"]:
            at = max(i for i, x in enumerate(e["standing"]) if x["op"] == "infiltrate") + 1
            e["standing"].insert(at, {"op": "strike"})
            site = {"coastal_city": "coastal cities", "city": "cities"}[e["standing"][at - 1]["site"]]
            reword(e, f"Infiltrate: {site}.", f"Infiltrate: {site}. {STRIKE}",
                   "a Strike icon in stage II, where games now end: troops strike only on an icon")
            swap_harm(e, "", STRIKE_HARMS)
        if e["id"] == "mil_s3_occupation" and {"op": "redeploy"} not in e["standing"]:
            e["standing"].insert(e["standing"].index({"op": "strike"}) + 1, {"op": "redeploy"})
            reword(e, STRIKE, f"{STRIKE} {REDEPLOY}", "a second, late Redeploy icon")
            swap_harm(e, "", ["actions or claims blocked"])
        steps = [t for t in e["twists"] if t["op"] == "troop_steps"]
        if steps:
            e["twists"].remove(steps[0])
            e["standing"].append({"op": "redeploy"})
            reword(e, "Every adversary Troop takes 1 more step this round.", REDEPLOY,
                   "the march is gone: troops garrison, and move only on this icon")
            swap_harm(e, "march brought forward", ["actions or claims blocked"])
            if e["name"] == "Forced March":
                e["name"] = "Redeployment"
                flag(e, "display name Forced March -> Redeployment (the id stays)")


# --- the board -------------------------------------------------------------------------------------------------
ICONS = [
    {"id": "garrison", "name": "Garrison", "glyph": "troop, then the factory glyph", "text": GARRISON,
     "effects": [{"op": "troops", "count": 1, "each_country": True}],
     "value": "values.json troops: a troop a country, which guards its site (no liberation on or next to it) and "
              "strikes on every Strike icon until it is removed."},
    {"id": "strike", "name": "Strike", "glyph": "troop with a burst", "text": STRIKE, "effects": [{"op": "strike"}],
     "value": "values.json strike: every troop on the board destroys one player piece beside it."},
    {"id": "redeploy", "name": "Redeploy", "glyph": "troop with an arrow to a dark-framed city", "text": REDEPLOY,
     "effects": [{"op": "redeploy"}],
     "value": "values.json redeploy: a troop away from every corrupted city goes to guard one."},
]
ROW = ["factory", "cells_factory", "income", "garrison", "strike", "garrison"]


def board(mil):
    b = mil["board"]
    have = {i["id"] for i in b["icons"]}
    b["icons"] += [i for i in ICONS if i["id"] not in have]
    r = b["research"]
    if r["slots"] != ROW:
        LOG.append(("War Research (the row)", ", ".join(r["slots"]), ", ".join(ROW)))
        r["slots"] = ROW
        r["text"] = ("Every 2 cubes a country on the track (4 cubes with two countries, 6 with three) are spent to "
                     "advance the marker one slot; the slot's icon happens once. Slot 7: the players lose.")
        r["value"] = ("294: K 2 a country (`adversary.research_at`; the row's text prints it). Slots 1-3 are an event "
                      "line's worth each (a site opening, an Infiltrate, +1 compute); slots 4-6 are the scary ones: a "
                      "troop a country, a Strike, a second troop a country; slot 7 loses the game.")
        flag(b, "the research row takes its troop icons: New factory, Cells: factories, Funding, Garrison, Strike, "
                "Garrison (was New factory, Cells: factories, Funding, Fallout, Cells: cities, Dread); K 2 a country")


# --- threats ---------------------------------------------------------------------------------------------------
# A threat's troop line becomes research cubes: (cubes, the printed line it replaces).
THREAT_CUBES = {
    "orbital_strike": (2, "1 adversary Troop lands here."),
    "supply_raid": (2, "1 adversary Troop lands here."),
    "call_up": (2, "1 adversary Troop lands here."),
    "total_war": (4, "2 adversary Troops land here and march at once."),
    "manhunt": (3, "1 adversary Troop lands here."),
    "conscription": (3, "1 adversary Troop lands here."),
    "garrison_state": (3, "1 adversary Troop lands on the factory here."),
    "iron_curtain": (3, "1 adversary Troop lands on the factory here."),
    "war_economy": (3, "1 adversary Troop lands on the factory here."),
}
# 293's cube lines, written for K 1 a country: doubled for K 2.
THREAT_DOUBLED = {"data_breach": ("1 cube goes onto the adversary's research track.", 2),
                  "zero_day": ("2 cubes go onto the adversary's research track.", 4)}


def threats(deck):
    for c in deck["cards"]:
        if c["id"] in THREAT_CUBES and any(e["op"] == "troops" for e in c["effects"]):
            n, old = THREAT_CUBES[c["id"]]
            i = next(i for i, e in enumerate(c["effects"]) if e["op"] == "troops")
            c["effects"][i] = {"op": "adversary_research", "amount": n,
                               "why": "294 (Matt, 2026-11-19): no threat lands a troop; its troop became cubes toward "
                                      "the research row, where troops now come from (a troop at values.json troops "
                                      "4.67, a cube 2)"}
            c["effects"] = [e for e in c["effects"] if e["op"] != "march_now"]
            reword(c, old, f"{n} cubes go onto the adversary's research track.",
                   "troops come from the research row, projects and Discord cards; a threat feeds the row")
            swap_harm(c, "adversary pieces placed" if not any(e["op"] in ("create", "adversary_factory", "place_botnet") for e in c["effects"]) else "", ["adversary research advanced"])
            swap_harm(c, "march brought forward", [])
            c["check"] = "294: bites through the research row: its cubes bring the row's Garrison and Strike slots nearer."
            c["value"] = (f"294: its troop line (priced at a troop, 9 with its strike) became {n} research cubes "
                          f"(values.json adversary_research, 2 a cube at K 1; K 2 halves a cube's share of a slot). "
                          f"Before: {c['value']}")
        if c["id"] in THREAT_DOUBLED:
            old, n = THREAT_DOUBLED[c["id"]]
            for e in c["effects"]:
                if e["op"] == "adversary_research" and old in c["text"]:
                    e["amount"] = n
            reword(c, old, f"{n} cubes go onto the adversary's research track.", "K 2 a country: a cube is half a K-1 cube")


# --- projects --------------------------------------------------------------------------------------------------
def projects(deck):
    for c in deck["cards"]:
        for e in c["effects"]:
            if e["op"] == "adversary_research" and e.get("each_country") and e["amount"] == 1:
                old = "1 cube onto the adversary's research track" if "1 cube onto" in c["text"] else "1 cube goes onto the adversary's research track"
                new = old.replace("1 cube goes", "2 cubes go").replace("1 cube onto", "2 cubes onto")
                if reword(c, old, new, "K 2 a country: a cube is half a K-1 cube"):
                    e["amount"] = 2
        if c["id"] == "botnet_conscripts":
            for e in c["effects"]:
                if e["op"] == "botnet_troops" and e["troops"] == 3:
                    e["troops"] = 2
                    reword(c, "Each country: its lowest-numbered botnet becomes 3 Troops on its hex.",
                           "Each country: its lowest-numbered botnet becomes 2 Troops, which garrison its "
                           "lowest-numbered corrupted factory (with none, its lowest-numbered corrupted city; with "
                           "none, its lowest-numbered city).",
                           "troops garrison a corrupted factory, else a city; 3 -> 2 since a troop now stays after "
                           "it strikes and guards its site")
        if c["id"] == "war_plants":
            for e in c["effects"]:
                if e["op"] == "factory_troops" and e["count"] == 2:
                    e["count"] = 1
                    reword(c, "Every adversary factory makes 2 Troops on its hex (a player Ambassador on it stops them).",
                           "Every adversary factory makes 1 Troop on its hex (a player Ambassador on it stops it).",
                           "2 -> 1: a troop now stays after it strikes and guards its factory and the city beside it")


# --- Unity and Discord cards -----------------------------------------------------------------------------------
DMZ_OLD = [
    ("on or next to", "by_player"), ("within 2 of", "near_player"), ("within 2 of", "near_player"),
]
FIRESTORM_OLD = ('{"text": "Each country: create 4 Agents on the lowest-numbered city holding Agents.",\n'
                 '    "effects": [{"op": "create", "count": 4, "at": "city", "if": "ambassadors"}]}')
FIRESTORM_TEXT = ("Each country: create 2 Agents on the lowest-numbered city holding Agents; then 1 Troop garrisons its "
                  "lowest-numbered corrupted factory (with none, its lowest-numbered corrupted city; with none, its "
                  "lowest-numbered city).")
FIRESTORM_NEW = ('{"text": "' + FIRESTORM_TEXT + '",\n'
                 '    "effects": [{"op": "create", "count": 2, "at": "city", "if": "ambassadors"}, '
                 '{"op": "troops", "count": 1, "each_country": true}]}')


def alignment():
    """`content/alignment.json` is laid out by hand: edited as text."""
    path = ROOT / "content/alignment.json"
    s = path.read_text(encoding="utf-8")
    for words, _ in set(DMZ_OLD):
        old = f"In the next adversary turn, no Troop is mustered on a site {words} an Ambassador or a Node: it stays in the supply."
        new = f"In the next adversary turn, no Troop strikes a hex {words} an Ambassador or a Node."
        if old in s:
            LOG.append(("Demilitarized Zone", old, new))
            s = s.replace(old, new)
    s = s.replace('"op": "prohibit", "step": "muster"', '"op": "prohibit", "step": "strike"')
    if FIRESTORM_OLD in s:
        LOG.append(("Firestorm III", "Each country: create 4 Agents on the lowest-numbered city holding Agents.", FIRESTORM_TEXT))
        s = s.replace(FIRESTORM_OLD, FIRESTORM_NEW)
    path.write_text(s, encoding="utf-8", newline="\n")


# --- the Protector's board -------------------------------------------------------------------------------------
WALL_OLD = ("Adversary Troops can't enter or strike a hex with your Node. Vigil: the first two times each round a "
            "Troop ends its march next to your Node, gain 1 insight.")
WALL_NEW = ("No adversary Troop is placed on, Redeploys onto or strikes a hex with your Node. Vigil: when Troops "
            "Strike, gain 1 insight for each Troop on or next to your Nodes (at most 2 a round).")


BOARD_OLD = "The defender: plays on the board; Troops stop at its Nodes and can't strike there."
BOARD_NEW = "The defender: plays on the board; Troops can't garrison or strike where its Nodes stand."


def ais(data):
    text = json.dumps(data, ensure_ascii=False)
    for b in data:
        if b.get("text") == BOARD_OLD:
            LOG.append(("The Protector (board summary)", BOARD_OLD, BOARD_NEW))
            b["text"] = BOARD_NEW
    if WALL_OLD in text:
        LOG.append(("Wall and Vigil (the Protector)", WALL_OLD, WALL_NEW))

    def walk(x):
        if isinstance(x, dict):
            for k, v in x.items():
                if v == WALL_OLD:
                    x[k] = WALL_NEW
                elif k == "event" and v == "march_ends_near":
                    x[k] = "strike_near"
                else:
                    walk(v)
        elif isinstance(x, list):
            for v in x:
                walk(v)
    walk(data)


def main():
    mil = load("drafts/adversary/militarist.json")
    events(mil)
    board(mil)
    dump("drafts/adversary/militarist.json", mil)
    for tier in (1, 2):
        path = f"drafts/adversary/threats_t{tier}.json"
        deck = load(path)
        threats(deck)
        dump(path, deck)
    deck = load("drafts/adversary/projects.json")
    projects(deck)
    dump("drafts/adversary/projects.json", deck)
    alignment()
    data = load("content/ais.json")
    ais(data)
    dump("content/ais.json", data)
    out = ROOT / "experiments" / "results" / "294_overhaul_3_troops_and_winning"
    out.mkdir(parents=True, exist_ok=True)
    if LOG:
        rows = ["| Card | Old | New |", "|---|---|---|"] + [f"| {n} | {a} | {b} |" for n, a, b in LOG]
        (out / "content_changes.md").write_text("# 294: content, old -> new\n\n" + "\n".join(rows) + "\n", encoding="utf-8", newline="\n")
    print(len(LOG), "lines reworded")


main()
