"""210's tables, from reads' games.jsonl (an experiment's folder, or a farm's out file).

    python experiments/results/210_settle_r91/read210.py LABEL=DIR [LABEL=DIR ...] [--measures] [--std]

Main table, per lever (a variant's suffix, pairs pooled) and level, then levels pooled: win %, the paired gap
against its own read's base on the same seeds (a later read's base: against the first read's base), rounds, land
corruption tokens on the board a round, Lift land corruption a seat-game, troops removed and nodes lost a game.
--measures: the two standing measures (210) by board, for each read's base: slack and bound seat-rounds, and
where node placements land and how spread each seat's nodes end. --std: standard actions a seat-game.
"""
import json
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KINDS = ('factory', 'city', 'station', 'none', 'own_factory', 'own_city', 'own_station')


def load(d):
    path = d if d.endswith('.jsonl') else os.path.join(d, 'games.jsonl')
    out = defaultdict(lambda: defaultdict(dict))  # lever -> diff -> {(pair, seed): game}
    with open(path, encoding='utf-8') as f:
        h = json.loads(f.readline())['spec']
        names = [v['name'] for v in h['variants']]
        diffs = h['difficulties']
        for line in f:
            g = json.loads(line)
            v = names[g['cell'] // len(diffs)]
            pair, _, lever = v.partition('_')
            out[lever or 'base'][g['difficulty']][(pair, g['seed'])] = g
    return out


def boards(g):
    return [p.split(':')[0] for p in g['build'].split('|')]


def per_round(g, key):
    """A per-round counter's values, rounds 1.. (`key.rNN` in counters)."""
    c = g['counters']
    return [c.get(f'{key}.r{r:02d}', 0) for r in range(1, g['rounds'] + 1)]


def paired(cur, base):
    ks = [k for k in cur if k in base]
    d = [cur[k]['won'] - base[k]['won'] for k in ks]
    if not d:
        return ''
    m = sum(d) / len(d)
    se = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1) / len(d))
    return f'{100 * m:+.1f} ± {100 * se:.1f}'


def cells(games):
    n = len(games) or 1
    w = sum(g['won'] for g in games) / n
    se = math.sqrt(w * (1 - w) / n)
    seats = sum(len(boards(g)) for g in games) or 1
    land = [x for g in games for x in per_round(g, 'adversary.land_tokens')]
    c = lambda k: sum(g['counters'].get(k, 0) for g in games)
    return (f"{100 * w:5.1f} ±{100 * se:3.1f}", f"{sum(g['rounds'] for g in games) / n:4.2f}",
            f"{sum(land) / max(1, len(land)):.3f}", f"{c('board.std.lift') / seats:.3f}",
            f"{c('adversary.troops_removed') / n:4.2f}", f"{c('board.node_lost') / n:4.2f}")


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    reads = [(a.split('=', 1)[0], load(a.split('=', 1)[1])) for a in args]
    first = reads[0][1]
    print('| read | lever | d | win % | gap (paired) | rounds | land tokens a round | Lift a seat-game | troops removed | nodes lost |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for i, (label, r) in enumerate(reads):
        for lever in r:
            base = r['base'] if lever != 'base' else (first['base'] if i > 0 else None)
            for d in sorted(r[lever]) + ['all']:
                if d == 'all':
                    cur = {(dd,) + k: g for dd in r[lever] for k, g in r[lever][dd].items()}
                    b = {(dd,) + k: g for dd in (base or {}) for k, g in base[dd].items()} if base else {}
                else:
                    cur = r[lever][d]
                    b = base.get(d, {}) if base else {}
                gap = paired(cur, b) if base else ''
                cs = cells(list(cur.values()))
                dl = f'd{d}' if d != 'all' else '**all**'
                print(f'| {label} | {lever} | {dl} | {cs[0]} | {gap} | ' + ' | '.join(cs[1:]) + ' |')
    if '--std' in sys.argv:
        print('\n| read | lever | standard actions a seat-game (levels pooled) |')
        print('|---|---|---|')
        for label, r in reads:
            for lever in r:
                gs = [g for d in r[lever] for g in r[lever][d].values()]
                ns = sum(len(boards(g)) for g in gs) or 1
                tot = defaultdict(float)
                for g in gs:
                    for k, v in g['counters'].items():
                        if k.startswith('board.std.'):
                            tot[k[10:]] += v
                print(f'| {label} | {lever} | ' + ', '.join(f'{k} {v / ns:.2f}' for k, v in sorted(tot.items())) + ' |')
    if '--measures' in sys.argv:
        measures(reads)


def measures(reads):
    print('\n**Slack and bound seat-rounds** (base, levels pooled): slack = kept every card it drafted, held no card it')
    print('could have played, insight left over; bound = held a keep or a play for want of insight alone.\n')
    print('| read | board | seat-rounds | slack | bound | both |')
    print('|---|---|---|---|---|---|')
    for label, r in reads:
        acc = defaultdict(lambda: [0, 0, 0, 0])
        for d in r['base']:
            for g in r['base'][d].values():
                for s, b in enumerate(boards(g)):
                    sl = per_round(g, f'cards.slack.{s}')
                    bo = per_round(g, f'cards.insight_bound.{s}')
                    a = acc[b]
                    a[0] += len(sl)
                    a[1] += sum(x > 0 for x in sl)
                    a[2] += sum(x > 0 for x in bo)
                    a[3] += sum(x > 0 and y > 0 for x, y in zip(sl, bo))
        for b in sorted(acc) + ['all']:
            a = [sum(v[i] for v in acc.values()) for i in range(4)] if b == 'all' else acc[b]
            n = a[0] or 1
            print(f'| {label} | {b} | {a[0]} | {100 * a[1] / n:.1f}% | {100 * a[2] / n:.1f}% | {100 * a[3] / n:.1f}% |')
    print('\n**Node placement spread** (base, levels pooled). Placements: the share landing on or next to a factory, a')
    print('city, a research station (a placement may count for several), none of them; own: a site holding the seat\'s')
    print('cube. End: each seat\'s nodes on the board at the game\'s end, the countries holding them, their connected')
    print('groups, the farthest two apart (hexes).\n')
    print('| read | board | placements a seat-game | factory | city | station | none | own factory | own city | own station '
          '| nodes at end | countries | groups | farthest |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    for label, r in reads:
        acc = defaultdict(lambda: defaultdict(float))
        for d in r['base']:
            for g in r['base'][d].values():
                c = g['counters']
                spread = g['margin'].get('node_spread', [])
                for s, b in enumerate(boards(g)):
                    a = acc[b]
                    a['sg'] += 1
                    for k in ('n',) + KINDS:
                        a[k] += c.get(f'board.node_at.{s}.{k}', 0)
                    if s < len(spread):
                        for i, k in enumerate(('nodes', 'countries', 'groups', 'far')):
                            a[k] += spread[s][i]
        for b in sorted(acc) + ['all']:
            if b == 'all':
                a = defaultdict(float)
                for v in acc.values():
                    for k, x in v.items():
                        a[k] += x
            else:
                a = acc[b]
            sg, n = a['sg'] or 1, a['n'] or 1
            print(f"| {label} | {b} | {a['n'] / sg:.2f} | " + ' | '.join(f"{100 * a[k] / n:.0f}%" for k in KINDS)
                  + ' | ' + ' | '.join(f'{a[k] / sg:.2f}' for k in ('nodes', 'countries', 'groups', 'far')) + ' |')


main()
