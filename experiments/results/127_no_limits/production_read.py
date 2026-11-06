"""127: what a seat's credit production was made of at the end, from the cards it played (the action stream): flat
`production` raises, per-tag raises at its tag count when played (programs count their own tags, approximated by
the tags of the cards played before and with it), and income passives on buildings. Per agent, the seat-games over
`--gained` credits, against the rest.

    python experiments/results/127_no_limits/production_read.py DIR [--agent strategist] [--gained 600] [--top 12]
"""
import argparse
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--gained', type=int, default=600)
    ap.add_argument('--top', type=int, default=12)
    a = ap.parse_args()
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cards = json.load(f)
    cards = {c['id']: c for c in (cards if isinstance(cards, list) else cards['cards'])}
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    groups = {'big': [], 'rest': []}
    by_card = {'big': collections.Counter(), 'rest': collections.Counter()}
    for g in lines:
        if not labels[g['cell']].endswith('/' + a.agent):
            continue
        tags = [collections.Counter(), collections.Counter()]
        flat = [0, 0]
        per = [0, 0]
        draws = [0, 0]
        plays = [0, 0]
        mine = [collections.Counter(), collections.Counter()]
        for act in g['actions']:
            if act.get('type') != 'play':
                continue
            s = act['seat']
            c = cards.get(act['card'])
            if not c:
                continue
            plays[s] += 1
            for t in c.get('tags', []):
                tags[s][t] += 1
            for o in c.get('effects') or []:
                if o.get('op') == 'production' and o.get('resource') == 'credits':
                    flat[s] += o['amount']
                    mine[s][c['id']] += o['amount']
                elif o.get('op') == 'production_per_tag' and o.get('resource') == 'credits':
                    n = tags[s][o['tag']] // max(o.get('per', 1), 1)
                    per[s] += n
                    mine[s][c['id']] += n
                elif o.get('op') == 'draw':
                    draws[s] += o['count']
        for s in range(2):
            k = 'big' if g['gained'][s][0] > a.gained else 'rest'
            groups[k].append((flat[s], per[s], draws[s], plays[s], g['gained'][s][0], g['rounds']))
            by_card[k].update(mine[s])
    for k, rows in groups.items():
        if not rows:
            continue
        n = len(rows)
        m = lambda i: sum(r[i] for r in rows) / n  # noqa: E731
        print(f'{k}: {n} seat-games; flat production +{m(0):.1f}, per-tag +{m(1):.1f}, cards drawn by plays '
              f'{m(2):.1f}, plays {m(3):.1f}, credits gained {m(4):.0f}, rounds {m(5):.1f}; '
              f'highest production +{max(r[0] + r[1] for r in rows)}')
        print('   by card, a seat-game: ' + ', '.join(f'{c} {v / n:.1f}' for c, v in by_card[k].most_common(a.top)))


if __name__ == '__main__':
    main()
