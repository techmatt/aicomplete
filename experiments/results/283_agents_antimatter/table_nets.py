"""283: each card's table net (values.json's own reading) under candidate rate sets.

net = the ledger's reasoned value - printed (credits + insight and other costs at the table's rates) - the keep
(rates.keep_tier1 / keep_tier2): 282's givget.md Net column (effect_ledger.py --stdout, reasoned - printed - keep).

    python experiments/results/283_agents_antimatter/table_nets.py [--write SET]

Prints the 17 cards 282 flagged (net at or under 0) and every card whose net is at or under 0 under each set.
--write SET rewrites configs/agents/values.json with that set (the shipped table).
"""
import copy
import csv
import io
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
VALUES = os.path.join(ROOT, 'configs', 'agents', 'values.json')
SCRATCH = os.path.join(ROOT, 'out', '283_agents_antimatter')

FLAGGED = ['orbital_embassy', 'lure_network', 'open_source_uprising', 'command_and_control_grid', 'patent_library',
           'bug_bounty_board', 'joint_task_force', 'agency_directorate', 'interdisciplinary_institute',
           'capital_projects_office', 'fusion_research_campus', 'orbital_observatory_array', 'lights_out_assembly',
           'recursive_self_improvement', 'singularity_exploit', 'positron_storage_ring', 'martial_cordon']

# Resources and rates at the unit anchors (card-guidelines.md, *Unit anchors*; antimatter 18 by Matt's 283 ruling).
RESOURCES = {'insight': 2.667, 'chips': 4, 'research_cubes': 4, 'ambassadors': 5.333, 'antimatter': 18}
RATES = {'drawn_card': 3.333}
# The keep at the anchors: 1 insight a tier-1 card, 3 a tier-2 one, at 2.67.
KEEP = {'keep_tier1': 2.667, 'keep_tier2': 8}
# The table's keep moved with its insight rate (1 and 2.6 insight, its own expected count, at 2.667): shipped. The
# keep rows change no play (screen 1: T and Tnk played the same games), only this reading.
KEEP_SCALED = {'keep_tier1': 2.667, 'keep_tier2': 6.933}
# An agent removed at its anchor 3.33 (rows priced a removal at 1.33-1.47); compute drained at 1.33 (0.73).
REMOVAL = {'remove_ambassador': 3.333, 'pushback': 3.333, 'clear_city': 3.333, 'remove_ambassadors_country': 3.333}
DRAIN = {'drain_project': 1.333, 'drain_all': 2, 'project_drain': 2}
HORIZON = {'horizon_t2': 3.5}
# A committed seat plays about 1 card of its tag a round (the anchor check); the table 1.5 x 0.3 = 0.45.
FIRES = {'tag_share': 0.5}
# A project discarded with its compute: about a mid project's cost at the drain anchor (16 x 1.33 / 2 countries ~ 10).
DISCARD = {'discard_project': 10}

SETS = {
    'base': ({}, {}, {}),
    'res': (RESOURCES, RATES, {}),
    'res+keep': (RESOURCES, {**RATES, **KEEP}, {}),
    'res+rm': (RESOURCES, RATES, {**REMOVAL, **DRAIN}),
    'all': (RESOURCES, {**RATES, **KEEP}, {**REMOVAL, **DRAIN}),
    # The anchor check counts an engine 3-4 rounds (mid 3.5) at a committed seat's rate; the table 2.5 at tier 2.
    'all+h': (RESOURCES, {**RATES, **KEEP, **HORIZON}, {**REMOVAL, **DRAIN}),
    'res+rm+h': (RESOURCES, {**RATES, **HORIZON}, {**REMOVAL, **DRAIN}),
    'all+h+f': (RESOURCES, {**RATES, **KEEP, **HORIZON, **FIRES}, {**REMOVAL, **DRAIN, **DISCARD}),
    # What screens 1-3 kept (the agent-removal, drain and discard rows lost: screen 2's Trm, -3.8 +- 2.0 judge).
    'shipped': (RESOURCES, {**RATES, **KEEP_SCALED, **HORIZON}, {}),
    'anchors-keep': (RESOURCES, {**RATES, **KEEP, **HORIZON}, {}),
    'shipped-keep': (RESOURCES, {**RATES, **HORIZON}, {}),
}


def table(name):
    with open(VALUES, encoding='utf-8') as f:
        t = json.load(f)
    res, rates, ops = SETS[name]
    t = copy.deepcopy(t)
    t['resources'].update(res)
    t['rates'].update(rates)
    for k, v in ops.items():
        t['ops'][k]['credits'] = v
    return t


def nets(t):
    path = os.path.join(SCRATCH, 'values_try.json')
    os.makedirs(SCRATCH, exist_ok=True)
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(t, f)
    out = subprocess.run([sys.executable, os.path.join(ROOT, 'scripts', 'cards', 'effect_ledger.py'), '--stdout',
                          '--values', path], capture_output=True, text=True, encoding='utf-8', check=True).stdout
    rows = {}
    for r in csv.DictReader(io.StringIO(out)):
        if not r.get('reasoned'):
            continue  # the fit's summary lines after the table
        keep = t['rates']['keep_tier2' if r['tier'] == '2' else 'keep_tier1']
        rows[r['card']] = float(r['reasoned']) - float(r['printed']) - keep
    return rows


def main():
    if '--write' in sys.argv:
        name = sys.argv[sys.argv.index('--write') + 1]
        print('use set flags in the agent spec; --write is', name)
        return
    allnets = {name: nets(table(name)) for name in SETS}
    print('card,' + ','.join(SETS))
    for c in FLAGGED:
        print(c + ',' + ','.join('%.1f' % allnets[n][c] for n in SETS))
    for n in SETS:
        under = sorted(c for c, v in allnets[n].items() if v <= 0)
        flagged_above = sum(1 for c in FLAGGED if allnets[n][c] > 0)
        print(f'{n}: {flagged_above} of 17 above 0; {len(under)} cards at or under 0 of {len(allnets[n])}')
        print('   under:', ' '.join(under))


if __name__ == '__main__':
    main()
