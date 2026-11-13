"""212: the item's specs, cells as `<pair>_<lever>` variants on one binary (the strategist, 211's defaults).

    python experiments/results/212_tags_node_powers/spec.py NAME > experiments/farm/NAME.json

NAME: s212-smoke (16 dev seeds from 36800, d2), s212-screen (dev seeds 36800-36831, d0-d4: base and the levers; run
before the pick, when base was the plain rider and Peer Review at 3 cubes), s212-r92 / f212-r92 (the old side's base
cells, run from `out/r92src` with r92's binary), f212 (the fresh read: the shipped base and the prompt's numbers), or
s212-<lever>[+<lever>]-... (a screen of the named levers beside base).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def rider(board, k, spot, then, fresh=False):
    r = {'op': 'if_node_next', 'spots': [spot], 'then': then}
    if fresh:
        r['fresh'] = True
    eff = [{'op': 'place_node', 'count': 1}, r]
    return f'ais.{board}.powers.{k}.effects=' + json.dumps(eff, separators=(',', ':'))


CHIP = [{'op': 'gain', 'resource': 'chips', 'amount': 1}]
INS = [{'op': 'gain', 'resource': 'insight', 'amount': 1}]
AMB = [{'op': 'ambassador_to_spot'}]
LEVERS = {
    'base': [],
    # Peer Review at 2 or 1 research cubes (the prompt's numbers keep 3)
    'pr2': ['ais.researcher.powers.0.cost=[["research_cubes",2]]'],
    'pr1': ['ais.researcher.powers.0.cost=[["research_cubes",1]]'],
    # the riders on a fresh spot: one none of your other nodes is on or next to (the smoke met "on or next to" 99-100%)
    'fresh': [rider('builder', 1, 'factory', CHIP, True), rider('protector', 1, 'city', AMB, True),
              rider('researcher', 0, 'station', INS, True)],
    # the prompt's numbers on the shipped base (the fresh rider and Peer Review at 1 cube since message 9565): the plain
    # "on or next to" rider and Peer Review at 3 cubes
    'prompt': [rider('builder', 1, 'factory', CHIP), rider('protector', 1, 'city', AMB), rider('researcher', 0, 'station', INS),
               'ais.researcher.powers.0.cost=[["research_cubes",3]]'],
    # Stand Down: 1 agent or 1 troop in one hex in reach (the base: 1 troop in reach)
    'sd': ['ais.protector.powers.2.effects=[{"op":"damage","ambassadors":1,"troops":1}]'],
    # the riders off: each node power's bonus unconditional again (r92's powers), tags and cards as 212
    'flat': [
        'ais.builder.powers.1.effects=[{"op":"place_node","count":1},{"op":"gain","resource":"chips","amount":1}]',
        'ais.protector.powers.1.effects=[{"op":"place_node","count":1},{"op":"gain_ambassadors","count":1}]',
        'ais.researcher.powers.0.effects=[{"op":"place_node","count":1},{"op":"gain","resource":"insight","amount":1}]',
    ],
    # bigger riders: 2 chips, 2 insight
    'big': [
        rider('builder', 1, 'factory', [{'op': 'gain', 'resource': 'chips', 'amount': 2}]),
        rider('researcher', 0, 'station', [{'op': 'gain', 'resource': 'insight', 'amount': 2}]),
    ],
}


def lever_set(lever):
    """A lever, or levers joined by `+` (a factorial cell: `fresh+pr1`)."""
    return [e for part in lever.split('+') for e in LEVERS[part]]


def spec(name, levers, diffs, agent, games, note):
    variants = []
    for lever in levers:
        for p, s in PAIRS:
            variants.append({'name': f'{p}_{lever}' if lever != 'base' else p, 'set': s + lever_set(lever)})
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
ALL = [0, 1, 2, 3, 4]
if name == 's212-smoke':
    out = spec(name, list(LEVERS), [2], 'strategist', 16, '212: engine smoke, dev seeds 36800')
elif name == 's212-screen':
    out = spec(name, ['base', 'fresh', 'pr2', 'pr1', 'sd', 'flat'], ALL, 'strategist', 32,
               '212: screen on the dev seeds 36800-36831')
elif name == 's212-r92':
    out = spec(name, ['base'], ALL, 'strategist', 32, "212: the old side's screen cells (r92, dev seeds 36800-36831)")
elif name == 'f212':
    out = spec(name, ['base', 'prompt'], ALL, 'strategist', 96,
               "212: the fresh read, r93 against r92 (seeds 46240-46335), and the prompt's numbers")
elif name == 'f212-r92':
    out = spec(name, ['base'], ALL, 'strategist', 96, "212: the fresh read's old side (r92, out/r92src)")
elif name.startswith('s212-'):
    levers = [x for x in name.split('-')[1:] if x != 'base']
    out = spec(name, ['base'] + levers, ALL, 'strategist', 32, '212: a screen on the dev seeds')
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
