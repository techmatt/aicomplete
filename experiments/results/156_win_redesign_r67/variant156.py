"""156: writes a card-strength variant of content/alignment.json (screens only; the chosen one is edited in by hand).

python variant156.py NAME OUT_PATH

dminus: Discord tier II a step softer (no Corrupt line or lost ambassador at tier II; Incursion and Two Fronts
deal tier-1 threats at tier II). uplus: Unity tier II and III a step stronger (tier II: every player also gains 1
ambassador; tier III: also every player gains 2 ambassadors).
"""
import copy
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))


def dminus(cards):
    for c in cards:
        if c["side"] != "discord":
            continue
        t2 = c["tiers"][1]
        ops = [o for o in t2["effects"] if o["op"] not in ("corrupt", "lose_ambassador")]
        for o in ops:
            if o["op"] == "deal_threats" and o.get("tier") == 2:
                o.pop("tier")
                o["count"] = o["count"] + 1
        if not ops:
            ops = copy.deepcopy(c["tiers"][0]["effects"])
        t2["effects"] = ops
        t2["text"] = t2["text"] + " (dminus)"


def uplus(cards):
    for c in cards:
        if c["side"] != "unity":
            continue
        c["tiers"][1]["effects"].append({"op": "gain", "resource": "ambassadors", "amount": 1})
        c["tiers"][2]["effects"].append({"op": "gain", "resource": "ambassadors", "amount": 2})


if __name__ == "__main__":
    name, out = sys.argv[1], sys.argv[2]
    cards = json.load(open(os.path.join(TOP, "content", "alignment.json"), encoding="utf-8"))
    for part in name.split("+"):
        {"dminus": dminus, "uplus": uplus}[part](cards)
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        json.dump(cards, f, indent=1, ensure_ascii=False)
        f.write("\n")
