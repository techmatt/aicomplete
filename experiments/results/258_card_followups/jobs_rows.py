"""258: update scripts/design_check_jobs.json's rows for the cards this pass changed (the diff review's finding 3).

    python experiments/results/258_card_followups/jobs_rows.py

Each edit is (card name, the old note's start, the fields to set); a row whose note doesn't start so is left alone, so a
rerun changes nothing.
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PATH = os.path.join(ROOT, 'scripts', 'design_check_jobs.json')

EDITS = [
    ('Orbital Relay Launcher', '256: Action 1 antimatter', {
        'units': '3 * uses', 'note': '258: Action 1 antimatter: 3 nodes and 3 energy, x4'}),
    ('Survey Drone Fleet', '256: Action 1 cube', {
        'other': '2 * cube', 'note': '258: Action 1 cube: 2 nodes (range 2), x4; 2 cubes when built'}),
    ('Mesh Repeater Network', '256: Action 1 energy', {
        'use': 0, 'note': '258: Action (free): a node, x4'}),
    ('Field Assay Team', 'Action 1 cube: 2 agents', {
        'units': '3 * uses', 'note': '258: Action 1 cube: 3 agents, x4'}),
    ('Dyson Swarm Tranche', '10; +2 income', {
        'units': '10 + 3 * 2', 'note': '258: 10; +3 income at tier 2 (4)'}),
    ('Exascale Training Cluster', '3 a round', {
        'units': '4 * uses', 'other': '4 * card', 'note': '258: 4 a round x4; 4 cards drawn'}),
    ('Citywide Microgrid', 'Action 1 energy: sway 2, x4; liberates', {
        'units': '3 * uses', 'other': 'city - 5 * sway',
        'note': '258: Action 1 energy: sway 3, x4; liberates (city less its sway, 25)'}),
    ('Citywide Microgrid', 'Action 1 energy: sway 2, x4; 256', {
        'other': '3 * uses * sway + 0.5 * station_owned',
        'note': '258: Action 1 energy: sway 3, x4; 256: claims a neutral site next to it (~0.5)'}),
    ('Antiproton Foundry', '1 when built', {
        'units': '2 + uses', 'note': '258: 2 when built; Action 1 chip, x4'}),
    ('Microgrid Lab Compact', 'Action 2 energy: claim, ~2', {
        'units': 3, 'note': '258: claims when built, then Action 2 energy: claim, ~2 neutral sites in reach (2 uses)'}),
    ('Microgrid Lab Compact', '256: Action 2 energy', {
        'units': 3, 'note': '258: claims a station or factory when built, then Action 2 energy: the same (2 uses)'}),
    # Amendment 11877 (r119): what a use pays.
    ('Positron Storage Ring', 'Action 2 energy x4', {
        'other': '2 * uses * energy + 2 * card + uses * card',
        'note': '258: Action 2 energy: 1 antimatter and draw 1, x4; 2 energy a Power play (8); draw 2'}),
    ('Treaty Isotope Bank', 'Action 8 credits', {
        'other': 'amb + uses * amb', 'note': '258: Action 8 credits: 1 antimatter and 1 ambassador, x4; 1 ambassador'}),
    ('Antiproton Foundry', '258: 2 when built', {
        'units': '1 + uses', 'other': '2 * uses * energy',
        'note': '258: 1 when built; Action 1 chip: 1 antimatter and 2 energy, x4'}),
    ('Survey Drone Fleet', '258: Action 1 cube: 2 nodes (range 2), x4; 2 cubes', {
        'other': '2 * cube + 2 * uses * energy',
        'note': '258: Action 1 cube: 2 nodes (range 2) and 2 energy, x4; 2 cubes when built'}),
]


def main():
    text = open(PATH, encoding='utf-8').read()
    d = json.loads(text)
    done = 0
    for job in d['jobs'].values() if isinstance(d['jobs'], dict) else d['jobs']:
        for c in job['cards']:
            for name, start, fields in EDITS:
                if c['name'] == name and str(c.get('note', '')).startswith(start):
                    c.update(fields)
                    done += 1
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(d, indent=1, ensure_ascii=False) + '\n')
    print(f'{done} rows updated')


if __name__ == '__main__':
    main()
