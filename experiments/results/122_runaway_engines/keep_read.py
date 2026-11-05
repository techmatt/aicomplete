"""Matt's insight check (122, message 5121): does a seat that banks insight keep every pick and take tier 2?

    python experiments/results/122_runaway_engines/keep_read.py DIR [--board Researcher] [--lever caps]

Per seat of the board, from each game's actions: drafted picks after the opening (`draft`), kept after the opening
(`keep` once the first draft is made), the keep rate, the tier-2 share of its tier choices (`tier`), and its insight
peak (`peaks`), split into seats flagged by runaway_read's peak rule (over 3x a round's insight and over 10) and the rest.
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
INSIGHT = 1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--board', default='Researcher')
    ap.add_argument('--lever', default=None, help='only variants named <pair>-<lever> (base: bare pairs)')
    a = ap.parse_args()
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    rows = collections.defaultdict(list)
    for g in lines:
        variant, agent = labels[g['cell']].split('/')[:2]
        pair, _, lever = variant.partition('-')
        if a.lever is not None and (lever or 'base') != a.lever:
            continue
        for s, ch in enumerate(pair):
            if BOARD[ch] != a.board:
                continue
            drafted = kept = t1 = t2 = 0
            drafting = False
            for act in g['actions']:
                if act.get('seat') != s:
                    continue
                t = act['type']
                if t == 'draft':
                    drafting = True
                    drafted += 1
                elif t == 'keep' and drafting:
                    kept += 1
                elif t == 'tier':
                    if act['tier'] == 2:
                        t2 += 1
                    else:
                        t1 += 1
            peak = g['peaks'][s][INSIGHT]
            inc = g['gained'][s][INSIGHT] / g['rounds']
            flagged = inc > 0 and peak > 10 and peak > 3 * inc
            rows[(agent, flagged)].append((drafted, kept, t1, t2, peak, g['rounds']))
    print(f'| agent | {a.board} seats | n | insight peak | drafted | kept | keep rate | tier-2 share | rounds |')
    print('|---|---|---|---|---|---|---|---|---|')
    for (agent, flagged), v in sorted(rows.items()):
        n = len(v)
        d = sum(x[0] for x in v)
        k = sum(x[1] for x in v)
        t1 = sum(x[2] for x in v)
        t2 = sum(x[3] for x in v)
        print(f'| {agent} | {"flagged" if flagged else "not flagged"} | {n} | {sum(x[4] for x in v) / n:.1f} | '
              f'{d / n:.1f} | {k / n:.1f} | {k / d if d else 0:.0%} | {t2 / (t1 + t2) if t1 + t2 else 0:.0%} | '
              f'{sum(x[5] for x in v) / n:.2f} |')


if __name__ == '__main__':
    main()
