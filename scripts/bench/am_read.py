"""Antimatter supply by source and spent / made, per lever and agent (230's reader, promoted in 241; 219's am_read.py
and 222's am_use.py under experiments/results name powers that no longer exist).

    python scripts/bench/am_read.py READ[,READ2...]

READ is a run folder, a games file with a header, or FILE=SPEC (a farm-seeds out file and its spec): see
`screens.stream`. A variant named `PAIR-LEVER` or `PAIR_LEVER` reads as lever LEVER of pair PAIR (`PAIR` alone: lever
`base`; `read_lib.split_variant`); a PAIR not in bp, br, pr stops the read. Per
lever and agent: antimatter made and spent a seat-game; supply shares by source (plants' Actions, Horizon's converter,
upkeep, Make antimatter, the rest: cards' on-play gains, other Actions and powers such as Overcharge II); the cheap
share (Make antimatter plus tier-1 cards, as design_check counts it); win rate and rounds. The board burns
(Teardown, Meltdown, Reclaim) were cut in 276, so no class counts them. The `sources` labels are listed in
docs/simulator.md.
"""
import os
import re
import sys
from collections import defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import PEAKS, cells_of, content, stream  # noqa: E402
from read_lib import split_variant  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = PEAKS.index('antimatter')
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
PLANTS = {'penning_trap_array', 'antiproton_foundry', 'positron_storage_ring', 'dark_fiber_collider',
          'treaty_isotope_bank', 'cyclotron_lease'}
CLASSES = ['plants', 'converter', 'upkeep', 'make', 'other']


def check_names():
    """Stop on a plant this reader names that content no longer has (219's and 222's readers went stale)."""
    cards = {c['id'] for c in content('cards')}
    gone = sorted(PLANTS - cards)
    if gone:
        sys.exit(f'am_read.py names what content no longer has: {", ".join(gone)}')


def klass(k):
    m = re.match(r'use:(\w+)$', k)
    if m and m.group(1) in PLANTS:
        return 'plants'
    if k.startswith('convert:'):
        return 'converter'
    if k == 'phase:economy.upkeep':
        return 'upkeep'
    if k == 'standard:antimatter':
        return 'make'
    return 'other'


def load(reads):
    rows = []
    for r in reads.split(','):
        games = stream(r)
        cells = cells_of(next(games)['spec'])
        for g in games:
            if 'cell' not in g:
                continue
            c = cells[g['cell']]
            pair, lv = split_variant(c['variant'])
            if pair not in PAIRS:
                sys.exit(f'am_read.py: variant {c["variant"]!r} names no pair it knows ({", ".join(PAIRS)}): '
                         f'name variants PAIR-LEVER or PAIR_LEVER')
            g['pair'], g['lever'], g['agent'], g['d'] = pair, lv, c['agent'], c['difficulty']
            rows.append(g)
    return rows


def read(label, games, t1):
    seats = 0
    made = spent = 0.0
    cls = defaultdict(float)
    cheap = 0.0
    for g in games:
        for per in g['sources']:
            seats += 1
            for k, row in per.items():
                v = row[AM]
                if not v:
                    continue
                k = re.sub(r'^(own|other):', '', k)
                made += v
                cls[klass(k)] += v
                m = re.match(r'(play|use):(\w+)$', k)
                if k.startswith('standard:') or (m and m.group(2) in t1):
                    cheap += v
        for per in g.get('uses') or []:
            spent += sum(row[AM] for row in per.values())
    s, tot = seats or 1, made or 1
    n = len(games)
    won = sum(bool(g.get('won')) for g in games) / max(n, 1)
    rounds = sum(g['rounds'] for g in games) / max(n, 1)
    shares = ' '.join(f'{c} {cls[c] / tot:.0%}' for c in CLASSES)
    print(f'{label}: {n} games, won {won:.1%}, rounds {rounds:.2f}; antimatter a seat-game made {made / s:.2f} '
          f'spent {spent / s:.2f} ({spent / tot:.0%}); cheap {cheap / tot:.0%}')
    print(f'  shares: {shares}')
    print('  a seat-game: ' + ' '.join(f'{c} {cls[c] / s:.2f}' for c in CLASSES))


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    check_names()
    games = load(sys.argv[1])
    t1 = {c['id'] for c in content('cards') if c.get('tier') == 1}
    keys = []
    for g in games:
        k = (g['lever'], g['agent'])
        if k not in keys:
            keys.append(k)
    for lv, a in sorted(keys):
        read(f'{lv} {a}', [g for g in games if (g['lever'], g['agent']) == (lv, a)], t1)


if __name__ == '__main__':
    main()
