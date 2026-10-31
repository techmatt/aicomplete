"""What the screen summaries share: reading an `aic-sim experiment` output directory
(DIR/results.json for the cell labels, DIR/games.jsonl for the games), the helpers every
table uses, the presence-source line and the dominance gate. `cards/screen.py` is the
summary; a script one folder down puts scripts/ on `sys.path` to import this.
"""
import collections
import json
import math
import re
import sys

# A per-round counter's suffix (`.r07`).
ROUND = re.compile(r'\.r\d\d$')


def load(d):
    """(cells by id, games by cell), without the games' action lists. A last line a
    killed run cut short is dropped."""
    cells = {c['cell']: c for c in json.load(open(f'{d}/results.json', encoding='utf-8'))['cells']}
    games = collections.defaultdict(list)
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            try:
                g = json.loads(line)
            except json.JSONDecodeError:
                continue
            g.pop('actions', None)
            games[g['cell']].append(g)
    return cells, games


def old_name(k):
    """The counter's name in results from before 026, when chips were called hardware."""
    return k.replace('chips', 'hardware')


def cnt(g, k):
    c = g['counters']
    return c.get(k, c.get(old_name(k), 0))


def mean(gs, f):
    return sum(f(g) for g in gs) / len(gs) if gs else 0.0


def avg(xs):
    return sum(xs) / len(xs) if xs else 0.0


def hist(rounds):
    """`3:1 5:2`: how many of each round."""
    c = collections.Counter(rounds)
    return ' '.join(f'{r}:{c[r]}' for r in sorted(c)) or '–'


def ends_text(gs):
    """`win purge 12, loss presence 3`: how the games ended, commonest first."""
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
        if k not in seen and old_name(k) not in seen and k not in warned:
            warned.add(k)
            print(f'warning: no game has counter {k}: it may be gone, so its 0 means nothing', file=sys.stderr)
        return cnt(g, k)
    return checked


def sources(games):
    """The presence lost, by source, over all cells."""
    tot = collections.Counter()
    for gs in games.values():
        for g in gs:
            for k, v in g['counters'].items():
                if k.startswith('board.presence_lost.'):
                    tot[k.split('.')[-1]] += v
    n = sum(tot.values()) or 1
    print('Presence lost by source: ' + ', '.join(f'{k} {v} ({100 * v / n:.0f}%)' for k, v in tot.most_common()))


def gate_games(rows):
    """Games per check from results.json's "gate" rows, by (config, variant, difficulty,
    check)."""
    return {(r.get('config'), r.get('variant'), r.get('difficulty'), r['check']): r['games'] for r in rows}


def gate(cells, games, rows=None):
    """Each check's gap to its own agent: `x:...` against `x` (win % points, ± one standard
    error; X: the check matched or beat it). Both sides are over the seeds both played: a
    gate stops a check early and its agent runs on. With `rows` (the "gate" key of
    results.json), a games column too: the row's count, else the games in the file."""
    per = gate_games(rows) if rows is not None else None
    if per is None:
        print('| variant | check | win % | gap to its agent |')
        print('|---|---|---|---|')
    else:
        print('| variant | check | games | win % | gap to its agent |')
        print('|---|---|---|---|---|')
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
            n = '' if per is None else f' {per.get((conf, v, d, a), len(gs))} |'
            name = f'{conf} {v}' if len(configs) > 1 else v
            print(f'| {name} d{d} | {a} |{n} {100 * q:.0f} | {100 * (p - q):+.0f} ±{100 * se:.0f}{flag} |')
