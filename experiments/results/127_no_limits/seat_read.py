"""One seat-game in detail (127): its peaks, gains and spends, every source of a resource, and the cards it played
(from the action stream), with the round each was played.

    python experiments/results/127_no_limits/seat_read.py DIR SEED VARIANT AGENT SEAT [--res credits]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('seed')
    ap.add_argument('variant')
    ap.add_argument('agent')
    ap.add_argument('seat', type=int)
    ap.add_argument('--res', default='credits,energy,chips,ambassadors')
    a = ap.parse_args()
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    for g in lines:
        if str(g['seed']) != a.seed or labels[g['cell']] != f'{a.variant}/{a.agent}':
            continue
        s = a.seat
        print(f'seed {a.seed} {a.variant}/{a.agent} seat {s}: won {g["won"]}, {g["rounds"]} rounds, {g["reason"]}')
        print('peaks  ', dict(zip(PEAKS, g['peaks'][s])))
        print('gained ', dict(zip(PEAKS, g['gained'][s])))
        print('spent  ', dict(zip(PEAKS, g['spent'][s])))
        for r in a.res.split(','):
            i = PEAKS.index(r)
            src = sorted(((v[i], k) for k, v in g['sources'][s].items() if v[i]), reverse=True)
            print(f'\n{r} sources: ' + ', '.join(f'{k} {v}' for v, k in src[:25]))
            use = sorted(((v[i], k) for k, v in g['uses'][s].items() if v[i]), reverse=True)
            print(f'{r} uses: ' + ', '.join(f'{k} {v}' for v, k in use[:15]))
        plays = collections.Counter()
        order = []
        rnd = 0
        for act in g['actions']:
            if act.get('type') == 'keep' and act.get('seat') == s:
                pass
            if act.get('type') in ('play', 'play_card') and act.get('seat') == s:
                c = act.get('card')
                plays[c] += 1
                order.append(c)
        print(f'\n{len(order)} plays: ' + ', '.join(f'{c} x{n}' if n > 1 else c for c, n in plays.most_common()))
        types = collections.Counter(act.get('type') for act in g['actions'] if act.get('seat') == s)
        print('action types:', dict(types))
        return


if __name__ == '__main__':
    main()
