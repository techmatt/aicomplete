"""213: the item's specs, cells as `<pair>_<lever>` variants (the strategist, 211's defaults).

    python experiments/results/213_tier2_design/spec.py NAME > experiments/farm/NAME.json

NAME: s213-smoke (16 dev seeds from 36800, d2, base and not2), s213-screen (dev seeds 36800-36831, d0-d4: base,
the tier-2-off arm `not2` and Breakthrough back on `bt`), s213-r93 (the old side's base cells, run from `out/r93src`
with r93's binary), f213 (the fresh read: base, not2 and bt), f213-r93 (its old side).
Lever `not2` runs experiments/configs/no_tier2.json (scripts/configs/no_tier2.py) instead of the baseline.
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
LEVERS = {
    'base': [],
    'not2': [],
    # Breakthrough back on (r93's standard antimatter spend: 1 antimatter, 2 Unity, once a round)
    'bt': ['board.breakthrough_unity=2'],
    # Tidal Grid's converter costlier: 2 research cubes + 1 energy -> 1 antimatter (r93: 1 cube + 1 energy -> 2)
    'tg': ['corps.horizon.project.converter.cost=[["research_cubes",2],["energy",1]]', 'corps.horizon.project.converter.effects.0.amount=1'],
}
CONFIG = {'not2': 'experiments/configs/no_tier2.json'}


def spec(name, levers, diffs, games, note, agent='strategist'):
    configs = sorted({CONFIG.get(lv, 'configs/baseline.json') for lv in levers})
    if len(configs) > 1:
        sys.exit('one config a spec: put not2 in its own spec')
    variants = []
    for lever in levers:
        for p, s in PAIRS:
            variants.append({'name': f'{p}_{lever}' if lever != 'base' else p, 'set': s + LEVERS[lever]})
    return {'name': name, 'note': note, 'configs': configs, 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
ALL = [0, 1, 2, 3, 4]
if name == 's213-smoke':
    out = spec(name, ['base', 'bt'], [2], 16, '213: engine smoke, dev seeds 36800')
elif name == 's213-smoke-not2':
    out = spec(name, ['not2'], [2], 16, '213: engine smoke, tier 2 off, dev seeds 36800')
elif name == 's213-screen':
    out = spec(name, ['base', 'bt'], ALL, 32, '213: screen on the dev seeds 36800-36831')
elif name == 's213-not2':
    out = spec(name, ['not2'], ALL, 32, '213: tier 2 off, dev seeds 36800-36831')
elif name == 's213-tg':
    out = spec(name, ['tg'], ALL, 32, '213: Tidal Grid converter lever, dev seeds 36800-36831')
elif name.startswith('s213b-'):
    # round 2 (the Tidal Grid converter at 2 cubes + 1 energy -> 1, the agents' antimatter sources): base and not2
    # with the agent changes on, and the agent arms off one at a time and both (`ag0`). `w.am_power_earn` was an eval
    # arm (an antimatter card's worth in hand counting the board's antimatter powers as income): earn0 played the
    # same games as base, so it was cut; these two specs (in experiments/archive/farm/) no longer run on the shipped build.
    arm = name[6:]
    agent = {'base': 'strategist', 'not2': 'strategist', 'ag0': 'strategist:am_source=0,w.am_power_earn=0',
             'src0': 'strategist:am_source=0', 'earn0': 'strategist:w.am_power_earn=0'}[arm]
    out = spec(name, ['not2' if arm == 'not2' else 'base'], ALL, 32, f'213 round 2: {arm}, dev seeds 36800-36831', agent)
elif name == 's213-r93':
    out = spec(name, ['base'], ALL, 32, "213: the old side's screen cells (r93, dev seeds 36800-36831)")
elif name == 'f213':
    out = spec(name, ['base', 'bt'], ALL, 96, '213: the fresh read, r94 base and Breakthrough back on')
elif name == 'f213-not2':
    out = spec(name, ['not2'], ALL, 96, '213: the fresh read, tier 2 off')
elif name == 'f213-r93':
    out = spec(name, ['base'], ALL, 96, "213: the fresh read's old side (r93, out/r93src)")
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
