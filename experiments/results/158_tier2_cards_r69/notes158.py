"""158: writes the ten new cards' pricing notes into docs/card-pricing-notes.json (sorted keys, indent 2), from the
effect ledger's cards.csv (printed = credits + insight x 2.5; get = the reasoned value at values.json). Rerunnable."""
import csv
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
WHEN = {
    "mass_defection": "Diplomacy seat, rounds 7-10, a city in reach with 3 adversary ambassadors around it",
    "embassy_of_exiles": "Diplomacy seat, from round 5: 2 ambassadors now, then 1 convert a round",
    "double_agents": "Security seat, rounds 6-10, a city in reach guarded by 2 and a troop in reach",
    "kettle_operation": "Security seat with 2 Security tags, rounds 7-10, troops and ambassadors around one hex",
    "lure_network": "Cyber seat, from round 5: a gather and 2 struck a round",
    "firewall_array": "Cyber seat with 2 Cyber tags, from round 6: a city kept out of the Rally and the Corrupt pull each round",
    "air_gap": "Cyber seat, the round before a Corrupt cities card: one city kept clean",
    "rally_the_allies": "Diplomacy seat, rounds 7-10: 4-6 ambassadors toward a Liberate (5 a city)",
    "surgical_strike": "Security seat, rounds 6-10: 2 troops or 3 ambassadors off one hex",
    "open_source_uprising": "Science or Diplomacy seat, rounds 6-10, 3 converted around a city in reach",
}


def main():
    rows = {r['card']: r for r in csv.DictReader(open(os.path.join(ROOT, 'experiments', 'results', 'effect-ledger', 'cards.csv'), encoding='utf-8'))}
    path = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')
    notes = json.load(open(path, encoding='utf-8'))
    for cid, when in WHEN.items():
        r = rows[cid]
        pay, get = float(r['printed']), float(r['reasoned'])
        notes[cid] = f"r69 (158, draft): {when}; values.json: pay {pay:g} (+5 keep); get {get:g} = {get / pay:.2f}."
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(dict(sorted(notes.items())), f, indent=2, ensure_ascii=False)
        f.write('\n')


if __name__ == '__main__':
    main()
