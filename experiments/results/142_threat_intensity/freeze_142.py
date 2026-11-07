"""142: fold the frozen cell's levers into the sources (rerunnable: each step is idempotent).

    python experiments/results/142_threat_intensity/freeze_142.py
    python scripts/cards/apply_overlay.py experiments/results/142_threat_intensity/overlay_142.txt
    python scripts/content/load_adversary.py && python scripts/regen.py

The frozen cell is I3+P1+C25+A1+E2+D2+H2 (gen_levers.py names them):
- C25: every credits counter item prints 25% more, rounded up (12 -> 15, 18 -> 23, 26 -> 33): the engine's
  `counter_credit_pct` 125 on the printed amounts, so `counter_credit_pct` stays 100 and games replay the same.
- A1: a tier-1 threat's "create 2 adversary ambassadors here" creates 1.
- E2, D2, H2 are config: `ai.start_energy` 3, `adversary.counter_bonus_all` 2, `adversary.harm_bonus` 2.
- I3 (boards 18 / 16 / 14) and P1 (credit programs and income buildings about 20% less) are content overlays in
  overlay_142.txt, written here from levers.json and applied by apply_overlay.py.
"""

import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
DONE = "142 freeze"


def credits(n):
    return (n * 125 + 99) // 100


def drafts():
    for tier in (1, 2, 3):
        p = ROOT / "drafts" / "adversary" / f"threats_t{tier}.json"
        d = json.loads(p.read_text(encoding="utf-8"))
        for c in d["cards"]:
            if any(DONE in f for f in c.get("flags", [])):
                continue
            moved = {}
            for opt in c["counters"]:
                for item in opt:
                    if item.get("resource") == "credits":
                        new = credits(item["amount"])
                        moved[item["amount"]] = new
                        item["amount"] = new
            for old, new in moved.items():
                c["value"] = re.sub(rf"\b{old} credits\b", f"{new} credits", c["value"])
                c["value"] = re.sub(rf"\bcredits {old}\b", f"credits {new}", c["value"])
            note = f"{DONE}: credits counter {', '.join(f'{a} -> {b}' for a, b in moved.items())}"
            if tier == 1:
                for e in c["effects"]:
                    if e["op"] == "create" and e.get("at") == "location" and e["count"] == 2:
                        e["count"] = 1
                        c["text"] = c["text"].replace("Create 2 adversary ambassadors here.", "Create 1 adversary ambassador here.")
                        note += "; creates 1 adversary ambassador here, not 2"
            c["flags"].append(note)
        p.write_text(json.dumps(d, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        print("wrote", p.name)


def overlay():
    lev = json.loads((HERE / "levers.json").read_text(encoding="utf-8"))
    lines = ["# 142's frozen income levers: I3 (boards' printed credits 3 lower) and P1 (credit programs 1 less, 20% from 6",
             "# up; flat credit income buildings 20% less). Written by freeze_142.py from levers.json."]
    lines += lev["I3"] + lev["P1"]
    (HERE / "overlay_142.txt").write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    print("wrote overlay_142.txt")


def config():
    p = ROOT / "configs" / "baseline.json"
    s = p.read_text(encoding="utf-8")
    s = s.replace('"start_energy": 1,', '"start_energy": 3,')
    s = s.replace('"counter_bonus_all": 1,', '"counter_bonus_all": 2,')
    s = s.replace('"harm_bonus": 1,', '"harm_bonus": 2,')
    p.write_text(s, encoding="utf-8", newline="\n")
    print("wrote baseline.json")


def main():
    if not (HERE / "levers.json").exists():
        raise SystemExit("run gen_levers.py first (before the freeze: it reads the unfrozen content)")
    overlay()
    drafts()
    config()


if __name__ == "__main__":
    main()
