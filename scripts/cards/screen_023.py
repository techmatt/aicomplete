"""Kept for the docs that cite it (docs/principles.md, screen-023-cards/cards.md); new work: scripts/cards/screen.py.

The card-set screen summary (023, rules r14).

    python scripts/cards/screen_023.py CARDS_DIR [DOMINANCE_DIR]

CARDS_DIR is an `aic-sim experiment` output of experiments/archive/screen-023-cards.json
(random, greedy, engine at d0-d4, 2 and 3 countries); DOMINANCE_DIR one of
experiments/archive/screen-023-dominance.json (the engine, 022's seven checks and 023's
mono-tag x6, ignore-X x6 and event-hoarder, d2 only, 32 seeds). Prints:

- win rates per cell, the card economy per game (kept / played, insight on keeps
  / on plays) and the by-round table (022's, from screen_022.py);
- the strategy-health gate (021's, in screen_021.py's docstring: every greedy and
  engine cell averages node placed >= 5, cards played >= 5 and kept >= 8;
  engine at d0 uses some AI power and replaces some building) and the 023 target:
  engine at d0 about 90% or better;
- per card, over the greedy and engine games: drafted, kept, keep rate (kept /
  drafted: every dealt card lands in the hand, so the keep is the choice), played,
  play rate (played / kept), with the dead cards (kept 3+ times and never played,
  or kept under 15% of the times drafted) and the must-picks (kept as often as
  drafted and played 80%+ of the times kept) named;
- per tag: the share of the engine's winning games that played at least one card
  carrying it, and the mean such cards played per winning game;
- the dominance table: each check's gap to the engine (points) with one standard
  error, and the gate: every check below the engine (gap > 0) at d2.
"""
import collections
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import screen_022 as s22  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TAGS = ['science', 'industry', 'cyber', 'power', 'diplomacy', 'security']
CHECKS = list(s22.CHECKS) + [(f'only_tag={i + 1}', f'mono-{t}') for i, t in enumerate(TAGS)] + \
    [(f'never_tag={i + 1}', f'ignore-{t}') for i, t in enumerate(TAGS)] + [('events_only=1', 'event-hoarder')]
s22.CHECKS[:] = CHECKS


def content():
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        return {c['id']: c for c in json.load(f)}


def played_ids(g):
    out = collections.Counter()
    for k, v in g['counters'].items():
        for p in ('cards.played.', 'played.'):
            if k.startswith(p) and not s22.ROUND.search(k):
                out[k[len(p):]] += v
                break
    return out


def health(cells, games):
    print('| variant | d | agent | win % | node | played | kept | powers | replaced | gate |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    fails = []
    for cell in sorted(games, key=lambda c: (cells[c]['variant'], cells[c]['difficulty'], cells[c]['agent'])):
        gs, c = games[cell], cells[cell]
        if c['agent'] not in ('greedy', 'engine'):
            continue
        pres = s22.mean(gs, lambda g: s22.cnt(g, 'board.node_placed'))
        played = s22.mean(gs, lambda g: s22.total(g, 'cards.played.'))
        kept = s22.mean(gs, lambda g: s22.total(g, 'cards.kept.'))
        powers = s22.mean(gs, lambda g: s22.cnt(g, 'ai.powers_used'))
        repl = s22.mean(gs, lambda g: s22.cnt(g, 'cards.replaced'))
        win = 100 * sum(g['won'] for g in gs) / len(gs)
        bad = []
        if pres < 5:
            bad.append('node')
        if played < 5:
            bad.append('played')
        if kept < 8:
            bad.append('kept')
        if c['agent'] == 'engine' and c['difficulty'] == 0:
            if powers <= 0:
                bad.append('powers')
            if repl <= 0:
                bad.append('replaced')
            if win < 85:
                bad.append('d0 win')
        if bad:
            fails.append(f"{c['variant']} d{c['difficulty']} {c['agent']}: {', '.join(bad)}")
        print(f"| {c['variant']} | {c['difficulty']} | {c['agent']} | {win:.0f} | {pres:.1f} | {played:.1f} "
              f"| {kept:.1f} | {powers:.1f} | {repl:.1f} | {'fail: ' + ', '.join(bad) if bad else 'pass'} |")
    print()
    print('Gate: ' + ('pass' if not fails else 'FAIL: ' + '; '.join(fails)))


def card_table(d, cards):
    res = json.load(open(f'{d}/results.json', encoding='utf-8'))['cards']
    tot = collections.defaultdict(lambda: collections.Counter())
    for agent in ('greedy', 'engine'):
        for cid, st in res.get(agent, {}).items():
            tot[cid].update(st)
    rows = []
    for cid, c in cards.items():
        if c['tier'] not in (1, 2):
            continue
        st = tot.get(cid, collections.Counter())
        off, dr, kept, pl = st['offered'], st['drafted'], st['kept'], st['played']
        rows.append((cid, c, off, dr, kept, pl))
    print('Greedy and engine games together. Every dealt card lands in the hand being drafted, so the '
          'choice is the keep: keep % = kept / drafted (the opening deal of 8 is kept too and not '
          'drafted, so it can pass 100%); play % = played / kept.\n')
    print('| card | tier | tags | drafted | kept | keep % | played | play % |')
    print('|---|---|---|---|---|---|---|---|')
    key = lambda r: (tag_order(r[1]), r[1]['tier'], r[1]['name'])
    for cid, c, off, dr, kept, pl in sorted(rows, key=key):
        print(f"| {c['name']} | {c['tier']} | {', '.join(c['tags'])} | {dr} | {kept} | "
              f"{100 * kept / dr if dr else 0:.0f} | {pl} | {100 * pl / kept if kept else 0:.0f} |")
    dead = [c['name'] for cid, c, off, dr, kept, pl in rows
            if (kept >= 3 and pl == 0) or (dr >= 10 and kept / dr < 0.15)]
    never = [c['name'] for cid, c, off, dr, kept, pl in rows if dr == 0 and kept == 0]
    must = [c['name'] for cid, c, off, dr, kept, pl in rows if dr >= 10 and kept / dr >= 1.0 and pl / kept >= 0.8]
    print(f'\n**Dead cards ({len(dead)})** (kept 3+ times and never played, or kept under 15% of the times'
          f' drafted, 10+ drafts): {", ".join(sorted(dead)) or "none"}.')
    if never:
        print(f'\nNever drafted or kept ({len(never)}): {", ".join(sorted(never))}.')
    print(f'\n**Must-picks ({len(must)})** (kept as often as drafted, 10+ drafts, and played 80%+ of the'
          f' times kept): {", ".join(sorted(must)) or "none"}.')


def tag_order(c):
    return TAGS.index(c['tags'][0]) if c['tags'] else 9


def tag_table(cells, games, cards):
    print('| variant | d | engine wins | ' + ' | '.join(TAGS) + ' |')
    print('|---|---|---|' + '---|' * len(TAGS))
    for cell in sorted(games, key=lambda c: (cells[c]['variant'], cells[c]['difficulty'])):
        c = cells[cell]
        if c['agent'] != 'engine':
            continue
        wins = [g for g in games[cell] if g['won']]
        if not wins:
            continue
        row = []
        for t in TAGS:
            per = [sum(n for cid, n in played_ids(g).items() if cid in cards and t in cards[cid]['tags'])
                   for g in wins]
            share = 100 * sum(p > 0 for p in per) / len(wins)
            row.append(f'{share:.0f}% ({sum(per) / len(wins):.1f})')
        print(f"| {c['variant']} | {c['difficulty']} | {len(wins)} | " + ' | '.join(row) + ' |')


def main(argv):
    sys.stdout.reconfigure(encoding='utf-8')
    cards = content()
    cells, games = s22.load(argv[0])
    print('## Win rates\n')
    s22.wins_table(cells, games)
    print('\n## Strategy health\n')
    health(cells, games)
    print('\n## Card economy per game\n')
    s22.economy_table(cells, games)
    print('\n## By round (c2; both seats summed)\n')
    s22.by_round(cells, games)
    print('\n## Tags in the engine\'s wins\n')
    print('Share of winning engine games that played a card carrying the tag (mean such cards played).\n')
    tag_table(cells, games, cards)
    print('\n## Cards\n')
    card_table(argv[0], cards)
    if len(argv) > 1:
        dcells, dgames = s22.load(argv[1])
        print('\n## Dominance checks: win rates\n')
        s22.wins_table(dcells, dgames)
        print('\n## Dominance checks: card economy per game\n')
        s22.economy_table(dcells, dgames)
        print('\n## Dominance table\n')
        s22.dominance(dcells, dgames)


if __name__ == '__main__':
    main(sys.argv[1:])
