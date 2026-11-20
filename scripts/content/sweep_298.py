"""298 (overhaul 6, calibration): the content edits, as one pass over the sources as they stood at r143.

    git checkout eca621c2be -- drafts/adversary content/alignment.json content/ais.json   # the r143 sources
    python scripts/content/sweep_298.py     # edit drafts/adversary/*.json, content/alignment.json, content/ais.json
    python scripts/regen.py

Matt's rulings of 2026-11-20: Redeploy moves the troops toward the corrupted city nearest a player piece; Offensive
takes the lowest-numbered node hex holding an agent or a troop, as Night Raid reads; too early or too hot is priced
higher, not given a rule (Martial Law's compute; every way to gain an ambassador: Accord here, Gain a player
ambassador in configs/baseline.json); level 3, Arms Race, gets a real rule. The text beside each op changes with it
(CLAUDE.md, *Text beside ops*). On sources already passed it changes nothing.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts" / "cards"))
from apply_overlay import dump_alignment  # noqa: E402  (alignment.json's hand layout)

FLAG = "298 (Matt, 2026-11-20): "
MARTIAL_LAW = 36  # compute a board (r143: 24)
ACCORD_ENERGY = 3  # (r143: 2)

OLD_REDEPLOY = "Redeploy: each Troop not on or next to a corrupted city moves onto the nearest corrupted city."
REDEPLOY = "Redeploy: each Troop moves onto its country's corrupted city nearest a player Node or Ambassador."
ARMS_RACE = "When the research track reaches a Garrison icon, each country takes 2 Troops, not 1."
OLD_OFFENSIVE = "on the Node hex with the most Agents and Troops."
OFFENSIVE = "on the lowest-numbered Node hex holding an Agent or a Troop (country by country)."
FIRST_OFFENSIVE = "on the lowest-numbered Node hex holding an Agent or a Troop."  # this item's first cut


def load(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def dump(path, data):
    (ROOT / path).write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def flag(card, text):
    if FLAG + text not in card.setdefault("flags", []):
        card["flags"].append(FLAG + text)


def militarist(mil):
    for icon in mil["board"]["icons"]:
        if icon["id"] == "redeploy":
            icon["text"] = icon["text"].replace(OLD_REDEPLOY, REDEPLOY)
    for e in mil["events"]:
        if OLD_REDEPLOY in e["text"]:
            e["text"] = e["text"].replace(OLD_REDEPLOY, REDEPLOY)
            flag(e, "Redeploy's line: the troops go to the corrupted city nearest a player piece (it moved ~0 troops a game)")
    for lv in mil["board"]["levels"]:
        if lv["name"] == "Arms Race" and lv["rule"] != ARMS_RACE:
            assert lv["rule"] == "", lv
            lv["rule"] = ARMS_RACE


def projects(deck):
    for p in deck["cards"]:
        if p["id"] == "martial_law":
            p["check"] = p["check"].replace("late (24 a board)", f"late ({MARTIAL_LAW} a board)")
        if p["id"] == "martial_law" and p["cost"] != MARTIAL_LAW:
            old = p["cost"]
            p["cost"] = MARTIAL_LAW
            p["value"] = (f"give {MARTIAL_LAW} compute a board ({2 * MARTIAL_LAW} at 2 countries); get 0.26 a compute at "
                          "values.json (2 troops 9.33): the late, significant one.")
            flag(p, f"cost {old} -> {MARTIAL_LAW}: too early, priced higher (its troops arrived by round 3 at d2-d3 at r143)")


def alignment(cards):
    for c in cards:
        if c["id"] == "offensive":
            for t in c["tiers"]:
                t["text"] = t["text"].replace(OLD_OFFENSIVE, OFFENSIVE).replace(FIRST_OFFENSIVE, OFFENSIVE)


def ais(boards):
    for b in boards:
        for p in b["powers"]:
            if b["id"] == "protector" and p["name"] == "Accord":
                p["cost"] = [["energy", ACCORD_ENERGY]]


def main():
    mil = load("drafts/adversary/militarist.json")
    militarist(mil)
    dump("drafts/adversary/militarist.json", mil)
    deck = load("drafts/adversary/projects.json")
    projects(deck)
    dump("drafts/adversary/projects.json", deck)
    cards = load("content/alignment.json")
    alignment(cards)
    (ROOT / "content/alignment.json").write_text(dump_alignment(cards), encoding="utf-8", newline="\n")
    boards = load("content/ais.json")
    ais(boards)
    dump("content/ais.json", boards)


if __name__ == "__main__":
    main()
