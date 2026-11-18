"""273: how each resource is spent and whether it binds, from a read's games.jsonl (all seats, levels pooled).

    python experiments/results/273_smaller_numbers/resource_use.py GAMES.jsonl [--agent strategist]

Per resource: gained and spent a seat-round, what a seat holds at its peak and left at the end (`margin.unspent`,
credits / chips / energy / antimatter / ambassadors), the share of gains spent, and the top spends by kind (a use
key's second part: play, keep, pay (a threat counter), std (a standard action), power, use (an Action), ...).
"""
import argparse
import collections
import json

RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
UNSPENT = ['credits', 'chips', 'energy', 'antimatter', 'ambassadors']
AGENTS = ('strategist', 'greedy')


def kind(key):
    parts = key.split(':')
    return parts[1] if len(parts) > 1 else parts[0]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('games')
    ap.add_argument('--agent', default='strategist')
    a = ap.parse_args()
    gained = collections.Counter()
    spent = collections.Counter()
    peak = collections.Counter()
    left = collections.Counter()
    uses = collections.defaultdict(collections.Counter)
    seat_rounds = seats = 0
    for line in open(a.games, encoding='utf-8'):
        g = json.loads(line)
        if 'gained' not in g or AGENTS[g['cell'] % 2] != a.agent:
            continue
        n = len(g['gained'])
        seats += n
        seat_rounds += n * g['rounds']
        for s in range(n):
            for i, r in enumerate(RES):
                gained[r] += g['gained'][s][i]
                spent[r] += g['spent'][s][i]
                peak[r] += g['peaks'][s][i]
            for i, r in enumerate(UNSPENT):
                left[r] += g['margin']['unspent'][s][i]
            for key, row in g['uses'][s].items():
                for i, r in enumerate(RES):
                    if row[i]:
                        uses[r][kind(key)] += row[i]
    print(f'{a.agent}: {seats} seats, {seat_rounds} seat-rounds')
    print('| Resource | gained / seat-round | spent / seat-round | spent ÷ gained | peak held (mean) | left at end '
          '(mean) | spent on (share) |')
    print('|---|---|---|---|---|---|---|')
    for r in RES:
        if not gained[r] and not spent[r]:
            continue
        tot = sum(uses[r].values()) or 1
        top = ', '.join(f'{k} {v / tot:.0%}' for k, v in uses[r].most_common(4))
        lf = f'{left[r] / seats:.1f}' if r in UNSPENT else '-'
        print(f'| {r} | {gained[r] / seat_rounds:.2f} | {spent[r] / seat_rounds:.2f} | '
              f'{spent[r] / max(gained[r], 1):.2f} | {peak[r] / seats:.1f} | {lf} | {top} |')


if __name__ == '__main__':
    main()
