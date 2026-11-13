"""211's reader: per cell and level, win %, the paired gap against a base cell, sales a seat-game (and the share in the
game's last round and in the adversary deck's last round), cube conversions and Breakthrough by board, and the
antimatter cards' plays a game.

    python experiments/results/211_agents_endgame_antimatter/read211.py GAMES.jsonl [--base KEY] [--vs A:B ...]

Cells are named by the spec's agent with `strategist:`/`greedy:` kept and the off switches dropped. --base names the
cell every other one is paired against, per agent kind (default: the cell with every 211 switch off). --vs 'A|B' adds
a paired row B against A (cell keys as printed).
"""
import collections
import json
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'scripts'))
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
from screens import cells_of  # noqa: E402

AM_CARDS = ['annihilation_lance', 'fusion_commons', 'nuclear_option', 'kinetic_interdiction', 'great_liberation',
            'singularity_exploit']


def short(agent):
    kind, _, params = agent.partition(':')
    keep = [p for p in params.split(',') if p and not p.endswith('=0') or p.startswith('w.')]
    return kind + (':' + ','.join(keep) if keep else '')


def seat_rounds(actions):
    """Each action's round (a draft's first tier pick after other actions starts a round)."""
    r, draft = 0, False
    out = []
    for a in actions:
        if a['type'] == 'tier' and not draft:
            r += 1
        draft = a['type'] in ('tier', 'draft', 'keep')
        out.append(r)
    return out


def read(path):
    lines = open(path, encoding='utf-8').read().splitlines()
    spec = json.loads(lines[0])['spec']
    cells = cells_of(spec)
    games = [json.loads(x) for x in lines[1:] if x.strip()]
    return cells, games


def mean_se(xs):
    n = len(xs)
    if n < 2:
        return (sum(xs) / max(n, 1), 0.0)
    m = sum(xs) / n
    v = sum((x - m) ** 2 for x in xs) / (n - 1)
    return m, math.sqrt(v / n)


def main():
    args = sys.argv[1:]
    vs = []
    base_key = None
    paths = []
    i = 0
    while i < len(args):
        if args[i] == '--vs':
            vs.append(tuple(args[i + 1].split('|', 1)))
            i += 2
        elif args[i] == '--base':
            base_key = args[i + 1]
            i += 2
        else:
            paths.append(args[i])
            i += 1
    by = collections.defaultdict(dict)  # (key, d) -> {(variant, seed): game}
    stats = collections.defaultdict(lambda: collections.Counter())
    seatgames = collections.Counter()
    boards_seen = collections.defaultdict(collections.Counter)
    for path in paths:
        cells, games = read(path)
        for g in games:
            c = cells[g['cell']]
            key, d = short(c['agent']), g['difficulty']
            by[(key, d)][(c['variant'], g['seed'])] = g
            st = stats[(key, d)]
            st['games'] += 1
            boards = [b.split(':')[0] for b in g['build'].split('|')]
            rounds = seat_rounds(g['actions'])
            last = g['rounds']
            for a, r in zip(g['actions'], rounds):
                t = a['type']
                s = a.get('seat', 0)
                b = boards[s] if s < len(boards) else '?'
                if t == 'sell':
                    st['sell'] += 1
                    st['sell_last'] += r >= last
                    st['sell_late'] += r >= last - 1
                    st['sell_r%d' % min(r, 9)] += 1
                elif t == 'standard' and a.get('act') in ('insight', 'antimatter', 'breakthrough'):
                    st[f"{a['act']}:{b}"] += 1
                elif t == 'play' and a.get('card') in AM_CARDS:
                    st['play:' + a['card']] += 1
            st['deck_last'] += sum(1 for a, r in zip(g['actions'], rounds) if a['type'] == 'sell'
                                   and g['margin'].get('deck_left', 1) == 0 and r >= last)
            for b in boards:
                boards_seen[(key, d)][b] += 1
            seatgames[(key, d)] += len(boards)
    keys = sorted({k for k, _ in by})
    diffs = sorted({d for _, d in by})
    print('| cell | d | games | win % | gap vs base (paired) | sells a seat-game | in the last two rounds | '
          'insight conv a seat-game (B/P/R) | antimatter made (B/P/R) | breakthrough a seat-game |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for k in keys:
        for d in diffs:
            if (k, d) not in by:
                continue
            gs = by[(k, d)]
            st = stats[(k, d)]
            n = st['games']
            win = 100 * sum(g['won'] for g in gs.values()) / n
            kind = k.split(':')[0]
            base = base_key if base_key and base_key.split(':')[0] == kind else kind
            gap = ''
            if base != k and (base, d) in by:
                bg = by[(base, d)]
                diffs_ = [100 * (gs[s]['won'] - bg[s]['won']) for s in gs if s in bg]
                m, se = mean_se(diffs_)
                gap = f'{m:+.1f} ± {se:.1f}'
            sg = seatgames[(k, d)]
            per_board = lambda act: '/'.join(
                f"{st[f'{act}:{b}'] / max(boards_seen[(k, d)][b], 1):.2f}" for b in ('builder', 'protector', 'researcher'))
            sells = st['sell']
            print(f"| {k} | d{d} | {n} | {win:.1f} | {gap} | {sells / sg:.3f} | "
                  f"{(st['sell_late'] / sells * 100 if sells else 0):.0f}% | {per_board('insight')} | "
                  f"{per_board('antimatter')} | "
                  f"{sum(v for kk, v in st.items() if kk.startswith('breakthrough:')) / sg:.2f} |")
    print()
    print('| cell | ' + ' | '.join(AM_CARDS) + ' | (plays a game, levels pooled) |')
    print('|---|' + '---|' * (len(AM_CARDS) + 1))
    for k in keys:
        tot = collections.Counter()
        n = 0
        for d in diffs:
            tot.update(stats[(k, d)])
            n += stats[(k, d)]['games']
        print(f'| {k} | ' + ' | '.join(f"{tot['play:' + c] / max(n, 1):.3f}" for c in AM_CARDS) + ' | |')
    print()
    print('| cell | sales by round, levels pooled (r1..r9+) |')
    print('|---|---|')
    for k in keys:
        tot = collections.Counter()
        for d in diffs:
            tot.update(stats[(k, d)])
        print(f'| {k} | ' + ' '.join(str(tot[f'sell_r{r}']) for r in range(1, 10)) + ' |')
    for a, b in vs:
        print()
        for d in diffs:
            if (a, d) in by and (b, d) in by:
                A, B = by[(a, d)], by[(b, d)]
                xs = [100 * (B[s]['won'] - A[s]['won']) for s in B if s in A]
                m, se = mean_se(xs)
                print(f'{b} vs {a} d{d}: {m:+.1f} ± {se:.1f} ({len(xs)} paired)')
        xs = []
        for d in diffs:
            if (a, d) in by and (b, d) in by:
                A, B = by[(a, d)], by[(b, d)]
                xs += [100 * (B[s]['won'] - A[s]['won']) for s in B if s in A]
        if xs:
            m, se = mean_se(xs)
            print(f'{b} vs {a} pooled: {m:+.1f} ± {se:.1f} ({len(xs)} paired)')


if __name__ == '__main__':
    main()
