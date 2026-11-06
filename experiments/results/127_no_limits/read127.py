"""127's read: wins and rounds (paired against a base on pair and seed), each resource's peaks per board (mean,
highest, flagged), credits gained a game, energy spent and held, the changed cards' plays a game, and for the flagged
seat-games the sources that fed the pile.

    python experiments/results/127_no_limits/read127.py DIR[=TAG]... [--base TAG] [--cards a,b] [--sources N]
        [--res credits,energy,...] [--agent strategist]

Each DIR is an `aic-sim experiment` output (`games.jsonl`). Variant names are `PAIR` or `PAIR-LEVER` (`bp`, `br-embassy`):
a row's label is `TAG LEVER/AGENT`. Flagged: a peak's rise over setup's `start` above 3x the seat's income a round
(`gained / rounds`) and above 10, as `scripts/cards/runaway_read.py` reads it.
"""
import argparse
import collections
import math
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
LEFT = ['credits', 'chips', 'energy', 'antimatter', 'ambassadors']


def load(arg):
    d, _, tag = arg.partition('=')
    tag = tag or os.path.basename(d.rstrip('/\\'))
    lines = stream(d)
    labels = cell_labels(next(lines)['spec'])
    out = []
    for g in lines:
        variant, agent = labels[g['cell']].split('/')[-2:]
        pair, _, lever = variant.partition('-')
        out.append((f'{tag} {lever}'.strip() + f'/{agent}', pair, g))
    return out


def flagged(g, s, i):
    rise = g['peaks'][s][i] - (g.get('start') or [[0] * 8] * 2)[s][i]
    inc = g['gained'][s][i] / max(g['rounds'], 1)
    return inc > 0 and rise > 3 * inc and rise > 10


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--cards', default='')
    ap.add_argument('--sources', type=int, default=0, help='top sources of each flagged pile')
    ap.add_argument('--res', default='credits,insight,energy,chips,ambassadors')
    ap.add_argument('--agent')
    a = ap.parse_args()
    rows = collections.defaultdict(list)
    for d in a.dirs:
        for label, pair, g in load(d):
            if a.agent and not label.endswith('/' + a.agent):
                continue
            rows[label].append((pair, g))
    labels = list(rows)
    base = {}
    for label in labels:
        agent = label.split('/')[1]
        if a.base:
            base.setdefault(agent, f'{a.base}/{agent}')
        else:
            base.setdefault(agent, label)

    print('## Wins and rounds\n')
    print('| run | games | win % | by pair (bp / br / pr) | rounds | gap vs base ± SE (paired) |')
    print('|---|---|---|---|---|---|')
    for label in labels:
        gs = rows[label]
        by = collections.defaultdict(list)
        for pair, g in gs:
            by[pair].append(g['won'])
        pairs = ' / '.join(f'{100 * sum(by[p]) / len(by[p]):.0f}' for p in ('bp', 'br', 'pr') if by[p])
        b = base[label.split('/')[1]]
        gap = ''
        if b != label and b in rows:
            bw = {(p, g['seed']): g['won'] for p, g in rows[b]}
            diffs = [int(g['won']) - int(bw[(p, g['seed'])]) for p, g in gs if (p, g['seed']) in bw]
            if len(diffs) > 1:
                m = sum(diffs) / len(diffs)
                sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / (len(diffs) - 1))
                gap = f'{100 * m:+.1f} ± {100 * sd / math.sqrt(len(diffs)):.1f} ({len(diffs)})'
        print(f'| {label} | {len(gs)} | {100 * sum(g["won"] for _, g in gs) / len(gs):.1f} | {pairs} | '
              f'{sum(g["rounds"] for _, g in gs) / len(gs):.2f} | {gap} |')

    res = a.res.split(',')
    print('\n## Peaks per seat by board: mean (highest game) [flagged / seats]\n')
    print('| run | board | ' + ' | '.join(res) + ' |')
    print('|---|---|' + '---|' * len(res))
    for label in labels:
        acc = collections.defaultdict(list)
        for pair, g in rows[label]:
            for s in range(2):
                acc[BOARD[pair[s]]].append((g, s))
        for board in sorted(acc):
            cells = []
            for r in res:
                i = PEAKS.index(r)
                ps = [g['peaks'][s][i] for g, s in acc[board]]
                fl = sum(flagged(g, s, i) for g, s in acc[board])
                cells.append(f'{sum(ps) / len(ps):.1f} ({max(ps)}) [{fl}/{len(ps)}]')
            print(f'| {label} | {board} | ' + ' | '.join(cells) + ' |')

    print('\n## Credits gained a game, energy spent and held, per seat by board\n')
    print('| run | board | credits gained a game (max) | a round | energy gained | energy spent | '
          'energy held after the last action phase | energy peak |')
    print('|---|---|---|---|---|---|---|---|')
    for label in labels:
        acc = collections.defaultdict(list)
        for pair, g in rows[label]:
            for s in range(2):
                acc[BOARD[pair[s]]].append((g, s))
        for board in sorted(acc):
            xs = acc[board]
            n = len(xs)
            cr = [g['gained'][s][0] for g, s in xs]
            crr = sum(g['gained'][s][0] / max(g['rounds'], 1) for g, s in xs) / n
            eg = sum(g['gained'][s][2] for g, s in xs) / n
            es = sum(g['spent'][s][2] for g, s in xs) / n
            eh = sum(g['margin']['held_after_actions'][s][LEFT.index('energy')] for g, s in xs) / n
            ep = sum(g['peaks'][s][2] for g, s in xs) / n
            print(f'| {label} | {board} | {sum(cr) / n:.0f} ({max(cr)}) | {crr:.1f} | {eg:.1f} | {es:.1f} | '
                  f'{eh:.2f} | {ep:.1f} |')

    cards = [c for c in a.cards.split(',') if c]
    if cards:
        print('\n## Plays a game of the changed cards\n')
        print('| run | ' + ' | '.join(cards) + ' |')
        print('|---|' + '---|' * len(cards))
        for label in labels:
            gs = rows[label]
            cells = []
            for c in cards:
                played = sum(g['cards'].get(c, {}).get('played', 0) for _, g in gs)
                cells.append(f'{played / len(gs):.3f}')
            print(f'| {label} | ' + ' | '.join(cells) + ' |')

    if a.sources:
        print('\n## What fed the flagged piles (summed over the flagged seat-games)\n')
        for label in labels:
            for r in res:
                i = PEAKS.index(r)
                tot = collections.Counter()
                n = 0
                worst = []
                for pair, g in rows[label]:
                    for s in range(2):
                        if flagged(g, s, i):
                            n += 1
                            worst.append((g['peaks'][s][i], BOARD[pair[s]], g['seed'], g['rounds']))
                            for k, v in g['sources'][s].items():
                                if v[i] > 0:
                                    tot[k] += v[i]
                if not n:
                    continue
                worst.sort(reverse=True)
                top = ', '.join(f'{k} {v / n:.1f}' for k, v in tot.most_common(a.sources))
                ws = '; '.join(f'{p} ({b}, {sd}, {rd} r)' for p, b, sd, rd in worst[:3])
                print(f'- **{label} {r}** ({n} seat-games; top: {ws}): {top}')


if __name__ == '__main__':
    main()
