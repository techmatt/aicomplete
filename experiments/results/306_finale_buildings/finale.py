"""306: the finale buildings in farmed reads: per 100 games offered / kept / played / kept-never-played and Action
uses; uses per seat that built one; the round each is first built (online) and first used.

    python experiments/results/306_finale_buildings/finale.py NAME=FILE [NAME=FILE ...] [--cards a,b,...]

FILE is a farmed games JSONL (a header line, if any, is skipped). Rounds are counted from the action log: each
draft opens with `tier` picks, so a round is a run of `tier` actions and what follows it (round 1 holds the
opening; checked on the screen: it matches the game's `rounds` but in losses that end before the next round's draft,
one short there). "By r3" is the share of first builds in rounds 1-3, before mid-game (games run 6-8 rounds).
Kept-never-played counts a game where a seat kept it and no seat played it.
"""
import collections
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
FINALE = ['siegebreaker_reactor', 'concord_reactor', 'fusion_torch_array', 'singularity_exploit']
OLD = ['kinetic_interdiction', 'orbital_relay_launcher', 'containment_reactor']


def read(path, ids):
    n = 0
    t = collections.Counter()
    first_play = collections.defaultdict(list)
    first_use = collections.defaultdict(list)
    uses_built = collections.defaultdict(list)
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            n += 1
            for i in ids:
                c = g['cards'].get(i, {})
                for k in ('offered', 'kept', 'played'):
                    if c.get(k, 0) > 0:
                        t[(i, k)] += 1
                if c.get('kept', 0) > 0 and c.get('played', 0) == 0:
                    t[(i, 'knp')] += 1
            r = 0
            prev = None
            played = {}
            used = collections.Counter()
            fu = {}
            for a in g['actions']:
                if a['type'] == 'tier' and prev != 'tier':
                    r += 1
                prev = a['type']
                c = a.get('card')
                if c not in ids:
                    continue
                key = (c, a['seat'])
                if a['type'] == 'play' and key not in played:
                    played[key] = r
                if a['type'] == 'use':
                    used[key] += 1
                    t[(c, 'uses')] += 1
                    fu.setdefault(key, r)
            for key, pr in played.items():
                first_play[key[0]].append(pr)
                uses_built[key[0]].append(used[key])
            for key, ur in fu.items():
                first_use[key[0]].append(ur)
    return n, t, first_play, first_use, uses_built


def main():
    args = sys.argv[1:]
    ids = FINALE + OLD
    if '--cards' in args:
        i = args.index('--cards')
        ids = args[i + 1].split(',')
        del args[i:i + 2]
    out = [(name, read(p, ids)) for name, p in (a.split('=', 1) for a in args)]
    print('| Card | Read | offered / kept / played / kept-never-played per 100 games | Action uses per 100 games |'
          ' uses a seat that built it | first built: mean round, by r3 | first use: mean round |')
    print('|---|---|---|---|---|---|---|')
    for i in ids:
        for name, (n, t, fp, fu, ub) in out:
            q = [100.0 * t[(i, k)] / n for k in ('offered', 'kept', 'played', 'knp')]
            p = fp.get(i, [])
            u = fu.get(i, [])
            b = ub.get(i, [])
            built = ('%.1f, %.0f%%' % (sum(p) / len(p), 100.0 * sum(1 for x in p if x <= 3) / len(p))) if p else '-'
            print('| %s | %s | %.1f / %.1f / %.1f / %.1f | %.1f | %s | %s | %s |' % (
                i, name, *q, 100.0 * t[(i, 'uses')] / n,
                '%.2f' % (sum(b) / len(b)) if b else '-', built,
                '%.1f' % (sum(u) / len(u)) if u else '-'))


if __name__ == '__main__':
    main()
