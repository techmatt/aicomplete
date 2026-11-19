"""282: one current pricing note a card for every card the balance pass repriced or changed (docs/card-pricing-notes.json,
keyed by card id; CLAUDE.md: the item that sets the price writes the line). Rerunnable: replaces the card's line.

python experiments/results/282_balance_pass/pricing_notes.py
"""
import json

NOTES = {
    'foundation_model_campus': '3 to 11 cr: income 2 a tag frozen at 4-5 tags for 3-4 rounds 24-40 + draw 2 6.67 against pay 30.33 (11 cr, 2 energy, tier-2 keep): 1.01-1.54, mid 1.28.',
    'vertical_monopoly': '3 to 11 cr: income 2 a tag at 4-5 tags, 3-4 rounds 24-40 against pay 24.33: 0.99-1.64, mid 1.29.',
    'orbital_mass_driver': '2 to 15 cr: 9 credits and 2 chips income 46.5-62 against pay 41.33 (15 cr, 3 energy, a chip, tier-2 keep): 1.13-1.50 (2.19 at 2); unplayed for its needs (3 Industry, 3 energy), not its price.',
    'orbital_embassy': '2 to 5 cr: draw 2 6.67 + an ambassador a Diplomacy play 5.33 x 3-4 = 22.7-28 against pay 18.33: 1.24-1.53; values.json reads it under 0 (flagged).',
    'zero_day_auction': '2 to 4 cr: 2 antimatter 24 + draw 3 10 = 34 against pay 17.33: 1.96 (needs 5 Cyber, about half its worth).',
    'dark_fiber_collider': '2 to 7 cr: Action 2 insight to 1 antimatter, 6.67 a use x 3-4 = 20-26.7 against pay 17.67: 1.13-1.51.',
    'lure_network': '3 to 12 cr: Action up to 3 agents, 2.5-3 x 3.33 x 3-4 = 25-40 against pay 22.67: 1.10-1.76, mid 1.32; values.json reads it under 0 (flagged).',
    'open_source_uprising': '2 to 10 cr: convert 3-4 agents at 7.07 + a Unity Sway = 23.9-31 against pay 20.67: 1.16-1.50; values.json reads it under 0 (flagged).',
    'kettle_operation': 'play insight 2 to 1 (2 cr): up to 6 agents or troops, 4-6 at 3.33 less a Discord Sway = 10.7-17.3 against pay 12.67: 0.84-1.37 (0.91 mid at 2 insight).',
    'command_and_control_grid': '13 to 4 cr; its Action draws 1 (was 2) and costs no energy (Matt: 2 with Reuse): 3.33 x 3-4 rounds = 10-13.3 against pay 17.33: 0.58-0.77 alone, 1.15-1.54 with Simulation Cluster; values.json reads it under 0 (flagged).',
    'simulation_cluster': '2 to 4 cr: Reuse one Action a round, 3.33-6.67 x 3-4 = 10-26.7 against pay 12: mid 1.46 (1.75 at 2).',
    'patent_library': '2 to 4 cr: Action 2 cr to 2 insight, 3.33 x 3-4 = 10-13.3 against pay 9.33: 1.07-1.43.',
    'bug_bounty_board': '2 to 5 cr: Action 3 cr to drain 5 (6.67), 3.67 x 3-4 = 11-14.7 against pay 10.33: 1.07-1.42.',
    'field_sensor_grid': '2 to 7 cr: lift 1.5-3 Fallout at 8 + draw 1 = 15.3-27.3 against pay 15: 1.02-1.82, mid 1.42 (2.1 at 2).',
    'joint_task_force': '2 to 7 cr: 2 agents a teammate\'s Security play, 0.5-1 a round x 3.5 = 11.7-23.3 against pay 12.33: 0.95-1.89, mid 1.42 (2.4 at 2).',
    'agency_directorate': '2 cr kept; stronger (unplayed): a Security play adds 2 Operatives (was 1), at 4 gain 4 insight (was 2): 2 fires in 3-4 rounds 21.3 against pay 15.33: 1.39 (0.35-0.52 before).',
    'interdisciplinary_institute': '2 cr kept; stronger: 2 insight a Science tag played (was 1): 16-21.3 against pay 15.33: 1.04-1.39 (0.52-0.70 before).',
    'capital_projects_office': '2 cr kept; stronger: 4 credits an Industry play (was 2): 12-16 against pay 12.67: 0.95-1.26 (0.47-0.63 before).',
    'fusion_research_campus': '2 cr kept; stronger: 2 energy a Power play (was 1): 18-24 against pay 15.33: 1.17-1.57 (0.59-0.78 before).',
    'orbital_observatory_array': '2 cr kept; stronger: Action 1 energy to 3 insight (was 2): 15-20 against pay 15.33: 0.98-1.30 (0.46-0.61 before).',
    'lights_out_assembly': '2 cr; stronger: 3 credits an Industry play (was a printed 1), play insight 2 to 1: 9-12 against pay 10.33: 0.87-1.16 (0.23-0.31 before).',
    'parts_exchange': '2 cr kept; Action 1 chip to 5 credits (was 3, a loss at chip 4): a gain a use, still weak (0.41-0.55).',
}
PATH = 'docs/card-pricing-notes.json'


def main():
    notes = json.load(open(PATH, encoding='utf-8'))
    for k, v in NOTES.items():
        notes[k] = f'r134 (282): {v}'
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        f.write('{\n' + ',\n'.join(f'  {json.dumps(k)}: {json.dumps(notes[k], ensure_ascii=False)}' for k in sorted(notes)) + '\n}\n')
    print(len(NOTES), 'notes')


if __name__ == '__main__':
    main()
