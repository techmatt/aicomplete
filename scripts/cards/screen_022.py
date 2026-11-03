"""Kept for the docs that cite it (docs/principles.md, screen-022-insight/insight.md); new work: scripts/cards/screen.py.

Insight-cost and dominance screen summary (022, rules r13).

    python scripts/cards/screen_022.py CARDS_DIR [DOMINANCE_DIR]

CARDS_DIR is an `aic-sim experiment` output of experiments/archive/screen-022-cards.json
(random, greedy, engine at d0-d4); DOMINANCE_DIR one of
experiments/archive/screen-022-dominance.json (the engine and the seven dominance
checks at d0, d2, d4). Prints:

- win rates per cell, with the rounds and how games ended;
- the card economy per game: cards kept / played, insight spent on keeps /
  on plays, credits left unspent (summed over rounds), node placed;
- by round (rounds 1-6, both seats summed, means over the games that reached
  the round): hand size after keeping, cards kept, cards played, insight on
  keeps, insight on plays, credits unspent; greedy and engine, d0 and d2, c2;
- the dominance table: per variant and difficulty, the engine's win rate and
  each check's gap to it (engine minus check, points) with its noise (one
  standard error of the difference), and the gate: healthy if every check is
  below the engine at d2 and d4 (gap > 0) and none matches it at d4 (gap
  above one standard error).
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from screens import ROUND, cnt, ends_text, load, mean  # noqa: E402

CHECKS = [
    ('keep_all=1', 'keep-all'),
    ('keep_k=2', 'keep-cheapest-k'),
    ('tier1_only=1', 'tier-1-only'),
    ('tier2_rush=1', 'tier-2-rush'),
    ('no_powers=1', 'no-powers'),
    ('no_counters=1', 'no-counters'),
    ('no_node=1', 'no-node'),
]


def total(g, prefix):
    return sum(v for k, v in g['counters'].items() if k.startswith(prefix) and not ROUND.search(k))


def per_rounds(g, key):
    return sum(v for k, v in g['counters'].items() if k.startswith(key + '.r') and ROUND.search(k))


def label(agent):
    for k, name in CHECKS:
        if agent.endswith(k):
            return name
    return agent


def wins_table(cells, games):
    print('| variant | d | agent | games | win % | rounds | ends |')
    print('|---|---|---|---|---|---|---|')
    for cell in sorted(games, key=lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])):
        gs, c = games[cell], cells[cell]
        n = len(gs)
        won = sum(g['won'] for g in gs)
        print(f"| {c['variant']} | {c['difficulty']} | {label(c['agent'])} | {n} | {100 * won / n:.0f} "
              f"| {mean(gs, lambda g: g['rounds']):.1f} | {ends_text(gs)} |")


def economy_table(cells, games):
    print('| variant | d | agent | kept / played | insight: keeps / plays | credits unspent | node |')
    print('|---|---|---|---|---|---|---|')
    for cell in sorted(games, key=lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])):
        gs, c = games[cell], cells[cell]
        print(f"| {c['variant']} | {c['difficulty']} | {label(c['agent'])} "
              f"| {mean(gs, lambda g: total(g, 'cards.kept.')):.1f} / {mean(gs, lambda g: total(g, 'cards.played.')):.1f} "
              f"| {mean(gs, lambda g: cnt(g, 'cards.insight_kept')):.1f} / {mean(gs, lambda g: cnt(g, 'cards.insight_played')):.1f} "
              f"| {mean(gs, lambda g: per_rounds(g, 'economy.credits_unspent')):.1f} "
              f"| {mean(gs, lambda g: cnt(g, 'board.node_placed')):.1f} |")


def by_round(cells, games, agents=('greedy', 'engine'), diffs=(0, 2)):
    print('| agent | d | measure | r1 | r2 | r3 | r4 | r5 | r6 |')
    print('|---|---|---|---|---|---|---|---|---|')
    for cell in sorted(games, key=lambda c: (cells[c]['agent'], cells[c]['difficulty'])):
        c = cells[cell]
        if c['difficulty'] not in diffs or c['variant'] != 'c2' or c['agent'] not in agents:
            continue
        gs = games[cell]
        for key, name in [('cards.hand_size', 'hand size'), ('cards.kept', 'kept'), ('cards.played', 'played'),
                          ('cards.insight_kept', 'insight on keeps'), ('cards.insight_played', 'insight on plays'),
                          ('economy.credits_unspent', 'credits unspent')]:
            row = []
            for r in range(1, 7):
                reach = [g for g in gs if g['rounds'] >= r]
                row.append(f"{mean(reach, lambda g: cnt(g, f'{key}.r{r:02}')):.1f}" if reach else '–')
            print(f"| {c['agent']} | {c['difficulty']} | {name} | {' | '.join(row)} |")


def dominance(cells, games):
    rate = {}
    for cell, gs in games.items():
        c = cells[cell]
        rate[(c['variant'], c['difficulty'], c['agent'])] = (sum(g['won'] for g in gs), len(gs))
    keys = sorted({(v, d) for v, d, _ in rate})
    print('Gap = engine win % minus the check\'s, ± one standard error of the difference.')
    print()
    print('| variant | d | engine | ' + ' | '.join(name for _, name in CHECKS) + ' |')
    print('|---|---|---|' + '---|' * len(CHECKS))
    fails = []
    for v, d in keys:
        if (v, d, 'engine') not in rate:
            continue
        we, ne = rate[(v, d, 'engine')]
        pe = we / ne
        row = []
        for k, name in CHECKS:
            hit = [a for (vv, dd, a) in rate if vv == v and dd == d and a.endswith(k)]
            if not hit:
                row.append('–')
                continue
            wc, nc = rate[(v, d, hit[0])]
            pc = wc / nc
            se = 100 * math.sqrt(max(pe * (1 - pe), 0.25 / ne) / ne + max(pc * (1 - pc), 0.25 / nc) / nc)
            gap = 100 * (pe - pc)
            mark = ''
            if d >= 2 and gap <= 0:
                mark = ' ✗'
                fails.append(f'{name} {v} d{d}: gap {gap:+.0f}')
            elif d >= 4 and gap <= se:
                mark = ' ~'
                fails.append(f'{name} {v} d{d}: gap {gap:+.0f} within noise ±{se:.0f}')
            row.append(f'{gap:+.0f} ±{se:.0f}{mark}')
        print(f"| {v} | {d} | {100 * pe:.0f} | " + ' | '.join(row) + ' |')
    print()
    print('Gate: ' + ('pass' if not fails else 'FAIL: ' + '; '.join(fails)))


def main(argv):
    sys.stdout.reconfigure(encoding='utf-8')
    cells, games = load(argv[0])
    print('## Win rates\n')
    wins_table(cells, games)
    print('\n## Card economy per game\n')
    economy_table(cells, games)
    print('\n## By round (c2; both seats summed)\n')
    by_round(cells, games)
    if len(argv) > 1:
        dcells, dgames = load(argv[1])
        print('\n## Dominance checks: win rates\n')
        wins_table(dcells, dgames)
        print('\n## Dominance checks: card economy per game\n')
        economy_table(dcells, dgames)
        print('\n## Dominance table\n')
        dominance(dcells, dgames)


if __name__ == '__main__':
    main(sys.argv[1:])
