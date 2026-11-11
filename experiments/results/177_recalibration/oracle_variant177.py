"""177: an oracle-only d4 companion of oracle177.json with extra `--set`s on every variant.

    python experiments/results/177_recalibration/oracle_variant177.py [--first SEED] TAG [KEY=VALUE ...]

Writes experiments/results/177_recalibration/oracle177-TAG.json: d4, the oracle alone, set a's first 8 seeds a pair
(as oracle177.json), the overrides appended to each variant's set. Run it locally with `--threads 3`.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    args = sys.argv[1:]
    first = None
    if '--first' in args:
        i = args.index('--first')
        first = int(args[i + 1])
        del args[i:i + 2]
    tag, sets = args[0], args[1:]
    with open(os.path.join(HERE, 'oracle177.json'), encoding='utf-8') as f:
        s = json.load(f)
    if first is not None:
        s['first_seed'] = first
    s['name'] = f's177-oracle-{tag}'
    s['difficulties'] = [4]
    s['agents'] = [a for a in s['agents'] if a.startswith('oracle')]
    s['note'] = f"177: the oracle at d4 with {', '.join(sets) or 'the baseline'}, set a's first 8 seeds a pair"
    for v in s['variants']:
        v['set'] = v['set'] + sets
    out = os.path.join(HERE, f'oracle177-{tag}.json')
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(s, f, indent=2)
        f.write('\n')
    print(out)


if __name__ == '__main__':
    main()
