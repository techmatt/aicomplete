"""210: the item's specs, cells as `<pair>_<lever>` variants on one binary.

    python experiments/results/210_settle_r91/spec.py NAME > experiments/farm/NAME.json

NAME: s210-picks (dev seeds 36800-36831, the r91 base: 206's screen-1 picks each reverted, and the stage-II Musters
back), s210-lift (dev seeds, r92: Lift land corruption off, at 2, at 3), f210-strategist (the fresh read, base only),
or s210-<lever>-... (a screen of the named levers beside base).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
TROOPS = {'op': 'troops', 'count': 1, 'each_country': True}
AMB = {'op': 'increase_income', 'ambassadors': 1}


def standing(card, lines):
    return f'adversary.{card}.standing=' + json.dumps(lines, separators=(',', ':'))


LEVERS = {
    'base': [],
    # 206's screen-1 picks, each reverted to the screened alternative (the pick is base)
    'st2': ['board.station_chips=2'],
    'cf3': ['board.claim_factory=3'],
    'hb': ['board.harden_credits=3'],
    'fac': ['board.factory_build_chips=3'],
    # amendment 9257's stage-II Muster re-read: r88's Muster 1 a country back on the five stage-II cards that had it
    'ms2': [
        standing('mil_s2_mobilization', [{'op': 'rally', 'site': 'coastal_city'}, TROOPS, AMB]),
        standing('mil_s2_station_raids', [{'op': 'rally', 'site': 'station'}, TROOPS, {'op': 'spread_botnets', 'count': 1}, AMB]),
        standing('mil_s2_border_war', [{'op': 'rally', 'site': 'factory'}, TROOPS, AMB]),
        standing('mil_s2_supply_lines', [{'op': 'rally', 'site': 'factory'}, TROOPS, {'op': 'increase_income', 'compute': 2}, AMB]),
        standing('mil_s2_forced_march', [{'op': 'rally', 'site': 'city'}, TROOPS, AMB]),
    ],
    # r92: Lift land corruption off, and at 3 cubes (base 2)
    'l0': ['board.land_cubes=0'],
    'l1': ['board.land_cubes=1'],
    'l3': ['board.land_cubes=3'],
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
if name == 's210-picks':
    out = spec(name, ['base', 'st2', 'cf3', 'hb', 'fac', 'ms2'], ALL, 'strategist', 32,
               '210: 206 screen-1 picks reverted and stage-II Musters back, on r91 (dev seeds 36800-36831)')
elif name == 's210-lift':
    out = spec(name, ['base', 'l0', 'l3'], ALL, 'strategist', 32, '210: Lift land corruption off / 2 / 3 (dev seeds)')
elif name.startswith('f210-'):
    out = spec(name, ['base'], ALL, name.split('-')[1], 96, '210: the fresh read, r92 against r91')
elif name.startswith('s210-'):
    levers = [x for x in name.split('-')[1:] if x != 'base']
    out = spec(name, ['base'] + levers, ALL, 'strategist', 32, '210: a screen on the dev seeds')
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
