"""Kept for the docs that cite it (screen-024-adversary/adversary.md); new work: scripts/cards/screen.py.

The scarier-adversary screen summary (024, rules r15).

    python scripts/cards/screen_024.py CARDS_DIR [DOMINANCE_DIR] [--before BEFORE_DIR]
    python scripts/cards/screen_024.py --brief DIR ...

CARDS_DIR is an `aic-sim experiment` output of experiments/archive/screen-024-cards.json,
DOMINANCE_DIR of experiments/archive/screen-024-dominance.json, BEFORE_DIR the same cards
screen run on the r14 binary. Prints what 023's summary prints (screen_023.py) and
024's gates:

- per cell: win %, earliest and mean win round, how games ended, and credits
  unspent per player after the last action phase played (the `done when`: under
  about 15), against BEFORE_DIR when given;
- standard actions and projects used per game, by act;
- cards freed and cities corrupted at the end.

--brief prints only the first table, for tuning probes.
"""
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import screen_022 as s22  # noqa: E402
import screen_023 as s23  # noqa: E402

KEY = lambda cs: lambda c: (cs[c]['variant'], cs[c]['difficulty'], cs[c]['agent'])


def last_unspent(g):
    """Credits left after the last action phase played, per player."""
    rounds = [k for k in g['counters'] if k.startswith('economy.credits_unspent.r')]
    return g['counters'][max(rounds)] / SEATS[g['_v']] if rounds else 0


SEATS = collections.defaultdict(lambda: 2, {'c3': 3})


def rows(cells, games):
    out = {}
    for cell, gs in games.items():
        c = cells[cell]
        for g in gs:
            g['_v'] = c['variant'][:2]
        wins = [g['rounds'] for g in gs if g['won']]
        ends = collections.Counter(('win ' if g['won'] else '') + g['reason'] for g in gs)
        out[(c['variant'], c['difficulty'], s22.label(c['agent']))] = {
            'win': 100 * len(wins) / len(gs),
            'first': min(wins) if wins else None,
            'mean': sum(wins) / len(wins) if wins else None,
            'rounds': s22.mean(gs, lambda g: g['rounds']),
            'unspent': s22.mean(gs, last_unspent),
            'ends': ', '.join(f'{k} {v}' for k, v in ends.most_common(4)),
        }
    return out


def table(cells, games, before=None):
    now = rows(cells, games)
    was = rows(*before) if before else {}
    print('| variant | d | agent | win % | was | earliest | mean win | was earliest | credits left | was | ends |')
    print('|---|---|---|---|---|---|---|---|---|---|---|')
    for k in sorted(now):
        r, b = now[k], was.get(k, {})
        f = lambda x, p='{:.0f}': p.format(x) if x is not None else '–'
        print(f"| {k[0]} | {k[1]} | {k[2]} | {r['win']:.0f} | {f(b.get('win'))} | {f(r['first'])} "
              f"| {f(r['mean'], '{:.1f}')} | {f(b.get('first'))} | {r['unspent']:.0f} | {f(b.get('unspent'))} | {r['ends']} |")


def standard(cells, games):
    acts = sorted({k[len('board.std.'):] for gs in games.values() for g in gs for k in g['counters']
                   if k.startswith('board.std.') and not s22.ROUND.search(k)})
    print('| variant | d | agent | ' + ' | '.join(acts) + ' |')
    print('|---|---|---|' + '---|' * len(acts))
    for cell in sorted(games, key=KEY(cells)):
        c = cells[cell]
        if c['agent'] == 'random':
            continue
        vals = [s22.mean(games[cell], lambda g: s22.cnt(g, f'board.std.{a}')) for a in acts]
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | " + ' | '.join(f'{v:.1f}' for v in vals) + ' |')


def adversary(cells, games):
    print('| variant | d | agent | cities freed | ambassadors removed | troops removed | projects resolved | compute drained | threats countered | threats executed |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    keys = ['board.cities_freed', 'adversary.ambassadors_removed', 'adversary.troops_removed',
            'adversary.projects_resolved', 'adversary.compute_drained', 'threats.countered', 'threats.executed']
    for cell in sorted(games, key=KEY(cells)):
        c = cells[cell]
        if c['agent'] == 'random':
            continue
        vals = [s22.mean(games[cell], lambda g: s22.cnt(g, k)) for k in keys]
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | " + ' | '.join(f'{v:.1f}' for v in vals) + ' |')


def main(argv):
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    if argv[0] == '--brief':
        for d in argv[1:]:
            print(f'\n### {d}\n')
            table(*s22.load(d))
        return
    before = None
    if '--before' in argv:
        i = argv.index('--before')
        before = s22.load(argv[i + 1])
        argv = argv[:i] + argv[i + 2:]
    cards = s23.content()
    cells, games = s22.load(argv[0])
    print('## Win rates, earliest win, credits left\n')
    table(cells, games, before)
    print('\n## Standard actions and projects per game\n')
    standard(cells, games)
    if before:
        print('\nBefore (r14):\n')
        standard(*before)
    print('\n## The adversary per game\n')
    adversary(cells, games)
    print('\n## Strategy health\n')
    s23.health(cells, games)
    print('\n## Card economy per game\n')
    s22.economy_table(cells, games)
    print('\n## By round (c2; both seats summed)\n')
    s22.by_round(cells, games)
    print('\n## Tags in the engine\'s wins\n')
    s23.tag_table(cells, games, cards)
    print('\n## Cards\n')
    s23.card_table(argv[0], cards)
    if len(argv) > 1:
        dcells, dgames = s22.load(argv[1])
        print('\n## Dominance checks: win rates\n')
        s22.wins_table(dcells, dgames)
        print('\n## Dominance table\n')
        s22.dominance(dcells, dgames)


if __name__ == '__main__':
    main(sys.argv[1:])
