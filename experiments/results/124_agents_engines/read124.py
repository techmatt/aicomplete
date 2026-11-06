"""124's read: per agent cell (the spec's agent label) of one or more experiment dirs, pooled over the pairs.

Reads r49-r50 runs only: 127 (r51) removed the holding limits and the `economy.hold_lost*` counters.

    python experiments/results/124_agents_engines/read124.py DIR[=TAG]... [--base LABEL] [--cards all|engine|none]
        [--md] [--pairs]

Each DIR's `games.jsonl` (an `aic-sim experiment` output). A label is `TAG/AGENT` when a dir is given `=TAG`, else
the spec's agent. `--base`: the label every other is paired against on the same pair and seed (the win gap and its
SE, the rounds gap); default the first label. Prints:

- win %, rounds, the paired gap against the base, and per pair (`--pairs`);
- what the holding limits cut a game (`economy.hold_lost.*`, `economy.production_lost.credits`), and each
  resource's mean and highest peak per board;
- the Researcher's keep rate (kept / drafted after the opening deal) and tier-2 share (of its tier choices),
  split by its insight flag: a peak over 3x a round's insight income and over 10 (122's rule), with the mean peak;
- plays (and keeps, offers) a game of the r41 engine cards (`aic-sim cards`' engine column) and the two the
  prompt names (Hyperscale Botnet, Moonshot Grant).
"""
import argparse
import collections
import math
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
SHOW = ['credits', 'insight', 'energy', 'chips', 'ambassadors']
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
NAMED = ['hyperscale_botnet', 'moonshot_grant']


def engine_cards(binary):
    out = subprocess.run([binary, 'cards'], capture_output=True, text=True, check=True).stdout.splitlines()
    head = out[0].split(',')
    i, e = head.index('id'), head.index('engine')
    return [r.split(',')[i] for r in out[1:] if r.split(',')[e] == '1']


def keep_split(g, s):
    drafted = kept = t1 = t2 = 0
    drafting = False
    for act in g['actions']:
        if act.get('seat') != s:
            continue
        t = act['type']
        if t == 'draft':
            drafting = True
            drafted += 1
        elif t == 'keep' and drafting:
            kept += 1
        elif t == 'tier':
            if act['tier'] == 2:
                t2 += 1
            else:
                t1 += 1
    return drafted, kept, t1, t2


def mean(xs):
    return sum(xs) / len(xs) if xs else float('nan')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--cards', default='engine')
    ap.add_argument('--pairs', action='store_true')
    ap.add_argument('--binary', default=os.path.join(ROOT, 'out', 'base', 'aic-sim.exe'))
    a = ap.parse_args()
    games = collections.defaultdict(dict)  # label -> (pair, seed) -> g
    order = []
    for arg in a.dirs:
        d, _, tag = arg.partition('=')
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        for g in lines:
            pair, agent = labels[g['cell']].split('/')[-2:]
            label = f'{tag}/{agent}' if tag else agent
            if label not in games:
                order.append(label)
            games[label][(pair, int(g['seed']))] = g
    base = a.base or order[0]

    print('## Win and rounds\n')
    print('| cell | games | win % | rounds | win gap vs base ± SE | rounds gap | ' +
          ('bp / br / pr win % |' if a.pairs else ''))
    print('|---|---|---|---|---|---|' + ('---|' if a.pairs else ''))
    for lab in order:
        gs = games[lab]
        win = 100 * mean([g['won'] for g in gs.values()])
        rounds = mean([g['rounds'] for g in gs.values()])
        common = [k for k in gs if k in games[base]]
        diffs = [100 * (gs[k]['won'] - games[base][k]['won']) for k in common]
        rd = [gs[k]['rounds'] - games[base][k]['rounds'] for k in common]
        se = math.sqrt(sum((x - mean(diffs)) ** 2 for x in diffs) / (len(diffs) - 1) / len(diffs)) if len(diffs) > 1 else 0
        gap = f'{mean(diffs):+.1f} ± {se:.1f}' if lab != base and common else '-'
        rg = f'{mean(rd):+.2f}' if lab != base and common else '-'
        extra = ''
        if a.pairs:
            per = []
            for p in ('bp', 'br', 'pr'):
                xs = [g['won'] for (pp, _), g in gs.items() if pp == p]
                per.append(f'{100 * mean(xs):.0f}' if xs else '-')
            extra = ' / '.join(per) + ' |'
        print(f'| {lab} | {len(gs)} | {win:.1f} | {rounds:.2f} | {gap} | {rg} | {extra}')

    print('\n## Lost at the limits, a game\n')
    keys = ['economy.hold_lost.credits', 'economy.hold_lost.energy', 'economy.hold_lost.chips',
            'economy.hold_lost.ambassadors', 'economy.production_lost.credits']
    print('| cell | ' + ' | '.join(k.split('economy.')[1] for k in keys) + ' | energy: games cut |')
    print('|---|' + '---|' * (len(keys) + 1))
    for lab in order:
        gs = list(games[lab].values())
        vals = [mean([g['counters'].get(k, 0) for g in gs]) for k in keys]
        cut = 100 * mean([g['counters'].get('economy.hold_lost.energy', 0) > 0 for g in gs])
        print(f'| {lab} | ' + ' | '.join(f'{v:.2f}' for v in vals) + f' | {cut:.0f}% |')

    print('\n## Peaks per seat (mean / highest)\n')
    print('| cell | board | ' + ' | '.join(SHOW) + ' |')
    print('|---|---|' + '---|' * len(SHOW))
    for lab in order:
        by = collections.defaultdict(lambda: collections.defaultdict(list))
        for (pair, _), g in games[lab].items():
            for s, ch in enumerate(pair[:2]):
                if s < len(g.get('peaks', [])):
                    for i, r in enumerate(PEAKS):
                        by[BOARD[ch]][r].append(g['peaks'][s][i])
        for b in ('Builder', 'Protector', 'Researcher'):
            cells = [f"{mean(by[b][r]):.1f} / {max(by[b][r])}" if by[b][r] else '-' for r in SHOW]
            print(f'| {lab} | {b} | ' + ' | '.join(cells) + ' |')

    print('\n## The Researcher: keeps against insight\n')
    print('Flag rules: `peak` is 122\'s (insight peak over 3x a round\'s insight income and over 10); `rise` is the fixed '
          'one (the peak\'s rise over the starting holding, same test). Keep rate: kept / offered after the opening; '
          'tier-2 share of its tier choices; insight at keep: mean held at its keep decisions.\n')
    print('| cell | rule | seats | n | insight peak | offered | kept | keep rate | tier-2 share | insight at keep |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for lab in order:
        for rule in ('peak', 'rise'):
            rows = collections.defaultdict(list)
            for (pair, _), g in games[lab].items():
                for s, ch in enumerate(pair[:2]):
                    if ch != 'r' or s >= len(g.get('peaks', [])):
                        continue
                    if 'keeps' in g:
                        k = g['keeps'][s]
                        dr, kp, t1, t2 = k['offered'], k['kept'], k['tiers'] - k['tier2'], k['tier2']
                        ins = mean(k['insight']) if k['insight'] else float('nan')
                    else:
                        dr, kp, t1, t2 = keep_split(g, s)
                        ins = float('nan')
                    peak = g['peaks'][s][1]
                    if rule == 'rise':
                        if 'start' not in g:
                            continue
                        peak_t = peak - g['start'][s][1]
                    else:
                        peak_t = peak
                    inc = g['gained'][s][1] / g['rounds'] if g['rounds'] else 0
                    flagged = inc > 0 and peak_t > 10 and peak_t > 3 * inc
                    rows[flagged].append((dr, kp, t1, t2, peak, ins))
                    rows['all'].append((dr, kp, t1, t2, peak, ins))
            for k, name in ((False, 'not flagged'), (True, 'flagged'), ('all', 'all')):
                r = rows[k]
                if not r or (rule == 'rise' and k == 'all'):
                    continue
                dr, kp = sum(x[0] for x in r), sum(x[1] for x in r)
                t1, t2 = sum(x[2] for x in r), sum(x[3] for x in r)
                ins = [x[5] for x in r if x[5] == x[5]]
                print(f'| {lab} | {rule} | {name} | {len(r)} | {mean([x[4] for x in r]):.1f} | {dr / len(r):.1f} | '
                      f'{kp / len(r):.1f} | {100 * kp / max(dr, 1):.0f}% | {100 * t2 / max(t1 + t2, 1):.0f}% | '
                      f'{mean(ins) if ins else float("nan"):.1f} |')

    if a.cards == 'none':
        return
    ids = engine_cards(a.binary) + NAMED if a.cards == 'engine' else None
    print('\n## Plays a game (kept, offered): the r41 engine cards and the two named\n')
    if ids is None:
        ids = sorted({c for lab in order for g in games[lab].values() for c in g['cards']})
    print('| card | ' + ' | '.join(order) + ' |')
    print('|---|' + '---|' * len(order))
    for c in ids:
        cells = []
        for lab in order:
            gs = list(games[lab].values())
            pl = mean([g['cards'].get(c, {}).get('played', 0) for g in gs])
            kp = mean([g['cards'].get(c, {}).get('kept', 0) for g in gs])
            of = mean([g['cards'].get(c, {}).get('offered', 0) for g in gs])
            cells.append(f'{pl:.3f} ({kp:.2f}, {of:.2f})')
        print(f'| {c} | ' + ' | '.join(cells) + ' |')
    zero = {lab: [c for c in ids if mean([g['cards'].get(c, {}).get('played', 0) for g in games[lab].values()]) == 0]
            for lab in order}
    print('\nPlayed 0 times: ' + '; '.join(f'{lab}: {", ".join(z) or "none"}' for lab, z in zero.items()))


if __name__ == '__main__':
    main()
