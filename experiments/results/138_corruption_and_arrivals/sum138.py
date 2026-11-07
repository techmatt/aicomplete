"""138's quick table: per cell (variant, difficulty, agent) of a run's games.jsonl.

    python experiments/results/138_corruption_and_arrivals/sum138.py DIR [DIR ...] [--by-pair]

Win %, rounds (mean / median), how games end, nodes lost a game by cause, corruptions a game by site type, adversary
ambassadors created a game by source, sites created, threats dealt / uncountered by tier, and tier-3 threats seen
(share of games). A variant named `<pair>_<lever>` is read as lever `<lever>`; `--by-pair` keeps pairs apart,
otherwise the pairs bp, br and pr pool into one row per lever.
"""
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def load(d):
    head, games = None, defaultdict(list)
    with open(Path(d) / 'games.jsonl', encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if head is None:
                head = g
                cells = []
                spec = g['spec']
                for cfg in spec['configs']:
                    for v in spec.get('variants') or [{'name': 'base'}]:
                        for diff in spec.get('difficulties', [0]):
                            for a in spec['agents']:
                                cells.append((v['name'], diff, a))
                head['cells'] = cells
                continue
            games[head['cells'][g['cell']]].append(g)
    return head, games


def c(g, k):
    return g['counters'].get(k, 0)


def prefixed(g, p):
    return {k[len(p):]: v for k, v in g['counters'].items() if k.startswith(p) and '.' not in k[len(p):]}


def row(gs):
    n = len(gs)
    out = {'games': n, 'win%': 100.0 * sum(g['won'] for g in gs) / n,
           'rounds': statistics.mean(g['rounds'] for g in gs), 'median': statistics.median(g['rounds'] for g in gs)}
    ends = defaultdict(int)
    for g in gs:
        ends[g['reason']] += 1
    out['ends'] = dict(ends)
    for name, pre in (('lost', 'board.node_lost.'), ('corrupt', 'adversary.corruptions.'),
                      ('created', 'adversary.created.'), ('sites', 'adversary.sites_created.')):
        tot = defaultdict(float)
        for g in gs:
            for k, v in prefixed(g, pre).items():
                tot[k] += v
        out[name] = {k: round(v / n, 2) for k, v in sorted(tot.items())}
    for k in ('threats.dealt', 'threats.countered', 'threats.uncountered', 'adversary.corruptions_natural',
              'adversary.pulled', 'adversary.created', 'board.node_lost'):
        out[k] = round(sum(c(g, k) for g in gs) / n, 2)
    tiers = defaultdict(float)
    t3 = 0
    for g in gs:
        for k, v in g['counters'].items():
            if k.startswith('threats.dealt.t') or k.startswith('threats.uncountered.t'):
                tiers[k] += v
        if c(g, 'threats.dealt.t3') > 0:
            t3 += 1
    out['tiers'] = {k: round(v / n, 2) for k, v in sorted(tiers.items())}
    out['tier3 games%'] = round(100.0 * t3 / n, 1)
    return out


def main():
    dirs = [a for a in sys.argv[1:] if not a.startswith('--')]
    by_pair = '--by-pair' in sys.argv
    for d in dirs:
        head, games = load(d)
        print(f'## {d} (r{head["rules_revision"]})')
        pooled = defaultdict(list)
        for (var, diff, agent), gs in games.items():
            pair, _, lever = var.partition('_')
            key = (var if by_pair else (lever or 'base'), diff, agent)
            pooled[key].extend(gs)
        for key in sorted(pooled, key=lambda k: (k[0], k[1], k[2])):
            r = row(pooled[key])
            print(f'- {key[0]} d{key[1]} {key[2]}: {r["games"]} games, win {r["win%"]:.1f}%, rounds {r["rounds"]:.2f} '
                  f'(median {r["median"]}), ends {r["ends"]}')
            print(f'    nodes lost {r["board.node_lost"]} {r["lost"]}; corruptions {r["corrupt"]} '
                  f'(natural {r["adversary.corruptions_natural"]}, pulled {r["adversary.pulled"]})')
            print(f'    created {r["adversary.created"]} {r["created"]}; sites {r["sites"]}')
            print(f'    threats dealt {r["threats.dealt"]}, countered {r["threats.countered"]}, uncountered '
                  f'{r["threats.uncountered"]}; by tier {r["tiers"]}; tier-3 in {r["tier3 games%"]}% of games')


if __name__ == '__main__':
    main()
