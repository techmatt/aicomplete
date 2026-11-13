"""214: the repriced cards' lines in docs/card-pricing-notes.json (one current line a card).

    python experiments/results/214_tier2_pricing_insight/notes.py
"""
import json
import os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
PATH = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')
K = 'anchors at a tier-2 keep of 3'
NOTES = {
    'mass_defection': f'r95 (214): 9 to 18 cr, over the ceiling at 9; {K}: pay 30; get 61 (a convert 10.6: removal 5 + 0.7 of an ambassador) = 2.03, 1.10 with a convert at the removal alone.',
    'embassy_of_exiles': f'r95 (214): 14 to 30 cr, over the ceiling at 14 (3.83); {K}: pay 42; get 3 ambassadors 24 + the Action\'s convert 2 at 3.5 uses 74 = 99.7 (2.37), 67.9 at 2 uses (1.62).',
    'penning_trap_array': f'r95 (214): 10 to 30 cr, over the ceiling at 10 (4.18); {K}: pay 46; get 1 antimatter 18 + the Action (2 antimatter 36 for a cube and an energy, 25.5 net) at 3.5 uses = 108.75 (2.36), 70.5 at 2 uses (1.53).',
    'antiproton_foundry': f'r95 (214): 10 to 30 cr, over the ceiling at 10 (4.18); {K}: pay 46; get 1 antimatter 18 + the Action (2 antimatter 36 for a chip and an energy, 25.5 net) at 3.5 uses = 108.75 (2.36), 70.5 at 2 uses (1.53).',
    'positron_storage_ring': f'r95 (214): 9 to 18 cr, over the ceiling at 9 (2.79); {K}: pay 18 + 2 energy 9 + 5 insight 20 = 47; get 1 antimatter a round 63 + 2 energy a Power card 31.5 + draw 2 10 + tags 1.5 = 106 (2.26), 79 with 2 antimatter used (1.68).',
    'forensic_audit': f'r95 (214): 27 to 6 cr, too dear at research cubes 6 (0.82); {K}: pay 6 + 4 cubes 24 + 4 insight 16 = 46; get a city liberated 45 + draw 2 10 = 55 (1.20).',
    'kettle_operation': f'r95 (214): 18 to 2 cr, too dear (0.68); {K}: pay 2 + 5 insight 20 = 22; get 6 removals 30 - 1 Discord 4 = 26 (1.18).',
    'surgical_strike': f'r95 (214): 14 to 2 cr, too dear (0.62); {K}: pay 2 + 12 = 14; get 4 agents 20 - 1 Discord 4 = 16 (1.14).',
    'decapitation_strike': f'r95 (214): 4 to 0 cr, under 1.0 at keep 3 (0.91); {K}: pay 1 antimatter 18 + 12 = 30; get 3 troops 15 + a botnet 16 = 31 (1.03); credits can\'t take it higher without dropping the antimatter.',
    'singularity_exploit': f'r95 (214): 8 to 4 cr, under 1.0 at keep 3 (0.95); {K}: pay 4 + 1 antimatter 18 + 4 insight 16 = 38; get a botnet 16 + a project discarded 24 = 40 (1.05).',
    'kinetic_interdiction': f'r95 (214): 8 to 2 cr, under 1.0 at keep 3 (0.95); {K}: pay 2 + 1 antimatter 18 + 12 = 32; get 2 troops 10 + 6 agents 30 - 1 Discord 4 = 36 (1.13).',
    'martial_cordon': f'r95 (214): 18 to 8 cr, too dear (0.76); {K}: pay 8 + 4 insight 16 = 24; get 6 agents 30 - 1 Discord 4 = 26 (1.08).',
    'nuclear_option': f'r95 (214): 23 to 6 cr, too dear (0.70; r90 left the keep out of its pay); {K}: pay 6 + 1 antimatter 18 + 12 = 36; get a city liberated 45 - 2 Discord 8 = 37 (1.03), just in band: the extreme act stays dear.',
    'annihilation_lance': f'r95 (214): 2 cr and 2 antimatter to 10 cr and 1, too dear at any credit price with 2 (0.84); {K}: pay 10 + 18 + 12 = 40; get an 8-piece hex 40 + 2 troops 10 - 2 Discord 8 = 42 (1.05), a late, big-stack card.',
}
notes = json.load(open(PATH, encoding='utf-8'))
notes.update(NOTES)
with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
    json.dump(dict(sorted(notes.items())), f, indent=2, ensure_ascii=False)
    f.write('\n')
print(f'{len(NOTES)} notes written')
