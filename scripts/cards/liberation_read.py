"""080: the liberation read. Per cell (config x variant x agent): win rate, cities corrupted and liberated
per game (by a card or the fallback), each liberate card's kept and played rate per game, Grant uses and
Wall stops per game.

    python scripts/cards/liberation_read.py DIR [DIR...] [--cards]

DIR is an `aic-sim experiment` output directory (games.jsonl and results.json).
"""
import argparse
import json
import math
import os
import sys
from collections import defaultdict

LIBERATE = ['nuclear_option', 'power_to_the_people', 'hostile_buyout', 'forensic_audit', 'root_access',
            'peacekeeping_mission', 'popular_uprising', 'great_liberation']


def load(dirs):
    cells = defaultdict(list)
    for d in dirs:
        names = {}
        for c in json.load(open(os.path.join(d, 'results.json'), encoding='utf-8'))['cells']:
            names[c['cell']] = (os.path.basename(c['config']).replace('.json', ''), c['variant'], c['agent'])
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                if 'cell' in g:
                    cells[names[g['cell']]].append(g)
    return cells


def per(gs, key):
    return sum(g['counters'].get(key, 0) for g in gs) / len(gs)


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--cards', action='store_true', help='kept and played per game for each liberate card')
    a = ap.parse_args()
    cells = load(a.dirs)
    print('| config | pair | agent | games | win % | ±SE | rounds | corrupted / game (games with one) | '
          'liberated: all (card / fallback) | per won game | games with a liberation | Grant / game | '
          'Wall strikes stopped / game | troops landed by Wall pressure |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for (cfg, var, agent), gs in sorted(cells.items()):
        n = len(gs)
        p = sum(g['won'] for g in gs) / n
        se = math.sqrt(p * (1 - p) / n) * 100
        freed = [g['counters'].get('board.cities_freed', 0) for g in gs]
        won = [g for g in gs if g['won']]
        corrupting = sum(1 for g in gs if g['counters'].get('adversary.corruptions', 0)) / n * 100
        print(f"| {cfg} | {var} | {agent} | {n} | {100 * p:.1f} | {se:.1f} | {sum(g['rounds'] for g in gs) / n:.2f} | "
              f"{per(gs, 'adversary.corruptions'):.2f} ({corrupting:.0f}%) | {per(gs, 'board.cities_freed'):.2f} ({per(gs, 'board.liberated_card'):.2f} / "
              f"{per(gs, 'board.liberated_fallback'):.2f}) | {per(won, 'board.cities_freed') if won else 0:.2f} | "
              f"{sum(1 for x in freed if x) / n * 100:.0f}% | "
              f"{per(gs, 'ai.power.researcher.3'):.2f} | {per(gs, 'board.wall_strikes'):.2f} | "
              f"{per(gs, 'adversary.wall_troops'):.2f} |")
    if a.cards:
        print()
        print('| config | pair | agent | ' + ' | '.join(LIBERATE) + ' |')
        print('|---|---|---|' + '---|' * len(LIBERATE))
        for (cfg, var, agent), gs in sorted(cells.items()):
            row = []
            for c in LIBERATE:
                k, pl, u = per(gs, f'cards.kept.{c}'), per(gs, f'cards.played.{c}'), per(gs, f'cards.used.{c}')
                row.append(f'{k:.2f} / {pl:.2f}' + (f' ({u:.2f} used)' if u else ''))
            print(f'| {cfg} | {var} | {agent} | ' + ' | '.join(row) + ' |')


if __name__ == '__main__':
    main()
