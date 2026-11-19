"""A paired read's games, decoded, and the tables 252-254's item readers each rebuilt (255; 252's `read_lib.py` note).

    python scripts/bench/read_lib.py READ[@LABEL] [READ ...] [--base ARM] [--levels D[,D]] [--agent-arms]
                                     [--arm NAME=SPEC ...] [--counters PREFIX ...] [--cards ID ...] [--md OUT]

A READ is a run's folder, a games file with its header, or FILE=SPEC (a bare farmed JSONL and its spec;
`screens.stream`); several READs pool when their arms share names, else each row leads with its READ's name (the
file stem, or the folder of a `games.jsonl`). `READ@LABEL` (281) leads that READ's arms with LABEL (`LABEL/base`)
even when the arm sets match, so an old native run pairs against a new farmed run on the same seeds; READs sharing a
LABEL pool, and with any LABEL given the unlabeled READs pool unled. Give the READs first: `--counters` and `--cards`
take every word after. Cells come from the spec (`screens.cells_of`: configs, then variants, then difficulties, then
agents); a variant named `PAIR-ARM` (or `PAIR_ARM`) is arm ARM of pair PAIR, a bare `PAIR` arm `base`, and a spec
with several configs leads each arm with the config's file stem. Every arm is paired against the base arm (`--base`,
default `base`, `off`, else the first) on the same pair, level, agent and seed. `--levels 3,4` (281) reads those
levels only. `--agent-arms` (281): an agent with options (`strategist:w.sink=1`) whose bare name (`strategist`) is an
agent of any READ is an arm of the bare agent, named by its options (`sink1`; `on-sink1` on arm `on`), so it pairs
against the bare agent's games on the same pair, level and seed. `--arm NAME=SPEC` (286, once per agent, implies
`--agent-arms`) names SPEC's arm NAME (`--arm sink-high=strategist:w.sink=10`); two specs of one agent squashing to one
name stop it (288: the judge's and greedy's arms may share one, each paired against its own agent).

Tables: wins by level with the paired gap ± one SE; the gap by level; losses by reason a game; board powers a seat-game
of their board (by their counters, `ai.power.<board>.<slug>`, so any revision reads right); standard actions a
seat-game; `--counters`: each counter whose key starts with a PREFIX, a game (per-round `.rNN` keys left out);
`--cards`: each card's plays a game; resource use a seat-game (spent, and spent / gained), in PEAKS order.

As a library: `games(read, label, agent_arms)` yields `Game`s; `seat_boards`, `counter_sums`, `resources` and `paired_gap` are what the
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


def option_tag(agent):
    """An agent's options as an arm name: `strategist:w.sink=0.5` -> `sink05` (258's agent_arms.py)."""
    return agent.split(':', 1)[1].replace('w.', '').replace('=', '').replace('.', '')


def games(read, label=None, agent_arms=(), names=None):
    """READ's games as `Game(arm, pair, level, agent, seed, g)`, decoded from its spec. ARM leads with LABEL (a
    READ's name, when several READs don't pool) and with the config's file stem when the spec has several configs.
    AGENT is the spec's name up to any `:` (its overrides dropped), or the whole spec where two agents share a name.
    AGENT_ARMS (`--agent-arms`): bare agent names; an agent with options whose bare name is one of them is that bare
    agent, its options an arm of the variant's arm: NAMES[spec] (`--arm NAME=SPEC`, 286), else `option_tag`. Two
    specs with one arm name stop it."""
    names = names or {}
    lines = stream(read)
    spec = next(lines)['spec']
    cells = cells_of(spec)
    many = len(spec['configs']) > 1
    as_arm = {a: a.split(':')[0] for a in spec['agents'] if ':' in a and a.split(':')[0] in agent_arms}
    tags = {a: names.get(a) or option_tag(a) for a in as_arm}
    # 286: `w.sink=1.0` and `w.sink=10` both squash to `sink10`; their games would pool as one arm (288: of one bare
    # agent: the judge's and greedy's arms of one name stay apart, each paired against its own agent)
    for bare, t in {(as_arm[a], tags[a]) for a in tags}:
        same = sorted(a for a in tags if tags[a] == t and as_arm[a] == bare)
        if len(same) > 1:
            sys.exit(f"--agent-arms: {' and '.join(same)} are both arm {t}: name them apart with --arm NAME=SPEC")
    # 261: an agent-lever screen ('strategist', 'strategist:w.sink=0.5', ...) keeps each spec whole: cut at ':' only
    # where the short names stay distinct (it read as "pooled reads overlap").
    specs = sorted({c['agent'] for c in cells} - set(as_arm))
    short = {a: a.split(':')[0] for a in specs}
    if len(set(short.values())) < len(specs):
        short = {a: a for a in specs}
    short.update(as_arm)
    for g in lines:
        if 'cell' not in g or 'won' not in g:
            continue
        c = cells[g['cell']]
        pair, arm = split_variant(c['variant'])
        if c['agent'] in as_arm:
            tag = tags[c['agent']]
            arm = tag if arm == 'base' else f'{arm}-{tag}'
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
    (base %, new %, gap in points, its SE (None for one key), n), or None with no key shared."""
    keys = [k for k in base if k in new and (keep is None or keep(k))]
    if not keys:
        return None
    n = len(keys)
    b = 100 * sum(base[k] for k in keys) / n
    diffs = [new[k] - base[k] for k in keys]
    # 266 (D117): `screens.gap` gives 0 below two pairs, so one paired seed read new = base. The gap is the mean
    # difference at any n; its SE is None below two.
    m, se, _ = gap(diffs) if n > 1 else (100 * diffs[0], None, n)
    return b, b + m, m, se, n


def gap_text(pr):
    """A `paired_gap` result as `+gap ± SE` (`+gap, n 1` with no SE), or '' for None."""
    if not pr:
        return ''
    return f'{pr[2]:+.1f} ± {pr[3]:.1f}' if pr[3] is not None else f'{pr[2]:+.1f}, n 1'


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
                             'pooled reads overlap; read them apart (READ@LABEL), or an agent\'s options as arms '
                             '(--agent-arms)')
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
    if base_arm is None:
        base = next((a for a in arms if a.split('/')[-1] in ('base', 'off')), arms[0])
    else:
        # an arm led by its read or config (`stem/base`) answers to its last part too, as the default reads it
        base = next((a for a in arms if base_arm in (a, a.split('/')[-1])), None)
        if base is None:
            # 266 (D117): a --base naming no arm fell back to the default silently, pairing against another arm.
            raise SystemExit(f"--base {base_arm}: no such arm; the arms: {', '.join(arms)}")
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
        g = gap_text(pr)
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
            cells.append(gap_text(pr) or '-')
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
    opts = {'--base': None, '--md': None, '--levels': None}
    lists = {'--counters': [], '--cards': []}
    reads, cur, agent_arms, names = [], None, False, {}
    for x in argv:
        if x == '--agent-arms':
            agent_arms, cur = True, None
        elif cur == '--arm':
            name, eq, spec = x.partition('=')
            if not eq or not name or ':' not in spec:
                sys.exit(f'--arm {x}: give NAME=SPEC, SPEC an agent with options (sink1=strategist:w.sink=1)')
            if spec in names:
                sys.exit(f'--arm: {spec} named twice')
            names[spec], cur = name, None
        elif x == '--arm':
            cur = x
        elif x in opts or x in lists:
            cur = x
        elif cur in opts:
            opts[cur], cur = x, None
        elif cur in lists:
            lists[cur].append(x)
        elif x.startswith('--'):
            sys.exit(f'read_lib: unknown flag {x}')
        else:
            reads.append(x)
    if cur == '--arm':
        sys.exit('--arm needs NAME=SPEC')
    if not reads:
        sys.exit(__doc__)
    agent_arms = agent_arms or bool(names)  # --arm implies --agent-arms
    # 281: READ@LABEL leads that read's arms with LABEL whatever its arms
    given = [r.rsplit('@', 1) if re.search(r'@[\w.+-]+$', r) else [r, None] for r in reads]
    reads = [r for r, _ in given]
    paths = [source(r)[0] for r in reads]
    labels = [os.path.basename(os.path.dirname(os.path.abspath(p))) if os.path.basename(p) == 'games.jsonl'
              else os.path.splitext(os.path.basename(p))[0] for p in paths]
    if any(lab for _, lab in given):
        labels = [lab for _, lab in given]
    elif len(reads) > 1 and len(set(labels)) == len(labels):
        # pool reads whose arms share names (a read farmed in seed ranges); label each otherwise
        arms = [{split_variant(c['variant'])[1] for c in cells_of(next(stream(r))['spec'])} for r in reads]
        if all(a == arms[0] for a in arms):
            labels = [None] * len(reads)
    else:
        labels = [None] * len(reads)
    bare = set()
    if agent_arms:
        bare = {a for r in reads for a in next(stream(r))['spec']['agents'] if ':' not in a}
        if not bare:
            sys.exit('--agent-arms: no READ has a bare agent (one with no options) to pair the option agents against')
        specs = {a for r in reads for a in next(stream(r))['spec']['agents']}
        stray = sorted(s for s in names if s not in specs or s.split(':')[0] not in bare)
        if stray:
            sys.exit(f"--arm: no READ has {', '.join(stray)} beside its bare agent")
    rows = [x for r, lab in zip(reads, labels) for x in games(r, lab, bare, names)]
    if opts['--levels']:
        keep = {int(d) for d in opts['--levels'].split(',')}
        rows = [r for r in rows if r.level in keep]
        if not rows:
            sys.exit(f"--levels {opts['--levels']}: no game at those levels")
    text = '\n'.join(tables(summarize(rows, lists['--counters'], lists['--cards']), opts['--base'], lists['--cards'])) + '\n'
    print(text)
    if opts['--md']:
        with open(opts['--md'], 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    main(sys.argv[1:])
