"""177: assemble a farmed run and read its levers by agent and level: win %, the paired delta against base on the
same seeds, and the share of losses by cause.

    python experiments/results/177_recalibration/screen177.py asm SPEC FIRST COUNT SHARDS OUT_DIR
    python experiments/results/177_recalibration/screen177.py read DIR [DIR ...] [--base LEVER] [--causes]

`asm` writes OUT_DIR/games.jsonl (screens.assemble). `read` pools the DIRs (independent seed sets: a seed pairs only
within its own DIR) and prints each lever's win % ± SE (games) at each level and agent, with its paired delta
against `--base` (default `base`) on the same (dir, pair, seed). `--causes` adds a table a lever: the share of
losses that are the round cap / the Discord deck / the last node, by level. Variants are `PAIR` or `PAIR_LEVER`
(specs177.py).
"""
import math
import os
import sys
from collections import defaultdict, Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import assemble, cells_of, stream  # noqa: E402

CAUSES = [('deck_empty', 'round cap'), ('discord_deck', 'Discord deck'), ('node', 'last node')]


def load(dirs):
    won = defaultdict(dict)  # (lever, agent, level) -> {(dir, pair, seed): won}
    why = defaultdict(Counter)  # (lever, agent, level) -> loss reasons
    for i, d in enumerate(dirs):
        it = stream(d)
        cells = cells_of(next(it)['spec'])
        for g in it:
            if 'cell' not in g:
                continue
            c = cells[g['cell']]
            pair, _, lever = c['variant'].partition('_')
            agent = c['agent'].split(':')[0] if c['agent'].startswith('oracle') else c['agent']
            k = (lever or 'base', agent, c['difficulty'])
            won[k][(i, pair, g['seed'])] = int(g['won'])
            if not g['won']:
                why[k][g.get('reason', '?')] += 1
    return won, why


def stats(xs):
    n = len(xs)
    m = sum(xs) / n
    se = math.sqrt(sum((x - m) ** 2 for x in xs) / max(1, n - 1) / n)
    return m, se, n


def read(dirs, base, causes):
    s, why = load(dirs)
    levers = sorted({k[0] for k in s}, key=lambda x: (x != base, x))
    agents = sorted({k[1] for k in s})
    levels = sorted({k[2] for k in s})
    for agent in agents:
        print(f'## {agent}')
        print()
        print('| Lever | ' + ' | '.join(f'd{l}' for l in levels) + ' |')
        print('|---|' + '---|' * len(levels))
        for lever in levers:
            row = []
            for lvl in levels:
                a = s.get((lever, agent, lvl))
                if not a:
                    row.append('-')
                    continue
                m, se, n = stats([100.0 * v for v in a.values()])
                cell = f'{m:.1f} ± {se:.1f} ({n})'
                b = s.get((base, agent, lvl))
                if lever != base and b:
                    keys = [k for k in a if k in b]
                    if keys:
                        dm, dse, _ = stats([100.0 * (a[k] - b[k]) for k in keys])
                        cell += f' [{dm:+.1f} ± {dse:.1f}]'
                row.append(cell)
            print(f'| {lever} | ' + ' | '.join(row) + ' |')
        print()
        if not causes:
            continue
        print(f'Losses by cause, % of losses ({" / ".join(n for _, n in CAUSES)}; other):')
        print()
        print('| Lever | ' + ' | '.join(f'd{l}' for l in levels) + ' |')
        print('|---|' + '---|' * len(levels))
        for lever in levers:
            row = []
            for lvl in levels:
                w = why.get((lever, agent, lvl))
                lost = sum(w.values()) if w else 0
                if not lost:
                    row.append('-')
                    continue
                parts = [f'{100 * w[c] / lost:.0f}' for c, _ in CAUSES]
                other = lost - sum(w[c] for c, _ in CAUSES)
                row.append(' / '.join(parts) + (f'; {other}' if other else '') + f' ({lost})')
            print(f'| {lever} | ' + ' | '.join(row) + ' |')
        print()


def main():
    a = sys.argv[1:]
    if a[0] == 'asm':
        assemble(a[1], int(a[2]), int(a[3]), [a[4]], a[5])
        print(os.path.join(a[5], 'games.jsonl'))
    elif a[0] == 'read':
        base = 'base'
        causes = '--causes' in a
        if causes:
            a.remove('--causes')
        if '--base' in a:
            i = a.index('--base')
            base = a[i + 1]
            del a[i:i + 2]
        read(a[1:], base, causes)
    else:
        sys.exit('asm or read')


if __name__ == '__main__':
    main()
