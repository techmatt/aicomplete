"""Read a 282 lever screen: per lever (pairs pooled) and level, win %, rounds, cities corrupted by round 4 and at the end,
node-loss endings, research troops and node losses by march; each lever's win gap to the base lever paired by seed.

python screen.py GAMES.jsonl SPEC.json [--base base] [--agent strategist]
"""
import json
import math
import sys
from collections import defaultdict


def cnt(g, k):
    v = g['counters'].get(k, 0)
    return v if isinstance(v, (int, float)) else 0


def main():
    path, spec_path = sys.argv[1], sys.argv[2]
    base = sys.argv[sys.argv.index('--base') + 1] if '--base' in sys.argv else 'base'
    want = sys.argv[sys.argv.index('--agent') + 1] if '--agent' in sys.argv else None
    spec = json.load(open(spec_path, encoding='utf-8'))
    levels, agents = spec['difficulties'], spec['agents']
    names = [v['name'] for v in spec['variants']]
    rows = defaultdict(list)  # (lever, level, agent) -> games
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            c = g['cell']
            a = agents[c % len(agents)]
            v = names[c // (len(agents) * len(levels))]
            if want and a != want:
                continue
            lever, pair = v.split('/')
            g['_pair'] = pair
            rows[(lever, g['difficulty'], a)].append(g)
    levers = list(dict.fromkeys(n.split('/')[0] for n in names))
    for a in ([want] if want else agents):
        print(f'## {a}')
        print('lever\tlvl\tn\twin%\tgap±SE\trounds\tcity_r4\tcity_end\tnode_end\tres_troops\tmarch_loss\tplays/sr')
        for lever in levers:
            for d in levels:
                gs = rows.get((lever, d, a), [])
                if not gs:
                    continue
                n = len(gs)
                win = 100 * sum(g['won'] for g in gs) / n
                bmap = {(g['seed'], g['_pair']): g['won'] for g in rows.get((base, d, a), [])}
                diffs = [100 * (g['won'] - bmap[(g['seed'], g['_pair'])]) for g in gs if (g['seed'], g['_pair']) in bmap]
                if diffs and lever != base:
                    m = sum(diffs) / len(diffs)
                    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(1, len(diffs) - 1))
                    gap = f'{m:+.1f}±{sd / math.sqrt(len(diffs)):.1f}'
                else:
                    gap = ''
                rounds = sum(g['rounds'] for g in gs) / n
                r4 = sum(cnt(g, 'map.cities_corrupted.r01') + cnt(g, 'map.cities_corrupted.r02') + cnt(g, 'map.cities_corrupted.r03') + cnt(g, 'map.cities_corrupted.r04') for g in gs) / n
                cend = sum(g['margin'].get('cities_corrupted_end', 0) for g in gs) / n
                node = sum(g['reason'] == 'node' for g in gs)
                rt = sum(cnt(g, 'adversary.research_troops') for g in gs) / n
                ml = sum(cnt(g, 'board.node_lost.march') for g in gs) / n
                plays = sum(sum(v for k, v in g['counters'].items() if k.startswith('cards.played.') and k.count('.') == 2) for g in gs)
                sr = sum(len(g['spent']) * g['rounds'] for g in gs)
                print(f'{lever}\td{d}\t{n}\t{win:.1f}\t{gap}\t{rounds:.2f}\t{r4:.2f}\t{cend:.2f}\t{node}\t{rt:.1f}\t{ml:.1f}\t{plays / sr:.2f}')
            print()


if __name__ == '__main__':
    main()
