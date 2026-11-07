"""138: bounded arrivals and Corrupt X, written into the adversary drafts and the Discord / Unity cards.

    python experiments/results/138_corruption_and_arrivals/arrivals_138.py      # rewrite drafts/adversary/*.json and content/alignment.json

A record of 138's rewrite (moved from scripts/content/ at 144): rerunning it now reverts later items' edits.
    python scripts/content/load_adversary.py    # then write content/ and docs/adversary-cards.md

Rerunnable from any revision: every card it touches is set from the tables below (its creation lines, Corrupt lines and
site creation), the retired ops (arrivals, escalation, infiltrate, city / station / clean-factory builds, spread reach,
corrupt sooner, botnet and clean-city ambassadors, the spread, the threat placements) are dropped, and the card's text
loses their sentences and gains the new ones, worded by `words()`. Each line takes the one creation shape: "create N per
board on the lowest-numbered X [condition]".
"""

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
DRAFTS = ROOT / "drafts" / "adversary"


def create(count, at, cond=None, sites=1):
    op = {"op": "create", "count": count, "at": at}
    if cond:
        op["if"] = cond
    if sites != 1:
        op["sites"] = sites
    return op


def corrupt(site, cut=0):
    op = {"op": "corrupt", "site": site}
    if cut:
        op["cut"] = cut
    return op


def create_site(kind):
    return {"op": "create_site", "kind": kind}


NOUN = {"city": ("city", "cities"), "coastal_city": ("coastal city", "coastal cities"),
        "station": ("research station", "research stations"), "factory": ("factory", "factories"),
        "botnet": ("botnet hex", "botnet hexes")}
PRE = {"adversary": "corrupted ", "clean": "clean ", "player": "player "}
POST = {"no_ambassadors": " with no adversary ambassador", "ambassadors": " holding adversary ambassadors",
        "near": " on or next to it"}
CORRUPT = {"city": "cities", "station": "research stations", "factory": "factories", "land": "land"}


def words(op, board="Each board"):
    """The printed sentence of a 138 line (`board`: "In its country" on a threat)."""
    if op["op"] == "create":
        n, at, cond, sites = op["count"], op["at"], op.get("if"), op.get("sites", 1)
        if at == "location":
            return f"Create {n} adversary ambassador{'s' * (n != 1)} here."
        one, many = NOUN[at]
        pre, post = PRE.get(cond, ""), POST.get(cond, "")
        amb = f"{n} adversary ambassador{'s' * (n != 1)}"
        if sites == 1:
            return f"{board}: create {amb} on the lowest-numbered {pre}{one}{post}."
        return f"{board}: create {amb} on each of the {sites} lowest-numbered {pre}{many}{post}."
    if op["op"] == "corrupt":
        cut = f" ({op['cut']} fewer)" if op.get("cut") else ""
        return f"Corrupt {CORRUPT[op['site']]}{cut}."
    if op["op"] == "create_site":
        if op["kind"] == "station":
            return "Each board: a neutral research station opens on its lowest-numbered empty station site."
        return "Each board: a neutral factory opens on its lowest-numbered free industrial zone."
    raise ValueError(op)


RETIRED = {"arrivals", "infiltrate", "city_build", "station_build", "clean_factories", "spread_reach", "corrupt_sooner",
           "botnet_ambassadors", "clean_city_ambassadors", "adversary_spread", "ambassadors"}
OURS = {"create", "corrupt", "create_site"}
OLD_SENTENCES = [
    r"Each country's clean city with the most adversary ambassadors gains \d+ more\.",
    r"Spread reaches \d+ hex(es)? further\.",
    r"Arrivals: [^.]*\.",
    r"Factories next to clean cities gain \d+ adversary ambassadors?\.",
    r"Every botnet makes \d+ adversary ambassadors? on its hex\.",
    r"Each corrupted city gains \d+ more adversary ambassadors?\.",
    r"Each corrupted research station gains \d+ adversary ambassadors?\.",
    r"Cities corrupt at \d+ fewer adversary ambassadors?\.",
    r"Escalation\.",
    r"([Ee]ach board|In its country): [^.]*\.",
    r"Corrupt [a-z ]+( \(\d+ fewer\))?\.",
    r"Create \d+ adversary ambassadors? here\.",
]


def retext(text, new_ops, drop=()):
    for pat in OLD_SENTENCES + list(drop):
        text = re.sub(pat, "", text)
    text = re.sub(r"\s+", " ", text).strip()
    return " ".join([text] + [words(o) for o in new_ops]).strip()


def keep(effects):
    return [e for e in effects if not (isinstance(e, dict) and e.get("op") in RETIRED | OURS)]


# Events: (creation lines, Corrupt lines, site creation). Stage I creates 1 a site, stage II more sites or 2,
# stage III 2 (tiers raise N). Old sources, kept in intent: arrivals by site type, Escalation (each country's
# lowest-numbered factory), infiltrate (the city already filling), city / station builds (the adversary's own sites),
# clean-factory builds (factories), botnet ambassadors (botnet hexes).
EVENTS = {
    "mil_s1_probing_strikes": ([create(1, "coastal_city", sites=2)], [corrupt("city")], []),
    "mil_s1_factory_drive": ([create(1, "factory", sites=2)], [corrupt("factory")], [create_site("factory")]),
    "mil_s1_long_patrols": ([create(1, "station", sites=2), create(1, "city", "ambassadors")], [corrupt("station")],
                            [create_site("station")]),
    "mil_s1_war_drums": ([create(1, "factory"), create(1, "city", "ambassadors")], [corrupt("city")], []),
    "mil_s1_signal_jamming": ([create(1, "coastal_city", sites=2), create(1, "botnet", sites=2)], [corrupt("city")], []),
    "mil_s2_mobilization": ([create(1, "coastal_city", sites=2), create(1, "factory")], [corrupt("city")], []),
    "mil_s2_station_raids": ([create(1, "station", sites=2), create(1, "factory")], [corrupt("station")], []),
    "mil_s2_fifth_column": ([create(2, "city", "ambassadors"), create(1, "factory")], [corrupt("city", 1)], []),
    "mil_s2_border_war": ([create(2, "factory")], [corrupt("factory")], []),
    "mil_s2_supply_lines": ([create(1, "factory", sites=3)], [corrupt("factory")], []),
    "mil_s2_forced_march": ([create(1, "factory", sites=2), create(1, "city", "ambassadors")], [corrupt("city")], []),
    "mil_s3_total_mobilization": ([create(1, "factory", sites=3), create(2, "city", "ambassadors")],
                                  [corrupt("factory"), corrupt("city")], []),
    "mil_s3_blitzkrieg": ([create(2, "factory"), create(2, "city", "ambassadors")], [corrupt("city"), corrupt("land")],
                          []),
    "mil_s3_occupation": ([create(2, "city", "ambassadors"), create(1, "station", sites=2)],
                          [corrupt("station"), corrupt("city", 1)], []),
    "mil_s3_naval_blockade": ([create(2, "coastal_city", sites=2)], [corrupt("city"), corrupt("land")], []),
}

# The board's own line, every build (it replaced city_build 2 on every corrupted city and factory_build 0).
BOARD_BUILD = [create(2, "city", "adversary")]

THREATS = {
    "call_up": ([create(2, "factory")], []),
    "agitators": ([create(1, "city", "clean", sites=2)], []),
    "insider_access": ([create(1, "station", "player", sites=2)], []),
    "overrun": ([create(2, "city", "ambassadors"), corrupt("city", 1)], [r"If the city here holds [^.]*\. If not, [^.]*\."]),
    "insurrection": ([create(3, "location")], []),
    "turncoats": ([create(1, "factory", "player", sites=2)], []),
    "propaganda_ministry": ([create(1, "city", "clean", sites=2), corrupt("city")],
                            [r"The adversary's next spread in this country happens twice\."]),
    "dark_cities": ([create(1, "city", "near", sites=3)], []),
    "garrison_state": ([create(2, "city", "near")], []),
}
THREAT_SENTENCES = [r"\d+ adversary ambassadors land on [^.]*\.", r"Each city on or next to it gains [^.]*\.",
                    r" and 2 adversary ambassadors on the city here"]

PROJECTS = {
    "recruitment_drive": ([create(1, "city", "clean", sites=2)], [r"every clean city gains 1 adversary ambassador\."]),
    "infiltration_drive": ([corrupt("city", 1)], [r"In each country, the uncorrupted city [^.]*\."]),
    "botnet_swarm": ([create(2, "botnet", sites=2)], [r"Every botnet makes [^.]*\."]),
    "grand_offensive": ([corrupt("city")], [r"The adversary builds and spreads now; "]),
    "propaganda_network": ([create(1, "city", "adversary")], [r",? and corrupted cities build 1 more"]),
}

ALIGNMENT = {
    "blight": {"effects": [create(1, "city", "ambassadors"), create_site("factory")],
               "steps": {5: [create(3, "city", "ambassadors"), {"op": "each_destroy_node", "count": 2}]}},
    "firestorm": {"effects": [create(2, "city", "ambassadors")],
                  "steps": {3: [create(2, "city", "ambassadors"), {"op": "each_destroy_node", "count": 1}],
                            4: [create(3, "city", "ambassadors"), {"op": "each_destroy_node", "count": 2}]}},
    "offensive": {"effects": [corrupt("city"), {"op": "destroy_node"}],
                  "steps": {3: [corrupt("city"), {"op": "destroy_node"}, {"op": "each_destroy_node", "count": 1}],
                            4: [{"op": "adversary_build"}, corrupt("city"), {"op": "each_destroy_node", "count": 2}]}},
    "civil_defense": {"effects": [{"op": "pushback", "ambassadors": 1}, create_site("station")]},
}
ALIGNMENT_TEXT = {
    "blight": "Each board: create 1 adversary ambassador on the lowest-numbered city holding adversary ambassadors, and "
              "a neutral factory opens on its lowest-numbered free industrial zone. 5+ earlier Discord tips: create 3 "
              "instead, and each player destroys 2 of their nodes.",
    "firestorm": "Each board: create 2 adversary ambassadors on the lowest-numbered city holding adversary ambassadors. "
                 "3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: create 3, and each "
                 "player destroys 2.",
    "offensive": "Corrupt cities, then destroy all nodes on the node hex with the most adversary ambassadors and "
                 "troops. 3+ earlier Discord tips: each player also destroys 1 of their nodes. 4+ instead: the "
                 "adversary builds, then Corrupt cities, and each player destroys 2.",
    "civil_defense": "In each country, remove 1 adversary ambassador, and a neutral research station opens on its "
                     "lowest-numbered empty station site.",
}


def rw(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path, data, indent):
    path.write_text(json.dumps(data, indent=indent, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def indent_of(path):
    m = re.search(r"\n( +)\S", path.read_text(encoding="utf-8"))
    return len(m.group(1)) if m else 1


def main():
    p = DRAFTS / "militarist.json"
    mil, ind = rw(p), indent_of(p)
    b = mil["board"]
    for k in ("city_build", "factory_build", "arrivals", "spread_range", "escalation_ambassadors"):
        b.get("build", {}).pop(k, None)
    b["creates"] = BOARD_BUILD
    b["creates_text"] = " ".join(words(o) for o in BOARD_BUILD)
    for e in mil["events"]:
        creates, corrupts, sites = EVENTS[e["id"]]
        e.pop("arrivals", None)
        e.pop("escalation", None)
        e["standing"] = keep(e["standing"]) + creates + sites
        e["twists"] = keep(e["twists"]) + corrupts
        e["text"] = retext(e["text"], creates + sites + corrupts)
    save(p, mil, ind)

    for tier in (1, 2, 3):
        p = DRAFTS / f"threats_t{tier}.json"
        d, ind = rw(p), indent_of(p)
        for c in d["cards"]:
            if c["id"] in THREATS:
                new, drop = THREATS[c["id"]]
                c["effects"] = [e for e in c["effects"] if not (e.get("op") in RETIRED | OURS or
                                                               (e.get("op") == "flag" and e.get("name") == "double_spread")
                                                               or (e.get("op") == "corrupt_city" and c["id"] == "overrun"))]
                c["effects"] = new + c["effects"]
                text = c["text"]
                for pat in drop + THREAT_SENTENCES + OLD_SENTENCES:
                    text = re.sub(pat, "", text)
                text = re.sub(r"\s+", " ", text).strip()
                c["text"] = " ".join([words(o, "In its country") for o in new] + [text]).strip()
        save(p, d, ind)

    p = DRAFTS / "projects.json"
    d, ind = rw(p), indent_of(p)
    for c in d["cards"]:
        if c["id"] in PROJECTS:
            new, drop = PROJECTS[c["id"]]
            rest = [e for e in c["effects"] if e.get("op") not in RETIRED | OURS]
            if c["id"] == "grand_offensive":
                # The adversary builds now (its creation lines), then Corrupt cities.
                i = next(i for i, e in enumerate(rest) if e.get("op") == "adversary_build") + 1
                c["effects"] = rest[:i] + new + rest[i:]
            else:
                c["effects"] = rest + new
            text = c["text"]
            for pat in drop + OLD_SENTENCES + [r"The adversary builds now, then Corrupt cities; "]:
                text = re.sub(pat, "", text)
            text = re.sub(r"\s+", " ", text).strip()
            if c["id"] == "grand_offensive":
                text = "The adversary builds now, then Corrupt cities; " + text
            else:
                text = " ".join([text] + [words(o) for o in new]).strip()
            c["text"] = text.replace("Ongoing: every round, .", "Ongoing: every round:").replace("every round, Each board", "every round, each board")
        save(p, d, ind)

    p = ROOT / "content" / "alignment.json"
    d, ind = rw(p), indent_of(p)
    for c in d:
        spec = ALIGNMENT.get(c["id"])
        if not spec:
            continue
        c["effects"] = spec["effects"]
        for st in c.get("steps", []):
            if st["at"] in spec.get("steps", {}):
                st["effects"] = spec["steps"][st["at"]]
        c["text"] = ALIGNMENT_TEXT[c["id"]]
    save(p, d, ind)
    print("rewrote the drafts and content/alignment.json")


if __name__ == "__main__":
    main()
