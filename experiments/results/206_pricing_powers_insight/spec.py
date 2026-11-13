"""206: the item's specs, cells as `<pair>_<lever>` variants on one binary.

    python experiments/results/206_pricing_powers_insight/spec.py NAME > experiments/farm/NAME.json

NAME: s206-screen (dev seeds 36800-36831; base = the prompt's and amendments' numbers), s206-old (r90's base, run by
the r90 binary in out/r90), f206-strategist / f206-greedy (the fresh read, base only), c206-* (confirm reads).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
LEVERS = {
    'base': [],
    # amendment 9208 / 9212: the station's price, and a factory of your own for chips
    'st2': ['board.station_chips=2'],
    'sta': ['board.station_ambassadors=1'],
    'fac': ['board.factory_build_chips=3'],
    # amendment 9208: the Claim / Liberate table's factory at 3 (5 / 3 / 3)
    'cf3': ['board.claim_factory=3'],
    # the Harden project back (3 credits), for node protection
    'hb': ['board.harden_credits=3'],
    # the troop hedge off
    'nt': ['board.troop_credits=0'],
    # amendment 9257: r89's troop_steps 1 back to 2
    'ts2': ['board.troop_steps=2'],
    # Place a node twice a round; its price at 4 credits; Remove a troop twice a round
    'n2': ['board.node_per_round=2'],
    'nc4': ['board.node_credits=4'],
    't2': ['board.troop_per_round=2'],
    # dials, last (fresh read: d1 81.9 over 70-80, d3 36.8 under 40-50): compute income at d1 and d3
    'k12': ['adversary.compute_income=0,12,17,25,30'],
    'k16': ['adversary.compute_income=0,16,17,25,30'],
    'k21': ['adversary.compute_income=0,8,17,21,30'],
    'k18': ['adversary.compute_income=0,8,17,18,30'],
    # insight binding, 199's next lever: a 5-card draft hand, with the early-heavy track t4 (199's screen 6)
    'deal5': ['cards.deal=5'],
    't4deal5': ['cards.deal=5',
                'ais.builder.play_track.0.insight=7', 'ais.builder.play_track.2.insight=8',
                'ais.builder.play_track.4.insight=null', 'ais.builder.play_track.5.insight=9',
                'ais.builder.play_track.6.insight=10',
                'ais.protector.play_track.0.insight=7', 'ais.protector.play_track.2.insight=8',
                'ais.protector.play_track.4.insight=null', 'ais.protector.play_track.5.insight=null',
                'ais.protector.play_track.6.insight=9',
                'ais.researcher.play_track.0.insight=6', 'ais.researcher.play_track.1.insight=null',
                'ais.researcher.play_track.3.insight=7', 'ais.researcher.play_track.4.insight=null',
                'ais.researcher.play_track.6.insight=8'],
    # amendment 9261: stage-I Musters back (one card, two cards), each Muster 1 a country as Blitzkrieg's
    'm1': ['adversary.mil_s1_war_drums.standing=[{"op":"rally","site":"city"},{"op":"troops","count":1,"each_country":true}]'],
    'm2': ['adversary.mil_s1_war_drums.standing=[{"op":"rally","site":"city"},{"op":"troops","count":1,"each_country":true}]',
           'adversary.mil_s1_probing_strikes.standing=[{"op":"rally","site":"coastal_city"},{"op":"troops","count":1,"each_country":true}]'],
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
if name == 's206-screen':
    out = spec(name, ['base', 'st2', 'sta', 'fac', 'cf3', 'hb', 'nt'], ALL, 'strategist', 32,
               '206: screen on the dev seeds 36800-36831')
elif name == 's206-old':
    out = spec(name, ['base', 'ts2'], ALL, 'strategist', 32, '206: r90 (e188cbf) on the dev seeds, the r90 binary')
elif name.startswith('s206-'):
    levers = name.split('-')[1:]
    out = spec(name, ['base'] + [x for x in levers if x != 'x'], ALL, 'strategist', 32, '206: a screen on the dev seeds')
elif name == 'd206-d1':
    # the dials on the spent fresh seeds 45631-45726 (204's route), the confirm on the next free ones
    out = spec(name, ['k12', 'k16'], [1], 'strategist', 96, '206: d1 compute income dial (base: f206-strategist)')
elif name == 'd206-d3':
    out = spec(name, ['k21', 'k18'], [3], 'strategist', 96, '206: d3 compute income dial (base: f206-strategist)')
elif name == 'c206-strategist':
    # the dials' confirm on the next free seeds 45727-45822, d1 and d3 (r90's side on its own binary)
    out = spec(name, ['base'], [1, 3], 'strategist', 96, '206: the d1 and d3 dial confirm, r91 against r90')
elif name in ('f206-strategist', 'f206-greedy'):
    out = spec(name, ['base'], ALL, name.split('-')[1], 96, '206: the fresh read, r91 against r90')
else:
    out = spec(name, ['base'], ALL, 'strategist', 96, '206: a confirm read')
print(json.dumps(out, indent=2))
