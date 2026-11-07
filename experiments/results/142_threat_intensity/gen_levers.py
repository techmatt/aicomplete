"""142: write levers.json (the --set lists specs.py's cells name) from content.

    python experiments/results/142_threat_intensity/gen_levers.py

I3 / I2: each board's printed credits 3 / 2 lower. T1: each track space paying credits income pays 1 less.
P1: each credits program (`production`, flat) 1 less, 20% less from 6 up; each flat credits `income` building 20%
less. C25 / C50: credits counter items cost 25 / 50% more. K25 / K50 / K75: every resource counter item 25 / 50 / 75% more. A1: a tier-1 threat creates 1 adversary ambassador at its location, not 2. L10: start credits 10 lower (30 / 36 / 30). D2: d2's counter step +2 an item. H2 / H3: d3's harm step +2 / +3 nodes. D3: d2's counter step +3 an item. B2: d3's build bonus 2. F1 / F2: d2's first counter option 1 / 2 more an item still. E2: every board starts with 3 energy (from 1). S1: every board prints 1 more insight a round. N: tier-1 threats destroy no nodes (threat_nodes 0, 2, 3). M25: every resource counter item 25% less. O1: owned stations and factories back to r58's 1.
"""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def main():
    ais = json.loads((ROOT / "content" / "ais.json").read_text(encoding="utf-8"))
    cards = json.loads((ROOT / "content" / "cards.json").read_text(encoding="utf-8"))
    lev = {}
    for k in (2, 3):
        lev[f"I{k}"] = [f"ais.{b['id']}.credits={b['credits'] - k}" for b in ais]
    t1 = []
    for b in ais:
        for track in ("economy_track", "play_track"):
            for i, sp in enumerate(b.get(track) or []):
                inc = sp.get("income") or []
                if any(r == "credits" for r, _ in inc):
                    new = [[r, n - 1 if r == "credits" else n] for r, n in inc]
                    new = [x for x in new if x[1] > 0]
                    t1.append(f"ais.{b['id']}.{track}.{i}.income={json.dumps(new, separators=(',', ':'))}")
    lev["T1"] = t1
    p1 = []
    for c in cards:
        for field in ("effects", "passives"):
            for i, op in enumerate(c.get(field) or []):
                if op.get("resource") != "credits":
                    continue
                if op.get("op") == "production" and field == "effects":
                    n = op["amount"]
                    new = n - 1 if n < 6 else round(n * 0.8)
                    if new >= 1 and new != n:
                        p1.append(f"cards.{c['id']}.{field}.{i}.amount={new}")
                if op.get("op") == "income" and field == "passives":
                    n = op["amount"]
                    new = round(n * 0.8)
                    if new >= 1 and new != n:
                        p1.append(f"cards.{c['id']}.{field}.{i}.amount={new}")
    lev["P1"] = p1
    lev["C25"] = ["adversary.counter_credit_pct=125"]
    lev["C50"] = ["adversary.counter_credit_pct=150"]
    lev["O1"] = ["board.owned_station_cubes=1", "board.factory_chips=1"]
    lev["N"] = ["adversary.threat_nodes=0,2,3"]
    lev["E2"] = ["ai.start_energy=3"]
    lev["L10"] = ["ai.start_credits=30,36,30"]
    lev["D2"] = ["adversary.counter_bonus_all=2"]
    lev["H2"] = ["adversary.harm_bonus=2"]
    lev["H3"] = ["adversary.harm_bonus=3"]
    lev["D3"] = ["adversary.counter_bonus_all=3"]
    lev["B2"] = ["adversary.build_bonus=2"]
    lev["F1"] = ["adversary.counter_bonus=1"]
    lev["F2"] = ["adversary.counter_bonus=2"]
    lev["S1"] = [f"ais.{b['id']}.income={json.dumps([[r, n + 1 if r == 'insight' else n] for r, n in b['income']], separators=(',', ':'))}"
                 for b in ais]
    threats = json.loads((ROOT / "content" / "threats.json").read_text(encoding="utf-8"))
    lev["A1"] = [f"threats.{t['id']}.effects.{i}.count=1" for t in threats if t["tier"] == 1
                 for i, e in enumerate(t["effects"]) if e["op"] == "create" and e.get("at") == "location" and e["count"] == 2]
    lev["M25"] = ["adversary.counter_pct=75,75,75"]
    for k in (25, 50, 75):
        lev[f"K{k}"] = [f"adversary.counter_pct={100 + k},{100 + k},{100 + k}"]
    (HERE / "levers.json").write_text(json.dumps(lev, indent=1) + "\n", encoding="utf-8", newline="\n")
    for k, v in lev.items():
        print(k, len(v), v[:3])


if __name__ == "__main__":
    main()
