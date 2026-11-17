"""254: hosted-city payouts by round (`map.hosted.rNN`: cities paying a host that round), a seat-game, per agent and
arm; and node placements on a city (`board.node_at.<seat>.city` counts on or next to one, so this reads `board.std` no).

    python hosted254.py GAMES.jsonl SPEC.json
"""
import json
import sys
from collections import defaultdict


def main():
    games, spec_path = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding='utf-8'))
    vs, ds, ags = spec['variants'], spec['difficulties'], spec['agents']
    by = defaultdict(lambda: defaultdict(float))
    sg = defaultdict(int)
    first = defaultdict(list)
    with open(games, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g or 'won' not in g:
                continue
            v, r = divmod(g['cell'], len(ds) * len(ags))
            d, a = divmod(r, len(ags))
            arm = vs[v]['name'].partition('-')[2]
            key = (ags[a], arm)
            sg[key] += 2
            c = g['counters']
            rounds = [int(k[-2:]) for k, x in c.items() if k.startswith('map.hosted.r') and x > 0]
            first[key].append(min(rounds) if rounds else 0)
            for k, x in c.items():
                if k.startswith('map.hosted.r'):
                    by[key][int(k[-2:])] += x
    for key in sorted(by):
        n = sg[key]
        row = ' '.join(f'r{r}:{by[key][r] / n:.2f}' for r in range(1, 11))
        fs = first[key]
        none = sum(1 for x in fs if x == 0)
        mean_first = sum(x for x in fs if x) / max(len(fs) - none, 1)
        print(f'{key[0]} {key[1]}: hosted cities paying a seat-round: {row}; first payout round (games with any) {mean_first:.2f}, games with none {none}/{len(fs)}')


if __name__ == '__main__':
    main()
