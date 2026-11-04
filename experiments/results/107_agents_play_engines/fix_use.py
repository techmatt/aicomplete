"""107: each fix's use a game, on and off: kept and played a game of the cards it acts on, summed.

    python experiments/results/107_agents_play_engines/fix_use.py

Reads the dev screens under out/ (96 dev seeds, the strategist): the keep-plan switches from one run's cells
(out/o96-v3, chain on by then), the value terms from binaries rebuilt with each rate off (out/d96-off-*, against
out/d96-v4). Per game = the three pairs' counts / 288.
"""
import csv
import json
from collections import defaultdict

cards = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}


def ops(c, slot):
    return c.get(slot) or []


def own_fed(c):
    p = ops(c, 'passives')
    return any(o['op'] == 'threshold' for o in p) and any(
        o['op'] == 'when' and any(e['op'] == 'add_tokens' and e.get('here') for e in o['then']) for o in p)


def tagged_trigger(c):
    return any((o['op'] == 'when' and o['on'].get('tag')) or (o['op'] == 'card_discount' and o.get('tag'))
               for o in ops(c, 'passives'))


GROUPS = {
    'plan_reach': ('cards with needs', lambda c: bool(c.get('needs'))),
    'plan_under': ('built-over cards', lambda c: bool(c.get('replaces'))),
    'plan_chain': ('cards with needs', lambda c: bool(c.get('needs'))),
    'plan_copy': ('Payroll Automation', lambda c: c['id'] == 'payroll_automation'),
    'own_fill': ('thresholds their own trigger fills', own_fed),
    'raised_rounds': ('Retained Earnings and the Capital feeders',
                      lambda c: c['id'] in ('retained_earnings', 'materials_hedge_desk', 'vendor_financing')),
    'insight_discount': ('Hyperscale Botnet', lambda c: c['id'] == 'hyperscale_botnet'),
    'own_mix': ('one-tag triggers and discounts', tagged_trigger),
}
RUNS = {
    'plan_reach': (('out/o96-v3', 'engine:strategy=1'), ('out/o96-v3', 'engine:strategy=1,plan_reach=0')),
    'plan_under': (('out/o96-v3', 'engine:strategy=1'), ('out/o96-v3', 'engine:strategy=1,plan_under=0')),
    'plan_chain': (('out/o96-v3', 'engine:strategy=1'), ('out/o96-v3', 'engine:strategy=1,plan_chain=0')),
    'plan_copy': (('out/o96-v3', 'engine:strategy=1'), ('out/o96-v3', 'engine:strategy=1,plan_copy=0')),
}
for k in ('own_fill', 'raised_rounds', 'insight_discount', 'own_mix'):
    RUNS[k] = (('out/d96-v4', 'engine:strategy=1'), (f'out/d96-off-{k}', 'engine:strategy=1'))

cache = {}


def counts(d, agent):
    if d not in cache:
        t = defaultdict(lambda: [0, 0])
        for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8')):
            t[(r['agent'], r['card'])][0] += int(r['kept'])
            t[(r['agent'], r['card'])][1] += int(r['played'])
        cache[d] = t
    return cache[d]


print('| fix | acts on (cards) | kept a game, on / off | played a game, on / off |')
print('|---|---|---|---|')
for fix, (what, pick) in GROUPS.items():
    ids = [i for i, c in cards.items() if pick(c)]
    (don, aon), (doff, aoff) = RUNS[fix]
    on, off = counts(don, aon), counts(doff, aoff)
    k = [sum(t[(a, i)][0] for i in ids) / 288 for t, a in ((on, aon), (off, aoff))]
    p = [sum(t[(a, i)][1] for i in ids) / 288 for t, a in ((on, aon), (off, aoff))]
    print(f'| `{fix}` | {what} ({len(ids)}) | {k[0]:.2f} / {k[1]:.2f} | {p[0]:.2f} / {p[1]:.2f} |')
