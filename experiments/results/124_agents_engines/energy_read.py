"""124: where energy is lost at the limit: per agent cell, energy lost a game by round (`economy.hold_lost.energy.rNN`),
and the energy gained per seat-game by source (the `sources` field, 124), top N.

Reads r49-r50 runs only: 127 (r51) removed the holding limits and the `economy.hold_lost*` counters.

    python experiments/results/124_agents_engines/energy_read.py DIR [--agent LABEL] [--top 8] [--res energy]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent')
    ap.add_argument('--top', type=int, default=8)
    ap.add_argument('--res', default='energy')
    a = ap.parse_args()
    ri = PEAKS.index(a.res)
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    by_round = collections.defaultdict(lambda: collections.defaultdict(float))
    games = collections.Counter()
    src = collections.defaultdict(lambda: collections.defaultdict(float))
    seats = collections.Counter()
    for g in lines:
        pair, agent = labels[g['cell']].split('/')[-2:]
        if a.agent and agent != a.agent:
            continue
        games[agent] += 1
        for k, v in g['counters'].items():
            if k.startswith(f'economy.hold_lost.{a.res}.r'):
                by_round[agent][int(k.rsplit('.r', 1)[1])] += v
        for s, ch in enumerate(pair[:2]):
            b = BOARD[ch]
            seats[(agent, b)] += 1
            for label, gains in g.get('sources', [{}] * 2)[s].items():
                src[(agent, b)][label] += gains[ri]
    for agent in sorted(games):
        n = games[agent]
        rs = sorted(by_round[agent])
        print(f'\n### {agent}: {a.res} lost at the limit a game by round ({n} games)\n')
        print('| ' + ' | '.join(f'r{r}' for r in rs) + ' |')
        print('|' + '---|' * len(rs))
        print('| ' + ' | '.join(f'{by_round[agent][r] / n:.2f}' for r in rs) + ' |')
        for b in ('Builder', 'Protector', 'Researcher'):
            k = (agent, b)
            if not seats[k]:
                continue
            top = sorted(src[k].items(), key=lambda x: -x[1])[:a.top]
            total = sum(src[k].values()) / seats[k]
            print(f'\n{b} ({seats[k]} seat-games), {a.res} gained a game {total:.1f}: ' +
                  '; '.join(f'{lab} {v / seats[k]:.2f}' for lab, v in top))


if __name__ == '__main__':
    main()
