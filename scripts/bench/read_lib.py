"""A paired read's games, decoded, and the tables 252-254's item readers each rebuilt (255; 252's `read_lib.py` note).

    python scripts/bench/read_lib.py READ [READ ...] [--base ARM] [--counters PREFIX ...] [--cards ID ...] [--md OUT]

A READ is a run's folder, a games file with its header, or FILE=SPEC (a bare farmed JSONL and its spec;
`screens.stream`); several READs pool when their arms share names, else each row leads with its READ's name (the
file stem, or the folder of a `games.jsonl`). Give the READs first: `--counters` and `--cards` take every word after.
Cells come from the spec (`screens.cells_of`: configs, then variants, then difficulties, then agents); a variant named
`PAIR-ARM` (or `PAIR_ARM`) is arm ARM of pair PAIR, a bare `PAIR` arm `base`, and a spec with several configs leads
each arm with the config's file stem. Every arm is paired against the base arm (`--base`, default `base`, `off`, else
the first) on the same pair, level, agent and seed.

Tables: wins by level with the paired gap ± one SE; the gap by level; losses by reason a game; board powers a seat-game
of their board (by their counters, `ai.power.<board>.<slug>`, so any revision reads right); standard actions a
seat-game; `--counters`: each counter whose key starts with a PREFIX, a game (per-round `.rNN` keys left out);
`--cards`: each card's plays a game; resource use a seat-game (spent, and spent / gained), in PEAKS order.

As a library: `games(read)` yields `Game`s; `seat_boards`, `counter_sums`, `resources` and `paired_gap` are what the
tables are made of. Python's standard library only.
"""
import os
import re
import sys
from collections import defaultdict, namedtuple

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import PEAKS, ROUND, cells_of, gap, source, stream  # noqa: E402

Game = namedtuple('Game', 'arm pair level agent seed g')


def split_variant(name):
    """(pair, arm) of a variant name: `bp-on` -> (bp, on), `bp_cap2` -> (bp, cap2), `bp` -> (bp, base)."""
    parts = re.split(r'[_-]', name, maxsplit=1)
    return parts[0], parts[1] if len(parts) > 1 else 'base'


def games(read, label=None):
    """READ's games as `Game(arm, pair, level, agent, seed, g)`, decoded from its spec. ARM leads with LABEL (a
    READ's name, when several READs don't pool) and with the config's file stem when the spec has several configs.
    AGENT is the spec's name up to any `:` (its overrides dropped), or the whole spec where two agents share a name."""
    lines = stream(read)
    spec = next(lines)['spec']
    cells = cells_of(spec)
    many = len(spec['configs']) > 1
    # 261: an agent-lever screen ('strategist', 'strategist:w.sink=0.5', ...) keeps each spec whole: cut at ':' only
    # where the short names stay distinct (it read as "pooled reads overlap").
    specs = sorted({c['agent'] for c in cells})
    short = {a: a.split(':')[0] for a in specs}
    if len(set(short.values())) < len(specs):
        short = {a: a for a in specs}
    for g in lines:
        if 'cell' not in g or 'won' not in g:
            continue
        c = cells[g['cell']]
        pair, arm = split_variant(c['variant'])
        if many:
            arm = f"{os.path.splitext(os.path.basename(c['config']))[0]}/{arm}"
        if label:
            arm = f'{label}/{arm}'
        yield Game(arm, pair, c['difficulty'], short[c['agent']], int(g['seed']), g)


def seat_boards(g):
    """Each seat's board id, from the game's `build` (`builder:card+card|protector:...`)."""
    return [p.split(':', 1)[0] for p in (g.get('build') or '').split('|') if p]


def counter_sums(g, prefixes, rounds=False):
    """{key: value} of G's counters whose key starts with one of PREFIXES (a key itself is its own prefix); `.rNN`
    per-round keys only with ROUNDS."""
    return {k: v for k, v in g['counters'].items() if k.startswith(tuple(prefixes)) and (rounds or not ROUND.search(k))}


def resources(g, seat):
    """{resource: (spent, gained)} of SEAT in G, in PEAKS order."""
    return {r: (g['spent'][seat][i], g['gained'][seat][i]) for i, r in enumerate(PEAKS)}


def paired_gap(base, new, keep=None):
    """BASE and NEW: {key: 0 or 1} (a key a pair, level and seed). On the keys both hold (and KEEP(key) passes):
    (base %, new %, gap in points, its SE, n), or None with no key shared."""
    keys = [k for k in base if k in new and (keep is None or keep(k))]
    if not keys:
        return None
    n = len(keys)
    b = 100 * sum(base[k] for k in keys) / n
    m, se, _ = gap([new[k] - base[k] for k in keys])
    return b, b + m, m, se, n


def summarize(rows, prefixes=(), cards=()):
    """(arm, agent) -> tallies over ROWS (`Game`s)."""
    S = defaultdict(lambda: {'wins': {}, 'reason': defaultdict(int), 'games': 0, 'rounds': 0, 'seatgames': 0,
                             'sg_board': defaultdict(int), 'power': defaultdict(float), 'std': defaultdict(float),
                             'ctr': defaultdict(float), 'plays': defaultdict(float),
                             'spent': defaultdict(float), 'gained': defaultdict(float)})
    for r in rows:
        s, g = S[(r.arm, r.agent)], r.g
        key = (r.pair, r.level, r.seed)
        if key in s['wins']:
            # 255 (review): pooled reads on overlapping seeds would leave the wins table one read short of the rest
            raise SystemExit(f'read_lib: {r.arm} {r.agent} pair {r.pair} level {r.level} seed {r.seed} twice: '
                             'pooled reads overlap; read them apart')
        s['wins'][key] = 1 if g['won'] else 0
        s['games'] += 1
        s['rounds'] += g['rounds']
        if not g['won']:
            s['reason'][g['reason']] += 1
        for seat, b in enumerate(seat_boards(g)):
            s['seatgames'] += 1
            s['sg_board'][b] += 1
            for res, (sp, ga) in resources(g, seat).items():
                s['spent'][res] += sp
                s['gained'][res] += ga
        for k, v in counter_sums(g, ['ai.power.']).items():
            p = k.split('.')
            if len(p) == 4:
                s['power'][(p[2], p[3])] += v
        for k, v in counter_sums(g, prefixes).items():
            s['ctr'][k] += v
        for a in g.get('actions') or ():
            if a.get('type') == 'standard':
                s['std'][a['act']] += 1
            elif a.get('type') == 'play' and a.get('card') in cards:
                s['plays'][a['card']] += 1
    return S


def tables(S, base_arm=None, cards=()):
    """The read's tables as Markdown lines."""
    arms = list(dict.fromkeys(a for a, _ in S))
    agents = list(dict.fromkeys(a for _, a in S))
    base = base_arm if base_arm in arms else next((a for a in arms if a.split('/')[-1] in ('base', 'off')), arms[0])
    levels = sorted({k[1] for s in S.values() for k in s['wins']})
    rows = [(ag, a, S[(a, ag)]) for ag in agents for a in arms if (a, ag) in S]
    L = []
    p = L.append
    p(f'## Wins (paired against {base}, same pair, level and seed)\n')
    p('| agent | arm | ' + ' | '.join(f'd{d}' for d in levels) + ' | all | gap ± SE (all) | n |')
    p('|---|---|' + '---|' * (len(levels) + 3))
    for ag, a, s in rows:
        cells = []
        for d in levels:
            ws = [v for k, v in s['wins'].items() if k[1] == d]
            cells.append(f'{100 * sum(ws) / len(ws):.1f}' if ws else '-')
        ws = list(s['wins'].values())
        pr = paired_gap(S[(base, ag)]['wins'], s['wins']) if (base, ag) in S and a != base else None
        g = f'{pr[2]:+.1f} ± {pr[3]:.1f}' if pr else ''
        p(f'| {ag} | {a} | ' + ' | '.join(cells) + f' | {100 * sum(ws) / len(ws):.1f} | {g} | {len(ws)} |')
    p('\n### Paired gaps by level\n')
    p('| agent | arm | ' + ' | '.join(f'd{d}' for d in levels) + ' |')
    p('|---|---|' + '---|' * len(levels))
    for ag, a, s in rows:
        if a == base or (base, ag) not in S:
            continue
        cells = []
        for d in levels:
            pr = paired_gap(S[(base, ag)]['wins'], s['wins'], lambda k: k[1] == d)
            cells.append(f'{pr[2]:+.1f} ± {pr[3]:.1f}' if pr else '-')
        p(f'| {ag} | {a} | ' + ' | '.join(cells) + ' |')
    reasons = sorted({r for s in S.values() for r in s['reason']})
    p('\n## Losses by reason a game\n')
    p('| agent | arm | rounds | ' + ' | '.join(reasons) + ' |')
    p('|---|---|---|' + '---|' * len(reasons))
    for ag, a, s in rows:
        n = s['games']
        p(f'| {ag} | {a} | {s["rounds"] / n:.2f} | ' + ' | '.join(f'{s["reason"][r] / n:.3f}' for r in reasons) + ' |')
    p("\n## Board powers a seat-game (that board's seat-games)\n")
    p('| agent | arm | board | power | uses |')
    p('|---|---|---|---|---|')
    for ag, a, s in rows:
        for (b, nm), v in sorted(s['power'].items()):
            p(f'| {ag} | {a} | {b} | {nm} | {v / max(s["sg_board"][b], 1):.2f} |')
    acts = sorted({act for s in S.values() for act in s['std']})
    p('\n## Standard actions a seat-game\n')
    p('| agent | arm | ' + ' | '.join(acts) + ' |')
    p('|---|---|' + '---|' * len(acts))
    for ag, a, s in rows:
        p(f'| {ag} | {a} | ' + ' | '.join(f'{s["std"][act] / s["seatgames"]:.2f}' for act in acts) + ' |')
    ctrs = sorted({c for s in S.values() for c in s['ctr']})
    if ctrs:
        p('\n## Counters a game\n')
        p('| agent | arm | ' + ' | '.join(ctrs) + ' |')
        p('|---|---|' + '---|' * len(ctrs))
        for ag, a, s in rows:
            p(f'| {ag} | {a} | ' + ' | '.join(f'{s["ctr"][c] / s["games"]:.2f}' for c in ctrs) + ' |')
    if cards:
        p('\n## Cards played a game\n')
        p('| agent | arm | ' + ' | '.join(cards) + ' |')
        p('|---|---|' + '---|' * len(cards))
        for ag, a, s in rows:
            p(f'| {ag} | {a} | ' + ' | '.join(f'{s["plays"][c] / s["games"]:.3f}' for c in cards) + ' |')
    res = [r for r in PEAKS if r != 'card_tokens']
    p('\n## Resource use: spent a seat-game (spent / gained)\n')
    p('| agent | arm | ' + ' | '.join(res) + ' |')
    p('|---|---|' + '---|' * len(res))
    for ag, a, s in rows:
        sg = s['seatgames']
        p(f'| {ag} | {a} | ' + ' | '.join(
            f'{s["spent"][r] / sg:.1f} ({100 * s["spent"][r] / max(s["gained"][r], 1):.0f}%)' for r in res) + ' |')
    return L


def main(argv):
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    opts = {'--base': None, '--md': None}
    lists = {'--counters': [], '--cards': []}
    reads, cur = [], None
    for x in argv:
        if x in opts or x in lists:
            cur = x
        elif cur in opts:
            opts[cur], cur = x, None
        elif cur in lists:
            lists[cur].append(x)
        else:
            reads.append(x)
    if not reads:
        sys.exit(__doc__)
    paths = [source(r)[0] for r in reads]
    labels = [os.path.basename(os.path.dirname(os.path.abspath(p))) if os.path.basename(p) == 'games.jsonl'
              else os.path.splitext(os.path.basename(p))[0] for p in paths]
    if len(reads) > 1 and len(set(labels)) == len(labels):
        # pool reads whose arms share names (a read farmed in seed ranges); label each otherwise
        arms = [{split_variant(c['variant'])[1] for c in cells_of(next(stream(r))['spec'])} for r in reads]
        pool = all(a == arms[0] for a in arms)
    else:
        pool = True
    rows = [x for r, lab in zip(reads, labels) for x in games(r, None if pool else lab)]
    text = '\n'.join(tables(summarize(rows, lists['--counters'], lists['--cards']), opts['--base'], lists['--cards'])) + '\n'
    print(text)
    if opts['--md']:
        with open(opts['--md'], 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main(sys.argv[1:])
