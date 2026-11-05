"""117: the deck's cost curve (deck cards in configs/baseline.json): medians by tier, cards at 5 or less, 25+, 30+."""

import json
import statistics as st

with open("content/cards.json", encoding="utf-8") as f:
    cards = {c["id"]: c for c in json.load(f)}
with open("configs/baseline.json", encoding="utf-8") as f:
    base = json.load(f)
deck = next(m for m in base["modules"] if m.get("module") == "cards")["cards"]
dc = [cards[i] for i in deck]
t1 = [c["cost"] for c in dc if c["tier"] == 1]
t2 = [c["cost"] for c in dc if c["tier"] == 2]
print(f"deck {len(dc)}: tier 1 {len(t1)}, tier 2 {len(t2)}")
print(f"median: tier 1 {st.median(t1)}, tier 2 {st.median(t2)}, deck {st.median(t1 + t2)}")
print(f"5 or less: {sum(1 for c in dc if c['cost'] <= 5)}; 25+: {sum(1 for c in dc if c['cost'] >= 25)}; "
      f"30+: {sum(1 for c in dc if c['cost'] >= 30)}")
print("30+:", ", ".join(f"{c['name']} {c['cost']}" for c in sorted(dc, key=lambda c: -c["cost"]) if c["cost"] >= 30))
