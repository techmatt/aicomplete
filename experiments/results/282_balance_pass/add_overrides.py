"""282: values.json `overrides` rows for every repriced or changed card whose table net was at or under 0 for the
seat that uses it (givget.md's FLAG column): the table, never the price (CLAUDE.md, *Give / get*). Each row's credits
put the table's get at about 1.2 x its give. Rerunnable: an existing row for the card is replaced.

python experiments/results/282_balance_pass/add_overrides.py
"""
import re

ROWS = {
    'lure_network': 15.5, 'singularity_exploit': 13.9, 'command_and_control_grid': 8.7, 'joint_task_force': 8.1,
    'positron_storage_ring': 7.6, 'bug_bounty_board': 7.5, 'orbital_observatory_array': 7.1,
    'interdisciplinary_institute': 6.9, 'orbital_embassy': 6.0, 'open_source_uprising': 6.0,
    'recursive_self_improvement': 5.7, 'martial_cordon': 4.9, 'patent_library': 4.4, 'capital_projects_office': 4.4,
    'fusion_research_campus': 3.9, 'lights_out_assembly': 3.6, 'agency_directorate': 3.2,
}
WHY = ("282: the table reads this card's give / get under 0 for the seat that uses it (its rates sit under the "
       "anchors: an agent removed 1.33 against 3.33, a drawn card 2.67 against 3.33, insight 1.67 against 2.67, drain "
       "0.73 against 1.33, or a trigger at the generic fire rate); +{c} puts its table get at about 1.2 x its give "
       "(experiments/results/282_balance_pass/givget.md).")

PATH = 'configs/agents/values.json'


def main():
    text = open(PATH, encoding='utf-8', newline='').read()
    for card in ROWS:
        text = re.sub(r'\n    \{"card": "%s", [^\n]*' % card, '', text)
    rows = ''.join(',\n    {"card": "%s", "credits": %s, "why": "%s"}' % (card, c, WHY.format(c=c)) for card, c in ROWS.items())
    i = text.rindex('}\n  ]\n}')
    text = text[:i + 1] + rows + text[i + 1:]
    open(PATH, 'w', encoding='utf-8', newline='\n').write(text)
    print(len(ROWS), 'rows')


if __name__ == '__main__':
    main()
