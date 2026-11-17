"""254: the paired on-off win gap pooled over levels (d0-d3 and d0-d4), per agent, from a farmed read and its spec.

    python pooled254.py GAMES.jsonl SPEC.json
"""
import json
import math
import sys
from collections import defaultdict


def main():
    games, spec_path = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding='utf-8'))
    vs, ds, ags = spec['variants'], spec['difficulties'], spec['agents']
    wins = defaultdict(dict)
    with open(games, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g or 'won' not in g:
                continue
            v, r = divmod(g['cell'], len(ds) * len(ags))
            d, a = divmod(r, len(ags))
            pair, _, arm = vs[v]['name'].partition('-')
            wins[(ags[a], arm)][(pair, ds[d], int(g['seed']))] = 1 if g['won'] else 0
    arms = sorted({arm for _, arm in wins if arm != 'off'})
    pairs = [(a, 'off') for a in arms] + [(b, a) for i, a in enumerate(arms) for b in arms[i + 1:]]
    for agent in ags:
        for new_arm, base_arm in pairs:
            base, new = wins[(agent, base_arm)], wins[(agent, new_arm)]
            for top in (3, 4):
                keys = [k for k in base if k in new and k[1] <= top]
                diffs = [new[k] - base[k] for k in keys]
                n = len(diffs)
                m = sum(diffs) / n
                sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / (n - 1))
                print(f'{agent} {new_arm} vs {base_arm}, d0-d{top}: {100 * sum(base[k] for k in keys) / n:.1f} -> '
                      f'{100 * sum(new[k] for k in keys) / n:.1f}, gap {100 * m:+.1f} ± {100 * sd / math.sqrt(n):.1f} (n {n})')


if __name__ == '__main__':
    main()
