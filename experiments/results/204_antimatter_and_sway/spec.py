"""204: the item's specs, one per read, cells as `<pair>_<lever>` variants on one binary.

    python experiments/results/204_antimatter_and_sway/spec.py NAME > experiments/farm/NAME.json

NAME: s204-screen (dev seeds; base = the prompt's numbers, c4 = 4 cubes a desperate antimatter), s204-old (r89's
base, run by the r89 binary in out/r89), s204-abl-<agent arg> (d3-d4 ablations), f204-strategist / f204-greedy (the
fresh read, base only).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
LEVERS = {
    'base': [],
    'c4': ['board.antimatter_cubes=4'],
    # screen 3 on (base 4 cubes from here): the prompt's 3 cubes; the botnet takedown's Unity back (r89's 1)
    'c3': ['board.antimatter_cubes=3'],
    'bn1': ['alignment.botnet=1'],
    # screen 4: Breakthrough, the standard antimatter spend (1 antimatter for N Unity, once a round)
    'bt2': ['board.breakthrough_unity=2'],
    'bt3': ['board.breakthrough_unity=3'],
    # dials (d1 only, over its band): compute income at d1, r89's 4
    'k8': ['adversary.compute_income=0,8,17,25,30'],
    'k12': ['adversary.compute_income=0,12,17,25,30'],
}


def spec(name, levers, diffs, agent, games, note):
    variants = []
    for lever in levers:
        for p, s in PAIRS:
            variants.append({'name': f'{p}_{lever}' if lever != 'base' else p, 'set': s + LEVERS[lever]})
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
ALL = [0, 1, 2, 3, 4]
if name == 's204-screen':
    out = spec(name, ['base', 'c4'], ALL, 'strategist', 32, '204: screen on the dev seeds 36800-36831')
elif name == 's204-old':
    out = spec(name, ['base'], ALL, 'strategist', 32, '204: r89 (accd80a) on the dev seeds, the r89 binary')
elif name.startswith('s204-abl'):
    # the ablations (200's no_spend: 1 chips, 2 research cubes, 3 antimatter), d3-d4
    out = spec(name, ['base'], [3, 4], 'strategist' + (':no_spend=' + name.split('-')[-1] if name != 's204-abl' else ''),
               32, '204: d3-d4 ablation on the dev seeds')
elif name == 's204-screen3':
    out = spec(name, ['base', 'c3', 'bn1'], ALL, 'strategist', 32,
               '204: screen 3 (Containment Reactor, w.antimatter 2, 4 cubes), dev seeds 36800-36831')
elif name == 's204-screen4':
    out = spec(name, ['base', 'bt2', 'bt3'], ALL, 'strategist', 32, '204: screen 4 (Breakthrough), dev seeds 36800-36831')
elif name == 'd204-d1':
    # d1's dial on the spent fresh seeds 45439-45534 (201's route), the confirm on the next free ones
    out = spec(name, ['base', 'k8', 'k12'], [1], 'strategist', 96, '204: d1 compute income dial')
elif name.startswith('s204-w'):
    # screen 2: the eval's held antimatter (`w.antimatter`, 5 since 200 saved it for Annihilate) at 4 cubes
    out = spec(name, ['c4'], ALL, f'strategist:w.antimatter={name[6:]}', 32,
               '204: screen 2, the held-antimatter weight, dev seeds')
elif name == 'c204-strategist':
    out = spec(name, ['base'], ALL, 'strategist', 96,
               '204: the confirm read (Pair Forge 1 chip, d1 compute 8) on 45535-45630, r90 against r89')
elif name.startswith('f204-'):
    agent = name.split('-', 1)[1]
    out = spec(name, ['base'], ALL, agent, 96 if agent == 'strategist' else 48,
               '204: the fresh read, r90 against r89 on the same seeds')
else:
    raise SystemExit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
