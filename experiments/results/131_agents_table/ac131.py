"""131: auto_counter's worth a fire, with the executions it saved, from a measure run (`measures.patch`).

    python experiments/results/131_agents_table/ac131.py DIR [--keys]

The patch counts each resolved threat by source, outcome (countered / executed) and whether an auto-counter of its
type stood for it (`m131.out.<src>.<outcome>.<held|open>`), each fire (`m131.fires.<src>`), and what each
execution did: the seats' holdings lost (`m131.exec.<src>.<held>.lost.<resource|nodes>`) and every counter it
moved (`...c.<counter>`).

Saved executions a fire, for a source: (p_open - p_held) x threats resolved while held / fires, where p is the
share of resolved threats that executed. A threat with no counter standing executes at p_open; with one, at p_held.
An execution's harm is priced at the table's rates (configs/agents/values.json): holdings at their resource rates,
a node at place_node x rates.node, a Discord sway at rates.sway, an adversary ambassador placed at
remove_ambassador, a troop at remove_troop, a compute made at drain_project. `--keys` lists every moved counter, to
see what the pricing leaves out (Unity pulled back, factories seized, sources made: unpriced, so a floor).
"""
import argparse
import collections
import json
import math
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import stream  # noqa: E402

V = json.load(open(os.path.join(ROOT, 'configs', 'agents', 'values.json'), encoding='utf-8'))
RES = dict(V['resources'])
NODE = V['ops']['place_node']['credits'] * V['rates']['node']
SWAY = V['rates']['sway']
# Counters an execution moves, priced: (prefix, credits each). Checked against --keys.
PRICED = [
    ('alignment.sway_discord', SWAY),
    ('adversary.placed.ambassador', V['ops']['remove_ambassador']['credits']),
    ('adversary.placed.troop', V['ops']['remove_troop']['credits']),
    ('adversary.compute_made', V['ops']['drain_project']['credits']),
]


def main():
    p = argparse.ArgumentParser()
    p.add_argument('dir')
    p.add_argument('--keys', action='store_true')
    p.add_argument('--agent', help='only this cell agent (as the spec names it)')
    a = p.parse_args()
    lines = stream(a.dir)
    spec = next(lines)['spec']
    labels = [f"{v['name']}/{ag}" for v in spec['variants'] for ag in spec['agents']]
    tot = collections.Counter()
    games = 0
    for g in lines:
        if a.agent and labels[g['cell']].split('/')[-1] != a.agent:
            continue
        games += 1
        for k, v in g['counters'].items():
            if k.startswith('m131.'):
                tot[k] += v
    print(f'{games} games')
    for src in ('cyber', 'economic', 'influence', 'military'):
        o = {(oc, h): tot[f'm131.out.{src}.{oc}.{h}'] for oc in ('countered', 'executed') for h in ('held', 'open')}
        n_open = o[('countered', 'open')] + o[('executed', 'open')]
        n_held = o[('countered', 'held')] + o[('executed', 'held')]
        fires = tot[f'm131.fires.{src}']
        if not n_open:
            continue
        p0 = o[('executed', 'open')] / n_open
        p1 = o[('executed', 'held')] / n_held if n_held else float('nan')
        se = math.sqrt(p0 * (1 - p0) / n_open + (p1 * (1 - p1) / n_held if n_held else 0))
        execs = o[('executed', 'open')] + o[('executed', 'held')]
        harm = collections.Counter()
        for k, v in tot.items():
            pre = f'm131.exec.{src}.'
            if not k.startswith(pre):
                continue
            rest = k[len(pre):].split('.', 1)[1]
            if rest.startswith('lost.'):
                r = rest[5:]
                harm['nodes' if r == 'nodes' else r] += v * (NODE if r == 'nodes' else RES[r])
            elif rest.startswith('c.'):
                c = rest[2:]
                for pref, price in PRICED:
                    if c == pref or c.startswith(pref + '.'):
                        if c == pref:
                            harm[pref] += v * price
        per_exec = sum(harm.values()) / execs if execs else 0.0
        saved = (p0 - p1) * n_held / fires if fires and n_held else 0.0
        print(f'\n## {src}: resolved open {n_open} (executed {p0:.3f}), held {n_held} (executed {p1:.3f}), '
              f'gap {p0 - p1:+.3f} ± {se:.3f}; fires {fires}')
        print(f'- harm an execution {per_exec:.1f} credits over {execs} executions: '
              + ', '.join(f'{k} {v / execs:.1f}' for k, v in harm.most_common()))
        print(f'- saved executions a fire {saved:.3f}; worth a fire {saved * per_exec:.1f} credits')
        if a.keys:
            keys = collections.Counter()
            for k, v in tot.items():
                pre = f'm131.exec.{src}.'
                if k.startswith(pre) and '.c.' in k:
                    keys[k.split('.c.', 1)[1]] += v
            for k, v in keys.most_common(40):
                print(f'    {k}: {v / execs:.2f} an execution')


if __name__ == '__main__':
    main()
