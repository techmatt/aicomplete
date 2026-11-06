"""127's dev screen spec from overlay.txt: the base (r51, no limits, printed cards), each `# lever:` group alone, and
all groups together, crossed with the three pairs, on the dev seeds.

    python experiments/results/127_no_limits/gen_screen.py [--out experiments/screen-127-levers.json]
        [--games 96] [--seed 35392] [--agents strategist,greedy] [--only all]
"""
import argparse
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, '..', '..', '..')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def groups(path):
    out, cur = {}, None
    with open(path, encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line.startswith('# lever:'):
                cur = line.split(':', 1)[1].strip()
                out[cur] = []
            elif line and not line.startswith('#'):
                out[cur].append(line)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=os.path.join(ROOT, 'experiments', 'screen-127-levers.json'))
    ap.add_argument('--games', type=int, default=96)
    ap.add_argument('--seed', type=int, default=35392)
    ap.add_argument('--agents', default='strategist,greedy')
    ap.add_argument('--only', default='', help='comma list of levers to keep (besides the base)')
    ap.add_argument('--name', default='screen-127-levers')
    a = ap.parse_args()
    g = groups(os.path.join(HERE, 'overlay.txt'))
    levers = {'base': []}
    levers.update(g)
    levers['all'] = [e for v in g.values() for e in v]
    if a.only:
        keep = set(a.only.split(','))
        levers = {k: v for k, v in levers.items() if k == 'base' or k in keep}
    variants = []
    for lever, sets in levers.items():
        for pair, pset in PAIRS:
            variants.append({'name': pair if lever == 'base' else f'{pair}-{lever}', 'set': pset + sets})
    spec = {
        'name': a.name,
        'note': f'127: the card pass by lever (overlay.txt groups) beside the base (r51, no limits), the three pairs at '
                f'd2, seeds {a.seed}-{a.seed + a.games - 1} (gen_screen.py writes it).',
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': [2],
        'agents': a.agents.split(','),
        'games': a.games,
        'first_seed': a.seed,
        'records': 0,
    }
    with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')
    print({k: len(v) for k, v in levers.items()})


if __name__ == '__main__':
    main()
