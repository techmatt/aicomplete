"""219: the item's specs (the strategist unless named), three pairs, as 215's.

    python experiments/results/219_antimatter_sources/spec.py NAME > experiments/farm/NAME.json

NAME: f219 (the fresh read on r97: new seeds 46624-46719, d0-d4), f219g (its greedy line), s219-dev (the 16-seed
smoke on the dev seeds 36800-36815, d0-d3). The r96 side runs the same spec natively from r96's tree (out/r96src).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


MC6 = ['board.antimatter_cubes=6']
MC5 = ['board.antimatter_cubes=5']
CHEAP = ['cards.penning_trap_array.ability.cost.0.1=1', 'cards.antiproton_foundry.ability.cost.0.1=1',
         'cards.treaty_isotope_bank.ability.cost.0.1=1', 'cards.positron_storage_ring.ability.cost.0.1=2',
         'cards.dark_fiber_collider.ability.cost.0.1=2', 'cards.cyclotron_lease.ability.cost.0.1=2']
MC4 = ['board.antimatter_cubes=4']  # the prompt's number: Make antimatter at r96's 4 cubes
LEVERS = {'base': [], 'mc4': MC4, 'mc5': MC5, 'mc6': MC6, 'cheap': CHEAP, 'mc6cheap': MC6 + CHEAP}


def spec(name, diffs, games, note, agent='strategist', levers=('base',)):
    variants = [{'name': p if lv == 'base' else f'{p}_{lv}', 'set': s + LEVERS[lv]} for lv in levers for p, s in PAIRS]
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
if name == 'f219':
    out = spec(name, [0, 1, 2, 3, 4], 96, '219: the fresh read (strategist), new seeds 46624-46719; mc4 the asked Make antimatter',
               levers=('base', 'mc4'))
elif name == 'f219g':
    out = spec(name, [0, 1, 2, 3, 4], 96, '219: the fresh read (greedy), new seeds 46624-46719', agent='greedy')
elif name == 's219-lev':
    out = spec(name, [0, 1, 2, 3], 16, '219: dev-seed levers (Make antimatter price, cheaper plants), 36800-36815',
               levers=('base', 'mc5', 'mc6', 'cheap', 'mc6cheap'))
elif name == 's219-dev':
    out = spec(name, [0, 1, 2, 3], 16, '219: the 16-seed smoke, dev seeds 36800-36815')
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
