"""Where two runs of the same spec first play differently, game by game (046).

    python scripts/bench/first_divergence.py DIR_OLD DIR_NEW
    python scripts/bench/first_divergence.py --cells DIR A B   # two cells of one run (061)

Each DIR is an `aic-sim experiment --out` directory on the same spec and seeds. For each game it
finds the first action that differs and classes it: `power` (the power used, or a choice inside
one), or the kind of action otherwise; `length` is the same actions, one game longer.
A last line a killed run cut short is dropped. An agent change meant to touch only powers
should print `same` for an agent with no rollouts, and `power` first wherever it first bites.
"""
import collections
import json
import sys


def games(d):
    out = {}
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        for line in f:
            try:
                g = json.loads(line)
            except json.JSONDecodeError:
                continue
            if 'seed' in g:
                out[(g['cell'], g['seed'])] = g['actions']
    return out


def cell_pairs(argv):
    """Both forms as {(label, seed): (actions, actions)}: `--cells DIR A B` is two cells of one
    run (two agents on the same seeds), labeled by cell A."""
    if argv[0] == '--cells':
        d, a, b = games(argv[1]), int(argv[2]), int(argv[3])
        return {(a, s): (acts, d[(b, s)]) for (c, s), acts in d.items() if c == a and (b, s) in d}
    old, new = games(argv[0]), games(argv[1])
    return {k: (old[k], new[k]) for k in old.keys() & new.keys()}


def main():
    tally = collections.defaultdict(collections.Counter)
    pairs = cell_pairs(sys.argv[1:])
    for key in sorted(pairs):
        a, b = pairs[key]
        n = next((i for i, (x, y) in enumerate(zip(a, b)) if x != y), None)
        if n is None:
            tally[key[0]]['same' if len(a) == len(b) else 'length'] += 1
            continue
        k = sorted({a[n]['type'], b[n]['type']})
        last = next((x['type'] for x in reversed(a[:n]) if x['type'] != 'choose'), '')
        if last == 'power' and k == ['choose']:
            k = ['power']
        tally[key[0]]['/'.join(k)] += 1
    for cell in sorted(tally):
        print(f'cell {cell}:', dict(tally[cell]))


if __name__ == '__main__':
    main()
