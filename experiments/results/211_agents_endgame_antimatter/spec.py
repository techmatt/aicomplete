"""211: the item's specs, one cell per agent (the levers are agent switches; no rules, content or dial changes).

    python experiments/results/211_agents_endgame_antimatter/spec.py NAME > experiments/farm/NAME.json

NAME: s211-smoke (16 dev seeds a pair from 36800, d2: base and all on, strategist and greedy), s211-screen (206's
spent fresh seeds from 45631, d2-d4: each switch alone, all on, the d3-gap cells and greedy), f211-strategist /
f211-greedy (the fresh read, base and final defaults).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
OFF = 'sell_rule=0,am_save=0,plan_am=0,make_short=0'
CELLS = {
    'base': f'strategist:{OFF}',
    'sell': 'strategist:sell_rule=1,am_save=0,plan_am=0,make_short=0',
    'sell2': 'strategist:sell_rule=2,am_save=0,plan_am=0,make_short=0',
    'sell3': 'strategist:sell_rule=3,am_save=0,plan_am=0,make_short=0',
    'am': 'strategist:sell_rule=0,am_save=1,plan_am=0,make_short=0',
    'pam': 'strategist:sell_rule=0,am_save=0,plan_am=1,make_short=0',
    'ampam': 'strategist:sell_rule=0,am_save=1,plan_am=1,make_short=0',
    'mk': 'strategist:sell_rule=0,am_save=0,plan_am=0,make_short=1',
    'all': 'strategist:sell_rule=1,am_save=1,plan_am=1,make_short=1',
    # the d3 gap: the strategist's own weights against greedy's
    'purge0': f'strategist:{OFF},w.purge=0',
    'ln0': f'strategist:{OFF},w.last_node=0',
    'g-base': f'greedy:{OFF}',
    'g-all': 'greedy:sell_rule=1,am_save=1,plan_am=1,make_short=1',
}


def spec(name, cells, diffs, games, note):
    variants = [{'name': p, 'set': s} for p, s in PAIRS]
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [CELLS[c] for c in cells], 'games': games, 'records': 0}


name = sys.argv[1]
if name == 's211-smoke':
    out = spec(name, ['base', 'sell', 'sell2', 'sell3', 'pam', 'ampam', 'mk', 'all', 'g-base', 'g-all'], [2], 16,
               '211: engine smoke, dev seeds 36800')
elif name == 's211-screen':
    out = spec(name, ['base', 'sell', 'sell2', 'pam', 'ampam', 'mk', 'all', 'purge0', 'ln0', 'g-base', 'g-all'],
               [2, 3, 4], 64, "211: screen on 206's spent fresh seeds 45631")
elif name.startswith('s211-'):
    cells = name.split('-')[1:]
    out = spec(name, cells, [2, 3, 4], 64, "211: a screen on 206's spent fresh seeds 45631")
elif name == 'f211':
    out = spec(name, ['base', 'all', 'g-base', 'g-all'], [0, 1, 2, 3, 4], 96, '211: the fresh read')
else:
    raise SystemExit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
