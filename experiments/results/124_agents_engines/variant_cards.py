"""124: plays, keeps and offers a game of named cards per lever (the part of a variant name after the pair,
`bp-<lever>`; a bare pair is `base`), pooled over the pairs, with the win rate and rounds.

    python experiments/results/124_agents_engines/variant_cards.py DIR CARD... [--agent LABEL]
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('cards', nargs='+')
    ap.add_argument('--agent')
    a = ap.parse_args()
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    by = collections.defaultdict(list)
    order = []
    for g in lines:
        variant, agent = labels[g['cell']].split('/')[-2:]
        if a.agent and agent != a.agent:
            continue
        lever = variant.partition('-')[2] or 'base'
        key = f'{lever}/{agent}'
        if key not in by:
            order.append(key)
        by[key].append(g)
    print('| lever | games | win % | rounds | ' + ' | '.join(f'{c} played (kept, offered)' for c in a.cards) + ' |')
    print('|---|---|---|---|' + '---|' * len(a.cards))
    for k in order:
        gs = by[k]
        n = len(gs)
        cells = []
        for c in a.cards:
            pl = sum(g['cards'].get(c, {}).get('played', 0) for g in gs) / n
            kp = sum(g['cards'].get(c, {}).get('kept', 0) for g in gs) / n
            of = sum(g['cards'].get(c, {}).get('offered', 0) for g in gs) / n
            cells.append(f'{pl:.3f} ({kp:.2f}, {of:.2f})')
        win = 100 * sum(g['won'] for g in gs) / n
        rounds = sum(g['rounds'] for g in gs) / n
        print(f'| {k} | {n} | {win:.1f} | {rounds:.2f} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
