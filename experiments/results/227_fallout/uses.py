"""227: per-game counters, base against new, all agents and difficulties: the green lift cards (played), Network
Cleanup (drawn, tokens lifted), Lift Fallout and the drip.

    python uses.py BASE.jsonl NEW.jsonl
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KEYS = [f'cards.played.{c}' for c in ['field_sensor_grid', 'open_weights_release', 'shadow_broker_feed',
                                       'shared_grid_accord', 'fusion_startup_stake', 'pumped_hydro_storage']]
KEYS += ['alignment.card.network_cleanup', 'fx.network_cleanup.land', 'board.std.lift', 'effects.land_lifted',
         'fallout.lifted', 'fallout.placed', 'alignment.discord_from.blight', 'alignment.card.scorched_earth',
         'effects.fallout']


def read(path):
    acc, n = defaultdict(float), 0
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if 'cell' not in g:
            continue
        n += 1
        for k in KEYS:
            acc[k] += g.get('counters', {}).get(k, 0)
    return {k: v / n for k, v in acc.items()}, n


def main():
    (b, nb), (w, nw) = read(sys.argv[1]), read(sys.argv[2])
    print(f'| counter, a game | base ({nb} games) | new ({nw}) |')
    print('|---|---|---|')
    for k in KEYS:
        print(f'| {k} | {b.get(k, 0):.3f} | {w.get(k, 0):.3f} |')


if __name__ == '__main__':
    main()
