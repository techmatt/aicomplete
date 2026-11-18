"""275: a screen's or read's adversary measures by arm (the variant name after its board pair), level and agent.

    python screen.py GAMES.jsonl [GAMES.jsonl ...] [--agent strategist] [--pair]

Each file's arms are its variants' suffixes (`bp_cap` -> `cap`; `bp` -> `base`); files are labeled by their path.
Per arm and level: games, win %, rounds, cities corrupted by round 4 and over the game (`map.cities_corrupted`),
corruptions, facilities grown (275: `adversary.grew.*`; before: `board.*_built_by_city`) and timeouts. With --pair,
the win % gap of each arm against the first file's first arm, paired by seed (a seed's mean over pairs and agents).
"""
import argparse
import collections
import json
import math
import statistics


def cells_of(spec):
    return [{'variant': v['name'], 'difficulty': d, 'agent': a}
            for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties'] for a in spec['agents']]


def read(path, label):
    out = []
    with open(path, encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        cells = cells_of(spec)
        for line in f:
            g = json.loads(line)
            cell = cells[g['cell']]
            arm = cell['variant'].split('_', 1)[1] if '_' in cell['variant'] else 'base'
            c = g['counters']
            cnt = lambda pre, upto=99: sum(v for k, v in c.items() if k.startswith(pre + '.r') and int(k[-2:]) <= upto)
            out.append({
                'arm': f'{label}:{arm}', 'd': cell['difficulty'], 'agent': cell['agent'], 'seed': int(g['seed']),
                'won': g['won'], 'rounds': g['rounds'], 'timeout': g.get('reason') == 'max_actions',
                'r4': cnt('map.cities_corrupted', 4), 'all': cnt('map.cities_corrupted'),
                'corr': c.get('adversary.corruptions', 0),
                'grown': c.get('adversary.grew.factory', 0) + c.get('adversary.grew.station', 0)
                + c.get('board.factory_built_by_city', 0) + c.get('board.station_built_by_city', 0),
                'reason': g.get('reason'),
            })
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('files', nargs='+')
    ap.add_argument('--agent', default=None)
    ap.add_argument('--pair', action='store_true')
    a = ap.parse_args()
    games = []
    for i, p in enumerate(a.files):
        games += read(p, f'f{i}')
    if a.agent:
        games = [g for g in games if g['agent'] == a.agent]
    by = collections.defaultdict(list)
    for g in games:
        by[(g['arm'], g['d'])].append(g)
    arms = list(dict.fromkeys(g['arm'] for g in games))
    print('| Arm | Level | Games | Win % | Rounds | Cities corrupted by r4 | Cities corrupted (game) | Corruptions | Grown | Node losses | Timeouts |')
    print('|---|---|---|---|---|---|---|---|---|---|---|')
    for arm in arms:
        for d in sorted({g['d'] for g in games}):
            gs = by.get((arm, d), [])
            if not gs:
                continue
            m = lambda k: statistics.mean(x[k] for x in gs)
            print(f"| {arm} | d{d} | {len(gs)} | {100 * m('won'):.0f} | {m('rounds'):.2f} | {m('r4'):.2f} | {m('all'):.2f} | "
                  f"{m('corr'):.1f} | {m('grown'):.1f} | {sum(x['reason'] == 'node' for x in gs)} | {sum(x['timeout'] for x in gs)} |")
    if a.pair:
        base = arms[0]
        print(f'\nWin % gap against {base}, paired by seed (mean of a seed over pairs and agents), ± SE:')
        for arm in arms[1:]:
            row = []
            for d in sorted({g['d'] for g in games}):
                s0 = collections.defaultdict(list)
                s1 = collections.defaultdict(list)
                for g in by.get((base, d), []):
                    s0[g['seed']].append(g['won'])
                for g in by.get((arm, d), []):
                    s1[g['seed']].append(g['won'])
                diffs = [100 * (statistics.mean(s1[s]) - statistics.mean(s0[s])) for s in s0 if s in s1]
                if len(diffs) > 1:
                    se = statistics.stdev(diffs) / math.sqrt(len(diffs))
                    row.append(f'd{d} {statistics.mean(diffs):+.1f} ± {se:.1f}')
            print(f'- {arm}: ' + '; '.join(row))


main()
