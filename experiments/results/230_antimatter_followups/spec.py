"""230: the item's farm specs. A variant `PAIR_LEVER` is a board pair with a content lever; agent params carry the
agent levers (`w.am_make`, `burn_late`).

    python experiments/results/230_antimatter_followups/spec.py NAME > experiments/farm/NAME.json
    farm-seeds FIRST COUNT out/230_antimatter_followups/NAME.jsonl NAME
    python experiments/results/230_antimatter_followups/am.py out/230_antimatter_followups/NAME.jsonl

Screens on 229's spent seeds 46976-47007 (its new side, main's r100, is the base: `new-f229.jsonl` in out/keep).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
DFC = ['cards.dark_fiber_collider.tags=["cyber","science"]']  # Matt's ruling 1, in every lever but base
CONV = lambda cubes, energy: [f'corps.horizon.project.converter.cost.0.1={cubes}',
                              f'corps.horizon.project.converter.cost.1.1={energy}']
# The capstones' free antimatter off (ruling 2's second step): Sanctuary and Open Archive.
CAP0 = ['ais.protector.economy_track.6.capstone.passives.1.amount=0',
        'ais.researcher.economy_track.6.capstone.passives.1.amount=0']
LEVERS = {
    'base': [],
    'dfc': DFC,
    'c31': DFC + CONV(3, 1),
    'c22': DFC + CONV(2, 2),
    'c31cap': DFC + CONV(3, 1) + CAP0,
}
OFF = 'w.am_make=0,burn_late=0'


def spec(name, levers, agents, diffs, note):
    variants = [{'name': p if lv == 'base' else f'{p}_{lv}', 'set': s + LEVERS[lv]} for lv in levers for p, s in PAIRS]
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': agents, 'games': 32, 'records': 0}


SPECS = {
    # Screen 1: the converter rate (ruling 2), agent levers off; base checks the build plays as main with them off.
    's230a': lambda: spec('s230a', ['base', 'dfc', 'c31', 'c22'], [f'strategist:{OFF}', f'greedy:{OFF}'], [1, 2],
                          '230: screen 1, Dark Fiber Collider retag and Horizon converter rates, agent levers off'),
}
# Screen 2: the two agent levers as a factorial on screen 1's rate (rulings 3-4), one spec per candidate rate.
for _lv in ('dfc', 'c31', 'c22'):
    SPECS[f's230b-{_lv}'] = (lambda lv: lambda: spec(
        f's230b-{lv}', [lv, lv + 'cap'] if lv + 'cap' in LEVERS else [lv], [f'{a}:{f}' for a in ('strategist', 'greedy')
                              for f in ('w.am_make=0,burn_late=0', 'w.am_make=1,burn_late=0',
                                        'w.am_make=0,burn_late=1', 'w.am_make=1,burn_late=1')],
        [1, 2], f'230: screen 2, w.am_make x burn_late on lever {lv}'))(_lv)

# Screen 3 (amendment 10186 and ruling 4's reach): burn_late in the search too (2), Make antimatter only through
# make_short (am_last), a factorial on c31 with w.am_make on.
SPECS['s230c'] = lambda: spec('s230c', ['c31'], [f'{a}:{f}' for a in ('strategist', 'greedy')
                                                for f in ('burn_late=1,am_last=0', 'burn_late=2,am_last=0',
                                                          'burn_late=1,am_last=1', 'burn_late=2,am_last=1')],
                              [1, 2], '230: screen 3, burn_late 1/2 x am_last on c31')
# Ruling 3's lever reaches only table-mode agents (hand_worth): the engine agent, w.am_make on and off, a smoke.
SPECS['s230d'] = lambda: dict(spec('s230d', ['c31'], ['engine', 'engine:w.am_make=0'], [2],
                                   '230: engine smoke, w.am_make on/off on c31'), games=16)

# Screen 4 (after the fresh read; the plants-largest target's reach): the shipped agents (burn_late 1, am_last) on
# the capstones' antimatter off and the converter at 2 cubes + 2 energy (unused by the agents), and both.
LEVERS['c22cap'] = DFC + CONV(2, 2) + CAP0
SPECS['s230e'] = lambda: spec('s230e', ['c31', 'c31cap', 'c22', 'c22cap'], ['greedy', 'strategist'], [1, 2],
                              '230: screen 4, the plants target: capstones off and the converter at 2+2, shipped agents')

# The fresh read (47008-47039): the shipped build's defaults, greedy and the strategist, d0-d3; the base side is
# main's exe (r100) on the same spec, played natively (`scripts/bench/shards.sh`).
SPECS['f230'] = lambda: spec('f230', ['base'], ['greedy', 'strategist'], [0, 1, 2, 3],
                             '230: fresh read 47008-47039, the shipped build against main r100 on the same seeds')

name = sys.argv[1]
if name not in SPECS:
    sys.exit(f'unknown spec {name}; known: {", ".join(SPECS)}')
print(json.dumps(SPECS[name](), indent=2))
