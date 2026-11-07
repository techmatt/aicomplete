"""Who counters what (070; 135's counter menu), per cell of experiment DIRs, read from games.jsonl:
threats dealt, countered by the kind of option paid (credits, energy, insight, ambassadors, research
cubes, tags, nodes, a card, a standing counter), paid abroad, rounds with every threat countered, and
the typed counters and capstones played.

    python scripts/cards/threat_read.py DIR [DIR ...] [--cards]

Each DIR's spec has one config and one difficulty.
"""
import argparse
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_labels, content, stream  # noqa: E402


def typed_and_capstones():
    typed, cap = [], []
    for c in content('cards'):
        ops = c.get('effects', []) + c.get('ability', {}).get('effects', [])
        if any(o['op'] == 'jam' and 'source' in o for o in ops):
            typed.append(c['id'])
        if c['tier'] == 2 and c['kind'] == 'building' and any(
                o['op'] in ('income_per_tag', 'auto_counter', 'project_drain') for o in c.get('passives', [])):
            cap.append(c['id'])
    return typed, cap


def read(d):
    """The cell names, and {cell id: games} without the action lists."""
    lines = stream(d)
    # (lead, agent): `cell_labels` takes multi-difficulty specs (`cell_names` asserts one difficulty).
    names = [tuple(lbl.rsplit('/', 1)) for lbl in cell_labels(next(lines)['spec'])]
    cells = defaultdict(list)
    for g in lines:
        g.pop('actions', None)
        cells[g['cell']].append(g)
    return names, cells


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--cards', action='store_true')
    a = ap.parse_args()
    typed, cap = typed_and_capstones()
    # 145: chips (142's menus) and free (an option cut to nothing) were left out.
    by = ['credits', 'energy', 'insight', 'ambassadors', 'research_cubes', 'chips', 'tags', 'nodes', 'card', 'standing', 'jam', 'free']
    keys = ([('dealt', 'threats.dealt'), ('counter', 'threats.countered')]
            + [(k[:5], f'threats.countered_by.{k}') for k in by]
            + [('exec', 'threats.executed'), ('abroad', 'threats.paid_abroad'), ('clean', 'threats.rounds_all_countered'),
               ('typed', 'threats.typed_counters'), ('auto', 'threats.auto_countered')])
    print('| cell | n | win | ' + ' | '.join(k for k, _ in keys) + ' | typed played | capstones played |')
    print('|' + '---|' * (len(keys) + 5))
    for d in a.dirs:
        names, cells = read(d)
        for cell, gs in cells.items():
            n = len(gs)
            cnt = lambda k: sum(g['counters'].get(k, 0) for g in gs) / n
            pl = lambda ids: sum(cnt(f'cards.played.{i}') for i in ids)
            won = sum(g['won'] for g in gs) / n * 100
            print(f"| {' '.join(names[cell])} | {n} | {won:.0f} | " + ' | '.join(f'{cnt(k):.1f}' for _, k in keys)
                  + f' | {pl(typed):.2f} | {pl(cap):.2f} |')
            if a.cards:
                for i in typed + cap:
                    if cnt(f'cards.played.{i}'):
                        print(f"  {i}: {cnt(f'cards.played.{i}'):.2f}")


if __name__ == '__main__':
    main()
