"""293 (overhaul 2, the adversary spine): the content edits, as one pass over the sources as they stood at r139.

    git checkout 4126a21b9a -- drafts/adversary content/alignment.json content/map.json   # the r139 sources
    python scripts/content/spine_293.py      # edit drafts/adversary/*.json, content/alignment.json, content/map.json
    python scripts/regen.py

Event cards lose their Spill and Grow lines (the escalate is a rule, read off last round's card). A project's or
Discord card's "Surge N: X" keeps its N agents and its type and loses its pick: a creation line, N agents on the
lowest-numbered X not yet the adversary's. Research-cube lines are rescaled to the row's K (1 a country): a threat's
1 cube (Zero-Day 2), a project's 1 a country; Backdoor Research's megaproject cubes become its research line.
Megaprojects lose their adversary payout. The Militarist's board gets its research row, the icon vocabulary and its
one rule. On sources already passed it changes nothing but would repeat no edit.
"""
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
PLURAL = {"city": "cities", "coastal_city": "coastal cities", "station": "research stations", "factory": "factories",
          "botnet": "botnet hexes"}
FLAG = "293 (Matt, 2026-11-19): "


def load(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def dump(path, data):
    (ROOT / path).write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def flag(card, text):
    if FLAG + text not in card.setdefault("flags", []):
        card["flags"].append(FLAG + text)


# --- event cards -----------------------------------------------------------------------------------------------
def events(mil):
    for e in mil["events"]:
        before = len(e["twists"])
        e["twists"] = [t for t in e["twists"] if t["op"] not in ("spill", "grow")]
        text = re.sub(r" Spill \(level 2\+\): .*?holding the most\.", "", e["text"])
        text = re.sub(r" Grow: [a-z ]+\.", "", text)
        if True:
            e["text"] = text
            e["value"] = ("293: Infiltrate lands an agent a site of each named type (about 2-3 a country); next round each "
                          "of those sites still holding an agent takes 1 more (escalate, a rule of the Card beat, priced "
                          "with the card that names the type: values.json's infiltrate and escalate rows).")
            e["check"] = ("293: the named sites take an agent each; a site at its number corrupts on arrival; next round "
                          "the card sits in the board's last-round space and its sites escalate.")
            if before != len(e["twists"]):
                flag(e, "Spill and Grow lines cut: an event card is Infiltrate X plus escalate (its other lines stay)")


# --- the board -------------------------------------------------------------------------------------------------
ICONS = [
    {"id": "factory", "name": "New factory", "glyph": "factory glyph with a plus",
     "text": "Each country: a neutral factory opens on its lowest-numbered free industrial zone.",
     "effects": [{"op": "create_site", "kind": "factory"}],
     "value": "values.json create_site: a claim target for the players, a corrupt target for the adversary (Grow's heir)."},
    {"id": "lab", "name": "New lab", "glyph": "station glyph with a plus",
     "text": "Each country: a neutral research station opens on its lowest-numbered empty station site.",
     "effects": [{"op": "create_site", "kind": "station"}],
     "value": "values.json create_site; only countries printing an empty station site take one."},
    {"id": "cells_city", "name": "Cells: cities", "glyph": "agent, then the city glyph",
     "text": "Infiltrate: cities.", "effects": [{"op": "infiltrate", "site": "city"}],
     "value": "values.json infiltrate: an extra Infiltrate, about 1-3 agents a country."},
    {"id": "cells_factory", "name": "Cells: factories", "glyph": "agent, then the factory glyph",
     "text": "Infiltrate: factories.", "effects": [{"op": "infiltrate", "site": "factory"}],
     "value": "values.json infiltrate."},
    {"id": "cells_station", "name": "Cells: research stations", "glyph": "agent, then the station glyph",
     "text": "Infiltrate: research stations.", "effects": [{"op": "infiltrate", "site": "station"}],
     "value": "values.json infiltrate."},
    {"id": "income", "name": "Funding", "glyph": "brown box: compute",
     "text": "Income: +1 compute a country a round.", "effects": [{"op": "increase_income", "compute": 1}],
     "value": "values.json increase_income: a compute point a country for the rest of the game."},
    {"id": "botnet", "name": "Botnet", "glyph": "botnet glyph with an arrow",
     "text": "Botnets spread (level 1+): each country, a Botnet onto its lowest-numbered site without one next to one of its Botnets (a country with none: its lowest-numbered research station).",
     "effects": [{"op": "spread_botnets", "count": 1}],
     "value": "values.json spread_botnets: a botnet a country, its compute a round."},
    {"id": "fallout", "name": "Fallout", "glyph": "Fallout glyph by a dark-framed city",
     "text": "Each country: a Fallout lands next to its lowest-numbered corrupted city.",
     "effects": [{"op": "fallout", "at": "corrupted_city"}],
     "value": "values.json fallout: a hex lost and its Discord drip."},
    {"id": "dread", "name": "Dread", "glyph": "Discord track, arrow up",
     "text": "Sway 3 toward Discord.", "effects": [{"op": "sway", "side": "discord", "amount": 3, "narrative": "fear"}],
     "value": "values.json sway × 3."},
    {"id": "blackout", "name": "Blackout", "glyph": "city glyph with energy struck through",
     "text": "Until the end of next round, cities make no energy and no Discord.",
     "effects": [{"op": "flag", "name": "blackout", "everywhere": True}],
     "value": "values.json flag: a round of city energy, every player's."},
]


def board(mil):
    b = mil["board"]
    b["rules"] = [{
        "name": "Commandeered Plants",
        "text": "Every round, after last round's sites escalate: each factory not yet the adversary's that holds an Agent takes 1 more.",
        "effects": [{"op": "escalate", "site": "factory"}],
    }]
    b["icons"] = ICONS
    b["research"] = {
        "name": "War Research",
        "text": "Each cube a country on the track (2 cubes with two countries, 3 with three) is spent to advance the marker one slot; the slot's icon happens once. Slot 7: the players lose.",
        "slots": ["factory", "cells_factory", "income", "fallout", "cells_city", "dread"],
        "value": "293: a slot is one event-card line's worth (a site opening, an Infiltrate, +1 compute, a Fallout, 3 Discord); "
                 "the row's weight is its clock: 7 slots of K cubes a country (K 1: `adversary.research_at`; the row's text prints it) lose the game.",
    }
    for v in b.get("levels", []):
        if v["name"] == "Arms Race":
            v["rule"] = ""
    b["value"] = ("293: Commandeered Plants: a factory holding an agent takes 1 more a round (values.json escalate), so an "
                  "infiltrated factory falls in two rounds unless its agent is cleared; then it feeds its city an agent a "
                  "round. Against one ambassador (cancels an agent) or Remove agents.")
    b["check"] = ("293: Botnet Recruiters went with botnet seeding (a botnet only makes compute); the board prints one "
                  "rule, Commandeered Plants, a research row of six icon slots and its threat-deck line.")
    b["creates_text"] = "Commandeered Plants: each factory holding an agent takes 1 more a round."
    flag(b, "the spine: Botnet Recruiters cut (no seeding); one rule, Commandeered Plants; the research rule Weapons Program "
            "became a row of six non-troop icon slots (item 3 swaps troop icons in by editing `research.slots`)")


# --- projects, threats, Discord cards ---------------------------------------------------------------------------
SITE = {"city": "city", "coastal_city": "coastal city", "station": "research station", "factory": "factory"}


def create_text(site, count):
    return f"Each country: create {count} Agents on its lowest-numbered {SITE[site]} not yet the adversary's."


def projects(deck):
    for p in deck["cards"]:
        ops = []
        for e in p["effects"]:
            if e["op"] == "surge":
                n, site = e["count"], e["site"]
                ops.append({"op": "create", "count": n, "at": site, "if": "clean"})
                old = re.search(r"Surge \d: .*?holding the most Agents\.", p["text"]).group(0)
                p["text"] = p["text"].replace(old, create_text(site, n))
                flag(p, f"\"{old}\" became \"{create_text(site, n)}\" (Surge's most-agents pick is gone; its agents and type stay)")
                p["value"] = (f"293: give {p['cost']} compute a board; its line lands {n} agents on one {SITE[site]} a country "
                              f"(values.json create, 1.333 an agent a country), corrupting it at its number as Surge {n} did, on "
                              "the lowest-numbered site not yet the adversary's in place of the one holding the most agents; "
                              "plus its other lines.")
                p["check"] = p["check"].replace("275: ", "293: ", 1) if p["check"].startswith("275: ") else p["check"]
            elif e["op"] == "adversary_cubes":
                continue
            elif e["op"] == "adversary_research" and e.get("each_country") and e["amount"] != 1:
                old_n = e["amount"]
                ops.append({**e, "amount": 1})
                if p["id"] != "backdoor_research":
                    p["text"] = re.sub(rf"{old_n} cubes (go )?onto the adversary's research track", lambda m: f"1 cube {'goes ' if m.group(1) else ''}onto the adversary's research track", p["text"])
                    flag(p, f"its research line is 1 cube a country (was {old_n}): the track's K went from 5 to 1 a country, so a cube is worth about what {old_n} were")
            else:
                ops.append(e)
        p["effects"] = ops
        if p["id"] == "backdoor_research":
            p["text"] = "Each country: 1 cube onto the adversary's research track."
            p["harms"] = [h for h in p["harms"] if h not in ("adversary cubes on megaprojects", "Discord sway", "projects advanced (compute)")]
            p["check"] = p["check"].replace("When it matters: while megaprojects are open (early to mid).", "When it matters: any round (a slot of the research row).").replace("players answer: finish megaprojects first; drain.", "players answer: drain it.")
            flag(p, "\"4 adversary cubes onto each open megaproject. Each country: 2 cubes onto the adversary's research "
                    "track.\" became \"Each country: 1 cube onto the adversary's research track.\" (adversary cubes go only "
                    "onto its track; at K 1 a cube a country is one slot)")


def threats():
    """A threat's research line at the row's K: 1 cube (Zero-Day 2)."""
    new = {"wiper_malware": 1, "data_breach": 1, "zero_day": 2}
    for path in ("drafts/adversary/threats_t1.json", "drafts/adversary/threats_t2.json"):
        deck = load(path)
        for t in deck["cards"]:
            if t["id"] not in new:
                continue
            n = new[t["id"]]
            for e in t["effects"]:
                if e["op"] == "adversary_research" and e["amount"] != n:
                    old_n = e["amount"]
                    e["amount"] = n
                    words = "1 cube goes" if n == 1 else f"{n} cubes go"
                    t["text"] = t["text"].replace(f"{old_n} cubes go onto the adversary's research track", f"{words} onto the adversary's research track")
                    flag(t, f"its research line is {n} cube{'s' if n > 1 else ''} (was {old_n}): the track's K went from 5 to 1 a country")
        dump(path, deck)


def alignment():
    path = ROOT / "content/alignment.json"
    s = path.read_text(encoding="utf-8")
    s = s.replace("Surge 2: each country, 2 Agents onto its city not yet the adversary's holding the most Agents",
                  "Each country: create 2 Agents on its lowest-numbered city not yet the adversary's")
    s = s.replace("then Surge 2: each country", "then Each country")
    s = s.replace("again; then Each country: create", "again; then, each country: create")
    s = s.replace('{"op": "surge", "site": "city", "count": 2}', '{"op": "create", "count": 2, "at": "city", "if": "clean"}')
    path.write_text(s, encoding="utf-8", newline="\n")


def megaprojects():
    path = ROOT / "content/map.json"
    s = path.read_text(encoding="utf-8")
    s = re.sub(r", each adversary cube 1 (Sway to Discord|adversary compute)\.", ".", s)
    s = s.replace(" Afterward an adversary station here puts its cubes onto the adversary's research track.", "")
    s = re.sub(r'\n    "adversary": \[.*?\n    \],', "", s, flags=re.S)
    path.write_text(s, encoding="utf-8", newline="\n")


def main():
    mil = load("drafts/adversary/militarist.json")
    events(mil)
    board(mil)
    dump("drafts/adversary/militarist.json", mil)
    deck = load("drafts/adversary/projects.json")
    projects(deck)
    dump("drafts/adversary/projects.json", deck)
    threats()
    alignment()
    megaprojects()


if __name__ == "__main__":
    main()
