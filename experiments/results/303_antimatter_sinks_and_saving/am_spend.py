"""303: antimatter spent and made by source per 100 games, and rounds, from farmed reads.

    python experiments/results/303_antimatter_sinks_and_saving/am_spend.py NAME=FILE[:N:K] [...] [--top 25]

FILE:N:K reads agent K (0-based) of a spec with N agents (as am_cards.py). Prints, per read: games, mean rounds, then
the antimatter spent by each use (`uses`: `own:play:<card>`, `own:use:<building>`) and made by each source
(`sources`), per 100 games, the largest first.
"""
import collections
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4


def read(path):
    arm = None
    if path.count(':') >= 2 and path.rsplit(':', 2)[1].isdigit():
        path, n, k = path.rsplit(':', 2)
        arm = (int(n), int(k))
    spent, made = collections.Counter(), collections.Counter()
    n = rounds = 0
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g or (arm and g['cell'] % arm[0] != arm[1]):
                continue
            n += 1
            rounds += g['rounds']
            for s in g['uses']:
                for k, v in s.items():
                    spent[k] += v[AM]
            for s in g['sources']:
                for k, v in s.items():
                    made[k] += v[AM]
    return n, rounds, spent, made


def main():
    top = 25
    args = sys.argv[1:]
    if '--top' in args:
        i = args.index('--top')
        top = int(args[i + 1])
        del args[i:i + 2]
    for a in args:
        name, path = a.split('=', 1)
        n, rounds, spent, made = read(path)
        print('## %s: %d games, %.2f rounds; spent %.1f, made %.1f per 100 games' % (
            name, n, rounds / n, 100.0 * sum(spent.values()) / n, 100.0 * sum(made.values()) / n))
        print('- spent: ' + ', '.join('%s %.1f' % (k, 100.0 * v / n) for k, v in spent.most_common(top) if v))
        print('- made: ' + ', '.join('%s %.1f' % (k, 100.0 * v / n) for k, v in made.most_common(top) if v))


if __name__ == '__main__':
    main()
