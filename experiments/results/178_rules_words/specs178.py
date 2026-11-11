"""178's farm specs, written into experiments/farm/ (run from the repo root).

    python experiments/results/178_rules_words/specs178.py

f178-strategist / f178-greedy: the fresh ladder, the three pairs (bp, br, pr) at d0-d4, one agent each.
f178-oracle: the oracle row (177's settings) at d4 on the three pairs.
f178-smoke: the 16-seed engine smoke on the dev seeds, strategist and greedy, d0-d4, with the move rules on and off
(`:move_rules=0`: every move heuristic, the strike's included, left to the search).
Farm one with `farm-seeds FIRST COUNT OUT NAME` after committing it (r77); run the r76 side natively from out/r76/
(`aic-sim farm NAME --seed S --games N --out F`, its own configs, the spec copied into its experiments/farm/).
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
ORACLE = 'oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1'


def spec(name, note, agents, levels, levers=(('', []),)):
    variants = []
    for pair, sets in PAIRS:
        for tag, extra in levers:
            variants.append({'name': f'{pair}_{tag}' if tag else pair, 'set': sets + extra})
    return {
        'name': name,
        'note': note,
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': levels,
        'agents': agents,
        'games': 48,
        'records': 0,
    }


def main():
    specs = [
        spec('f178-strategist', '178: the fresh ladder, r77 against r76 on the same seeds: the strategist, d0-d4.',
             ['strategist'], [0, 1, 2, 3, 4]),
        spec('f178-greedy', '178: the fresh ladder, r77 against r76 on the same seeds: greedy, d0-d4.',
             ['greedy'], [0, 1, 2, 3, 4]),
        spec('f178-oracle', '178: the oracle row at d4 (177\'s settings), r77 against r76 on the same seeds.',
             [ORACLE], [4]),
        spec('f178-smoke', '178: the engine smoke (dev seeds): strategist and greedy, move rules on and off.',
             ['strategist', 'strategist:move_rules=0', 'greedy', 'greedy:move_rules=0'], [0, 1, 2, 3, 4]),
    ]
    for s in specs:
        p = os.path.join(ROOT, 'experiments', 'farm', s['name'] + '.json')
        with open(p, 'w', encoding='utf-8', newline='\n') as f:
            json.dump(s, f, indent=2)
            f.write('\n')
        print(p.replace('\\', '/'))


if __name__ == '__main__':
    main()
