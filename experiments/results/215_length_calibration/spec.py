"""215: the item's specs, cells as `<pair>_<lever>` variants (the strategist unless named).

    python experiments/results/215_length_calibration/spec.py NAME > experiments/farm/NAME.json

Levers (each a config overlay on r95, no build):
- `u4`, `u5`: the Unity tip size, `alignment.tip_per_player` 4 or 5 (3 at r95);
- `dc4`: the Discord tip size, `alignment.discord_tip_per_player` 4 (3; a candidate lever);
- `s5`, `s55`: the deck length, `adversary.stage_cards` 5, 4, 3 or 5, 5, 3 (4, 4, 3: the round cap 12 or 13 at d0-d2).
NAME: s215-reach (the reachability check: spent fresh seeds 46432-46479, d0-d2, every lever), s215-a (screen 1:
spent fresh seeds 46240-46431, 192 seeds a pair: 576 games a cell and level, d0-d3).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]

U4 = ['alignment.tip_per_player=4']
U5 = ['alignment.tip_per_player=5']
DC4 = ['alignment.discord_tip_per_player=4']
S5 = ['adversary.stage_cards=5,4,3']
S55 = ['adversary.stage_cards=5,5,3']
DC5 = ['alignment.discord_tip_per_player=5']
S56 = ['adversary.stage_cards=5,6,3']  # stage I has only 5 cards: 6,5,3 is refused
P7 = ['alignment.purge_at=3,7,7']
LEVERS = {'base': [], 'u4': U4, 'u5': U5, 'dc4': DC4, 's5': S5, 's55': S55, 'u4s5': U4 + S5,
          's5dc4': S5 + DC4, 's55dc4': S55 + DC4,
          's56dc4': S56 + DC4, 's55dc5': S55 + DC5, 's55dc4p7': S55 + DC4 + P7, 's55dc4u4': S55 + DC4 + U4}


def spec(name, levers, diffs, games, note, agent='strategist'):
    variants = []
    for lever in levers:
        for p, s in PAIRS:
            variants.append({'name': f'{p}_{lever}' if lever != 'base' else p, 'set': s + LEVERS[lever]})
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
if name == 's215-reach':
    out = spec(name, ['base', 'u4', 'u5', 'dc4', 's5', 's55', 'u4s5'], [0, 1, 2], 48,
               '215 reachability: every lever, spent fresh seeds 46432-46479, d0-d2')
elif name == 's215-a':
    # screen 1: deck +1 x Discord tip 4 as a factorial, and deck +2 on top of dc4
    out = spec(name, ['base', 's5', 'dc4', 's5dc4', 's55dc4'], [0, 1, 2, 3], 192,
               '215 screen 1: deck x Discord tip, spent fresh seeds 46240-46431, d0-d3')
elif name == 's215-b':
    # screen 2: one change each on s55dc4 (screen 1's longest), paired with s215-a's cells on the same seeds:
    # a sixth stage-II card, Discord tip 5, the purge's last step at 7 Unity tips, Unity tip 4
    out = spec(name, ['s56dc4', 's55dc5', 's55dc4p7', 's55dc4u4'], [0, 1, 2, 3], 192,
               '215 screen 2: one change each on s55dc4, spent fresh seeds 46240-46431, d0-d3')
elif name == 's215-c':
    # after the fresh read (d0 10.12 rounds, d2 8.07): r96 (the baseline) and r96 with one stage-III card fewer
    # (`s552`: 5, 5, 2, a cap of 12), spent fresh seeds 46240-46431, d0-d2
    LEVERS['s552'] = ['adversary.stage_cards=5,5,2']
    out = spec(name, ['base', 's552'], [0, 1, 2], 192, '215 screen 3: r96 and a 12-card deck, spent fresh 46240-46431')
elif name in ('f215', 'f215g'):
    # the fresh read on r96 (s55dc4p7 in baseline.json), new seeds 46528-46623, d0-d4; f215 adds r95 put back by
    # overlay on the same seeds (`r95`); f215g is r96's greedy line
    LEVERS['r95'] = ['adversary.stage_cards=4,4,3', 'alignment.discord_tip_per_player=3', 'alignment.purge_at=3,6,6']
    if name == 'f215':
        out = spec(name, ['base', 'r95'], [0, 1, 2, 3, 4], 96, '215: the fresh read (strategist), r96 and r95')
    else:
        out = spec(name, ['base'], [0, 1, 2, 3, 4], 96, '215: the fresh read (greedy), r96', agent='greedy')
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
