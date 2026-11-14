"""222: win % by agent and difficulty in farmed reads, each agent paired against a base agent on the same pair,
difficulty and seed; rounds beside.

    python read.py OUT.jsonl[,OUT2.jsonl] SPEC.json [--base AGENT] [--other OUT.jsonl SPEC.json AGENT]

Several OUT files (comma-separated) farmed from the same SPEC on other seed ranges pool. `--other` pairs the base
against an agent of another read (another build on the same seeds): its rows print as `other:AGENT`.
"""
import argparse
import json
import math
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def cells_of(spec):
    return [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
            for a in spec['agents']]


def load(paths, spec_path, tag=''):
    with open(spec_path, encoding='utf-8') as f:
        spec = json.load(f)
    cells = cells_of(spec)
    games = {}
    for p in paths.split(','):
        with open(p, encoding='utf-8') as f:
            for line in f:
                g = json.loads(line)
                if 'cell' not in g:
                    continue
                v, d, a = cells[g['cell']]
                games[(tag + a, d, v, g['seed'])] = g
    return games, [tag + a for a in spec['agents']], spec['difficulties']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('out')
    ap.add_argument('spec')
    ap.add_argument('--base', default=None)
    ap.add_argument('--other', nargs=3, action='append', default=[])
    args = ap.parse_args()
    games, agents, diffs = load(args.out, args.spec)
    for out, sp, agent in args.other:
        g2, _, _ = load(out, sp, 'other:')
        games.update({k: v for k, v in g2.items() if k[0] == 'other:' + agent})
        agents.append('other:' + agent)
    base = args.base or agents[0]
    print(f'base {base}; gap = agent - base, paired on pair, difficulty and seed, +- 1 SE')
    print(f'{"agent":58} {"d":>3} {"n":>4} {"win":>6} {"gap":>7} {"se":>5} {"rounds":>6}')
    for a in agents:
        for d in diffs + ['all']:
            ks = [k for k in games if k[0] == a and (d == 'all' or k[1] == d)]
            if not ks:
                continue
            win = 100 * sum(games[k]['won'] for k in ks) / len(ks)
            rounds = sum(games[k]['rounds'] for k in ks) / len(ks)
            diff = [100 * (games[k]['won'] - games[(base,) + k[1:]]['won']) for k in ks if (base,) + k[1:] in games]
            if diff and a != base:
                m = sum(diff) / len(diff)
                sd = math.sqrt(sum((x - m) ** 2 for x in diff) / max(1, len(diff) - 1))
                gap, se = f'{m:+7.1f}', f'{sd / math.sqrt(len(diff)):5.1f}'
            else:
                gap, se = f'{"":>7}', f'{"":>5}'
            print(f'{a[:58]:58} {str(d):>3} {len(ks):>4} {win:6.1f} {gap} {se} {rounds:6.2f}')


if __name__ == '__main__':
    main()
