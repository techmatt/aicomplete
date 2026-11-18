"""Each engine resource's use, per seat a game, by level, new against old (or one side alone). 200's reader, used by
189-233, promoted in 237.

    python scripts/bench/resource_use.py NEW [OLD] [--top 8] [--limits]

NEW and OLD are a run's folder (its games.jsonl), a games file with its header (a farmed read, `shards.sh`
output) or FILE=SPEC (a headerless farmed JSONL and its spec; `screens.stream`); a comma list pools several. From each game's `spent`, `gained`, `uses` and `sources` (in PEAKS order): for
chips, energy, research cubes and antimatter, gained and spent per seat a game at each level, the share of games in
which some seat spent antimatter and the share of won games; then the top uses (what each buys) and sources, pooled
over levels. `--limits` adds 189's limit table, per seat a game: `limit.keep_short.<r>` (a drafted card not kept
while its keep cost more than the seat held), `limit.play.short.<r>` (a card left in hand at the end of the action
phase lacking that resource; one card may count for two), `.needs_tags`, `.held` (playable, kept back).
"""
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from screens import PEAKS, ROUND, stream  # noqa: E402

SHOW = ['chips', 'energy', 'research_cubes', 'antimatter']


def load(paths):
    rows = []
    for p in paths.split(','):
        lines = stream(p)
        next(lines)  # the header
        rows.extend(lines)
    return rows


def summarize(rows):
    lv = defaultdict(lambda: {'n': 0, 'won': 0, 'seats': 0, 'spent': defaultdict(float), 'gained': defaultdict(float),
                              'any': defaultdict(int), 'won_any': defaultdict(int)})
    uses = defaultdict(float)
    srcs = defaultdict(float)
    limits = defaultdict(float)
    seats_all = 0
    for g in rows:
        for k, v in (g.get('counters') or {}).items():
            if k.startswith('limit.') and not ROUND.search(k):
                limits[k] += v
        L = lv[int(g['difficulty'])]
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
                L['won_any'][r] += won
        for into, key in ((uses, 'uses'), (srcs, 'sources')):
            for u in g.get(key) or []:
                for src, row in u.items():
                    for r in SHOW:
                        if row[PEAKS.index(r)]:
                            into[(r, src)] += row[PEAKS.index(r)]
    per = max(seats_all, 1)
    return (lv, {k: v / per for k, v in uses.items()}, {k: v / per for k, v in srcs.items()},
            {k: v / per for k, v in limits.items()})


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    argv = sys.argv[1:]
    top = 8
    limits = '--limits' in argv
    argv = [a for a in argv if a != '--limits']
    if '--top' in argv:
        i = argv.index('--top')
        top = int(argv[i + 1])
        del argv[i:i + 2]
    if not argv or len(argv) > 2:
        sys.exit(__doc__)
    sides = [('new', argv[0])] + ([('old', argv[1])] if len(argv) > 1 else [])
    res = {name: summarize(load(p)) for name, p in sides}
    print(f"Per seat a game. Sides: {', '.join(f'{n} = {p}' for n, p in sides)}.\n")
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
    if limits:
        print('\n## The limit, per seat a game\n')
        print('| counter | ' + ' | '.join(n for n, _ in sides) + ' |')
        print('|---|' + '---|' * len(sides))
        for k in sorted(set().union(*[set(res[n][3]) for n, _ in sides])):
            print(f'| {k} | ' + ' | '.join(f'{res[n][3].get(k, 0):.2f}' for n, _ in sides) + ' |')


if __name__ == '__main__':
    main()
