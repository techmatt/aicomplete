"""130: for a board's flagged and unflagged seat-games, the energy peak against the most energy the seat held at the
end of any action phase (`unspent.<seat>.energy.rNN`), and how many seat-games would be flagged on that held figure by
the same rule (rise over the start > 3 x the income a round and > 10).

    python experiments/results/130_late_energy_sinks/peak_vs_held.py DIR... [--agent strategist] [--board Builder]
"""
import argparse
import os
import statistics
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '128_draft_and_price'))
import read128 as R  # noqa: E402

E = 2


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--board', default='Builder')
    a = ap.parse_args()
    print('| run | seat-games | flagged (peak) | flagged on held after actions | peak, flagged | most held after '
          'actions, flagged |')
    print('|---|---|---|---|---|---|')
    for d in a.dirs:
        rows = []
        tag = d
        for label, pair, g in R.load(d):
            if not label.endswith('/' + a.agent):
                continue
            tag = label.split('/')[0]
            for s in range(len(g['peaks'])):
                if R.BOARD[pair[s]] != a.board:
                    continue
                start = (g.get('start') or [[0] * 8] * 2)[s][E]
                inc = g['gained'][s][E] / max(g['rounds'], 1)
                peak = g['peaks'][s][E]
                held = max([v for k, v in g['counters'].items() if k.startswith(f'unspent.{s}.energy.r')] or [0])
                rows.append((inc > 0 and peak - start > 3 * inc and peak - start > 10,
                             inc > 0 and held - start > 3 * inc and held - start > 10, peak, held))
        fl = [r for r in rows if r[0]]
        print(f'| {tag} | {len(rows)} | {len(fl)} | {sum(r[1] for r in rows)} | '
              f'{statistics.mean(r[2] for r in fl):.1f} | {statistics.mean(r[3] for r in fl):.1f} |')


if __name__ == '__main__':
    main()
