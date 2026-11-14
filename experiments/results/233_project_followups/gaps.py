"""233: an arm's win % and its gap to r102, paired on (pair, difficulty, agent, seed), with small and big project
resolutions, per agent and difficulty. The base may be several games files (seed ranges of one base build).

    python experiments/results/233_project_followups/gaps.py NEW.jsonl BASE.jsonl[,BASE2.jsonl] [--arm ARM]

NEW and BASE are games files with their header (`shards.sh` output). ARM picks a screen's lever (`small2`: variants
`bp_small2` ...); without it the unlabeled variants (bp, br, pr). Small / big by effects at content/projects.json.
"""
import json
import math
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BIG = {p['id'] for p in json.loads((ROOT / 'content' / 'projects.json').read_text(encoding='utf-8'))
       if len(p['effects']) > 1}


def load(path, arm):
    out = {}
    with open(path, encoding='utf-8') as f:
        spec = json.loads(f.readline())['spec']
        cells = [(v['name'], d, a) for v in spec['variants'] for d in spec['difficulties'] for a in spec['agents']]
        for line in f:
            g = json.loads(line)
            name, d, agent = cells[g['cell']]
            pair, _, lever = name.partition('_')
            if lever != arm:
                continue
            c = g.get('counters', {})
            done = [(k[len('adversary.project.'):], v) for k, v in c.items()
                    if k.startswith('adversary.project.') and '.r' not in k[len('adversary.project.'):]]
            big = sum(v for k, v in done if k in BIG)
            small = sum(v for k, v in done if k not in BIG)
            out[(pair, d, agent, g['seed'])] = (1.0 if g['won'] else 0.0, small, big)
    return out


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    arm = sys.argv[sys.argv.index('--arm') + 1] if '--arm' in sys.argv else ''
    new = load(sys.argv[1], arm)
    base = {}
    for p in sys.argv[2].split(','):
        base.update(load(p, ''))
    print('| agent | d | games | win % | r102 | gap | SE | small | big |')
    print('|---|---|---|---|---|---|---|---|---|')
    for agent in sorted({k[2] for k in new}):
        for d in sorted({k[1] for k in new}):
            keys = [k for k in new if k[1] == d and k[2] == agent and k in base]
            if not keys:
                continue
            n = len(keys)
            diffs = [new[k][0] - base[k][0] for k in keys]
            mean = sum(diffs) / n
            sd = math.sqrt(sum((x - mean) ** 2 for x in diffs) / (n - 1)) if n > 1 else 0.0
            win = sum(new[k][0] for k in keys) / n
            old = sum(base[k][0] for k in keys) / n
            small = sum(new[k][1] for k in keys) / n
            big = sum(new[k][2] for k in keys) / n
            print(f'| {agent} | {d} | {n} | {100 * win:.1f} | {100 * old:.1f} | {100 * mean:+.1f} | '
                  f'{100 * sd / math.sqrt(n):.1f} | {small:.2f} | {big:.2f} |')


if __name__ == '__main__':
    main()
