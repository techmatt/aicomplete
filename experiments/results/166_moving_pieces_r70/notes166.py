"""166: the viewer's pricing note for each card 166 changed (docs/card-pricing-notes.json, one current line a card).

    python experiments/results/166_moving_pieces_r70/notes166.py

Table numbers are the effect ledger's at r70 (experiments/results/effect-ledger/cards.csv: printed, reasoned);
anchor numbers follow card-guidelines.md (pay = credits + 4 x (keep + play insight); a move 2 a piece, a removal 5).
"""
import csv
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
NOTES = ROOT / 'docs' / 'card-pricing-notes.json'
LEDGER = ROOT / 'experiments' / 'results' / 'effect-ledger' / 'cards.csv'

WHY = {
    'kettle_operation': ('r70 (166): gather up to 6 agents or troops into a hex in reach and remove them; '
                         'anchors: pay 34; get 38 (6 removals, 2 Sway) = 1.12'),
    'lure_network': ('r70 (166): Action: gather up to 3 agents into a hex in reach and remove them; '
                     'anchors: pay 24; get 10 draw + 15 a use (3 rounds: 2.29)'),
    'interceptor_battery': ('r70 (166): push up to 3 agents or troops; Action: push up to 2 (2 a piece); '
                            'anchors: pay 15; get 20 = 1.33'),
    'exodus_corridors': ('r70 (166): gather up to 6 ambassadors into a hex in reach (2 a piece) and gain 4; '
                         'anchors: pay 30; get 29.5 = 0.98'),
    'honeypot_relay': ('r70 (166): draw 1, then gather up to 2 agents and remove them (its draw 2 was the weakest '
                       'line); anchors: pay 20; get 30.7 = 1.54'),
    'rally_the_allies': ('r70 (166): gather up to 3 ambassadors (2 a piece) replaces consolidate 3; same price, '
                         'under the tier-2 ceiling'),
    'mass_defection': 'r70 (166): raised 8 to 11 credits to the tier-2 ceiling (2.5 at values.json)',
    'open_source_uprising': 'r70 (166): raised 8 to 9 credits to the tier-2 ceiling (2.5 at values.json)',
    'surgical_strike': 'r70 (166): raised 14 to 15 credits to the tier-2 ceiling (2.5 at values.json)',
    'open_weights_release': 'r70 (166): raised 10 to 12 credits to the tier-2 ceiling (2.5 at values.json)',
    'collateralized_grid': 'r70 (166): raised 10 to 11 credits to the tier-2 ceiling (2.5 at values.json)',
}


def main():
    ledger = {r['card']: r for r in csv.DictReader(open(LEDGER, encoding='utf-8'))}
    notes = json.loads(NOTES.read_text(encoding='utf-8'))
    for card, why in WHY.items():
        r = ledger[card]
        pay, get = float(r['printed']), float(r['reasoned'])
        notes[card] = f"{why}; values.json: pay {pay:g}; get {get:g} = {get / pay:.2f}."
    NOTES.write_text(json.dumps(dict(sorted(notes.items())), indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
    print(f'{len(WHY)} notes written')


if __name__ == '__main__':
    main()
