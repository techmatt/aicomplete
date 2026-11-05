"""108: the engine probe read, r42 against r43 on the same seeds.

    python probe_read.py R42_DIR R43_DIR [--md]

Each dir is an `aic-sim experiment` output of experiments/screen-108-rush.json (or the same spec at other seeds).
Prints, per pair and pooled, each agent's win rate and the probe's gap to the plain strategist with its paired SE
(game by game on the same seed), then per engine card (107's `is_engine`: any trigger, threshold, per-tag income,
discount, or a per-tag income raise or copy on play, in either revision) its kept / played a game for both agents.
"""
import csv
import io
import json
import math
import sys
from collections import defaultdict

PLAIN = 'strategist'  # the spec's names (games.jsonl); cards.csv names the resolved agents
RUSH = 'strategist:engine_rush=1'
CSV = {PLAIN: 'engine:strategy=1', RUSH: 'engine:strategy=1,engine_rush=1'}


def games(d):
    lines = open(f'{d}/games.jsonl', encoding='utf-8').read().splitlines()
    spec = json.loads(lines[0])['spec']
    cells = [(v['name'], a) for v in spec['variants'] for _ in spec['difficulties'] for a in spec['agents']]
    out = {}
    for ln in lines[1:]:
        g = json.loads(ln)
        if 'cell' in g:
            v, a = cells[g['cell']]
            out[(v, a, int(g['seed']))] = 1.0 if g['won'] else 0.0
    return out


def gap(gs, pair=None):
    diffs = [gs[(v, RUSH, s)] - gs[(v, PLAIN, s)] for (v, a, s) in gs
             if a == PLAIN and (v, RUSH, s) in gs and (pair is None or v == pair)]
    n = len(diffs)
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / (n - 1)) if n > 1 else 0
    return 100 * m, 100 * sd / math.sqrt(n), n


def rate(gs, agent, pair=None):
    xs = [w for (v, a, s), w in gs.items() if a == agent and (pair is None or v == pair)]
    return 100 * sum(xs) / len(xs)


def is_engine(c):
    ps = c.get('passives', [])
    return any(p['op'] in ('when', 'threshold', 'income_per_tag', 'card_discount') for p in ps) or any(
        e['op'] in ('production_per_tag', 'copy_income') for e in c.get('effects', []))


def counts(d, n_games):
    out = defaultdict(lambda: [0, 0])
    for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8')):
        o = out[(r['agent'], r['card'])]
        o[0] += int(r['kept'])
        o[1] += int(r['played'])
    return {k: (v[0] / n_games, v[1] / n_games) for k, v in out.items()}


def main():
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    a, b = sys.argv[1], sys.argv[2]
    ga, gb = games(a), games(b)
    print('| pair | strategist r42 → r43 | probe r42 → r43 | probe − strategist r42 | r43 |')
    print('|---|---|---|---|---|')
    for pair in ('bp', 'br', 'pr', None):
        ra, rb = gap(ga, pair), gap(gb, pair)
        print(f'| {pair or "pooled"} | {rate(ga, PLAIN, pair):.1f} → {rate(gb, PLAIN, pair):.1f} | '
              f'{rate(ga, RUSH, pair):.1f} → {rate(gb, RUSH, pair):.1f} | {ra[0]:+.1f} ±{ra[1]:.1f} | **{rb[0]:+.1f} ±{rb[1]:.1f}** |')
    n_a = sum(1 for k in ga if k[1] == PLAIN)
    n_b = sum(1 for k in gb if k[1] == PLAIN)
    ca, cb = counts(a, n_a), counts(b, n_b)
    import subprocess
    old = {c['id']: c for c in json.loads(subprocess.run(['git', 'show', '57469c6:content/cards.json'], capture_output=True).stdout.decode('utf-8'))}
    new = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    ids = sorted({i for i, c in old.items() if is_engine(c)} | {i for i, c in new.items() if is_engine(c)},
                 key=lambda i: (new.get(i) or old[i])['name'])
    print()
    print('| engine | strategist kept / played, r42 → r43 | probe kept / played, r42 → r43 |')
    print('|---|---|---|')
    tot = [0.0] * 4
    for i in ids:
        name = (new.get(i) or old[i])['name']
        s0, s1 = ca.get((CSV[PLAIN], i), (0, 0)), cb.get((CSV[PLAIN], i), (0, 0))
        p0, p1 = ca.get((CSV[RUSH], i), (0, 0)), cb.get((CSV[RUSH], i), (0, 0))
        tot[0] += s0[1]
        tot[1] += s1[1]
        tot[2] += p0[1]
        tot[3] += p1[1]
        print(f'| {name} | {s0[0]:.2f} / {s0[1]:.2f} → {s1[0]:.2f} / {s1[1]:.2f} | {p0[0]:.2f} / {p0[1]:.2f} → {p1[0]:.2f} / {p1[1]:.2f} |')
    print(f'| **all engines, played** | {tot[0]:.2f} → {tot[1]:.2f} | {tot[2]:.2f} → {tot[3]:.2f} |')


if __name__ == '__main__':
    main()
