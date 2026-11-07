"""142: add the energy-claim cards to content/cards.json and the baseline deck (rerunnable: replaces them if present).

    python experiments/results/142_threat_intensity/add_claim_cards.py
then python scripts/configs/shared_row.py.
"""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CARDS = [
    {
        "id": "power_purchase_agreement",
        "name": "Power Purchase Agreement",
        "tier": 1,
        "kind": "event",
        "cost": 2,
        "energy": 2,
        "insight": 0,
        "tags": ["power", "science"],
        "effects": [{"op": "claim_station"}],
    },
    {
        "id": "microgrid_lab_compact",
        "name": "Microgrid Lab Compact",
        "tier": 2,
        "kind": "building",
        "cost": 8,
        "insight": 1,
        "tags": ["power", "science"],
        "ability": {"cost": [["energy", 2]], "effects": [{"op": "claim_station"}]},
    },
]


def main():
    p = ROOT / "content" / "cards.json"
    cards = json.loads(p.read_text(encoding="utf-8"))
    ids = {c["id"] for c in CARDS}
    cards = [c for c in cards if c["id"] not in ids] + CARDS
    p.write_text(json.dumps(cards, indent=1, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    b = ROOT / "configs" / "baseline.json"
    text = b.read_text(encoding="utf-8")
    cfg = json.loads(text)
    deck = next(m for m in cfg["modules"] if m["module"] == "cards")["cards"]
    for c in CARDS:
        if c["id"] not in deck:
            deck.append(c["id"])
    b.write_text(json.dumps(cfg, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    print("cards:", len(cards), "deck:", len(deck))


if __name__ == "__main__":
    main()
