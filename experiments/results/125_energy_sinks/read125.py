"""125's read: energy (lost at the limit, spent, where it goes), peaks, wins, and the plays of the cards 125 changed,
per agent cell of one or more experiment dirs, pooled over the pairs.

Reads r49-r50 runs only: 127 (r51) removed the holding limits and the `economy.hold_lost*` counters.

    python experiments/results/125_energy_sinks/read125.py DIR[=TAG[@REV]]... [--base LABEL] [--top 8] [--pairs]

Each DIR's `games.jsonl` (an `aic-sim experiment` output, 125's measures: `spent`, `uses`,
`economy.hold_lost_seat.<seat>.<r>`, `energy_cards.*`). A label is `TAG/AGENT` when a dir is given `=TAG`, else the
spec's agent. `@REV` reads the energy-cost cards from `content/cards.json` at that git revision (default: the working
tree). `--base`: the label every other is paired against on the same pair and seed; default the first.
"""
import argparse
import collections
import json
import math
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
SHOW = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'ambassadors']
BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
BOARDS = ('Builder', 'Protector', 'Researcher')
CHANGED = ['fabrication_megaplex', 'lights_out_assembly', 'orbital_mass_driver', 'foundation_model_campus',
           'positron_storage_ring', 'particle_accelerator', 'citywide_microgrid', 'moonshot_grant']
E = PEAKS.index('energy')


def energy_cards(rev):
    if rev:
        text = subprocess.run(['git', 'show', f'{rev}:content/cards.json'], cwd=ROOT, capture_output=True,
                              text=True, encoding='utf-8', check=True).stdout
    else:
        text = open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8').read()
    return {c['id'] for c in json.loads(text) if c.get('energy', 0) > 0}


def mean(xs):
    xs = list(xs)
    return sum(xs) / len(xs) if xs else float('nan')


def se_of(xs):
    xs = list(xs)
    if len(xs) < 2:
        return 0.0
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def seats(pair, g):
    return [(s, BOARD[ch]) for s, ch in enumerate(pair[:2]) if s < len(g.get('peaks', []))]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--top', type=int, default=8)
    ap.add_argument('--pairs', action='store_true')
    a = ap.parse_args()
    games = collections.defaultdict(dict)  # label -> (pair, seed) -> g
    ecards = {}
    order = []
    for arg in a.dirs:
        d, _, tagrev = arg.partition('=')
        tag, _, rev = tagrev.partition('@')
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        ec = energy_cards(rev)
        for g in lines:
            pair, agent = labels[g['cell']].split('/')[-2:]
            pair = pair.split('-')[0]  # a lever's variant, `bp-light`, pairs with `bp`
            label = f'{tag}/{agent}' if tag else agent
            if label not in games:
                order.append(label)
                ecards[label] = ec
            games[label][(pair, int(g['seed']))] = g
    base = a.base or order[0]

    print('## Win and rounds\n')
    print('| cell | games | win % | rounds | win gap vs base ± SE | rounds gap |' + (' bp / br / pr win % |' if a.pairs else ''))
    print('|---|---|---|---|---|---|' + ('---|' if a.pairs else ''))
    for lab in order:
        gs = games[lab]
        common = [k for k in gs if k in games[base]]
        diffs = [100 * (gs[k]['won'] - games[base][k]['won']) for k in common]
        rd = [gs[k]['rounds'] - games[base][k]['rounds'] for k in common]
        gap = f'{mean(diffs):+.1f} ± {se_of(diffs):.1f}' if lab != base and common else '-'
        rg = f'{mean(rd):+.2f}' if lab != base and common else '-'
        extra = ''
        if a.pairs:
            per = [mean(g['won'] for (pp, _), g in gs.items() if pp == p) for p in ('bp', 'br', 'pr')]
            extra = ' ' + ' / '.join(f'{100 * x:.0f}' for x in per) + ' |'
        print(f'| {lab} | {len(gs)} | {100 * mean(g["won"] for g in gs.values()):.1f} | '
              f'{mean(g["rounds"] for g in gs.values()):.2f} | {gap} | {rg} |{extra}')

    print('\n## Energy a game, per seat of each board\n')
    print('Lost: cut at the 10-energy limit (`economy.hold_lost_seat`); games cut: share of seat-games losing any. '
          'Spent: every fall between actions (`spent`). The game total is both seats.\n')
    print('| cell | game total lost | board | lost | games cut | gained | spent | end held |')
    print('|---|---|---|---|---|---|---|---|')
    for lab in order:
        gs = list(games[lab].values())
        tot = mean(g['counters'].get('economy.hold_lost.energy', 0) for g in gs)
        for b in BOARDS:
            lost, gained, spent, end = [], [], [], []
            for (pair, _), g in games[lab].items():
                for s, bb in seats(pair, g):
                    if bb != b:
                        continue
                    lost.append(g['counters'].get(f'economy.hold_lost_seat.{s}.energy', 0))
                    gained.append(g['gained'][s][E])
                    if 'spent' in g:
                        spent.append(g['spent'][s][E])
                        end.append(g['start'][s][E] + g['gained'][s][E] - g['spent'][s][E])
            if not lost:
                continue
            print(f'| {lab} | {tot:.2f} | {b} | {mean(lost):.2f} | {100 * mean(x > 0 for x in lost):.0f}% | '
                  f'{mean(gained):.1f} | {mean(spent):.1f} | {mean(end):.1f} |')

    print('\n## Where energy goes (spent a seat-game, by use; top uses)\n')
    for lab in order:
        for b in BOARDS:
            use = collections.defaultdict(float)
            n = 0
            for (pair, _), g in games[lab].items():
                for s, bb in seats(pair, g):
                    if bb != b or 'uses' not in g:
                        continue
                    n += 1
                    for k, v in g['uses'][s].items():
                        use[k] += v[E]
            if not n:
                continue
            top = sorted(use.items(), key=lambda x: -x[1])[:a.top]
            print(f'- {lab}, {b} ({n}): ' + '; '.join(f'{k} {v / n:.2f}' for k, v in top))

    print('\n## Energy lost a game by round (both seats)\n')
    rounds = sorted({int(k.rsplit('.r', 1)[1]) for lab in order for g in games[lab].values()
                     for k in g['counters'] if k.startswith('economy.hold_lost.energy.r')})
    print('| cell | ' + ' | '.join(f'r{r}' for r in rounds) + ' |')
    print('|---|' + '---|' * len(rounds))
    for lab in order:
        gs = list(games[lab].values())
        print(f'| {lab} | ' + ' | '.join(f'{mean(g["counters"].get(f"economy.hold_lost.energy.r{r:02}", 0) for g in gs):.2f}'
                                         for r in rounds) + ' |')

    print('\n## Peaks per seat (mean / highest)\n')
    print('| cell | board | ' + ' | '.join(SHOW) + ' |')
    print('|---|---|' + '---|' * len(SHOW))
    for lab in order:
        by = collections.defaultdict(lambda: collections.defaultdict(list))
        for (pair, _), g in games[lab].items():
            for s, b in seats(pair, g):
                for i, r in enumerate(PEAKS):
                    by[b][r].append(g['peaks'][s][i])
        for b in BOARDS:
            if by[b]:
                print(f'| {lab} | {b} | ' + ' | '.join(f'{mean(by[b][r]):.1f} / {max(by[b][r])}' for r in SHOW) + ' |')

    print('\n## Plays a game of the changed cards (kept, offered); Actions used a game; plays by board a seat-game\n')
    print('| card | cell | played | kept | offered | Action uses | Builder | Protector | Researcher |')
    print('|---|---|---|---|---|---|---|---|---|')
    for c in CHANGED:
        for lab in order:
            gs = games[lab]
            pl = mean(g['cards'].get(c, {}).get('played', 0) for g in gs.values())
            kp = mean(g['cards'].get(c, {}).get('kept', 0) for g in gs.values())
            of = mean(g['cards'].get(c, {}).get('offered', 0) for g in gs.values())
            uses = mean(sum(1 for x in g['actions'] if x.get('type') == 'use' and x.get('card') == c) for g in gs.values())
            byb = collections.defaultdict(list)
            for (pair, _), g in gs.items():
                for s, b in seats(pair, g):
                    byb[b].append(sum(1 for x in g['actions'] if x.get('type') == 'play' and x.get('card') == c
                                      and x.get('seat') == s))
            cells = [f'{mean(byb[b]):.3f}' if byb[b] else '-' for b in BOARDS]
            print(f'| {c} | {lab} | {pl:.3f} | {kp:.2f} | {of:.2f} | {uses:.2f} | ' + ' | '.join(cells) + ' |')

    print('\n## Energy-cost cards by board: plays a seat-game, and held at the end of an action phase short of energy\n')
    print('Held: energy-cost cards in hand as the seat passed its action phase, summed over rounds; short: of those, '
          'needing more energy than it held. A seat-game.\n')
    print('| cell | board | energy-cost plays | held at pass | short | seat-games with one short |')
    print('|---|---|---|---|---|---|')
    for lab in order:
        ec = ecards[lab]
        for b in BOARDS:
            plays, held, short = [], [], []
            for (pair, _), g in games[lab].items():
                for s, bb in seats(pair, g):
                    if bb != b:
                        continue
                    plays.append(sum(1 for x in g['actions'] if x.get('type') == 'play' and x.get('card') in ec
                                     and x.get('seat') == s))
                    held.append(g['counters'].get(f'energy_cards.held.{s}', 0))
                    short.append(g['counters'].get(f'energy_cards.short.{s}', 0))
            if plays:
                print(f'| {lab} | {b} | {mean(plays):.2f} | {mean(held):.2f} | {mean(short):.2f} | '
                      f'{100 * mean(x > 0 for x in short):.0f}% |')
    print('\nEnergy-cost cards (per label): ' + '; '.join(f'{lab}: {", ".join(sorted(ecards[lab]))}' for lab in order))


if __name__ == '__main__':
    main()
