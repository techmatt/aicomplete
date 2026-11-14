"""223: a new build's farmed read against the base's on the same seeds (same spec): win % by agent, difficulty
and board pair, paired on pair, difficulty and seed; rounds; and counters a game.

    python read.py NEW.jsonl BASE.jsonl SPEC.json [--counters PREFIX,...] [--powers]

`--powers`: each AI power's uses a game in games seating its board (bp seats Builder + Protector, br Builder +
Researcher, pr Protector + Researcher), new beside base.
"""
import argparse
import json
import math
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
SEATS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}


def load(path, spec):
    cells = [(v['name'], d, a) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
             for a in spec['agents']]
    games = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            v, d, a = cells[g['cell']]
            games[(a, d, v, g['seed'])] = g
    return games


def gap(new, base, ks):
    diff = [100 * (new[k]['won'] - base[k]['won']) for k in ks if k in base]
    if not diff:
        return '', ''
    m = sum(diff) / len(diff)
    sd = math.sqrt(sum((x - m) ** 2 for x in diff) / max(1, len(diff) - 1))
    return f'{m:+6.1f}', f'{sd / math.sqrt(len(diff)):4.1f}'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('new')
    ap.add_argument('base')
    ap.add_argument('spec')
    ap.add_argument('--counters', default='')
    ap.add_argument('--powers', action='store_true')
    args = ap.parse_args()
    spec = json.load(open(args.spec, encoding='utf-8'))
    new, base = load(args.new, spec), load(args.base, spec)
    agents, diffs, pairs = spec['agents'], spec['difficulties'], [v['name'] for v in spec['variants']]
    print('| agent | slice | n | base win | new win | gap ± SE | base rounds | new rounds |')
    print('|---|---|---|---|---|---|---|---|')
    for a in agents:
        slices = [(f'd{d}', lambda k, d=d: k[1] == d) for d in diffs] + \
                 [(p, lambda k, p=p: k[2] == p) for p in pairs] + [('all', lambda k: True)]
        for name, keep in slices:
            ks = [k for k in new if k[0] == a and keep(k)]
            bk = [k for k in ks if k in base]
            if not ks:
                continue
            w1 = 100 * sum(new[k]['won'] for k in ks) / len(ks)
            w0 = 100 * sum(base[k]['won'] for k in bk) / max(1, len(bk))
            r1 = sum(new[k]['rounds'] for k in ks) / len(ks)
            r0 = sum(base[k]['rounds'] for k in bk) / max(1, len(bk))
            m, se = gap(new, base, ks)
            print(f'| {a} | {name} | {len(ks)} | {w0:.1f} | {w1:.1f} | {m} ± {se} | {r0:.2f} | {r1:.2f} |')
    for pre in [p for p in args.counters.split(',') if p]:
        print(f'\n{pre}* a game (all agents): key | base | new')
        keys = sorted({c for g in list(new.values()) + list(base.values()) for c in g.get('counters', {})
                       if c.startswith(pre) and '.r' not in c[len(pre):]})
        for c in keys:
            b = sum(g.get('counters', {}).get(c, 0) for g in base.values()) / max(1, len(base))
            n = sum(g.get('counters', {}).get(c, 0) for g in new.values()) / max(1, len(new))
            print(f'  {c} | {b:.2f} | {n:.2f}')
    if args.powers:
        print('\n| agent | power | base uses a game | new uses a game |')
        print('|---|---|---|---|')
        for a in agents:
            names = sorted({c for g in list(new.values()) + list(base.values()) for c in g.get('counters', {})
                            if c.startswith('ai.power.') and c.count('.') == 3})
            for c in names:
                board = c.split('.')[2]
                def per(gs):
                    sel = [g for k, g in gs.items() if k[0] == a and board in SEATS[k[2]]]
                    return sum(g.get('counters', {}).get(c, 0) for g in sel) / max(1, len(sel))
                print(f'| {a} | {c[len("ai.power."):]} | {per(base):.2f} | {per(new):.2f} |')


if __name__ == '__main__':
    main()
