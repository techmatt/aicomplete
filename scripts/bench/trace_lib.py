"""Reads `aic-sim run CONFIG --games 1 --trace` output, the walk per-item trace scripts share (281).

    from trace_lib import walk, summary, card_names, hexes

- `walk(trace)`: a `Step` a JSON line (header lines skipped): `round` (0 in setup), `phase` ('' before the first
  phase_started), `seat` (the action's or event's own, else None), `actor` (the seat of the last action in this
  phase, for events that carry none), `kind` ('action', or the event's type), `action` / `event` (the dict),
  `options` (an action's option count), `ask` (the prompt of the `asked` event just before an action, else None),
  `line` (1-based line number). round_started and phase_started steps carry their own round and phase.
- `summary(trace)`: the run summary printed after the trace: `header` (the `seeds ...` line), `overrides` (the
  `run overrides:` line's KEY=VALUE list, [] with none), `rows` (label -> raw value), `counters` (first word of a
  `key /game`, `/game reaching it` or `mean where reached` row -> float).
- `card_names()`: id -> display name over content's cards, threats, projects, alignment and adversary event cards.
- `hexes(countries=2)`: (q, r) -> 'Country #i', i 1-16 row by row from each layout piece's anchor along its axes.

Paths are relative to the repo root unless given. Standard library only.
"""
import collections
import itertools
import json
import os
import re

REPO = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
Step = collections.namedtuple('Step', 'round phase seat actor kind action options event ask line')
# Hex steps on the map's axial coordinates (268).
DIRS = {'E': (1, 0), 'W': (-1, 0), 'SE': (0, 1), 'NW': (0, -1), 'SW': (-1, 1), 'NE': (1, -1)}


def lines_of(trace):
    """A path's lines, or the iterable of lines given."""
    if isinstance(trace, (str, os.PathLike)):
        with open(trace, encoding='utf-8') as f:
            return f.read().splitlines()
    return list(trace)


def walk(trace):
    rnd, phase, actor, ask = 0, '', None, None
    for n, line in enumerate(lines_of(trace), 1):
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if 'action' in d:
            a = d['action']
            actor = a.get('seat', actor)
            yield Step(rnd, phase, a.get('seat'), actor, 'action', a, d.get('options', 1), None, ask, n)
            ask = None
            continue
        e = d.get('event')
        if not e:
            continue  # a game's header line
        t = e.get('type')
        if t == 'round_started':
            rnd = e['round']
        elif t == 'phase_started':
            phase, actor = e['phase'], None
        elif t == 'asked':
            ask = e.get('prompt')
        yield Step(rnd, phase, e.get('seat'), e.get('seat', actor), t, None, 0, e, None, n)


ROW = re.compile(r'^(\S.*?)\s{2,}(\S.*)$')
COUNTER = re.compile(r'^(\S+) (/game|/game reaching it|mean where reached)$')


def summary(trace):
    out = {'header': None, 'overrides': [], 'rows': {}, 'counters': {}}
    for line in lines_of(trace):
        if line.startswith('{') or not line.strip():
            continue
        if line.startswith('seeds '):
            out['header'] = line
        elif line.startswith('run overrides:'):
            out['overrides'] = line.split(':', 1)[1].split()
        elif (m := ROW.match(line)):
            label, value = m.group(1), m.group(2).strip()
            out['rows'][label] = value
            if (c := COUNTER.match(label)):
                try:
                    out['counters'][c.group(1)] = float(value)
                except ValueError:
                    pass
    return out


def content(name):
    with open(os.path.join(REPO, 'content', name), encoding='utf-8') as f:
        return json.load(f)


def card_names():
    names = {}
    for f in ('cards.json', 'threats.json', 'projects.json', 'alignment.json'):
        names.update((c['id'], c['name']) for c in content(f))
    names.update((c['id'], c['name']) for c in content('adversary.json')['events'])
    return names


def hexes(countries=2):
    m = content('map.json')
    size = m['piece']['size']
    lay = next(x for x in m['layouts'] if x['countries'] == countries)
    names = {}
    for ci, p in enumerate(lay['pieces']):
        (a0, b0), (a1, b1) = DIRS[p['axes'][0]], DIRS[p['axes'][1]]
        for i in range(size * size):
            row, col = divmod(i, size)
            names[(p['anchor'][0] + row * a0 + col * a1, p['anchor'][1] + row * b0 + col * b1)] = \
                f"{m['countries'][ci]['name']} #{i + 1}"
    return names


def first_diff(a, b, ignore=()):
    """The first differing JSON line of two traces, as (line_a, line_b, context): context the last round_started
    and phase_started lines of A before it. None if the JSON lines match. `ignore`: a line goes whose event has a
    string value equal to a KEY (a KEY ending in '.' matches as a prefix), and a field named KEY goes from every line."""
    def keep(text):
        out = []
        for n, line in enumerate(lines_of(text), 1):
            if not line.startswith('{'):
                continue
            d = json.loads(line)
            body = d.get('event') or d.get('action') or d
            vals = [v for v in body.values() if isinstance(v, str)]
            if any(v == k or (k.endswith('.') and v.startswith(k)) for k in ignore for v in vals):
                continue
            for part in (d, body):
                for k in ignore:
                    part.pop(k, None)
            out.append((n, line, d))
        return out
    ka, kb = keep(a), keep(b)
    end = (None, '(end of trace)', None)
    for i, ((na, la, da), (nb, lb, db)) in enumerate(itertools.zip_longest(ka, kb, fillvalue=end)):
        if da != db:
            ctx = {}
            for _, line, d in ka[:i]:
                t = (d.get('event') or {}).get('type')
                if t in ('round_started', 'phase_started'):
                    ctx[t] = line
                    if t == 'round_started':
                        ctx.pop('phase_started', None)
            return (na, la), (nb, lb), [ctx[t] for t in ('round_started', 'phase_started') if t in ctx]
    return None
