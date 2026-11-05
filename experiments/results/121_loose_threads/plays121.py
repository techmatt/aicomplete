"""121: plays per game of the cards this item changed, r47 against r48 (CLAUDE.md: re-read a
card's plays per game after any edit to it), and the reshuffle's use.

    python experiments/results/121_loose_threads/plays121.py OLD_DIR NEW_DIR
"""
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

CARDS = ['peace_conference', 'popular_uprising', 'endowed_research_chair', 'autonomous_defense_grid']


def read(d):
    lines = stream(d)
    head = next(lines)
    names = cell_labels(head['spec'])
    by = {}
    for g in lines:
        agent = names[g['cell']].split('/')[1]
        by.setdefault(agent, []).append(g)
    return head.get('rules_revision'), by


def main():
    sys.stdout.reconfigure(encoding='utf-8')
    (ro, old), (rn, new) = read(sys.argv[1]), read(sys.argv[2])
    print(f'| agent | card | kept / game r{ro} | r{rn} | played / game r{ro} | r{rn} |')
    print('|---|---|---|---|---|---|')
    for agent in new:
        for c in CARDS:
            row = []
            for gs in (old[agent], new[agent]):
                row.append((sum(g['cards'].get(c, {}).get('kept', 0) for g in gs) / len(gs),
                            sum(g['cards'].get(c, {}).get('played', 0) for g in gs) / len(gs)))
            print(f'| {agent} | {c} | {row[0][0]:.2f} | {row[1][0]:.2f} | {row[0][1]:.2f} | {row[1][1]:.2f} |')
    for agent in new:
        for rev, gs in ((ro, old[agent]), (rn, new[agent])):
            lib = sum(g['counters'].get('board.liberated_card', 0) for g in gs) / len(gs)
            pu_uses = sum(v for g in gs for k, v in g['counters'].items() if k.startswith('cards.used.popular_uprising')) / len(gs)
            print(f'- {agent} r{rev}: Popular Uprising Action uses a game {pu_uses:.2f}; liberations by a card a game {lib:.2f}')


if __name__ == '__main__':
    main()
