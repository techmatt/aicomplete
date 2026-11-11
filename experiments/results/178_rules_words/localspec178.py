"""Write a farm spec as a local experiment spec: out/<NAME>-<FIRST>.json with first_seed and games set.

    python experiments/results/178_rules_words/localspec178.py NAME FIRST GAMES

Run it with `aic-sim experiment out/NAME-FIRST.json --threads 6 --out DIR` (each side from its own folder).
"""
import json
import os
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))


def main():
    name, first, games = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    with open(os.path.join(ROOT, 'experiments', 'farm', name + '.json'), encoding='utf-8') as f:
        s = json.load(f)
    s['name'] = f'{name}-{first}'
    s['first_seed'] = first
    s['games'] = games
    out = os.path.join(ROOT, 'out', f'{name}-{first}.json')
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(s, f, indent=2)
        f.write('\n')
    print(out.replace('\\', '/'))


if __name__ == '__main__':
    main()
