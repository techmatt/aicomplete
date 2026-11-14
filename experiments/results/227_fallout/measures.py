"""227: Fallout's measures a game, by agent, over a difficulty slice (default d1-d2), for each farmed read given.

    python measures.py SPEC.json NAME=games.jsonl [NAME=games.jsonl ...] [--diffs 1,2]

Placed (new: `fallout.placed` and its sources; r98: `adversary.corruptions.land` + Teardown's burns, its only
sources), lifted (standard Lift, cards' `lift_fallout`, Network Cleanup's `remove_where`), the drip's Discord, the
green lift cards' plays, Network Cleanup's draws, Discord and Unity tips, rounds and wins.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
GREEN = ['field_sensor_grid', 'open_weights_release', 'shadow_broker_feed', 'shared_grid_accord',
         'fusion_startup_stake', 'pumped_hydro_storage']
SOURCES = ['threat1', 'threat2', 'project', 'discord', 'event', 'burn']


def rows(c):
    placed = c.get('fallout.placed', c.get('adversary.corruptions.land', 0) + c.get('board.burned.factory', 0))
    cleanup = c.get('fx.network_cleanup.land', 0)
    out = {
        'placed': placed,
        **{f'  from {s}': c.get(f'fallout.placed.{s}', 0) for s in SOURCES},
        'lifted (Lift)': c.get('board.std.lift', 0),
        'lifted (cards)': c.get('effects.land_lifted', 0),
        'Network Cleanup lifts': cleanup,
        'drip Discord': c.get('alignment.discord_from.blight', 0),
        'green lift cards played': sum(c.get(f'cards.played.{g}', 0) for g in GREEN),
        'Network Cleanup drawn': c.get('alignment.card.network_cleanup', 0),
        'Scorched Earth drawn': c.get('alignment.card.scorched_earth', 0),
        'Discord tips': c.get('alignment.tips_discord', 0),
        'Unity tips': c.get('alignment.tips_unity', 0),
    }
    # `fx.network_cleanup.land` is the token change (negative); its lifts are inside `effects.land_lifted`'s count?
    # No: `fallout.lifted` (227) counts every lift; r98 has none, so its total is the two counters.
    out['Network Cleanup lifts'] = -cleanup
    out['lifted (all)'] = c.get('fallout.lifted', out['lifted (Lift)'] + out['lifted (cards)'])
    return out


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    diffs = [1, 2]
    if '--diffs' in sys.argv:
        diffs = [int(x) for x in sys.argv[sys.argv.index('--diffs') + 1].split(',')]
        args = [a for a in args if a != sys.argv[sys.argv.index('--diffs') + 1]]
    spec = json.load(open(args[0], encoding='utf-8'))
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    table = {}
    names = []
    for item in args[1:]:
        name, path = item.split('=', 1)
        names.append(name)
        acc = defaultdict(lambda: defaultdict(float))
        n = defaultdict(int)
        for line in open(path, encoding='utf-8'):
            g = json.loads(line)
            if 'cell' not in g:
                continue
            _, d, a = cells[g['cell']]
            if d not in diffs:
                continue
            n[a] += 1
            for k, v in rows(g.get('counters', {})).items():
                acc[a][k] += v
            acc[a]['won %'] += 100 * g['won']
            acc[a]['rounds'] += g.get('rounds', 0)
        table[name] = {a: {k: v / n[a] for k, v in acc[a].items()} for a in acc}
    agents = spec['agents']
    keys = list(next(iter(next(iter(table.values())).values())).keys())
    print(f"d{','.join(map(str, diffs))}, a game | " + ' | '.join(f'{nm} {a}' for nm in names for a in agents))
    print('|---' * (1 + len(names) * len(agents)) + '|')
    for k in keys:
        print(f'| {k} | ' + ' | '.join(f'{table[nm][a].get(k, 0):.2f}' for nm in names for a in agents) + ' |')


if __name__ == '__main__':
    main()
