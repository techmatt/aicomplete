"""275: resource use, the judge, pooled over a read: gained and spent a seat-round, the share spent, left at the end,
and each resource's top uses (games.jsonl's `start`, `gained`, `spent`, `uses`; experiment.rs's PEAKS order).

    python resources.py READ/games.jsonl [--agent strategist]
"""
import argparse
import collections
import json

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('games')
    ap.add_argument('--agent', default='strategist')
    a = ap.parse_args()
    gained, spent, left, start = (collections.Counter() for _ in range(4))
    uses = collections.defaultdict(collections.Counter)
    seat_rounds = 0
    seat_games = [0]
    with open(a.games, encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        cells = [ag for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties'] for ag in spec['agents']]
        for line in f:
            g = json.loads(line)
            if cells[g['cell']] != a.agent:
                continue
            for s, (st, ga, sp) in enumerate(zip(g['start'], g['gained'], g['spent'])):
                seat_rounds += g['rounds']
                seat_games[0] += 1
                for i, r in enumerate(PEAKS):
                    gained[r] += ga[i]
                    spent[r] += sp[i]
                    start[r] += st[i]
                    left[r] += st[i] + ga[i] - sp[i]
                for k, v in g['uses'][s].items():
                    kind = k.split(':')[1] if k.startswith('own:') else k.split(':')[0]
                    for i, r in enumerate(PEAKS):
                        if v[i]:
                            uses[r][kind] += v[i]
    n_seats = seat_rounds  # per seat-round
    print('| Resource | Gained a seat-round | Spent a seat-round | Spent of gained | Left at the end (a seat, a game) | Top uses |')
    print('|---|---|---|---|---|---|')
    for r in PEAKS:
        if r == 'card_tokens' or not gained[r]:
            continue
        tot = sum(uses[r].values()) or 1
        top = ', '.join(f'{k} {100 * v / tot:.0f}%' for k, v in uses[r].most_common(3))
        print(f'| {r} | {gained[r] / n_seats:.1f} | {spent[r] / n_seats:.1f} | {100 * spent[r] / gained[r]:.0f}% | '
              f'{left[r] / seat_games[0]:.1f} | {top} |')


main()
