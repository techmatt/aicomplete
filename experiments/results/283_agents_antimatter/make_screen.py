"""283: an agent-lever screen spec: each arm is the judge (strategist) and greedy with its `w.` / `t.` sets, three
pairs (bp, br, pr), d0-d4, the dev seeds 36800-36831 (32 a cell).

    python experiments/results/283_agents_antimatter/make_screen.py NAME ARM[,ARM...] [--agents strategist,greedy]

Writes experiments/farm/NAME.json. Arms are named in ARMS below; `base` is the agents as built.
"""
import json
import sys

RES = {'resources.insight': 2.667, 'resources.chips': 4, 'resources.research_cubes': 4,
       'resources.ambassadors': 5.333, 'resources.antimatter': 18, 'rates.drawn_card': 3.333}
KEEP = {'rates.keep_tier1': 2.667, 'rates.keep_tier2': 8}
RM = {'ops.remove_ambassador': 3.333, 'ops.pushback': 3.333, 'ops.clear_city': 3.333,
      'ops.remove_ambassadors_country': 3.333, 'ops.drain_project': 1.333, 'ops.drain_all': 2, 'ops.project_drain': 2,
      'ops.discard_project': 10}
HOR = {'rates.horizon_t2': 3.5}

ARMS = {
    'base': {},
    # Item 1: the table's rates at the unit anchors (table_nets.py's sets).
    'T': {**{'t.' + k: v for k, v in {**RES, **KEEP, **RM, **HOR}.items()}},
    'Tnk': {**{'t.' + k: v for k, v in {**RES, **RM, **HOR}.items()}},
    'K': {'w.keep_units': 1},
    'D': {'w.research_delay': 1},
    # Item 2: the flat valuation of a held antimatter (2 points, about 2.2 credits) raised, and the sink on.
    'A4': {'w.antimatter': 4},
    'A4s': {'w.antimatter': 4, 'w.sink': 1},
    'A6s': {'w.antimatter': 6, 'w.sink': 1},
    # Screen 2: T split into its parts, and a committed seat's tag share; the flat antimatter weight at 3, the least
    # at which Antiproton Foundry's Action (1 chip, 4 points -> 1 antimatter and 2 energy, 1.6) reads as a gain.
    'Tr': {'t.' + k: v for k, v in RES.items()},
    'Trm': {'t.' + k: v for k, v in RM.items()},
    'Th': {'t.' + k: v for k, v in HOR.items()},
    'Tf': {'t.rates.tag_share': 0.5},
    'A3': {'w.antimatter': 3},
    'A3s': {'w.antimatter': 3, 'w.sink': 1},
    # Screen 3: what screens 1-2 kept, together (the table's resources, keep and tier-2 horizon at the anchors;
    # antimatter 3; D106 on), then keep_units and 237's plan_target on it.
    'C': {**{'t.' + k: v for k, v in {**RES, **KEEP, **HOR}.items()}, 'w.antimatter': 3, 'w.research_delay': 1},
}
ARMS['CK'] = {**ARMS['C'], 'w.keep_units': 1}
ARMS['CP'] = {**ARMS['C'], 'plan_target': 1}
# Screen 4: C's table part alone (R: resources, keep, horizon at the anchors as shipped), with keep_units, and with
# antimatter 3 on top, to split C's d0 dip between the table and the antimatter weight.
ARMS['R'] = {'t.' + k: v for k, v in {**RES, 'rates.keep_tier1': 2.667, 'rates.keep_tier2': 6.933, **HOR}.items()}
ARMS['RK'] = {**ARMS['R'], 'w.keep_units': 1}
ARMS['RKA'] = {**ARMS['RK'], 'w.antimatter': 3}
# Screen 5, on the shipped build (base is RK): what the table's antimatter at the ruling's 18 costs the spending
# (Containment Reactor never kept; Orbital Relay Launcher played less), against 12.
ARMS['am12'] = {'t.resources.antimatter': 12}
# Screen 6, on the shipped build: the brief's lever again where it now stands (the sink alone, and with the weight).
ARMS['S1'] = {'w.sink': 1}

PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]


def agent(kind, sets):
    if not sets:
        return kind
    return kind + ':' + ','.join(f'{k}={v}' for k, v in sets.items())


def main():
    name, arms = sys.argv[1], sys.argv[2].split(',')
    kinds = ['strategist', 'greedy']
    if '--agents' in sys.argv:
        kinds = sys.argv[sys.argv.index('--agents') + 1].split(',')
    extra = {}
    for a in sys.argv[3:]:
        if a.startswith('+'):
            # +ARM=k=v;k=v : an extra arm defined on the command line
            arm, _, body = a[1:].partition('=')
            extra[arm] = dict(kv.split('=', 1) for kv in body.split(';') if kv)
    table = {**ARMS, **extra}
    agents = [agent(k, table[a]) for a in arms for k in kinds]
    spec = {
        "name": name,
        "note": "283 agent screen: arms " + ", ".join(arms) + " x the judge and greedy (see "
                "experiments/results/283_agents_antimatter/make_screen.py), three pairs, d0-d4, dev seeds.",
        "configs": ["configs/baseline.json"],
        "variants": [{"name": p, "set": s} for p, s in PAIRS],
        "difficulties": [0, 1, 2, 3, 4],
        "agents": agents,
        "games": 32,
        "first_seed": 36800,
        "records": 0,
    }
    out = f'experiments/farm/{name}.json'
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=1)
        f.write('\n')
    print(out, len(agents), 'agents', len(agents) * 15 * 32, 'games')


if __name__ == '__main__':
    main()
