"""What the screen summaries share: reading an `aic-sim experiment` output directory
(DIR/results.json for the cell labels, DIR/games.jsonl for the games), the helpers every
table uses, the node-source line and the dominance gate. `cards/screen.py` is the
summary; a script one folder down puts scripts/ on `sys.path` to import this.
"""
import collections
import functools
import json
import math
import os
import re
import sys

CONTENT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'content')

# A per-round counter's suffix (`.r07`).
ROUND = re.compile(r'\.r\d\d$')


@functools.lru_cache(maxsize=None)
def content(name):
    """content/NAME.json, parsed. Shared: copy before changing it."""
    with open(os.path.join(CONTENT, f'{name}.json'), encoding='utf-8') as f:
        return json.load(f)


def stream(d):
    """DIR/games.jsonl line by line: its header first, then each game. A live run or one a
    kill cut short can end in half a line: that line is dropped. Needs no results.json."""
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        yield json.loads(f.readline())
        for line in f:
            try:
                yield json.loads(line)
            except json.JSONDecodeError:
                continue


def cell_names(spec):
    """(variant, agent) of each cell, by cell id: the order of `cells` in
    crates/aic-sim/src/experiment.rs, which is why a spec here has one config and difficulty."""
    assert len(spec['configs']) == 1 and len(spec['difficulties']) == 1, \
        f"{spec['name']}: needs one config and one difficulty"
    return [(v['name'], a) for v in spec['variants'] for a in spec['agents']]


def cell_labels(spec):
    """Each cell's name, by cell id, for any spec: VARIANT/AGENT, led by the config's file
    stem and `dN` when the spec has more than one config or difficulty
    (CONFIG/dN/VARIANT/AGENT). The order is `cells` in crates/aic-sim/src/experiment.rs:
    config, variant, difficulty, agent."""
    configs, diffs = spec['configs'], spec['difficulties']
    out = []
    for c in configs:
        stem = os.path.splitext(os.path.basename(c))[0]
        for v in spec['variants']:
            for d in diffs:
                lead = (f'{stem}/' if len(configs) > 1 else '') + (f'd{d}/' if len(diffs) > 1 else '')
                out.extend(f"{lead}{v['name']}/{a}" for a in spec['agents'])
    assert len(set(out)) == len(out), f"{spec['name']}: two cells share a name (configs with one file name?)"
    return out


def load(d):
    """(cells by id, games by cell), without the games' action lists."""
    with open(f'{d}/results.json', encoding='utf-8') as f:
        cells = {c['cell']: c for c in json.load(f)['cells']}
    games = collections.defaultdict(list)
    lines = stream(d)
    next(lines)
    for g in lines:
        g.pop('actions', None)
        games[g['cell']].append(g)
    return cells, games


# Renamed counters: name -> former name. A former name may itself be renamed: `old_names`
# follows the chain, newest first.
RENAMED = {
    # 086: presence -> node, the adversary's node -> botnet (also `board.node_lost.<source>`,
    # in `old_names`).
    'adversary.compute_botnets': 'adversary.compute_nodes',
    'adversary.botnets_made': 'adversary.nodes_made',
    'adversary.botnets_removed': 'adversary.nodes_removed',
    'alignment.sacrificed_node': 'alignment.sacrificed_presence',
    'alignment.purge_blocked_by_botnets': 'alignment.purge_blocked_by_nodes',
    'board.last_node_safe': 'board.last_presence_safe',
    'board.node_placed': 'board.presence_placed',
    'board.node_capstones': 'board.presence_capstones',
    'board.node_lost': 'board.presence_lost',
    'board.node_drained': 'board.presence_drained',
    'effects.botnets_cut': 'effects.nodes_cut',
    # 102 (D40): `board.stations_free` (a station built paying no chips) read as
    # `board.stations_freed` (the Free a station action).
    'board.stations_built_free': 'board.stations_free',
}


def old_names(k):
    """The counter's former names, newest first: the `RENAMED` chain (and 086's
    `board.node_lost.<source>`), then its name before 026, when chips were called hardware."""
    names, n = [], k
    while True:
        if n in RENAMED:
            n = RENAMED[n]
        elif n.startswith('board.node_lost.'):
            n = 'board.presence_lost.' + n[len('board.node_lost.'):]
        else:
            break
        names.append(n)
    names.append(k.replace('chips', 'hardware'))
    return [n for n in dict.fromkeys(names) if n != k]


def cnt(g, k):
    """Counter `k` of game `g`, read under its former name in older results; 0 if absent."""
    c = g['counters']
    if k in c:
        return c[k]
    return next((c[o] for o in old_names(k) if o in c), 0)



def mean(gs, f):
    return sum(f(g) for g in gs) / len(gs) if gs else 0.0


def avg(xs):
    return sum(xs) / len(xs) if xs else 0.0


def hist(rounds):
    """`3:1 5:2`: how many of each round."""
    c = collections.Counter(rounds)
    return ' '.join(f'{r}:{c[r]}' for r in sorted(c)) or '–'


def ends_text(gs):
    """`win purge 12, loss node 3`: how the games ended, commonest first."""
    e = collections.Counter(('win ' if g['won'] else 'loss ') + g['reason'] for g in gs)
    return ', '.join(f'{k} {v}' for k, v in e.most_common())


def seats(c):
    """A cell's seats (a seat a country). Outputs from before 052 have no `countries`: their
    variant's name decides, the c3 variants three and everything else two."""
    return c.get('countries', 3 if c['variant'].startswith('c3') else 2)


def checked_cnt(games):
    """`cnt` for a script on older rules: a key in no game warns once on stderr, since
    it may be a counter that is gone and its 0 would pass for a measurement."""
    seen = {k for gs in games.values() for g in gs for k in g['counters']}
    warned = set()

    def checked(g, k):
        if k not in seen and not seen.intersection(old_names(k)) and k not in warned:
            warned.add(k)
            print(f'warning: no game has counter {k}: it may be gone, so its 0 means nothing', file=sys.stderr)
        return cnt(g, k)
    return checked


def sources(games):
    """The node lost, by source, over all cells."""
    tot = collections.Counter()
    for gs in games.values():
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.node_lost.'):
                    tot[k.split('.')[-1]] += v
    n = sum(tot.values()) or 1
    print('Node lost by source: ' + ', '.join(f'{k} {v} ({100 * v / n:.0f}%)' for k, v in tot.most_common()))


def gate_games(rows):
    """Games per check from results.json's "gate" rows, by (config, variant, difficulty,
    check)."""
    return {(r.get('config'), r.get('variant'), r.get('difficulty'), r['check']): r['games'] for r in rows}


def gap(pairs):
    """Mean of paired differences and its SE, in points, and how many."""
    n = len(pairs)
    if n < 2:
        return 0.0, 0.0, n
    m = sum(pairs) / n
    var = sum((x - m) ** 2 for x in pairs) / (n - 1)
    return 100 * m, 100 * math.sqrt(var / n), n


# 068: in the order of `board::LEFT`, which `UNSPENT` in crates/aic-sim/src/experiment.rs takes,
# each with `UNSPENT_HIGH` there.
UNSPENT = (('credits', 12), ('chips', 4), ('energy', 4), ('antimatter', 2), ('ambassadors', 3))


def unspent(cells, games):
    """Resources a seat held after its last action phase: each cell's mean by seat, ! over the flag
    (docs/principles.md, *Unspent resources*). Says which cells flag and how many means."""
    print()
    print('| variant | check | seat | ' + ' | '.join(n for n, _ in UNSPENT) + ' |')
    print('|---|---|---|' + '---|' * len(UNSPENT))
    flagged = collections.Counter()
    for cell, gs in sorted(games.items()):
        c = cells[cell]
        rows = [g['margin'].get('unspent', []) for g in gs]
        for seat in range(max((len(r) for r in rows), default=0)):
            means = [sum(r[seat][k] for r in rows if len(r) > seat) / len(rows) for k in range(len(UNSPENT))]
            marks = []
            for (name, high), m in zip(UNSPENT, means):
                marks.append(f'{m:.1f}{"!" if m > high else ""}')
                flagged[name] += m > high
            print(f"| {c['variant']} d{c['difficulty']} | {c['agent']} | {seat} | " + ' | '.join(marks) + ' |')
    print(f'\nFlagged seat means: {dict(flagged) or "none"}.')


def gate(cells, games, rows=None):
    """Each check's gap to its own agent: `x:...` against `x`, in the gate's sign, agent minus
    check (win % points, ± one standard error, and the paired SE seed by seed; X: the check
    matched or beat it). Both sides are over the seeds both played: a gate stops a check early
    and its agent runs on. With `rows` (the "gate" key of results.json), a games column too:
    the row's count, else the games in the file. `close`: a check within 2 paired SEs of its
    agent, whatever stopped it: the close-out list, not `stopped` or `decided`."""
    per = gate_games(rows) if rows is not None else None
    if per is None:
        print('| variant | check | win % | gap to its agent | paired se | close |')
        print('|---|---|---|---|---|---|')
    else:
        print('| variant | check | games | win % | gap to its agent | paired se | close |')
        print('|---|---|---|---|---|---|---|')
    by = collections.defaultdict(dict)
    for cell, gs in games.items():
        c = cells[cell]
        by[(c['config'], c['variant'], c['difficulty'])][c['agent']] = gs
    configs = {k[0] for k in by}
    for (conf, v, d), agents in sorted(by.items()):
        for a, gs in sorted(agents.items()):
            base = agents.get(a.split(':')[0])
            if not base:
                continue
            seeds = {g['seed'] for g in gs} & {g['seed'] for g in base}
            if not seeds:
                continue
            base = [g for g in base if g['seed'] in seeds]
            gs = [g for g in gs if g['seed'] in seeds]
            p = sum(g['won'] for g in base) / len(base)
            q = sum(g['won'] for g in gs) / len(gs)
            se = math.sqrt(p * (1 - p) / len(base) + q * (1 - q) / len(gs))
            flag = '' if p - q > 0 or ':' not in a else ' X'
            won = {g['seed']: g['won'] for g in gs}
            paired = gap([g['won'] - won[g['seed']] for g in base])[1]
            n = '' if per is None else f' {per.get((conf, v, d, a), len(gs))} |'
            name = f'{conf} {v}' if len(configs) > 1 else v
            close = 'close' if ':' in a and abs(100 * (p - q)) < 2 * max(paired, 1e-9) else ''
            print(f'| {name} d{d} | {a} |{n} {100 * q:.0f} | {100 * (p - q):+.0f} ±{100 * se:.0f}{flag} | {paired:.0f} | {close} |')
