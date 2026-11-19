"""276: the player side of a traced game, before and after the board rework, by seat and round, at 268's rates.

    python trace_seats.py BEFORE_DIR AFTER_DIR [--base-minutes 52.7] [--level 2]

Each DIR holds `aic-sim run --trace` outputs named d<level>-<seed>.out (the same seeds on each side: 268000-268013,
the judge, Builder and Protector, as 268, 274 and 275 traced). Per seat-round:
  decisions   the seat's actions with more than one option (268's "real decisions"), every phase
  turns       its action-phase actions (a power, standard action, card play, building Action, converter or pass)
  powers      its board-power uses (Growth options and tiers included)
  power_steps the choices a power's use asks (track, hex, target, sacrifice) with more than one option
  options     the mean number of options at its action-phase turns (the menu a human scans)
  components  resources it gains or pays (one each) and tokens moved during its actions
  offered     board uses (a group once) the seat could make as its action phase began: the yes/no items it
              weighs, each a check at 268's rate (the run summary's `ai.offered.<board>`, patched into r130's tree)
  checks      fresh-spot checks (before: Buy In, Accord and Peer Review's riders read every node of the seat) and
              the targets its powers pick (a hex in reach meeting the target box)
Minutes at 268's rates: 15 s a decision, 3 s a component, 10 s a check. `--base-minutes`: 275's d2 minutes a round.
"""
import argparse
import collections
import glob
import json
import os
import statistics

RATES = {'decisions': 15, 'components': 3, 'checks': 10, 'offered': 10}
TURN = {'power', 'standard', 'play', 'use', 'convert', 'pass'}
# The powers whose rider checked a fresh spot before 276 (r130's indices on each board; read by name below).
RIDERS = {'Buy In', 'Accord', 'Peer Review'}


def boards():
    """Each board's powers by index, from the content the trace's binary ran (the caller's tree)."""
    with open('content/ais.json', encoding='utf-8') as f:
        return [[p['name'] for p in b['powers']] for b in json.load(f)]


def read(path, names, riders):
    rows = collections.defaultdict(collections.Counter)  # (round, seat) -> counts
    opts = collections.defaultdict(list)
    rnd, actor, in_power = 0, None, False
    offered = {}
    for line in open(path, encoding='utf-8'):
        if line.startswith('ai.offered.'):
            # The run summary's `ai.offered.<board> /game N` (276's stats-only count, patched into r130's tree):
            # board uses the seat could make as its action phase began, a group counted once.
            parts = line.split()
            offered[parts[0].split('.')[2]] = float(parts[-1])
        if not line.startswith('{'):
            continue
        d = json.loads(line)
        if 'action' in d:
            a, n = d['action'], d.get('options', 1)
            s = a.get('seat')
            if s is None:
                continue
            actor = s
            c = rows[(rnd, s)]
            if n > 1:
                c['decisions'] += 1
            t = a['type']
            if t in TURN:
                c['turns'] += 1
                opts[(rnd, s)].append(n)
                in_power = t == 'power'
                if in_power:
                    c['powers'] += 1
                    name = names[s % len(names)][a['power']] if a['power'] < len(names[s % len(names)]) else ''
                    if name in riders:
                        c['checks'] += 1
            elif t == 'choose' and in_power:
                if n > 1:
                    c['power_steps'] += 1
            elif t != 'choose':
                in_power = False
            continue
        e = d.get('event')
        if not e:
            continue
        t = e['type']
        if t == 'round_started':
            rnd = e['round']
            actor, in_power = None, False
        elif t == 'phase_started':
            actor, in_power = None, False
        elif t == 'resource_changed' and e.get('seat') is not None:
            rows[(rnd, e['seat'])]['components'] += 1
        elif t == 'tokens_changed' and actor is not None:
            rows[(rnd, actor)]['components'] += abs(e['delta'])
        elif t == 'asked' and in_power and 'target' in e.get('prompt', '').lower():
            rows[(rnd, actor)]['checks'] += 1
    played = max((r for r, _ in rows), default=1) or 1
    for s, b in enumerate(('builder', 'protector')):
        if b in offered:
            for r in range(1, played + 1):
                rows[(r, s)]['offered'] += offered[b] / played
    return rows, opts


def load(folder, level, names, riders):
    out = {}
    for p in sorted(glob.glob(os.path.join(folder, f'd{level}-*.out'))):
        out[os.path.basename(p)] = read(p, names, riders)
    return out


def summary(games, seat):
    keys = ['decisions', 'turns', 'powers', 'power_steps', 'components', 'checks', 'offered']
    per = collections.defaultdict(list)
    opt = []
    for rows, opts in games.values():
        for (r, s), c in rows.items():
            if s != seat or r == 0:
                continue
            for k in keys:
                per[k].append(c[k])
            opt.extend(opts.get((r, s), []))
    out = {k: statistics.mean(v) if v else 0 for k, v in per.items()}
    out['options'] = statistics.mean(opt) if opt else 0
    out['minutes'] = sum(out.get(k, 0) * RATES[k] for k in RATES) / 60
    out['rounds'] = len(per['decisions'])
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('before')
    ap.add_argument('after')
    ap.add_argument('--base-minutes', type=float, default=52.7)
    ap.add_argument('--level', type=int, default=2)
    ap.add_argument('--before-content', default=None, help='content/ais.json of the before binary')
    a = ap.parse_args()
    now = boards()
    if a.before_content:
        with open(a.before_content, encoding='utf-8') as f:
            old = [[p['name'] for p in b['powers']] for b in json.load(f)]
    else:
        old = now
    before = load(a.before, a.level, old, RIDERS)
    after = load(a.after, a.level, now, set())
    print(f'### d{a.level}: the traced games ({len(before)} before, {len(after)} after), a seat-round\n')
    print('| Seat | Side | Decisions | Action turns | Powers used | Power steps | Menu (options a turn) | Weighed (board uses offered) | Components | Checks | Minutes |')
    print('|---|---|---|---|---|---|---|---|---|---|---|')
    total = 0.0
    for seat, name in ((0, 'Builder'), (1, 'Protector')):
        b, n = summary(before, seat), summary(after, seat)
        for side, x in (('before', b), ('after', n)):
            print(f"| {name} | {side} | {x['decisions']:.1f} | {x['turns']:.1f} | {x['powers']:.1f} | {x['power_steps']:.1f} | "
                  f"{x['options']:.1f} | {x['offered']:.1f} | {x['components']:.1f} | {x['checks']:.1f} | {x['minutes']:.1f} |")
        total += n['minutes'] - b['minutes']
        print(f"| {name} | **moved** | {n['decisions'] - b['decisions']:+.1f} | {n['turns'] - b['turns']:+.1f} | "
              f"{n['powers'] - b['powers']:+.1f} | {n['power_steps'] - b['power_steps']:+.1f} | {n['options'] - b['options']:+.1f} | {n['offered'] - b['offered']:+.1f} | "
              f"{n['components'] - b['components']:+.1f} | {n['checks'] - b['checks']:+.1f} | **{n['minutes'] - b['minutes']:+.1f}** |")
    rb = statistics.mean(len({r for (r, _) in rows if r}) for rows, _ in before.values())
    ra = statistics.mean(len({r for (r, _) in rows if r}) for rows, _ in after.values())
    print(f'\nRounds a game: {rb:.2f} before, {ra:.2f} after.')
    if a.level == 2:
        print(f'Both seats: {total:+.1f} minutes a round; 275\'s {a.base_minutes:.1f} becomes {a.base_minutes + total:.1f} by these rates.')
    else:
        print(f'Both seats: {total:+.1f} minutes a round.')


main()
