#!/usr/bin/env python3
"""The energy and new-mechanic read (090, r35): per cell, pooled across DIRs: the win rate,
the energy each seat holds as the action phase ends (`unspent.<seat>.energy`, exported per
round from 090) at rounds 3 and 6 and at its last round, and the use per game of each energy
sink and r35 mechanic: energy paid to threat counters, the energy-cost cards played, the
energy gift Actions (only while a card has one: none since 106), stations freed (and player
stations an effect could not flip), the three capstones and their twists, Popular Uprising's
plays and Action uses (either half: no counter splits off its liberations) and the three
formerly dear powers.

    python scripts/cards/energy_read.py DIR [DIR ...] [--md]

Each DIR holds an experiment's games.jsonl and results.json. A variant's seats follow its
`ai.boards=` override, else the baseline's 0, 1. Seconds.
"""
import argparse
import collections
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cnt, content, load, seats  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

BOARDS = ['builder', 'protector', 'researcher']
# The dear powers before r35, by board and index in content/ais.json.
DEAR = {'Rush Build': 'builder', 'Shelter': 'protector', 'Particle Run': 'researcher'}


def power_index():
    out = {}
    for b in content('ais'):
        for k, p in enumerate(b['powers']):
            if p['name'] in DEAR:
                out[p['name']] = f"ai.power.{b['id']}.{k}"
    return out


def energy_cards():
    """Cards whose play costs energy, and buildings whose Action gives another player
    something for energy (the gift)."""
    pay, gift = [], []
    for c in content('cards'):
        if c.get('energy', 0) > 0:
            pay.append(c['id'])
        ab = c.get('ability')
        if ab and any(n > 0 for r, n in ab.get('cost', []) if r == 'energy') \
                and any(e['op'] == 'other_gain' for e in ab['effects']):
            gift.append(c['id'])
    return sorted(pay), sorted(gift)


def rows(games):
    n = len(games) or 1
    m = lambda f: sum(f(g) for g in games) / n  # noqa: E731
    pay, gift = energy_cards()
    powers = power_index()
    out = [
        ('energy paid to threat counters', m(lambda g: cnt(g, 'threats.counter_paid.energy'))),
        ('energy-cost cards played', m(lambda g: sum(cnt(g, f'cards.played.{c}') for c in pay))),
    ]
    out += [(f'  {c}', m(lambda g, c=c: cnt(g, f'cards.played.{c}'))) for c in pay]
    # 118: no card has had an energy gift Action since 106; the row shows only if one returns.
    if gift:
        out += [('energy gift Actions used', m(lambda g: sum(cnt(g, f'cards.used.{c}') for c in gift)))]
        out += [(f'  {c} (played / used)', m(lambda g, c=c: cnt(g, f'cards.played.{c}')),
                 m(lambda g, c=c: cnt(g, f'cards.used.{c}'))) for c in gift]
    out += [
        # 145: `board.stations_freed` was retired at 137 (read 0): the Liberate action on a station.
        ('stations liberated (the Liberate action)', m(lambda g: cnt(g, 'board.liberated.station'))),
        ('player stations an effect could not flip', m(lambda g: cnt(g, 'adversary.station_flip_held'))),
        ('stations flipped', m(lambda g: cnt(g, 'adversary.stations_flipped'))),
        ('Reclamation Crew played (frees a station)', m(lambda g: cnt(g, 'cards.played.station_prefab_order'))),
    ]
    for c in ['hostile_buyout', 'peacekeeping_mission', 'root_access']:
        out.append((f'{c} played', m(lambda g, c=c: cnt(g, f'cards.played.{c}'))))
    out += [
        ('  twist: ambassadors gained', m(lambda g: cnt(g, 'board.liberated_ambassadors'))),
        ('  twist: troops cleared', m(lambda g: cnt(g, 'board.liberated_troops'))),
        ('  twist: botnets removed', m(lambda g: cnt(g, 'board.liberated_botnet'))),
        # 118: this read `cards.once_discarded`, 0 since 106 took `once` off its liberate half.
        # No counter splits that half from its token half, so this is the Action's uses, both halves.
        ('Popular Uprising played / Action uses (either half)', m(lambda g: cnt(g, 'cards.played.popular_uprising')),
         m(lambda g: cnt(g, 'cards.used.popular_uprising'))),
        ('cities liberated by a card / the Liberate action', m(lambda g: cnt(g, 'board.liberated_card')),
         m(lambda g: cnt(g, 'board.liberated.city'))),
    ]
    for name, key in powers.items():
        out.append((f'{name} used', m(lambda g, key=key: cnt(g, key))))
    return out


def energy_by_round(games, nseats):
    """Mean energy a seat holds at rounds 3 and 6 (games that reached them) and at its last round."""
    at = {3: [], 6: []}
    end = []
    for g in games:
        for s in range(nseats):
            for r in at:
                # 099: a round counts once its action phase ended (its tag key), as at the end:
                # a game lost mid-round read 0 here. Since 100 every round up to the last
                # record is written, zeros included, so the key is there from round 1; on an
                # older games.jsonl a seat with no card in play wrote none (a later round
                # proves it then).
                if g['rounds'] > r or f'ai.tag_top.{s}.r{r:02d}' in g['counters']:
                    at[r].append(cnt(g, f'unspent.{s}.energy.r{r:02d}'))
            # The last round whose action phase ended: the round of its tag key (recorded
            # beside the energy; before 100, missing while it was 0).
            last = max((r for r in range(1, g['rounds'] + 1)
                        if f'ai.tag_top.{s}.r{r:02d}' in g['counters']), default=g['rounds'] - 1)
            last = max(last, g['rounds'] - 1)  # 099: every round before the last one ended
            end.append(cnt(g, f'unspent.{s}.energy.r{last:02d}') if last else 0)
    avg = lambda xs: sum(xs) / len(xs) if xs else float('nan')  # noqa: E731
    return avg(at[3]), avg(at[6]), avg(end)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    a = ap.parse_args()
    pooled = collections.defaultdict(list)
    nseats = {}
    for d in a.dirs:
        cells, games = load(d)
        for cid, c in cells.items():
            key = (c['variant'], c['agent'], c['difficulty'])
            pooled[key].extend(games[cid])
            nseats[key] = seats(c)  # 099: a c3 cell has three (it read two)
    keys = sorted(pooled)
    print('| cell | games | win % | energy r3 | energy r6 | energy at end |')
    print('|---|---|---|---|---|---|')
    for k in keys:
        gs = pooled[k]
        w = 100 * sum(g['won'] for g in gs) / len(gs)
        e3, e6, ee = energy_by_round(gs, nseats[k])
        print(f'| {k[0]}/{k[1]}/d{k[2]} | {len(gs)} | {w:.0f} | {e3:.1f} | {e6:.1f} | {ee:.1f} |')
    print()
    table = {k: rows(pooled[k]) for k in keys}
    # 100: the difficulty too, or a spec with two prints duplicate headers.
    print('| use a game | ' + ' | '.join(f'{k[0]}/{k[1]}/d{k[2]}' for k in keys) + ' |')
    print('|---|' + '---|' * len(keys))
    for i, r in enumerate(table[keys[0]]):
        cells = []
        for k in keys:
            vals = table[k][i][1:]
            cells.append(' / '.join(f'{v:.2f}' for v in vals))
        print(f'| {r[0]} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
