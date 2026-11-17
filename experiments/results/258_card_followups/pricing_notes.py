"""258: the pricing note of each card this pass changed or added (docs/card-pricing-notes.json), with the effect
ledger's values.json reading (experiments/results/effect-ledger/cards.csv: printed and reasoned, keep excluded).

    python experiments/results/258_card_followups/pricing_notes.py

Anchors: pay = credits + 4 x (keep + play insight) + other costs (energy 4.5, cube 6, chip 6, antimatter 18); keep 4
at tier 1, 12 at tier 2; an Action at 4 uses; a node 8 (its track step paid on top, as by every route).
"""
import csv
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
NOTES = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')
LEDGER = os.path.join(ROOT, 'experiments', 'results', 'effect-ledger', 'cards.csv')

ANCHORS = {
    'mesh_repeater_network': ('stronger and dearer: Action (free): 1 node (was 1 energy); 17 cr (was 6)', 21, 32),
    'research_reactor': ('stronger: Action 1 cube: 3 energy (was 2); 8 cr', 36, 54),
    'survey_drone_fleet': ('stronger: needs 1 Science (was 2), 2 cubes when built; Action 1 cube: 2 nodes and 2 energy '
                           '(r119: their track steps); 12 cr', 48, 112),
    'orbital_relay_launcher': ('stronger: Action 1 antimatter: 3 nodes (was 2) and 3 energy; 12 cr', 96, 150),
    'antiproton_foundry': ('stronger and dearer (r119, amendment 11877): Action 1 chip: 1 antimatter and 2 energy (was the '
                           'antimatter alone); 1 antimatter when built; 12 cr (was 10), 1 ins', 52, 126),
    'positron_storage_ring': ('stronger (r119): Action 2 energy: 1 antimatter and draw 1 (was the antimatter alone); '
                              '2 energy a Power play; draw 2 when built; 10 cr, 2 ins, 2 energy', 75, 118),
    'parts_exchange': ('stronger (r119): Action 1 chip: draw 3 (was 2); 9 cr, 1 ins', 41, 60),
    'bug_bounty_board': ('stronger (r119): Action 4 cr: drain 4 compute (was 3) and draw 1; 10 cr, 1 ins', 34, 52),
    'treaty_isotope_bank': ('stronger (r119): Action 8 cr: 1 antimatter and 1 ambassador (was the antimatter alone); '
                            '1 ambassador when built; 6 cr, 1 ins', 54, 112),
    'synthetic_data_forge': ('easier (r119): needs 1 Science (was 2; kept 0.03 a game); 10 cr; a dataset a round, cashed '
                             'for insight, about 4 by its cash-out', 14, 16),
    'microgrid_lab_compact': ('stronger: claims a station or factory when built too; Action 2 energy: claim; 18 cr', 66, 72),
    'dyson_swarm_tranche': ('stronger: needs 1 Power (was 2), +3 energy income (was 2); 12 cr, 1 ins, 1 am', 46, 87),
    'exascale_training_cluster': ('credits for energy at the anchor (26 cr and 2 energy, was 15 and 5); +4 insight income '
                                  '(was 3), 4 cards drawn (was 6 cubes)', 55, 68),
    'citywide_microgrid': ('half its energy into credits at the anchor (26 cr and 4 energy, was 8 and 8); Action 1 energy: '
                           'sway 3 (was 2); liberates and claims next to it', 60, 81),
    'field_assay_team': ('stronger: Action 1 cube: 3 damage on agents (was 2); 6 cr, 1 ins', 38, 60),
    'twin_city_compact': ('new (the host-a-second-city card): place 2 nodes; you may host 1 more city (a second slot '
                          'measured at 0.55 energy a seat-round: about 11 over 4.5 rounds); 6 cr', 18, 27),
}

# The cards last changed at r119 (amendment 11877); the rest at r118.
R119 = {'antiproton_foundry', 'positron_storage_ring', 'parts_exchange', 'bug_bounty_board', 'treaty_isotope_bank',
        'synthetic_data_forge', 'survey_drone_fleet'}


def main():
    ledger = {r['card']: r for r in csv.DictReader(open(LEDGER, encoding='utf-8'))}
    notes = json.load(open(NOTES, encoding='utf-8'))
    for card, (what, pay, get) in ANCHORS.items():
        r = ledger[card]
        rev = 'r119' if card in R119 else 'r118'
        notes[card] = (f"{rev} (258): {what}; anchors: pay {pay}; get {get} = {get / pay:.2f}; "
                       f"values.json: printed {float(r['printed']):g}; reasoned {float(r['reasoned']):g}.")
    out = dict(sorted(notes.items()))
    with open(NOTES, 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(out, indent=2, ensure_ascii=False) + '\n')
    print(f'{len(ANCHORS)} notes written')


if __name__ == '__main__':
    main()
