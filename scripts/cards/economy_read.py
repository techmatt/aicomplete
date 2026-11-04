#!/usr/bin/env python3
"""The economy and card-throughput read (078): per cell, pooled across DIRs, per seat and game:
draft cards kept (opening keeps apart) and played, kept and never played, nodes placed, credit
income a round, credits and insight left unspent a round, and each board's dear power
(the one costing 2 or more of a tag) used a game with the seat-rounds it lacked the tags. Then the
tags by round: each seat's most-held tag on its cards in play when the action phase ends
(`ai.tag_top.<seat>`, 078; older binaries lack it), as a mean and the share of seats holding 3 or more.

    python scripts/cards/economy_read.py DIR [DIR ...] [--rounds 10]

Each DIR holds an experiment's games.jsonl and results.json. A variant's seats follow its
`ai.boards=` override, else the baseline's 0, 1, 2.
"""
import argparse
import collections
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cnt, content, load  # noqa: E402


def dear(p):
    """The engine's dear power (ai::power_is_dear): 2+ tags, or a tag and an energy (r35's three).
    099: 2+ tags alone matched no power after r35."""
    tags = sum(n for _, n in p['tags'])
    places = any(e['op'] == 'place_node' for e in p['effects'])
    return tags >= 2 or (tags >= 1 and not places and any(r == 'energy' and n > 0 for r, n in p.get('cost') or []))


def boards_of(spec, variant):
    for v in spec['variants']:
        if v['name'] == variant:
            for s in v['set']:
                if s.startswith('ai.boards='):
                    return [int(x) for x in s.split('=', 1)[1].split(',')]
    return [0, 1, 2]


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--rounds', type=int, default=10, help='rounds to show in the tags-by-round table')
    a = ap.parse_args()
    ais = content('ais')
    pooled = collections.defaultdict(list)
    seats_of = {}
    for d in a.dirs:
        with open(os.path.join(d, 'results.json'), encoding='utf-8') as f:
            spec = json.load(f)['spec']
        cells, games = load(d)
        for cid, gs in games.items():
            c = cells[cid]
            key = (c['variant'], c['agent'])
            n_seats = c.get('countries', 2)
            seats_of[key] = boards_of(spec, c['variant'])[:n_seats]
            pooled[key].extend(gs)

    print('| cell | games | win % | rounds | kept / seat (opening) | played / seat '
          '| kept, never played / seat | node / seat | credit income / seat-round | credits left / seat-round | insight left / seat-round |')
    print('|---|' + '---|' * 10)
    for key, gs in sorted(pooled.items()):
        seats = len(seats_of[key])
        n = len(gs) * seats
        kept = played = idle = 0
        for g in gs:
            for cid, c in g['cards'].items():
                kept += c.get('kept', 0)
                played += c.get('played', 0)
                if c.get('kept', 0) > 0 and c.get('played', 0) == 0:
                    idle += c['kept']
        opening = sum(cnt(g, 'cards.kept.r01') for g in gs)
        node = sum(cnt(g, 'board.node_placed') for g in gs)
        seat_rounds = sum(g['rounds'] for g in gs) * seats
        income = sum(cnt(g, 'economy.credits_income') for g in gs)
        left = sum(cnt(g, k) for g in gs for k in g['counters'] if k.startswith('economy.credits_unspent.r'))
        insight_left = sum(cnt(g, k) for g in gs for k in g['counters'] if k.startswith('economy.insight_unspent.r'))
        wins = sum(1 for g in gs if g.get('won'))
        p = wins / len(gs)
        se = 100 * (p * (1 - p) / len(gs)) ** 0.5
        print(f"| {key[0]} {key[1]} | {len(gs)} | {100 * p:.1f} ±{se:.1f} | {sum(g['rounds'] for g in gs) / len(gs):.1f} "
              f"| {kept / n:.1f} ({opening / n:.1f}) | {played / n:.1f} | {idle / n:.1f} | {node / n:.1f} "
              f"| {income / seat_rounds:.1f} | {left / seat_rounds:.1f} | {insight_left / seat_rounds:.1f} |")

    print('\nDear powers (the first of each board: 2+ tags, or a tag and an energy since r35), a game per seat of that board: used, and seat-rounds it '
          'lacked the tags.\n')
    print('| cell | ' + ' | '.join(f"{b['name']}" for b in ais) + ' |')
    print('|---|' + '---|' * len(ais))
    for key, gs in sorted(pooled.items()):
        parts = []
        for bi, b in enumerate(ais):
            k = next((k for k, p in enumerate(b['powers']) if dear(p)), None)
            if k is None or bi not in seats_of[key]:
                parts.append('')
                continue
            used = sum(cnt(g, f"ai.power.{b['id']}.{k}") for g in gs) / len(gs)
            no_tags = sum(cnt(g, f"ai.dear.{b['id']}.{k}.no_tags") for g in gs) / len(gs)
            parts.append(f"{b['powers'][k]['name']} {used:.2f} (no tags {no_tags:.1f})")
        print(f"| {key[0]} {key[1]} | " + ' | '.join(parts) + ' |')

    print('\nTags by round: each seat\'s most-held tag on its cards in play as the action phase ends, mean, '
          '(share of seats holding 3 or more). A game drops out once it ends; its last round counts only if it reached production.\n')
    print('| cell | seat | ' + ' | '.join(f'r{r}' for r in range(1, a.rounds + 1)) + ' |')
    print('|---|---|' + '---|' * a.rounds)
    for key, gs in sorted(pooled.items()):
        for s, bi in enumerate(seats_of[key]):
            row = []
            for r in range(1, a.rounds + 1):
                # A game's last round counts only if it reached production, where the count is taken.
                vals = [cnt(g, f'ai.tag_top.{s}.r{r:02d}') for g in gs
                        if g['rounds'] > r or (g['rounds'] == r and any(k.startswith('ai.tag_top.') and k.endswith(f'.r{r:02d}') for k in g['counters']))]
                if not vals or not any(k.startswith('ai.tag_top.') for g in gs for k in g['counters']):
                    row.append('')
                    continue
                three = sum(1 for v in vals if v >= 3) / len(vals)
                row.append(f'{sum(vals) / len(vals):.1f} ({100 * three:.0f}%)')
            print(f"| {key[0]} {key[1]} | {ais[bi]['name']} | " + ' | '.join(row) + ' |')


if __name__ == '__main__':
    main()
