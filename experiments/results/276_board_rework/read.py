"""276: the paired read's power use and resource use, new (r131) against old (r130), from farmed JSONL files.

    python read.py NEW=SPEC OLD=SPEC [--agent strategist] [--uses]

Per board and level, each power's uses a seat-game (the seats playing that board), and its total a seat-round:
the decisions a board takes a round, Growth and tiers included. `--uses`: also the judge's resource use (gained and
spent a seat-round, share spent, top uses), pooled over levels, new side only, as 275's resources.py printed it.
Run from the repo root (it reads scripts/screens.py and content/ais.json for the board names).
"""
import argparse
import collections
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
BOARDS = ['builder', 'protector', 'researcher']


def seats_of(label):
    """The boards of a cell's seats (variant bp, br, pr: Builder, Protector, Researcher)."""
    v = label.split('/')[-2]
    return [BOARDS['bpr'.index(c)] for c in v]


def read(path, agent):
    games = stream(path)
    spec = next(games)['spec']
    labels = cell_labels(spec)
    use = collections.defaultdict(collections.Counter)  # (level, board) -> power slug -> uses
    seat_games = collections.Counter()  # (level, board)
    seat_rounds = collections.Counter()
    res = {'gained': collections.Counter(), 'spent': collections.Counter(), 'uses': collections.defaultdict(collections.Counter)}
    res_rounds = [0]
    for g in games:
        lab = labels[g['cell']]
        if not lab.endswith('/' + agent):
            continue
        lvl = int(lab.split('/')[0][1:])
        boards = seats_of(lab)
        for b in set(boards):
            n = boards.count(b)
            seat_games[(lvl, b)] += n
            seat_rounds[(lvl, b)] += n * g['rounds']
        for k, v in g['counters'].items():
            if k.startswith('ai.power.') and k.count('.') == 3:
                _, _, b, slug = k.split('.')
                use[(lvl, b)][slug] += v
        for s, (ga, sp) in enumerate(zip(g['gained'], g['spent'])):
            res_rounds[0] += g['rounds']
            for i, r in enumerate(PEAKS):
                res['gained'][r] += ga[i]
                res['spent'][r] += sp[i]
            for k, v in g['uses'][s].items():
                kind = k.split(':')[1] if k.startswith('own:') else k.split(':')[0]
                for i, r in enumerate(PEAKS):
                    if v[i]:
                        res['uses'][r][kind] += v[i]
    return use, seat_games, seat_rounds, res, res_rounds[0]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('new')
    ap.add_argument('old')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--uses', action='store_true')
    a = ap.parse_args()
    new = read(a.new, a.agent)
    old = read(a.old, a.agent)
    for b in BOARDS:
        print(f'\n### {b.title()} ({a.agent}): uses a seat-game by level; "all a round" is every power use a seat-round\n')
        levels = sorted({lvl for (lvl, bb) in new[1] if bb == b})
        names = sorted({k for (lvl, bb), c in new[0].items() if bb == b for k in c})
        print('| Power | ' + ' | '.join(f'd{lvl}' for lvl in levels) + ' |')
        print('|---|' + '---|' * len(levels))
        for k in names:
            row = [new[0][(lvl, b)][k] / max(new[1][(lvl, b)], 1) for lvl in levels]
            print(f'| {k} | ' + ' | '.join(f'{x:.2f}' for x in row) + ' |')
        for side, d in (('r131', new), ('r130', old)):
            row = [sum(d[0][(lvl, b)].values()) / max(d[2][(lvl, b)], 1) for lvl in levels]
            print(f'| **all a round, {side}** | ' + ' | '.join(f'{x:.2f}' for x in row) + ' |')
    if a.uses:
        _, _, _, res, sr = new
        _, _, _, ores, osr = old
        print(f'\n### Resource use ({a.agent}, pooled over levels): a seat-round\n')
        print('| Resource | Gained (r130) | Spent (r130) | Spent of gained (r130) | Top uses (r131) |')
        print('|---|---|---|---|---|')
        for r in PEAKS:
            if r == 'card_tokens':
                continue
            ga, sp = res['gained'][r] / sr, res['spent'][r] / sr
            oga, osp = ores['gained'][r] / osr, ores['spent'][r] / osr
            tot = sum(res['uses'][r].values()) or 1
            top = ', '.join(f'{k} {v / tot:.0%}' for k, v in res['uses'][r].most_common(3))
            share = f'{sp / ga:.0%}' if ga else '-'
            oshare = f'{osp / oga:.0%}' if oga else '-'
            print(f'| {r} | {ga:.2f} ({oga:.2f}) | {sp:.2f} ({osp:.2f}) | {share} ({oshare}) | {top} |')


main()
