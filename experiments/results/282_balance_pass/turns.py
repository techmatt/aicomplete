"""Non-card turns a seat-round against card plays (282, done-when item 3).

python turns.py FILE [--agent judge|greedy|all] [--top N]

Counts, per seat-round by level: card plays (`cards.played.*`), standard actions (`board.std.*`), board powers
(`ai.power.*`, Growth options apart), building Actions (`cards.used.*`); then the top non-card turns a seat-game.
Growth (the board's one pick a round) is not a non-card turn (Matt, amendment 12831).
"""
import json
import sys
from collections import defaultdict

GROWTH = {'Buy In', 'Sprawl', 'Annex', 'Levy', 'Outpost', 'Accord', 'Field Survey', 'Peer Review', 'Field Station'}


def growth(key):
    return any(key.endswith('.' + g) for g in GROWTH)


def main():
    path = sys.argv[1]
    agent = sys.argv[sys.argv.index('--agent') + 1] if '--agent' in sys.argv else 'judge'
    top = int(sys.argv[sys.argv.index('--top') + 1]) if '--top' in sys.argv else 15
    seat_rounds = defaultdict(float)
    seat_games = defaultdict(float)
    tot = defaultdict(lambda: defaultdict(float))
    each = defaultdict(lambda: defaultdict(float))
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            a = 'judge' if g['cell'] % 2 == 0 else 'greedy'
            if agent != 'all' and a != agent:
                continue
            d = g['difficulty']
            seats = len(g['spent'])
            seat_rounds[d] += seats * g['rounds']
            seat_games[d] += seats
            for k, v in g['counters'].items():
                if not isinstance(v, (int, float)):
                    continue
                parts = k.split('.')
                if k.startswith('cards.played.') and len(parts) == 3:
                    tot['plays'][d] += v
                elif k.startswith('board.std.') and len(parts) == 3:
                    tot['standard'][d] += v
                    each['std ' + parts[2]][d] += v
                elif k.startswith('ai.power.') and len(parts) == 4 and not parts[3].startswith('r'):
                    if growth(k):
                        tot['growth'][d] += v
                    else:
                        tot['powers'][d] += v
                        each['power ' + parts[2] + ' ' + parts[3]][d] += v
                elif k.startswith('cards.used.') and len(parts) == 3:
                    tot['building actions'][d] += v
                    each['use ' + parts[2]][d] += v
    ds = sorted(seat_rounds)
    print('A seat-round', *[f'd{d}' for d in ds], sep='\t')
    for k in ['plays', 'standard', 'powers', 'building actions', 'growth']:
        print(k, *[f'{tot[k][d] / seat_rounds[d]:.2f}' for d in ds], sep='\t')
    nc = [sum(tot[k][d] for k in ['standard', 'powers', 'building actions']) / seat_rounds[d] for d in ds]
    print('non-card', *[f'{x:.2f}' for x in nc], sep='\t')
    print('non-card / plays', *[f'{x / (tot["plays"][d] / seat_rounds[d]):.2f}' for x, d in zip(nc, ds)], sep='\t')
    print()
    print('Top non-card turns, a seat-game (all levels)')
    allg = sum(seat_games.values())
    rows = sorted(each.items(), key=lambda kv: -sum(kv[1].values()))
    for k, v in rows[:top]:
        print(f'{sum(v.values()) / allg:.2f}', k, *[f'{v[d] / seat_games[d]:.2f}' for d in ds], sep='\t')


if __name__ == '__main__':
    main()
