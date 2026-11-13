"""203: dead keeps and plant plays a game, by level and agent, with win %; paired against a base on the same games.

    python keep_read.py RUN [RUN...] [--base BASE] [--agent-split]

RUN is a games.jsonl or a folder holding one; several pool. A label `RUN=NAME` names it. With --base, each RUN is
read on the (cell, seed) games both played, and the win gap is paired.
Each RUN's games may hold several agents (a screen's levers); the table splits by the game's agent (cell name).
Definitions follow 199's plan_read.py: a dead keep is a drafted keep never played (`cards.keep_at.<s>.<id>` above 0 at
the end, or `cards.keep_plan.left_*` when it left the hand unplayed); "held back" if it was ever playable at the end
of an action phase (`cards.keep_ok`), else "never had"; keeps in the game's last round count apart (not in either).
Plants: Penning Trap Array and Antiproton Foundry, kept / played (`cards`) and their Action's uses (`uses`
`own:use:<id>`), both seats.
"""
import json
import math
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PLANTS = ('penning_trap_array', 'antiproton_foundry')
KEEP_AT = re.compile(r'^cards\.keep_at\.(\d+)\.(.+)$')


def games(path):
    """Each game with its cell (`cells_of` on the header) as `variant`, `d` and `agent`."""
    path = path if os.path.isfile(path) else os.path.join(path, 'games.jsonl')
    cells = []
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'spec' in g:
                s = g['spec']
                cells = [(v['name'], d, a) for _ in s['configs'] for v in s['variants']
                         for d in s['difficulties'] for a in s['agents']]
            elif 'counters' in g:
                g['variant'], g['d'], g['agent'] = cells[g['cell']]
                yield g


def measures(g):
    c = g['counters']
    m = defaultdict(float)
    for k, v in c.items():
        x = KEEP_AT.match(k)
        if x and v > 0:
            s, card = x.group(1), x.group(2)
            if v >= g.get('rounds', 0):
                m['dead_last'] += 1
            elif c.get(f'cards.keep_ok.{s}.{card}', 0) > 0:
                m['dead_held'] += 1
            else:
                m['dead_never'] += 1
    m['dead_held'] += c.get('cards.keep_plan.left_held', 0)
    m['dead_never'] += c.get('cards.keep_plan.left_never', 0)
    m['kept_played'] = c.get('cards.keep_plan.played', 0)
    for p in PLANTS:
        cd = g.get('cards', {}).get(p, {})
        m['plant_kept'] += cd.get('kept', 0)
        m['plant_played'] += cd.get('played', 0)
        for seat in g.get('uses', []):
            m['plant_uses'] += sum(seat.get(f'own:use:{p}', []))
    m['won'] = 1.0 if g['won'] else 0.0
    m['rounds'] = g['rounds']
    return m


def load(spec):
    path, _, name = spec.partition('=')
    out = {}
    for g in games(path):
        out[(f"d{g['d']}", g['agent'], g['variant'], g['seed'])] = measures(g)
    return name or path, out


COLS = ['dead_held', 'dead_never', 'dead_last', 'kept_played', 'plant_kept', 'plant_played', 'plant_uses', 'rounds']


def main():
    args = sys.argv[1:]
    base = None
    if '--base' in args:
        i = args.index('--base')
        base = load(args[i + 1])
        del args[i:i + 2]
    runs = [load(a) for a in args]
    print('| run | d | agent | games | win % | gap ± SE (paired) | ' + ' | '.join(COLS) + ' |')
    print('|' + '---|' * (6 + len(COLS)))
    for name, run in runs:
        groups = defaultdict(list)
        for (d, agent, variant, seed), m in run.items():
            groups[(d, agent)].append((variant, seed, m))
        for (d, agent) in sorted(groups):
            rows = groups[(d, agent)]
            gap = ''
            if base is not None:
                bkeys = {}
                for (bd, ba, bv, bs), bm in base[1].items():
                    # a lever `strategist:x=1` pairs with base's `strategist`
                    if bd == d and ba == agent.split(':')[0]:
                        bkeys[(bv, bs)] = bm
                pairs = [(m, bkeys[(v, s)]) for v, s, m in rows if (v, s) in bkeys]
                rows = [(v, s, m) for v, s, m in rows if (v, s) in bkeys]
                if pairs:
                    diffs = [a['won'] - b['won'] for a, b in pairs]
                    mean = sum(diffs) / len(diffs)
                    var = sum((x - mean) ** 2 for x in diffs) / max(1, len(diffs) - 1)
                    gap = f"{100 * mean:+.1f} ± {100 * math.sqrt(var / len(diffs)):.1f}"
            n = len(rows)
            if not n:
                continue
            avg = lambda k: sum(m[k] for _, _, m in rows) / n
            print(f"| {name} | {d} | {agent} | {n} | {100 * avg('won'):.1f} | {gap} | "
                  + ' | '.join(f'{avg(k):.2f}' for k in COLS) + ' |')


main()
