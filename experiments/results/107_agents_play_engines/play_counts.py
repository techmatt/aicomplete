"""107: offered / kept / played per game for the engine cards, before and after, on the same seeds.

    python experiments/results/107_agents_play_engines/play_counts.py BEFORE_DIR AFTER_DIR [--agent strategist|greedy] [--md]

Each dir is an `aic-sim experiment` output (its cards.csv). Per game = the cells' counts summed over the three
pairs / (3 x games). The target set is 106's list (strategist played under 0.05 a game, r42.md *Play counts*),
the Capital engine and the six tier-2 every-tag triggers. Kept counts the opening deal and draws.
"""
import csv
import json
import subprocess
import sys

AGENTS = {'strategist': 'engine:strategy=1', 'greedy': 'greedy'}

# 106's 29 (play-counts.md, strategist played under 0.05), then the Capital engine and the six triggers.
TARGET = [
    'citation_engine', 'foundation_model_campus', 'security_contractor_hq', 'orbital_mass_driver',
    'dyson_swarm_tranche', 'continental_supergrid', 'tidal_barrage', 'smart_grid_backbone',
    'dispatch_control_room', 'conglomerate_headquarters', 'retained_earnings', 'recursive_self_improvement',
    'positron_storage_ring', 'power_to_the_people', 'hyperscale_botnet', 'payroll_automation',
]
CAPITAL = ['retained_earnings', 'materials_hedge_desk', 'vendor_financing', 'liquidity_event', 'collateralized_grid']
TRIGGERS = ['interdisciplinary_institute', 'conglomerate_headquarters', 'command_and_control_grid',
            'smart_grid_backbone', 'orbital_embassy', 'agency_directorate']


def read(d, agent, games):
    out = {}
    for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8')):
        if r['agent'] != agent:
            continue
        o = out.setdefault(r['card'], [0, 0, 0])
        for i, k in enumerate(('offered', 'kept', 'played')):
            o[i] += int(r[k])
    return {c: [x / games for x in v] for c, v in out.items()}


def games_in(d):
    n = 0
    for line in open(f'{d}/games.jsonl', encoding='utf-8'):
        n += 1
    return n


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    agent = 'strategist'
    if '--agent' in sys.argv:
        agent = sys.argv[sys.argv.index('--agent') + 1]
        args = [a for a in args if a != agent]
    before, after = args[:2]
    spec = AGENTS[agent]
    # games a cell x 3 pairs, per agent: the jsonl holds every agent's games.
    gb = games_in(before) / len(AGENTS)
    ga = games_in(after) / len(AGENTS)
    b = read(before, spec, gb)
    a = read(after, spec, ga)
    cards = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    # The full 29: every new or changed card the 106 table lists under 0.05 (computed from the before dir).
    old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', '97eff1f:content/cards.json'],
                                                          capture_output=True, encoding='utf-8', check=True).stdout)}
    changed = {i for i in cards if old.get(i) != cards[i]}
    low = sorted((c for c, v in b.items() if v[2] < 0.05 and c in changed), key=lambda c: cards[c]['name'])
    ids = list(dict.fromkeys(TARGET + CAPITAL + TRIGGERS + low))
    print(f'{agent}: per game, offered / kept / played, before -> after ({gb:.0f} and {ga:.0f} games)\n')
    print('| card | group | before | after |')
    print('|---|---|---|---|')
    for c in ids:
        grp = ', '.join(g for g, s in (('106 list', TARGET), ('Capital', CAPITAL), ('trigger', TRIGGERS),
                                       ('under 0.05', low)) if c in s)
        f = lambda v: ' / '.join(f'{x:.2f}' for x in v) if v else '0 / 0 / 0'
        print(f"| {cards[c]['name']} | {grp} | {f(b.get(c))} | {f(a.get(c))} |")
    print(f'\nBefore, every card played under 0.05 a game: {len(low)}')
    print(', '.join(cards[c]['name'] for c in low))
    moved = [c for c in low if a.get(c, [0, 0, 0])[2] >= 0.05]
    print(f'\nOf those, played 0.05 a game or more after: {len(moved)}')


main()
