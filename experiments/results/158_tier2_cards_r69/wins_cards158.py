"""158: the new cards in won and lost games, per level: the share of games that played (or used the Action of) at
least one new card, new-card plays a game, and per card the win % of games that played it against those that didn't
(a correlation, not an effect: a long game plays more cards).

    python wins_cards158.py DIR [DIR ...]
"""
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '..', '..', 'scripts'))
sys.path.insert(0, HERE)
from screens import cell_labels, stream  # noqa: E402
from rates158 import NEW  # noqa: E402


def main():
    by = collections.defaultdict(list)
    for d in sys.argv[1:]:
        it = stream(d)
        labels = cell_labels(next(it)['spec'])
        for g in it:
            lv = next(p for p in labels[g['cell']].split('/') if p.startswith('d') and p[1:].isdigit())
            cards = g.get('cards') or {}
            c = g.get('counters') or {}
            n = {cid: cards.get(cid, {}).get('played', 0) + c.get(f'cards.used.{cid}', 0) for cid in NEW}
            by[lv].append((g['won'], n))
    for lv in sorted(by):
        gs = by[lv]
        won = [n for w, n in gs if w]
        lost = [n for w, n in gs if not w]
        share = lambda xs: 100 * sum(1 for n in xs if sum(n.values()) > 0) / max(len(xs), 1)
        plays = lambda xs: sum(sum(n.values()) for n in xs) / max(len(xs), 1)
        print(f'\n### {lv}: {len(gs)} games, {len(won)} won\n')
        print(f'Played a new card: won {share(won):.1f}%, lost {share(lost):.1f}%; new-card plays and uses a game: won '
              f'{plays(won):.2f}, lost {plays(lost):.2f}\n')
        print('| card | games played | win % played | win % not |')
        print('|---|---|---|---|')
        for cid in NEW:
            on = [w for w, n in gs if n[cid] > 0]
            off = [w for w, n in gs if n[cid] == 0]
            pct = lambda xs: 100 * sum(xs) / max(len(xs), 1)
            print(f'| {cid} | {len(on)} | {pct(on):.1f} | {pct(off):.1f} |')


if __name__ == '__main__':
    main()
