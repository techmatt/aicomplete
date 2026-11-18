"""274: a farmed file's 274 counters, per game-round, by level (and by arm: a cell's variant name, `_jam` the floor on).

    python counters.py OUT.jsonl SPEC.json

Cells are spec order: config x variant x difficulty x agent (the farm's cell index). Prints win %, rounds and,
per game-round, the counters each cut reads.
"""
import collections
import json
import sys

KEYS = ['board.troop_moves', 'board.stack_steps', 'board.stacks_marched', 'board.troop_strikes',
        'adversary.project_hexes_read', 'threats.dealt', 'threats.turned_rejected', 'threats.skipped_no_place',
        'threats.skip_tagged_would', 'threats.pull_forced']


def cells(spec):
    out = []
    for c in spec['configs']:
        for v in spec['variants']:
            for d in spec['difficulties']:
                for a in spec['agents']:
                    out.append((v['name'], d, a))
    return out


def main():
    spec = json.load(open(sys.argv[2], encoding='utf-8'))
    cl = cells(spec)
    agg = collections.defaultdict(lambda: collections.Counter())
    for line in open(sys.argv[1], encoding='utf-8'):
        if not line.startswith('{'):
            continue
        g = json.loads(line)
        name, d, a = cl[g['cell']]
        arm = 'jam' if name.endswith('_jam') else 'base'
        k = (arm, d, a)
        c = agg[k]
        c['games'] += 1
        c['won'] += g['won']
        c['rounds'] += g['rounds']
        c['timeout'] += g.get('reason') == 'timeout'
        cnt = g.get('counters', {})
        for key in KEYS:
            c[key] += cnt.get(key, 0)
        for key, v in cnt.items():
            if key.startswith('threats.countered.') or key == 'threats.countered':
                c[key] += v
    for k in sorted(agg):
        c = agg[k]
        r = c['rounds']
        line = f"{k[0]:4s} d{k[1]} {k[2]:10s} n={c['games']} win {100 * c['won'] / c['games']:.0f}% rounds {r / c['games']:.1f}"
        print(line)
        for key in KEYS + sorted(x for x in c if x.startswith('threats.countered')):
            if c[key]:
                print(f'    {key:32s} {c[key] / r:7.2f} a round')


main()
