"""Per-card rates and win deltas over `aic-sim experiment` outputs (055).

    python scripts/cards/price_read.py DIR [--agent planner] [--csv OUT.csv] [--flags N]

`cards.csv` has no outcome, so this reads games.jsonl. For each card, in each cell:

    offered, kept, played   share of games, and play % (games played / games kept or drawn).
                            kept includes the opening deal and draws, so it is no share of
                            the games a card was offered in
    d_kept, d_played        win % of games where the team kept (or drew) / played it, minus
                            win % of games where it didn't
    d_offered               the same for being dealt into a draft packet: nearly random
                            given the seed, so it is the placebo. What it reads is bias.

A raw delta is biased: long games see more cards and wins run long. So each delta is
stratified on rounds played, and `rel_*` subtracts the mean delta of the card's tier, so a
card reads against its tier. `z` is rel / SE. 150 cards x 2 sizes means about 15 reads
past 2 SE by chance: a flag asks for the same sign at c2 and c3 and |z| >= 2.5 pooled.
These are correlations: confirm a fix with a paired overlay sweep.
"""
import argparse
import collections
import csv
import json
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import screens as s  # noqa: E402

CONTENT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'content')


def strata(g):
    return min(max(g['rounds'], 7), 11)


def delta(games, yes):
    """Win % with `yes(g)` minus without, stratified on rounds: (delta, SE, games with)."""
    by = collections.defaultdict(lambda: [[0, 0], [0, 0]])
    for g in games:
        side = by[strata(g)][1 if yes(g) else 0]
        side[0] += 1
        side[1] += 1 if g['won'] else 0
    total = var = weight = 0.0
    with_ = 0
    for (n0, w0), (n1, w1) in by.values():
        with_ += n1
        if not n0 or not n1:
            continue
        p0, p1 = w0 / n0, w1 / n1
        m = n0 * n1 / (n0 + n1)
        total += m * (p1 - p0)
        pool = (w0 + w1) / (n0 + n1)
        var += m * m * pool * (1 - pool) * (1 / n0 + 1 / n1)
        weight += m
    if not weight:
        return None, None, with_
    return 100 * total / weight, 100 * math.sqrt(var) / weight, with_


def field(g, card, name):
    n = g['cards'].get(card)
    return n[name] if n else 0


def read_cell(games, cards):
    rows = {}
    n = len(games)
    for c in cards:
        offered = sum(1 for g in games if field(g, c, 'offered'))
        kept = sum(1 for g in games if field(g, c, 'kept') or field(g, c, 'drawn'))
        played = sum(1 for g in games if field(g, c, 'played'))
        d_o, se_o, _ = delta(games, lambda g: field(g, c, 'offered') > 0)
        d_k, se_k, _ = delta(games, lambda g: field(g, c, 'kept') + field(g, c, 'drawn') > 0)
        d_p, se_p, _ = delta(games, lambda g: field(g, c, 'played') > 0)
        rows[c] = dict(games=n, offered=100 * offered / n, kept=100 * kept / n, played=100 * played / n,
                       play_rate=100 * played / kept if kept else None,
                       d_offered=d_o, se_offered=se_o, d_kept=d_k, se_kept=se_k, d_played=d_p, se_played=se_p)
    return rows


def relative(rows, tier):
    """rel_*, each delta less its tier's mean, and z."""
    for t in set(tier.values()):
        for k in ('kept', 'played'):
            vals = [r[f'd_{k}'] for c, r in rows.items() if tier[c] == t and r[f'd_{k}'] is not None]
            base = sum(vals) / len(vals) if vals else 0.0
            for c, r in rows.items():
                if tier[c] != t:
                    continue
                d, se = r[f'd_{k}'], r[f'se_{k}']
                r[f'rel_{k}'] = d - base if d is not None else None
                r[f'z_{k}'] = (d - base) / se if d is not None and se else None


def fmt(v, digits=1):
    return '' if v is None else f'{v:.{digits}f}'


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dir')
    ap.add_argument('--agent', default='planner')
    ap.add_argument('--csv')
    ap.add_argument('--flags', type=int, default=20)
    a = ap.parse_args()
    sys.stdout.reconfigure(encoding='utf-8')
    with open(os.path.join(CONTENT, 'cards.json'), encoding='utf-8') as f:
        content = [c for c in json.load(f) if c['tier'] in (1, 2)]
    tier = {c['id']: c['tier'] for c in content}
    cells, games = s.load(a.dir)
    by_variant = {}
    for cid, c in cells.items():
        if c['agent'] == a.agent:
            by_variant[c['variant']] = games[cid]
    if not by_variant:
        sys.exit(f"no cell for agent {a.agent}: {sorted({c['agent'] for c in cells.values()})}")

    tables = {}
    for v, gs in sorted(by_variant.items()):
        tables[v] = read_cell(gs, list(tier))
        relative(tables[v], tier)
    # Pooled: the strata and the delta both split by size.
    pooled = {}
    variants = sorted(tables)
    for c in tier:
        p = {}
        for k in ('kept', 'played'):
            ds = [(t[c][f'rel_{k}'], t[c][f'se_{k}']) for t in tables.values() if t[c].get(f'rel_{k}') is not None and t[c][f'se_{k}']]
            if ds:
                w = sum(1 / se ** 2 for _, se in ds)
                p[f'pool_{k}'] = sum(d / se ** 2 for d, se in ds) / w
                p[f'pool_z_{k}'] = p[f'pool_{k}'] * math.sqrt(w)
        pooled[c] = p

    print(f'**{a.agent}**, {", ".join(f"{v} {len(by_variant[v])} games, {100 * sum(g["won"] for g in by_variant[v]) / len(by_variant[v]):.0f}% won" for v in variants)}')
    print('\nBy tier: mean share of games, % (mean placebo d_offered; mean d_kept; mean d_played)\n')
    print('| variant | tier | offered | kept | played | play % | d_offered | d_kept | d_played |')
    print('|---|---|---|---|---|---|---|---|---|')
    for v in variants:
        for t in (1, 2):
            rs = [r for c, r in tables[v].items() if tier[c] == t]

            def m(k):
                xs = [r[k] for r in rs if r[k] is not None]
                return sum(xs) / len(xs) if xs else None
            print(f"| {v} | {t} | {fmt(m('offered'))} | {fmt(m('kept'))} | {fmt(m('played'))} | "
                  f"{fmt(m('play_rate'))} | {fmt(m('d_offered'))} | {fmt(m('d_kept'))} | {fmt(m('d_played'))} |")

    def agree(c, k):
        vals = [tables[v][c].get(f'rel_{k}') for v in variants]
        return all(x is not None for x in vals) and (all(x > 0 for x in vals) or all(x < 0 for x in vals))

    for k in ('played', 'kept'):
        print(f'\nFlags on {k}: same sign in every size, by pooled z\n')
        print(f'| card | t | pooled rel | z | ' + ' | '.join(f'{v} rel | {v} kept | {v} play %' for v in variants) + ' |')
        print('|---|---|---|---|' + '---|---|---|' * len(variants))
        order = sorted((c for c in tier if pooled[c].get(f'pool_z_{k}') is not None and agree(c, k)),
                       key=lambda c: -abs(pooled[c][f'pool_z_{k}']))
        for c in order[:a.flags]:
            cols = ' | '.join(f"{fmt(tables[v][c].get(f'rel_{k}'))} | {fmt(tables[v][c]['kept'], 0)} | {fmt(tables[v][c]['play_rate'], 0)}" for v in variants)
            print(f"| {c} | {tier[c]} | {fmt(pooled[c][f'pool_{k}'])} | {fmt(pooled[c][f'pool_z_{k}'])} | {cols} |")

    if a.csv:
        names = ['variant', 'card', 'tier', 'games', 'offered', 'kept', 'played', 'play_rate', 'd_offered', 'se_offered',
                 'd_kept', 'se_kept', 'rel_kept', 'z_kept', 'd_played', 'se_played', 'rel_played', 'z_played']
        with open(a.csv, 'w', newline='', encoding='utf-8') as f:
            w = csv.writer(f, lineterminator='\n')
            w.writerow(['agent'] + names)
            for v in variants:
                for c, r in tables[v].items():
                    row = dict(r, variant=v, card=c, tier=tier[c])
                    w.writerow([a.agent] + [fmt(row[n], 2) if isinstance(row.get(n), float) else row.get(n, '') for n in names])


if __name__ == '__main__':
    main()
