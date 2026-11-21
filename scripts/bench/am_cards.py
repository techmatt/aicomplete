"""The antimatter sinks per 100 games (offered, kept, played) and when antimatter idles, from farmed reads (303's
am_cards.py, promoted in 304).

    python scripts/bench/am_cards.py NAME=FILE [NAME=FILE ...] [--also ID[,ID...]]

FILE is a farmed games JSONL (a line without `cell`, a header, is skipped); FILE:N:K reads only agent K (0-based) of a
spec with N agents. A sink is a card whose `antimatter` cost is above 0 in content/cards.json (this tree's);
`--also` lists more ids after them (a card a screen ran that the tree has dropped). Columns per read: offered / kept /
played per 100 games of each sink; antimatter gained / spent a seat-game per level (two seats a game); then the
antimatter a seat holds unspent at the end of each round (`unspent.S.antimatter.rNN`, the mean over seats that
reach it) and the share of seat-games ending with antimatter held. Spends by use and source: 303's am_spend.py.
"""
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
AM = 4  # scripts/screens.py PEAKS: credits, insight, energy, chips, antimatter, ...


def sinks(also=()):
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cs = json.load(f)
    return [c['id'] for c in cs if c.get('antimatter', 0) > 0] + list(also)


def read(path, ids):
    n = 0
    tally = collections.Counter()
    unspent = collections.defaultdict(list)
    end_held = 0
    seat_games = 0
    gained = spent = 0.0
    by_level = collections.defaultdict(lambda: [0, 0.0, 0.0, 0])
    # FILE:N:K keeps the games of agent K of N (a spec's cells run configs, variants, difficulties, then agents).
    arm = None
    if path.count(':') >= 2 and path.rsplit(':', 2)[1].isdigit():
        path, n_agents, k = path.rsplit(':', 2)
        arm = (int(n_agents), int(k))
    with open(path, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            g = json.loads(line)
            if 'cell' not in g or (arm and g['cell'] % arm[0] != arm[1]):
                continue
            n += 1
            c = g['counters']
            for i in ids:
                for k in ('offered', 'kept', 'played'):
                    if g['cards'].get(i, {}).get(k, 0) > 0:
                        tally[(i, k)] += 1
            for k, v in c.items():
                if k.startswith('unspent.') and '.antimatter.r' in k:
                    unspent[int(k[-2:])].append(v)
            last = collections.defaultdict(int)
            for k, v in c.items():
                if k.startswith('unspent.') and '.antimatter.r' in k:
                    s = k.split('.')[1]
                    r = int(k[-2:])
                    if r >= last.get(s + 'r', 0):
                        last[s + 'r'] = r
                        last[s] = v
            seats = len(g.get('start', [])) or 2
            seat_games += seats
            end_held += sum(1 for s in range(seats) if last.get(str(s), 0) > 0)
            ga = sum(x[AM] for x in g['gained'])
            sp = sum(x[AM] for x in g['spent'])
            gained += ga
            spent += sp
            b = by_level[g['difficulty']]
            b[0] += 1
            b[1] += ga
            b[2] += sp
            b[3] += 1 if g['won'] else 0
    return n, tally, unspent, end_held, seat_games, gained, spent, by_level


def main():
    argv = sys.argv[1:]
    also = []
    if '--also' in argv:
        i = argv.index('--also')
        also = argv[i + 1].split(',')
        del argv[i:i + 2]
    ids = sinks(also)
    reads = [a.split('=', 1) for a in argv]
    out = [(name, read(p, ids)) for name, p in reads]
    print('| Sink, per 100 games | ' + ' | '.join('%s offered / kept / played' % n for n, _ in out) + ' |')
    print('|---|' + '---|' * len(out))
    for i in ids:
        cells = []
        for _, (n, t, *_r) in out:
            cells.append('%.1f / %.1f / %.1f' % tuple(100.0 * t[(i, k)] / n for k in ('offered', 'kept', 'played')))
        print('| %s | %s |' % (i, ' | '.join(cells)))
    print()
    print('| Read | d | games | win % | antimatter gained / spent a seat-game | spent ÷ gained |')
    print('|---|---|---|---|---|---|')
    for name, (n, t, u, eh, sg, ga, sp, bl) in out:
        for d in sorted(bl):
            b = bl[d]
            print('| %s | d%d | %d | %.1f | %.2f / %.2f | %.0f%% |' % (
                name, d, b[0], 100.0 * b[3] / b[0], b[1] / b[0] / 2, b[2] / b[0] / 2, 100.0 * b[2] / max(b[1], 1e-9)))
        print('| %s | all | %d | | %.2f / %.2f | %.0f%% |' % (name, n, ga / n / 2, sp / n / 2, 100.0 * sp / max(ga, 1e-9)))
    print()
    print('Unspent antimatter at the end of each round (mean over seats reaching it); seat-games ending with any held:')
    for name, (n, t, u, eh, sg, *_r) in out:
        rows = ' '.join('r%d %.2f' % (r, sum(u[r]) / len(u[r])) for r in sorted(u) if u[r])
        print('- %s: %s; held at the end %.0f%%' % (name, rows, 100.0 * eh / sg))


if __name__ == '__main__':
    main()
