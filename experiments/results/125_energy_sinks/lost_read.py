"""125: where energy is lost at the limit: a game, by round (`economy.hold_lost.energy.rNN`) and by the phase it
happened in (`economy.hold_lost_phase.<phase index>.energy`, named from `aic-sim`'s phase list order), and the part
megaproject completion payouts lost (`board.completion_lost.energy`).

    python experiments/results/125_energy_sinks/lost_read.py DIR [--res energy]
"""
import argparse
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

# The round's phases in order (docs/rules/economy.md, *The round*), g.phase 0-based.
PHASES = ['alignment.sway', 'adversary.build', 'adversary.spread', 'adversary.corrupt', 'board.march', 'threats.deal',
          'adversary.reveal', 'cards.tier', 'cards.deal', 'cards.draft', 'cards.keep', 'actions', 'board.bases',
          'board.war', 'board.produce', 'economy.upkeep', 'threats.execute', 'board.cleanup']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--res', default='energy')
    a = ap.parse_args()
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    n = collections.Counter()
    by_round = collections.defaultdict(collections.Counter)
    by_phase = collections.defaultdict(collections.Counter)
    done = collections.Counter()
    for g in lines:
        agent = labels[g['cell']].split('/')[-1]
        n[agent] += 1
        done[agent] += g['counters'].get(f'board.completion_lost.{a.res}', 0)
        for k, v in g['counters'].items():
            if k.startswith(f'economy.hold_lost.{a.res}.r'):
                by_round[agent][int(k.rsplit('.r', 1)[1])] += v
            elif k.startswith('economy.hold_lost_phase.') and k.endswith('.' + a.res):
                i = int(k.split('.')[2])
                by_phase[agent][PHASES[i] if i < len(PHASES) else str(i)] += v
    for agent in sorted(n):
        rs = sorted(by_round[agent])
        print(f'\n### {agent}: {a.res} lost at the limit a game ({n[agent]} games)\n')
        print('By round: ' + ', '.join(f'r{r} {by_round[agent][r] / n[agent]:.2f}' for r in rs))
        print('By phase: ' + ', '.join(f'{p} {v / n[agent]:.2f}' for p, v in by_phase[agent].most_common()))
        print(f'Of it, megaproject completion payouts (`board.completion_lost`): {done[agent] / n[agent]:.2f}')


if __name__ == '__main__':
    main()
