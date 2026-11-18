"""275: the adversary turn's per-site checks and components a round, before (r129) and after (r130), on the same
seeds, and the minutes a round they move at 268's rates (10 s a check, 3 s a component).

    python table_time.py OLD/games.jsonl NEW/games.jsonl [--seed 268003] [--base-minutes 57.3]

Checks, before (268's proxy, `design_check.json` table_cost at r129): every research station and factory on the map
twice (the Corrupt lines' and natural corruption's passes), every city twice (city growth's chip and build passes),
each troop's march step, each agent Rally dealt and each threat dealt. After: each site a line names, each arrival,
each botnet seeding (`map.adversary_checks`), each troop's march step and each threat dealt: nothing else is looked at.
Components: the adversary's ops a round (each agent, troop, cube or facility it moves) plus city chips (before).
`--base-minutes`: 274's d2 minutes a round (268's hand count, 490 minutes over 8 rounds and setup, less 274's 2.7).
"""
import argparse
import collections
import json
import statistics

R = {'component': 3, 'check': 10}


def per_round(path, old):
    out = collections.defaultdict(lambda: collections.defaultdict(list))  # (d) -> round -> [(checks, comps)]
    games = {}
    with open(path, encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        diffs = spec['difficulties']
        for line in f:
            g = json.loads(line)
            d = diffs[g['cell'] % len(diffs)] if len(spec['variants']) == 1 and len(spec['agents']) == 1 else g['difficulty']
            c = g['counters']
            seats = 2
            cities = 6  # Horizon and Cintamani: 3 cities each (docs/rules/board.md's countries table)
            rallied = c.get('adversary.rallied', 0) / max(g['rounds'], 1)
            dealt = c.get('threats.dealt', 0) / max(g['rounds'], 1)
            rows = []
            for i in range(1, g['rounds'] + 1):
                troops = c.get(f'map.troops.r{i:02d}', 0)
                ops = (g.get('adversary_ops') or [0] * 99)[i - 1] if i - 1 < len(g.get('adversary_ops') or []) else 0
                if old:
                    sites = sum(c.get(f'map.{k}.{own}.r{i:02d}', 0) for k in ('factories', 'stations')
                                for own in ('adversary', 'neutral', 'player'))
                    checks = 2 * sites + 2 * cities + troops + rallied + dealt
                    comps = ops + c.get(f'map.upkeep.r{i:02d}', 0)
                else:
                    checks = c.get(f'map.adversary_checks.r{i:02d}', 0) + troops + dealt
                    comps = ops
                rows.append((checks, comps))
                out[d][i].append((checks, comps))
            games[(d, int(g['seed']))] = rows
    return out, games


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('old')
    ap.add_argument('new')
    ap.add_argument('--seed', type=int, default=268003)
    ap.add_argument('--base-minutes', type=float, default=57.3)
    a = ap.parse_args()
    old, og = per_round(a.old, True)
    new, ng = per_round(a.new, False)
    for d in sorted(old):
        print(f'\n### d{d}: the traced games (14 seeds), a round\n')
        print('| Round | Checks before | Checks after | Components before | Components after | Minutes moved |')
        print('|---|---|---|---|---|---|')
        tot = collections.Counter()
        for i in sorted(set(old[d]) | set(new[d])):
            o, n = old[d].get(i, []), new[d].get(i, [])
            if not o or not n:
                continue
            oc, nc = statistics.mean(x[0] for x in o), statistics.mean(x[0] for x in n)
            om, nm = statistics.mean(x[1] for x in o), statistics.mean(x[1] for x in n)
            mins = ((nc - oc) * R['check'] + (nm - om) * R['component']) / 60
            print(f'| {i} | {oc:.1f} | {nc:.1f} | {om:.1f} | {nm:.1f} | {mins:+.1f} |')
        allo = [x for r in old[d].values() for x in r]
        alln = [x for r in new[d].values() for x in r]
        oc, nc = statistics.mean(x[0] for x in allo), statistics.mean(x[0] for x in alln)
        om, nm = statistics.mean(x[1] for x in allo), statistics.mean(x[1] for x in alln)
        mins = ((nc - oc) * R['check'] + (nm - om) * R['component']) / 60
        print(f'| **all rounds** | **{oc:.1f}** | **{nc:.1f}** | **{om:.1f}** | **{nm:.1f}** | **{mins:+.1f}** |')
        if d == 2:
            print(f'\nAt d2: {a.base_minutes:.1f} minutes a round (274) {mins:+.1f} = {a.base_minutes + mins:.1f} by these rates.')
        for key, name in ((og.get((d, a.seed)), 'before'), (ng.get((d, a.seed)), 'after')):
            if key:
                print(f'seed {a.seed} {name}: {len(key)} rounds; checks a round '
                      + ', '.join(f'{x[0]:.0f}' for x in key) + '; components ' + ', '.join(f'{x[1]:.0f}' for x in key))


main()
