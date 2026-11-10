"""168: the item's farm specs, written into experiments/farm/ (run from the repo root).

    python experiments/results/168_recalibration/specs168.py NAME LEVELS AGENTS [LEVER ...]

Writes experiments/farm/s168-NAME.json: the three pairs (bp, br, pr) at LEVELS (','-separated), AGENTS
(';'-separated), base plus each LEVER `tag:a=1;b=2` (its `--set` entries, ';'-separated) as variant `PAIR_tag`.
A tag holds no '_' (the readers split the variant on the first one). Farm it with
`farm-seeds FIRST COUNT OUT s168-NAME` after committing it; read it with `screen168.py`.
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def main():
    name, levels, agents = sys.argv[1], sys.argv[2], sys.argv[3]
    levers = [('base', [])]
    for a in sys.argv[4:]:
        tag, sets = a.split(':', 1)
        if '_' in tag:
            sys.exit(f'lever tag {tag} holds a _')
        levers.append((tag, sets.split(';')))
    variants = []
    for pair, sets in PAIRS:
        for tag, extra in levers:
            variants.append({'name': pair if tag == 'base' else f'{pair}_{tag}', 'set': sets + extra})
    spec = {'name': f's168-{name}',
            'note': f'168 {name}: ' + ', '.join(f'{t} [{"; ".join(s)}]' for t, s in levers[1:]) or 'base',
            'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': [int(x) for x in levels.split(',')], 'agents': agents.split(';'),
            'games': 32, 'records': 0}
    path = os.path.join(ROOT, 'experiments', 'farm', f's168-{name}.json')
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')
    print(path)


if __name__ == '__main__':
    main()
