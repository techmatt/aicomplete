"""Before against after for a rules cut, from two farmed reads of the same seeds (292).

    python scripts/bench/cuts_read.py BEFORE.jsonl AFTER.jsonl

Each file is a `farm-seeds` out file (one game a line). Prints, by board pair and overall: games, wins, rounds a
game, energy income a seat a round (the income step: cities in production, the board and cards in upkeep), the
cities' share of it (hosted before r139, paid by cities after), energy gained from every source (income, bought,
one-time gains), Discord tips a game, threat cards rejected a round (r138's `threats.turned_rejected`; from r139,
cards turned up less cards dealt) and whether every game finished
(an outcome, replayed exactly). Information for a report, not a gate: nothing here is a paired test.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ENERGY = 2  # the resource's place in a game's `gained` rows


def pair(game):
    """The boards seated, from the build string ("builder:...|protector:...")."""
    return '+'.join(part.split(':')[0][0].upper() for part in game['build'].split('|'))


def read(path):
    games = []
    with open(path, encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line.startswith('{') and '"seed"' in line:
                games.append(json.loads(line))
    return games


def city_energy(c):
    return c.get('board.city_energy', 0) + c.get('board.city_energy_hosted', 0)


def income(game):
    """Energy the income step paid every seat: production's (cities) and upkeep's (board, cards)."""
    return sum(seat.get(k, [0] * (ENERGY + 1))[ENERGY] for seat in game['sources'] for k in ('phase:board.produce', 'phase:economy.upkeep'))


def rejected(c):
    if 'threats.turned' in c:
        return c['threats.turned'] - c.get('threats.dealt', 0)
    return c.get('threats.turned_rejected', 0)


def rows(games):
    by = defaultdict(list)
    for g in games:
        by[pair(g)].append(g)
        by['all'].append(g)
    out = {}
    for k, gs in by.items():
        n = len(gs)
        rounds = sum(g['rounds'] for g in gs)
        seat_rounds = sum(g['rounds'] * len(g['gained']) for g in gs)
        out[k] = {
            'games': n,
            'finished': sum(1 for g in gs if g.get('reason') and g.get('replay_ok', True)),
            'wins': sum(1 for g in gs if g['won']),
            'rounds': rounds / n,
            'income': sum(income(g) for g in gs) / seat_rounds,
            'energy': sum(sum(seat[ENERGY] for seat in g['gained']) for g in gs) / seat_rounds,
            'city_energy': sum(city_energy(g['counters']) for g in gs) / seat_rounds,
            'tips': sum(g['counters'].get('alignment.tips_discord', 0) for g in gs) / n,
            'rejected': sum(rejected(g['counters']) for g in gs) / rounds,
        }
    return out


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    if len(args) != 2:
        sys.exit(__doc__)
    before, after = rows(read(args[0])), rows(read(args[1]))
    keys = [k for k in sorted(before) if k != 'all'] + ['all']
    head = ['pair', 'games', 'finished', 'wins', 'rounds a game', 'energy income a seat a round', 'of it from cities',
            'energy gained, every source', 'Discord tips a game', 'threat cards rejected a round']
    print('| ' + ' | '.join(head) + ' |')
    print('|' + '---|' * len(head))
    for k in keys:
        b, a = before[k], after.get(k)
        if a is None:
            continue
        print(f"| {k} | {b['games']} → {a['games']} | {b['finished']} → {a['finished']} | {b['wins']} → {a['wins']} | "
              f"{b['rounds']:.2f} → {a['rounds']:.2f} | {b['income']:.2f} → {a['income']:.2f} | "
              f"{b['city_energy']:.2f} → {a['city_energy']:.2f} | {b['energy']:.2f} → {a['energy']:.2f} | {b['tips']:.2f} → {a['tips']:.2f} | "
              f"{b['rejected']:.2f} → {a['rejected']:.2f} |")


if __name__ == '__main__':
    main()
