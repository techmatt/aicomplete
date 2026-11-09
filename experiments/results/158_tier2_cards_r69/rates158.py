"""158: the new tier-2 cards' rates per game, per agent (and config/level cell group): offered, drafted, kept,
played, Action uses (`cards.used.<id>`), and won games' plays; plus the new ops' tallies a game.

    python rates158.py DIR [DIR ...] [--by agent|cell] [--md]

Run from the repo root. Each DIR holds an experiment's games.jsonl and results.json.
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

NEW = ["mass_defection", "embassy_of_exiles", "double_agents", "kettle_operation", "lure_network", "firewall_array",
       "air_gap", "rally_the_allies", "surgical_strike", "open_source_uprising"]
TALLIES = ["effects.converted", "effects.gathered", "effects.gathered_troops", "effects.struck", "effects.firewalls",
           "adversary.firewalled", "effects.if_tags_met"]


def group_of(label, by):
    parts = label.split('/')
    if by == 'agent':
        return parts[-1]
    # cell: config and level, agent (pairs pooled)
    return '/'.join(p for p in parts if p not in ('bp', 'br', 'pr') and not p.startswith(('bp_', 'br_', 'pr_')))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--by', default='agent')
    a = ap.parse_args()
    rows = collections.defaultdict(lambda: collections.defaultdict(float))
    games = collections.Counter()
    wins = collections.Counter()
    for d in a.dirs:
        it = stream(d)
        head = next(it)
        labels = cell_labels(head['spec'])
        for g in it:
            k = group_of(labels[g['cell']], a.by)
            games[k] += 1
            wins[k] += 1 if g['won'] else 0
            r = rows[k]
            cards = g.get('cards') or {}
            c = g.get('counters') or {}
            for cid in NEW:
                s = cards.get(cid, {})
                for f in ('offered', 'drafted', 'kept', 'played'):
                    r[f'{cid}.{f}'] += s.get(f, 0)
                r[f'{cid}.used'] += c.get(f'cards.used.{cid}', 0)
                if g['won']:
                    r[f'{cid}.won_played'] += s.get('played', 0) + c.get(f'cards.used.{cid}', 0)
            for t in TALLIES:
                r[t] += c.get(t, 0)
    for k in sorted(rows):
        n = games[k]
        print(f'\n### {k}: {n} games, won {100 * wins[k] / n:.1f}%\n')
        print('| card | offered | drafted | kept | played | Action uses | plays+uses in won games (per win) |')
        print('|---|---|---|---|---|---|---|')
        r = rows[k]
        for cid in NEW:
            w = wins[k] or 1
            print(f'| {cid} | ' + ' | '.join(f"{r[f'{cid}.{f}'] / n:.2f}" for f in ('offered', 'drafted', 'kept', 'played', 'used'))
                  + f" | {r[f'{cid}.won_played'] / w:.2f} |")
        print('\nTallies a game: ' + ', '.join(f'{t} {r[t] / n:.2f}' for t in TALLIES))


if __name__ == '__main__':
    main()
