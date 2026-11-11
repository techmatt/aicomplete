"""177: the item's farm specs, written into experiments/farm/ (run from the repo root).

    python experiments/results/177_recalibration/specs177.py NAME LEVELS AGENTS [--games N] [LEVER ...]

Writes experiments/farm/s177-NAME.json: the three pairs (bp, br, pr) at LEVELS (','-separated), AGENTS
(';'-separated), base plus each LEVER `tag:a=1;b=2` (its `--set` entries, ';'-separated) as variant `PAIR_tag`.
A tag holds no '_' (the readers split the variant on the first one); the tag `base` alone is the base cell.
`--nobase` leaves the base cell out (a fresh read of one setting). Farm it with
`farm-seeds FIRST COUNT OUT s177-NAME` after committing it; read it with `screen177.py`.
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def main():
    args = sys.argv[1:]
    games = 32
    nobase = False
    if '--games' in args:
        i = args.index('--games')
        games = int(args[i + 1])
        del args[i:i + 2]
    if '--nobase' in args:
        args.remove('--nobase')
        nobase = True
    name, levels, agents = args[0], args[1], args[2]
    levers = [] if nobase else [('base', [])]
    for a in args[3:]:
        tag, sets = a.split(':', 1)
        if '_' in tag:
            sys.exit(f'lever tag {tag} holds a _')
        levers.append((tag, [s for s in sets.split(';') if s]))
    variants = []
    for pair, sets in PAIRS:
        for tag, extra in levers:
            variants.append({'name': pair if tag == 'base' else f'{pair}_{tag}', 'set': sets + extra})
    note = ', '.join(f'{t} [{"; ".join(s)}]' for t, s in levers if t != 'base') or 'base'
    spec = {'name': f's177-{name}', 'note': f'177 {name}: {note}',
            'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': [int(x) for x in levels.split(',')], 'agents': agents.split(';'),
            'games': games, 'records': 0}
    path = os.path.join(ROOT, 'experiments', 'farm', f's177-{name}.json')
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')
    print(path)


if __name__ == '__main__':
    main()
