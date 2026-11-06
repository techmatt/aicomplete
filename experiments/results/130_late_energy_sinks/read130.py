"""130's read: the late energy sinks. Wins and rounds (paired against a base on pair and seed), energy per board
(gained, spent by where it went, held after the last action phase, flags, the flag paired seat by seat against the
base), keep / play / Action-use rates of a card list per board, Particle Accelerator's uses by half, and the
energy-cost cards held in hand and short of energy at the play pass.

    python experiments/results/130_late_energy_sinks/read130.py DIR[=TAG]... [--base TAG] [--cards a,b] [--agent A]

Each DIR is an `aic-sim experiment` output (`games.jsonl`). Rows are `TAG VARIANT/AGENT` as in 128's read128.py,
whose loader this reuses. The energy flag is 128's (and runaway_read.py's): the seat's energy peak rose over its
setup holding by more than 3 x its energy income a round and more than 10. Energy spent comes from each seat's
`uses` record (index 2 of the PEAKS order: credits, insight, energy, ...): node powers and the track (`power`),
cards' printed energy (`play`), building Actions (`use`), threat counters (`pay`), corp converters (`convert`).
"""
import argparse
import collections
import json
import os
import statistics
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '128_draft_and_price'))
import read128 as R  # noqa: E402
from screens import gap  # noqa: E402

E, CUBES = 2, 5  # indexes in the PEAKS order
SINKS = ['particle_accelerator', 'demand_response_grid', 'citywide_microgrid']
NEW = ['directed_energy_array', 'holographic_assembly_hall', 'wafer_fab_line', 'exascale_training_cluster']
ECARDS = sorted(c['id'] for c in R.CARDS.values() if c.get('energy'))
WHERE = ['power', 'play', 'use', 'pay', 'convert', 'standard', 'other']


def where(key):
    parts = key.split(':')
    if parts[0] != 'own':
        return 'other'
    return parts[1] if parts[1] in WHERE else 'other'


def seat_rows(dirs, agent):
    games = collections.defaultdict(list)
    seats = collections.defaultdict(list)
    for d in dirs:
        for label, pair, g in R.load(d):
            if agent and not label.endswith('/' + agent):
                continue
            games[label].append((pair, g))
            n = len(g['peaks'])
            acts, _ = R.seat_stream(g, n)
            c = g['counters']
            for s in range(n):
                held = (g['margin'].get('held_after_actions') or [[0] * 5] * n)[s]
                rise = g['peaks'][s][E] - (g.get('start') or [[0] * 8] * n)[s][E]
                inc = g['gained'][s][E] / max(g['rounds'], 1)
                spent = collections.Counter()
                for k, v in g['uses'][s].items():
                    spent[where(k)] += v[E]
                pa_cubes = g['sources'][s].get('own:use:particle_accelerator', [0] * 8)[CUBES]
                seats[(label, R.BOARD[pair[s]])].append(dict(
                    key=(pair, g['seed'], s), acts=acts[s], held_e=held[E], gained=g['gained'][s][E],
                    peak=g['peaks'][s][E], spent=spent, pa_cubes=pa_cubes, rounds=g['rounds'],
                    eheld=c.get(f'energy_cards.held.{s}', 0), eshort=c.get(f'energy_cards.short.{s}', 0),
                    short_by={k.rsplit('.', 1)[1]: v for k, v in c.items()
                              if k.startswith(f'energy_cards.short.{s}.')},
                    eflag=inc > 0 and rise > 3 * inc and rise > 10))
    return games, seats


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--cards', default='')
    ap.add_argument('--agent', default='', help='only this agent (strategist, greedy)')
    a = ap.parse_args()
    cards = a.cards.split(',') if a.cards else SINKS + [c for c in NEW if c in R.CARDS]
    games, seats = seat_rows(a.dirs, a.agent)

    print('## Wins and rounds\n')
    print('| run | games | win % | bp / br / pr | rounds | gap vs base ± SE |')
    print('|---|---|---|---|---|---|')
    for label in sorted(games):
        gs = games[label]
        w = 100 * sum(g['won'] for _, g in gs) / len(gs)
        by = {p: [g['won'] for q, g in gs if q == p] for p in ('bp', 'br', 'pr')}
        bys = ' / '.join(f'{100 * sum(v) / len(v):.0f}' if v else '-' for v in by.values())
        r = statistics.mean(g['rounds'] for _, g in gs)
        gp = ''
        base_label = a.base and a.base + '/' + label.split('/')[-1].split(',')[0]
        if base_label and base_label in games and base_label != label:
            bk = {(p, g['seed']): g['won'] for p, g in games[base_label]}
            m, se, n = gap([g['won'] - bk[(p, g['seed'])] for p, g in gs if (p, g['seed']) in bk])
            gp = f'{m:+.1f} ± {se:.1f} ({n})'
        print(f'| {label} | {len(gs)} | {w:.1f} | {bys} | {r:.2f} | {gp} |')

    print('\n## Energy per board, a seat-game\n')
    print('Held: after the last action phase. Flags: the energy peak rose over the start by more than 3 x the income a '
          'round and more than 10. Paired: flags against the base on the same seat of the same game (both / only base / '
          'only this run). Spent by where it went: node powers and track / card costs / Actions / counters / '
          'converters / standard.\n')
    print('| run | board | seats | gained | peak | held | flags | paired flags | spent: power / play / use / pay / '
          'convert / std |')
    print('|---|---|---|---|---|---|---|---|---|')
    for (label, board), rs in sorted(seats.items()):
        n = len(rs)
        paired = ''
        base_label = a.base and a.base + '/' + label.split('/')[-1].split(',')[0]
        if base_label and (base_label, board) in seats and base_label != label:
            bf = {r['key']: r['eflag'] for r in seats[(base_label, board)]}
            both = sum(1 for r in rs if r['eflag'] and bf.get(r['key']))
            ob = sum(1 for r in rs if not r['eflag'] and bf.get(r['key']))
            ot = sum(1 for r in rs if r['eflag'] and r['key'] in bf and not bf[r['key']])
            paired = f'{both} / {ob} / {ot}'
        sp = ' / '.join(f"{sum(r['spent'][k] for r in rs) / n:.1f}" for k in WHERE[:6])
        print(f"| {label} | {board} | {n} | {sum(r['gained'] for r in rs) / n:.1f} | "
              f"{sum(r['peak'] for r in rs) / n:.1f} | {sum(r['held_e'] for r in rs) / n:.2f} | "
              f"{sum(r['eflag'] for r in rs)} | {paired} | {sp} |")

    print('\n## Cards per board: drafted / kept / played / Action uses, a seat-game (uses per play)\n')
    print('| run | board | ' + ' | '.join(cards) + ' |')
    print('|---|---|' + '---|' * len(cards))
    for (label, board), rs in sorted(seats.items()):
        n = len(rs)
        cells = []
        for c in cards:
            v = [sum(r['acts'][(t, c)] for r in rs) / n for t in ('draft', 'keep', 'play', 'use')]
            cell = f'{v[0]:.2f}/{v[1]:.2f}/{v[2]:.2f}'
            if R.CARDS.get(c, {}).get('ability'):
                cell += f'/{v[3]:.2f}' + (f' ({v[3] / v[2]:.1f})' if v[2] else '')
            cells.append(cell)
        print(f'| {label} | {board} | ' + ' | '.join(cells) + ' |')

    print('\n## Particle Accelerator: Action uses a game, by half (both seats)\n')
    print('The research-cube half is read from the research cubes its Action gained (each use gains '
          '`ALT` of them; pass `--alt N`, default 3).\n')
    print('| run | uses | megaproject half | research-cube half |')
    print('|---|---|---|---|')
    for label in sorted(games):
        rs = [r for (l, _), v in seats.items() if l == label for r in v]
        ng = len(games[label])
        uses = sum(r['acts'][('use', 'particle_accelerator')] for r in rs)
        alt = sum(r['pa_cubes'] for r in rs) / ALT
        print(f'| {label} | {uses / ng:.3f} | {(uses - alt) / ng:.3f} | {alt / ng:.3f} |')

    print('\n## Energy-cost cards: plays a seat-game, and the share of play passes a kept one sat short of energy\n')
    print('| run | board | plays of the energy-cost cards | in hand at a pass | short of energy | short share |')
    print('|---|---|---|---|---|---|')
    for (label, board), rs in sorted(seats.items()):
        n = len(rs)
        plays = sum(r['acts'][('play', c)] for r in rs for c in ECARDS) / n
        h, sh = sum(r['eheld'] for r in rs), sum(r['eshort'] for r in rs)
        print(f'| {label} | {board} | {plays:.2f} | {h / n:.2f} | {sh / n:.2f} | {100 * sh / h if h else 0:.0f}% |')

    print('\n## Energy-cost cards per board: kept / played a seat-game\n')
    labels = sorted({k for k in seats})
    print('| card | ' + ' | '.join(f'{l} {b}' for l, b in labels) + ' |')
    print('|---|' + '---|' * len(labels))
    for c in ECARDS:
        cells = []
        for k in labels:
            rs = seats[k]
            n = len(rs)
            cells.append(f"{sum(r['acts'][('keep', c)] for r in rs) / n:.2f} / "
                         f"{sum(r['acts'][('play', c)] for r in rs) / n:.2f}")
        print(f'| {c} | ' + ' | '.join(cells) + ' |')


ALT = 3
if __name__ == '__main__':
    if '--alt' in sys.argv:
        i = sys.argv.index('--alt')
        ALT = int(sys.argv[i + 1])
        del sys.argv[i:i + 2]
    main()
