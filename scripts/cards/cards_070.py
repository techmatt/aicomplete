"""070: the typed counters and tier-2 capstones as first added (candidate prices; the
calibration moved some since, so read content/cards.json for what stands). One-shot: kept as
the record of what was added and retired.
"""
import json

RETIRED = ["threat_neutralization_mandate", "cascade_failure", "mesh_intrusion_pact",
           "full_automation_mandate", "asteroid_salvage_rights"]


def typed(id, name, tag, source, amount, cost, insight):
    return {"id": id, "name": name, "tier": 1, "kind": "program", "cost": cost, "insight": insight,
            "effects": [{"op": "jam", "amount": amount, "source": source}], "tags": [tag]}


def desk(id, name, tag, source):
    return {"id": id, "name": name, "tier": 1, "kind": "building", "cost": 5, "insight": 1,
            "ability": {"cost": [["energy", 1]],
                        "effects": [{"op": "jam", "amount": 3, "source": source}]},
            "tags": [tag]}


def capstone(id, name, tag, passive, cost, needs=2, chips=1):
    return {"id": id, "name": name, "tier": 2, "kind": "building", "cost": cost, "insight": 2,
            "chips": chips, "needs": [[tag, needs]], "replaces": tag, "passives": [passive],
            "tags": [tag]}


NEW = [
    typed("rapid_response", "Rapid Response", "security", "military", 4, 2, 0),
    typed("force_protection_order", "Force Protection Order", "security", "military", 5, 3, 1),
    desk("garrison_command", "Garrison Command", "security", "military"),
    typed("fact_check_network", "Fact-Check Network", "diplomacy", "influence", 4, 2, 0),
    typed("truth_commission", "Truth Commission", "diplomacy", "influence", 5, 3, 1),
    desk("civic_watch", "Civic Watch", "diplomacy", "influence"),
    typed("incident_response", "Incident Response", "cyber", "cyber", 4, 2, 0),
    typed("threat_hunt", "Threat Hunt", "cyber", "cyber", 5, 3, 1),
    desk("security_ops_floor", "Security Ops Floor", "cyber", "cyber"),
    typed("antitrust_action", "Antitrust Action", "industry", "economic", 4, 2, 0),
    typed("supply_chain_audit", "Supply Chain Audit", "industry", "economic", 5, 3, 1),
    desk("market_watchdog", "Market Watchdog", "industry", "economic"),
    capstone("vertical_monopoly", "Vertical Monopoly", "industry",
             {"op": "income_per_tag", "resource": "credits", "tag": "industry", "per": 1}, 14, needs=3),
    capstone("market_stabilization_board", "Market Stabilization Board", "industry",
             {"op": "auto_counter", "source": "economic", "amount": 3}, 12),
    capstone("sentinel_command", "Sentinel Command", "security",
             {"op": "auto_counter", "source": "military", "amount": 3}, 12),
    capstone("civic_trust_bureau", "Civic Trust Bureau", "diplomacy",
             {"op": "auto_counter", "source": "influence", "amount": 3}, 12),
    capstone("autonomous_defense_grid", "Autonomous Defense Grid", "cyber",
             {"op": "auto_counter", "source": "cyber", "amount": 3}, 12),
]


def main():
    cards = json.load(open("content/cards.json", encoding="utf-8"))
    cards = [c for c in cards if c["id"] not in RETIRED]
    for c in cards:
        if c["id"] == "black_ice_citadel":  # was: Action, 1 energy, drain 2 from each project
            c.update(cost=10, needs=[["cyber", 2]], passives=[{"op": "project_drain", "amount": 1}])
            c.pop("ability")
            c.pop("keywords")
    cards += NEW
    with open("content/cards.json", "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(cards, indent=1, ensure_ascii=False) + "\n")

    with open("configs/baseline.json", encoding="utf-8") as f:
        text = f.read()
    for r in RETIRED:
        text = text.replace(f'        "{r}",\n', "")
    last = '        "turnkey_factory"'
    add = "".join(f'        "{c["id"]}",\n' for c in NEW)
    assert '"cards": [\n' in text
    text = text.replace('"cards": [\n', '"cards": [\n' + add, 1)
    with open("configs/baseline.json", "w", encoding="utf-8", newline="\n") as f:
        f.write(text)


if __name__ == "__main__":
    main()
