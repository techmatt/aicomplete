"""271: plays and draws a seat a round from `aic-sim run --trace` output(s). Run from anywhere.

    python trace_plays.py TRACE.out [TRACE.out ...]

One trace: a table per round and seat (plays, cards drawn off the decks outside the draft, building Actions and
board powers used). Several: the same, averaged over the games that reached each round.
"""
import collections
import json
import sys


def read(path):
    rnd = 0
    rows = collections.defaultdict(collections.Counter)
    for line in open(path, encoding='utf-8'):
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if 'action' in d:
            a = d['action']
            if a['type'] in ('play', 'use', 'power'):
                rows[(rnd, a['seat'])][a['type']] += 1
            continue
        e = d.get('event') or {}
        if e.get('type') == 'round_started':
            rnd = e['round']
        elif e.get('type') == 'card_moved' and e['from'].startswith('cards.deck.') and e['to'].startswith('cards.hand.'):
            rows[(rnd, int(e['to'].rsplit('.', 1)[1]))]['draw'] += 1
    return rows


def main():
    games = [read(p) for p in sys.argv[1:]]
    seats = sorted({s for g in games for (_, s) in g})
    rounds = sorted({r for g in games for (r, _) in g})
    head = ['Round'] + [f'seat {s} {k}' for s in seats for k in ('plays', 'draws', 'Actions', 'powers')]
    print('| ' + ' | '.join(head) + ' |')
    print('|' + '---|' * len(head))
    tot = collections.Counter()
    for r in rounds:
        live = [g for g in games if any(rr == r for (rr, _) in g)]
        cells = [str(r)]
        for s in seats:
            for k in ('play', 'draw', 'use', 'power'):
                v = sum(g[(r, s)][k] for g in live) / len(live)
                tot[(s, k)] += sum(g[(r, s)][k] for g in live)
                cells.append(f'{v:.1f}' if len(games) > 1 else str(int(v)))
        print('| ' + ' | '.join(cells) + ' |')
    n = len(games)
    cells = ['**game**'] + [f'**{tot[(s, k)] / n:.1f}**' for s in seats for k in ('play', 'draw', 'use', 'power')]
    print('| ' + ' | '.join(cells) + ' |')
    nr = sum(len({r for (r, _) in g if r}) for g in games) / n
    print(f'\n{n} game(s), {nr:.1f} rounds a game; plays a seat a round: ' + ', '.join(
        f'seat {s} {tot[(s, "play")] / n / nr:.1f}' for s in seats) + '; draws a seat a round: ' + ', '.join(
        f'seat {s} {tot[(s, "draw")] / n / nr:.1f}' for s in seats))


if __name__ == '__main__':
    main()
