"""256: writes experiments/farm/s256-screen.json's arms: all (the card pass) and each group restored to r115.

    python experiments/results/256_item_c_cards/arms.py

The r115 arm is the same spec run natively by r115's binary (main 4981702). A group off restores its cards' r115
fields by overlay and blanks its new cards (`cost` 99: never kept or played, still drafted, so the deck is the
pass's). Power Purchase Agreement and Microgrid Lab Compact stay widened in every arm: their r115 op
(`claim_station`) left the code.
"""
import json
import os
import subprocess

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
BASE = '4981702'
old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', f'{BASE}:content/cards.json'], cwd=ROOT,
                                                       capture_output=True, text=True, encoding='utf-8').stdout)}


def restore(cid, *paths):
    out = []
    for p in paths:
        v = old[cid]
        for k in p.split('.'):
            v = v[k]
        out.append(f'cards.{cid}.{p}={json.dumps(v, separators=(",", ":"))}')
    return out


def blank(*ids):
    return [f'cards.{i}.cost=99' for i in ids]


GROUPS = {
    'sites': restore('open_research_campus', 'cost', 'effects') + restore('industrial_commons', 'cost', 'effects'),
    'rider': (restore('nuclear_option', 'effects') + restore('power_to_the_people', 'effects')
              + restore('forensic_audit', 'effects') + restore('root_access', 'effects')
              + restore('peacekeeping_mission', 'effects') + restore('popular_uprising', 'ability.or.effects')
              + restore('citywide_microgrid', 'effects')),
    'nodes': restore('relay_mast_rollout', 'cost', 'effects') + blank('mesh_repeater_network'),
    'sinks': blank('research_reactor', 'survey_drone_fleet', 'orbital_relay_launcher'),
}
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = []
for pair, pset in PAIRS:
    variants.append({'name': f'{pair}-all', 'set': pset})
    for g, sets in GROUPS.items():
        variants.append({'name': f'{pair}-no-{g}', 'set': pset + sets})
spec = {
    'name': 's256-screen',
    'note': "256's dev screen (Item C): the judge, three pairs, d0-d4, the dev seeds 36800-36831. Arms: all (the card "
            "pass) and each group restored to r115 (no-sites: Campus and Commons; no-rider: the seven liberate cards' "
            "claim rider; no-nodes: Relay Mast Rollout, Mesh Repeater Network blanked; no-sinks: Research Reactor, "
            "Survey Drone Fleet, Orbital Relay Launcher blanked), written by "
            "experiments/results/256_item_c_cards/arms.py. The r115 arm: r115's binary natively on the same seeds, "
            "variants bp, br, pr. Variant-major: pair, then arm.",
    'configs': ['configs/baseline.json'],
    'variants': variants,
    'difficulties': [0, 1, 2, 3, 4],
    'agents': ['strategist'],
    'games': 32,
    'first_seed': 36800,
    'records': 0,
}
with open(os.path.join(ROOT, 'experiments', 'farm', 's256-screen.json'), 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(spec, indent=1) + '\n')
# The same arms on fresh seeds at d3-d4 (the levels the read asks about), after the fresh read.
groups = dict(spec, name='f256-groups', difficulties=[3, 4], games=48, first_seed=49164,
              note="256: the screen's group arms on fresh seeds (49164-49211), the judge, three pairs, d3-d4, on the "
                   "frozen build: which groups moved d3 and d4 (the fresh read's d3 fell 16.7 points). Written by "
                   "experiments/results/256_item_c_cards/arms.py. Variant-major: pair, then arm.")
with open(os.path.join(ROOT, 'experiments', 'farm', 'f256-groups.json'), 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(groups, indent=1) + '\n')
print(len(variants), 'variants')
