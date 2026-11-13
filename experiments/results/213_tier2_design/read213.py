"""213's reader: win rate by level per arm (pairs pooled), each arm paired against the first on the same seeds,
tier-2 keeps by round 4, antimatter made and spent (by source), won games that spent antimatter, and the touched
cards' plays a game.

    python experiments/results/213_tier2_design/read213.py LABEL=PATH[:LEVER] ...

PATH: an experiment folder, a games.jsonl or a farm out file (header line first). LEVER (optional) keeps only that
lever's cells (`base` is the cells with no `_` suffix). The first arm is the reference for the paired columns.
"""
import json
import math
import os
import sys
from collections import defaultdict, Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
AM = RES.index('antimatter')
CARDS = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
TOUCHED = ['dyson_swarm_tranche', 'recursive_self_improvement', 'peacekeeping_mission', 'root_access', 'hostile_buyout',
           'decapitation_strike', 'collider_consortium', 'catalyzed_fab_run', 'antimatter_accord', 'cyclotron_lease',
           'accelerator_diplomacy', 'nuclear_option', 'annihilation_lance', 'fusion_commons', 'kinetic_interdiction',
           'great_liberation', 'singularity_exploit', 'penning_trap_array', 'antiproton_foundry', 'positron_storage_ring',
           'zero_day_auction']


def load(path, lever=None):
    p = path if os.path.isfile(path) else os.path.join(path, 'games.jsonl')
    head, games = None, []
    with open(p, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                head = g
            else:
                games.append(g)
    if head is None:  # a farm out file: no header; its spec is experiments/farm/<file name>.json
        name = os.path.splitext(os.path.basename(p))[0]
        head = {'spec': json.load(open(os.path.join(ROOT, 'experiments', 'farm', name + '.json'), encoding='utf-8'))}
    spec = head['spec'] if isinstance(head['spec'], dict) else json.loads(head['spec'])
    names = [v['name'] for v in spec['variants']]
    per = len(spec['difficulties']) * len(spec['agents'])
    out = []
    for g in games:
        v = names[(g['cell'] // per) % len(names)]
        g['pair'], _, lv = v.partition('_')
        g['lever'] = lv or 'base'
        if lever is None or g['lever'] == lever:
            out.append(g)
    return out


def kept_t2_r4(g):
    c = g['counters']
    return sum(c.get(f'cards.kept_t2.r0{r}', 0) for r in range(1, 5))


def am_spent(g):
    return sum(s[AM] for s in g['spent'])


def main():
    arms = []
    for a in sys.argv[1:]:
        label, _, rest = a.partition('=')
        path, lever = rest, None
        if ':' in rest[2:]:
            i = rest.rindex(':')
            if i > 2:
                path, lever = rest[:i], rest[i + 1:]
        arms.append((label, load(path, lever)))
    ref_label, ref = arms[0]
    ref_by = {(g['pair'], g['difficulty'], g['seed']): g for g in ref}
    levels = sorted({g['difficulty'] for _, gs in arms for g in gs})
    print('## Win rate (pairs pooled), paired against', ref_label)
    print()
    print('| arm | ' + ' | '.join(f'd{d}' for d in levels) + ' | rounds |')
    print('|---|' + '---|' * (len(levels) + 1))
    for label, gs in arms:
        cells = []
        for d in levels:
            sel = [g for g in gs if g['difficulty'] == d]
            if not sel:
                cells.append('-')
                continue
            w = 100 * sum(g['won'] for g in sel) / len(sel)
            if label != ref_label:
                diffs = [g['won'] - ref_by[(g['pair'], d, g['seed'])]['won'] for g in sel
                         if (g['pair'], d, g['seed']) in ref_by]
                if diffs:
                    m = sum(diffs) / len(diffs)
                    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1))
                    cells.append(f'{w:.1f} ({100 * m:+.1f} ± {100 * sd / math.sqrt(len(diffs)):.1f})')
                    continue
            cells.append(f'{w:.1f}')
        rounds = sum(g['rounds'] for g in gs) / max(1, len(gs))
        print(f'| {label} ({len(gs)}) | ' + ' | '.join(cells) + f' | {rounds:.2f} |')
    print()
    print('## Tier 2 and antimatter, a game (seat-game for resources)')
    print()
    print('| arm | tier-2 keeps r1-4 | tier-2 plays | am gained / spent a seat | won games spending am | top am sources | top am spends |')
    print('|---|---|---|---|---|---|---|')
    for label, gs in arms:
        n = len(gs)
        seats = sum(len(g['spent']) for g in gs)
        k4 = sum(kept_t2_r4(g) for g in gs) / n
        t2p = sum(v for g in gs for k, v in g['counters'].items()
                  if k.startswith('cards.played.') and CARDS.get(k[13:], {}).get('tier') == 2) / n
        gained = sum(s[AM] for g in gs for s in g['gained']) / seats
        spent = sum(am_spent(g) for g in gs) / seats
        won = [g for g in gs if g['won']]
        wam = sum(1 for g in won if am_spent(g) > 0) / max(1, len(won))
        src, spd = Counter(), Counter()
        for g in gs:
            for seat in g['sources']:
                for k, v in seat.items():
                    if v[AM] > 0 and not k.startswith('other:'):
                        src[k.replace('own:', '')] += v[AM]
            for seat in g['uses']:
                for k, v in seat.items():
                    if v[AM] > 0 and not k.startswith('other:'):
                        spd[k.replace('own:', '')] += v[AM]
        ts, tp = sum(src.values()) or 1, sum(spd.values()) or 1
        fs = ', '.join(f'{k} {100 * v / ts:.0f}%' for k, v in src.most_common(4))
        fp = ', '.join(f'{k} {100 * v / tp:.0f}%' for k, v in spd.most_common(4))
        print(f'| {label} | {k4:.2f} | {t2p:.2f} | {gained:.2f} / {spent:.2f} | {100 * wam:.0f}% of {len(won)} | {fs} | {fp} |')
    print()
    print('## Touched cards, plays a game')
    print()
    print('| card | ' + ' | '.join(l for l, _ in arms) + ' |')
    print('|---|' + '---|' * len(arms))
    for cid in TOUCHED:
        row = []
        for _, gs in arms:
            row.append(f"{sum(g['counters'].get(f'cards.played.{cid}', 0) for g in gs) / max(1, len(gs)):.2f}")
        print(f'| {CARDS[cid]["name"]} | ' + ' | '.join(row) + ' |')


main()
