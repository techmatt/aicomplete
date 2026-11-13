"""201: the vibe measures, per cell (a variant's suffix after its pair, `base` for the bare pair), from games.jsonl.

    python experiments/results/201_threat_troop_pass/vibe_read.py DIR [DIR ...] [--cells a,b] [--diffs 0,1,2,3]

(It prints Markdown tables always; 208: the `--md` this line once promised was never a flag, so it is dropped.)

Per cell: win %, rounds; troops placed a game by source and by stage (rounds 1-4 / 5-8 / 9+ as a proxy for stage I /
II / III: the stage-II card arrives at round 5-6 by level), troop strikes (marches that reach a player's piece) and
what they destroyed, troops the players removed; troops on the map, botnets on the map, agents and ambassadors on the
map, by round (as production starts); botnets removed; countries where agents outnumber ambassadors and the first
round that happens; games lost to troops (a node-loss loss with a troop node kill in its last round); and win % and
troop losses in games where the players answered troops (removed at least a quarter of those placed) against games
where they didn't.
"""
import argparse
import json
from collections import defaultdict

import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
SRC = ['factory', 'muster', 'wall', 'threat', 'project']


def rk(c, key, r):
    return c.get(f'{key}.r{r:02d}', 0)


def per_game(g):
    c = g['counters']
    R = g['rounds']
    out = {'won': bool(g['won']), 'rounds': R, 'reason': g.get('reason', '')}
    placed = 0
    for s in SRC:
        n = sum(rk(c, f'map.troops_placed.{s}', r) for r in range(1, R + 2))
        out[f'placed.{s}'] = n
        placed += n
    out['placed'] = placed
    for lo, hi, name in ((1, 4, 'I'), (5, 8, 'II'), (9, 30, 'III')):
        out[f'placed.{name}'] = sum(rk(c, f'map.troops_placed.{s}', r) for s in SRC for r in range(lo, hi + 1))
    out['strikes'] = sum(rk(c, 'map.troop_strikes', r) for r in range(1, R + 2))
    for k in ('node', 'factory', 'station'):
        out[f'hit.{k}'] = sum(rk(c, f'map.troop_destroyed.{k}', r) for r in range(1, R + 2))
    out['removed'] = sum(rk(c, 'map.troops_removed', r) for r in range(1, R + 2))
    out['botnets_removed'] = sum(rk(c, 'map.botnets_removed', r) for r in range(1, R + 2))
    for key in ('troops', 'botnets', 'agents', 'ambassadors', 'agents_over', 'ambassadors_held', 'agent_pool'):
        for r in range(1, R + 1):
            out[f'{key}.r{r}'] = rk(c, f'map.{key}', r)
    out['troops_end'] = rk(c, 'map.troops', R)
    out['vigil'] = c.get('board.vigil', 0)
    over = [r for r in range(1, R + 1) if rk(c, 'map.agents_over', r) > 0]
    out['first_over'] = over[0] if over else None
    last_node_kill = rk(c, 'map.troop_destroyed.node', R) + rk(c, 'map.troop_destroyed.node', R + 1)
    out['lost_to_troops'] = (not out['won']) and 'node' in out['reason'].lower() and last_node_kill > 0
    out['answered'] = placed > 0 and out['removed'] * 4 >= placed
    return out


def read(dirs, diffs):
    cells = defaultdict(list)
    for d in dirs:
        with open(f'{d}/games.jsonl', encoding='utf-8') as f:
            head = json.loads(next(f))
            sp = head['spec']
            names = [v['name'] for v in sp['variants']]
            ds = sp['difficulties']
            ags = sp.get('agents') or ['?']
            for line in f:
                g = json.loads(line)
                if int(g['difficulty']) not in diffs:
                    continue
                cell = int(g['cell'])
                v = names[cell // (len(ds) * len(ags)) % len(names)] if len(names) > 1 else names[0]
                lever = v.partition('_')[2] or 'base'
                cells[lever].append(per_game(g))
    return cells


def mean(xs):
    xs = list(xs)
    return sum(xs) / len(xs) if xs else float('nan')


def by_round(games, key, rounds=10):
    out = []
    for r in range(1, rounds + 1):
        live = [g[f'{key}.r{r}'] for g in games if f'{key}.r{r}' in g]
        out.append(f'{mean(live):.1f}' if len(live) >= max(5, len(games) // 10) else '-')
    return ' / '.join(out)


def report(cells, order):
    rows = []
    for name in order:
        gs = cells.get(name)
        if not gs:
            continue
        n = len(gs)
        ans = [g for g in gs if g['answered']]
        ign = [g for g in gs if not g['answered']]
        rows.append(f'### {name} ({n} games)')
        rows.append('')
        rows.append(f"- win {100 * mean(g['won'] for g in gs):.1f}%, rounds {mean(g['rounds'] for g in gs):.2f}")
        rows.append('- troops placed a game: ' + f"{mean(g['placed'] for g in gs):.1f} (" + ', '.join(
            f"{s} {mean(g['placed.' + s] for g in gs):.1f}" for s in SRC) + '); by round band 1-4 / 5-8 / 9+: ' + ' / '.join(
            f"{mean(g['placed.' + s] for g in gs):.1f}" for s in ('I', 'II', 'III')))
        rows.append(f"- marches reaching a player (strikes) {mean(g['strikes'] for g in gs):.2f}; destroyed: nodes "
                    f"{mean(g['hit.node'] for g in gs):.2f}, factories {mean(g['hit.factory'] for g in gs):.2f}, stations "
                    f"{mean(g['hit.station'] for g in gs):.2f}; troops removed by players {mean(g['removed'] for g in gs):.2f}")
        rows.append(f"- games lost to troops {100 * mean(g['lost_to_troops'] for g in gs):.1f}% of games; troops on the map "
                    f"in the last round {mean(g['troops_end'] for g in gs):.2f} (won games "
                    f"{mean(g['troops_end'] for g in gs if g['won']):.2f}); Vigil insight a game "
                    f"{mean(g['vigil'] for g in gs):.2f} (all pairs; the Protector sits in two of three)")
        rows.append(f"- answered troops (removed >= 1/4 of placed): {len(ans)} games, win {100 * mean(g['won'] for g in ans):.1f}%, "
                    f"nodes lost to troops {mean(g['hit.node'] for g in ans):.2f}; ignored: {len(ign)} games, win "
                    f"{100 * mean(g['won'] for g in ign):.1f}%, nodes lost to troops {mean(g['hit.node'] for g in ign):.2f}, "
                    f"lost to troops {100 * mean(g['lost_to_troops'] for g in ign):.1f}%")
        rows.append(f"- troops on the map by round: {by_round(gs, 'troops')}")
        rows.append(f"- botnets on the map by round: {by_round(gs, 'botnets')}; removed a game {mean(g['botnets_removed'] for g in gs):.2f}")
        rows.append(f"- agents on the map by round: {by_round(gs, 'agents')}")
        rows.append(f"- player ambassadors on the map by round: {by_round(gs, 'ambassadors')}")
        rows.append(f"- agents waiting in the pools by round: {by_round(gs, 'agent_pool')}")
        rows.append(f"- player ambassadors in hand by round: {by_round(gs, 'ambassadors_held')}")
        rows.append(f"- countries with agents over ambassadors (on the map) by round: {by_round(gs, 'agents_over')}")
        fo = [g['first_over'] for g in gs if g['first_over']]
        rows.append(f"- games where agents ever outnumber ambassadors in a country {100 * len(fo) / n:.0f}%, first round "
                    f"{mean(fo):.1f}")
        rows.append('')
    return '\n'.join(rows)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--cells')
    ap.add_argument('--diffs', default='0,1,2,3')
    a = ap.parse_args()
    cells = read(a.dirs, {int(x) for x in a.diffs.split(',')})
    order = a.cells.split(',') if a.cells else sorted(cells, key=lambda k: (k != 'base', k))
    print(report(cells, order))


if __name__ == '__main__':
    main()
