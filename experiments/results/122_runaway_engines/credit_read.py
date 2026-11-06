"""Matt's credit check (122, message 5134): credits gained per game per seat (`gained`, rises after the limit),
credits lost at the limit (`economy.hold_lost.credits`), and per round, per agent and run.

Reads r49-r50 runs only: 127 (r51) removed the holding limits and the `economy.hold_lost*` counters.

    python experiments/results/122_runaway_engines/credit_read.py DIR... [--top 5]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--top', type=int, default=5)
    a = ap.parse_args()
    print('| run | agent | seats | credits gained a game (mean) | a round | max a game (seat, seed, rounds) | '
          'max a round | lost at the limit a game (mean / max) | production raises lost a game (mean / max) |')
    print('|---|---|---|---|---|---|---|---|---|')
    tops = []
    for d in a.dirs:
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        rows = collections.defaultdict(list)
        lost = collections.defaultdict(list)
        for g in lines:
            variant, agent = labels[g['cell']].split('/')[:2]
            pair, _, lever = variant.partition('-')
            agent = f'{agent} {lever}' if lever else agent
            lost[agent].append((g['counters'].get('economy.hold_lost.credits', 0),
                                g['counters'].get('economy.production_lost.credits', 0)))
            for s, gained in enumerate(g['gained']):
                c = gained[0]
                rows[agent].append((c, c / g['rounds'], BOARD[pair[s]], g['seed'], g['rounds']))
                tops.append((c, os.path.basename(d), agent, BOARD[pair[s]], g['seed'], g['rounds']))
        for agent, v in sorted(rows.items()):
            n = len(v)
            top = max(v)
            l = lost[agent]
            print(f'| {os.path.basename(d)} | {agent} | {n} | {sum(x[0] for x in v) / n:.0f} | '
                  f'{sum(x[1] for x in v) / n:.1f} | {top[0]} ({top[2]}, {top[3]}, {top[4]}) | '
                  f'{max(x[1] for x in v):.0f} | {sum(x[0] for x in l) / len(l):.1f} / {max(x[0] for x in l)} | '
                  f'{sum(x[1] for x in l) / len(l):.1f} / {max(x[1] for x in l)} |')
    tops.sort(reverse=True)
    print(f'\nTop {a.top} seats by credits gained:\n')
    for t in tops[:a.top]:
        print(f'- {t[0]} credits: {t[1]} {t[2]} {t[3]}, seed {t[4]}, {t[5]} rounds')


if __name__ == '__main__':
    main()
