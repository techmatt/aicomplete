"""200: each engine resource's use, per seat a game, by level, new against old (or one side alone).

    python experiments/results/200_engine_overhaul/resource_use.py NEW_DIR [OLD_DIR] [--top 8] [--by-board]

From games.jsonl (`spent`, `gained`, `uses`, `sources`, in PEAKS order). For chips, energy, research cubes and
antimatter: gained and spent per seat a game at each level, the share of games in which some seat spent any, and the
share of WON games in which some seat spent any (the done-when's antimatter bar); then the top uses (what each buys)
and sources, pooled over levels. A DIR may be a comma list of directories, pooled.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
SHOW = ['chips', 'energy', 'research_cubes', 'antimatter']
BOARDS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}


def load(dirs):
    rows = []
    for d in dirs.split(','):
        with open(f'{d}/games.jsonl', encoding='utf-8') as f:
            head = json.loads(next(f))
            names = [v['name'] for v in head['spec']['variants']]
            for line in f:
                g = json.loads(line)
                rows.append((g, names))
    return rows


def cell_pair(g, names):
    # cells are numbered variant-major: cell = variant * levels + level index (as the experiment lays them out)
    return None


def summarize(rows, by_board=False):
    lv = defaultdict(lambda: {'n': 0, 'won': 0, 'seats': 0, 'spent': defaultdict(float), 'gained': defaultdict(float),
                              'any': defaultdict(int), 'won_any': defaultdict(int)})
    uses = defaultdict(float)
    srcs = defaultdict(float)
    seats_all = 0
    for g, _ in rows:
        d = int(g['difficulty'])
        L = lv[d]
        L['n'] += 1
        won = bool(g['won'])
        L['won'] += won
        L['seats'] += len(g['spent'])
        seats_all += len(g['spent'])
        for r in SHOW:
            i = PEAKS.index(r)
            s = sum(row[i] for row in g['spent'])
            L['spent'][r] += s
            L['gained'][r] += sum(row[i] for row in g['gained'])
            if s > 0:
                L['any'][r] += 1
                if won:
                    L['won_any'][r] += 1
        for u in g.get('uses') or []:
            for src, row in u.items():
                for r in SHOW:
                    v = row[PEAKS.index(r)]
                    if v:
                        uses[(r, src)] += v
        for u in g.get('sources') or []:
            for src, row in u.items():
                for r in SHOW:
                    v = row[PEAKS.index(r)]
                    if v:
                        srcs[(r, src)] += v
    return lv, {k: v / max(seats_all, 1) for k, v in uses.items()}, {k: v / max(seats_all, 1) for k, v in srcs.items()}


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    top = 8
    if '--top' in sys.argv:
        top = int(sys.argv[sys.argv.index('--top') + 1])
        args = [a for a in args if a != str(top)]
    sides = [('new', args[0])] + ([('old', args[1])] if len(args) > 1 else [])
    res = {name: summarize(load(d)) for name, d in sides}
    print(f"Per seat a game. Sides: {', '.join(f'{n} = {d}' for n, d in sides)}.\n")
    print('| d | side | games | win % | ' + ' | '.join(f'{r} gained / spent' for r in SHOW)
          + ' | antimatter spent: games / won games |')
    print('|---|---|---|---|' + '---|' * len(SHOW) + '---|')
    levels = sorted(set().union(*[set(r[0].keys()) for r in res.values()]))
    for d in levels:
        for name, _ in sides:
            L = res[name][0].get(d)
            if not L:
                continue
            s = max(L['seats'], 1)
            cells = ' | '.join(f"{L['gained'][r] / s:.2f} / {L['spent'][r] / s:.2f}" for r in SHOW)
            am = f"{100 * L['any']['antimatter'] / L['n']:.0f}% / {100 * L['won_any']['antimatter'] / max(L['won'], 1):.0f}%"
            print(f"| d{d} | {name} | {L['n']} | {100 * L['won'] / L['n']:.1f} | {cells} | {am} |")
    for title, idx in (('What each resource buys (uses)', 1), ('Where it comes from (sources)', 2)):
        print(f'\n## {title}, per seat a game, all levels\n')
        print('| resource | src | ' + ' | '.join(n for n, _ in sides) + ' |')
        print('|---|---|' + '---|' * len(sides))
        for r in SHOW:
            keys = defaultdict(float)
            for name, _ in sides:
                for (rr, src), v in res[name][idx].items():
                    if rr == r:
                        keys[src] = max(keys[src], v)
            for src in sorted(keys, key=lambda k: -keys[k])[:top]:
                vals = ' | '.join(f"{res[name][idx].get((r, src), 0):.2f}" for name, _ in sides)
                print(f'| {r} | {src} | {vals} |')


if __name__ == '__main__':
    main()
