"""274: give / get for the jam cards under the jam floor (a jam never takes a resource item below 1).

    python experiments/results/274_table_time_cuts/jam_give_get.py [T1_WEIGHT T2_WEIGHT]   (from the repo root)

For each threat (both tiers, their counter options at the config's counter_pct / counter_credit_pct), a jam of `a`
saves, on the option a payer would use (the cheapest after the jam), sum min(a, s) per resource item before 274 and
sum min(a, s - 1) after (an item s >= 1). The share kept, after / before, over every threat (dealing is uniform
within a tier; tiers weighted by the arguments, default 0.5 / 0.5: 274 passed 0.4 / 0.6, the smoke's
`threats.dealt.t1` / `t2`, 2,092 / 3,196), scales the jam's reasoned value
(values.json `jam`, x `typed_jam` with a source). A card's get is the effect ledger's reasoned value less the jam's
lost share; its give, its printed price (the ledger's `printed`: credits and other resources at the table's rates).
"""
import csv
import io
import json
import subprocess
import sys

cfg = json.load(open('configs/baseline.json', encoding='utf-8'))


def param(module, key):
    for m in cfg['modules']:
        if m['module'] == module:
            return m['params'][key]
    raise KeyError(key)


pct = param('adversary', 'counter_pct')
credit_pct = param('adversary', 'counter_credit_pct')
threats = json.load(open('content/threats.json', encoding='utf-8'))
threats = threats if isinstance(threats, list) else threats['threats']
values = json.load(open('configs/agents/values.json', encoding='utf-8'))


def row(name):
    for section in values.values():
        if isinstance(section, dict) and name in section and isinstance(section[name], dict):
            return section[name]['credits']
    raise KeyError(name)


def items(t):
    out = []
    for opt in t['counters']:
        its = []
        ok = True
        for it in opt:
            if 'resource' not in it:  # a tag option: no jam reaches it
                ok = False
                break
            s = (it['amount'] * pct[t['tier'] - 1] + 99) // 100
            if it['resource'] == 'credits':
                s = (s * credit_pct + 99) // 100
            its.append(s)
        if ok:
            out.append(its)
    return out


def saving(t, a, floor):
    """What a jam of `a` saves on the option left cheapest (in items' units)."""
    best = None
    for its in items(t):
        left = [max(s - a, min(s, 1)) if floor else max(s - a, 0) for s in its]
        cost = sum(left)
        if best is None or cost < best[0]:
            best = (cost, sum(its) - cost)
    return best[1] if best else 0


def share(a, tier_weight):
    num = den = 0.0
    for t in threats:
        w = tier_weight[t['tier']]
        num += w * saving(t, a, True)
        den += w * saving(t, a, False)
    return num / den if den else 1.0


def jam_ops(card):
    found = []

    def walk(x):
        if isinstance(x, dict):
            if x.get('op') == 'jam':
                found.append(x)
            for v in x.values():
                walk(v)
        elif isinstance(x, list):
            for v in x:
                walk(v)
    walk(card)
    return found


def main():
    tier_weight = {1: float(sys.argv[1]) if len(sys.argv) > 1 else 0.5, 2: float(sys.argv[2]) if len(sys.argv) > 2 else 0.5}
    led = subprocess.run([sys.executable, 'scripts/cards/effect_ledger.py', '--stdout'], capture_output=True, text=True,
                         encoding='utf-8').stdout
    ledger = {r['card']: r for r in csv.DictReader(io.StringIO(led[led.index('card,name'):]))}
    cards = json.load(open('content/cards.json', encoding='utf-8'))
    cards = cards if isinstance(cards, list) else cards['cards']
    print(f'tier weights {tier_weight}; counter_pct {pct}, credit_pct {credit_pct}')
    for a in (1, 2, 3):
        print(f'jam {a}: share kept under the floor {share(a, tier_weight):.2f}')
    print()
    print('| Card | Jam | Give (printed) | Get before | Get after | Net after |')
    print('|---|---|---|---|---|---|')
    for c in cards:
        ops = jam_ops(c)
        if not ops:
            continue
        r = ledger[c['id']]
        give, got = float(r['printed']), float(r['reasoned'])
        lost = 0.0
        for o in ops:
            typed = row('typed_jam') if o.get('source') else 1.0
            a = o['amount']
            lost += row('jam') * typed * a * (1 - share(a, tier_weight))
        after = got - lost
        flag = '' if after > give else ' **break-even or worse**'
        what = ', '.join(f"{o['amount']}{' ' + o['source'] if o.get('source') else ''}" for o in ops)
        print(f"| {r['name']} | {what} | {give:.1f} | {got:.1f} | {after:.1f} | {after - give:+.1f}{flag} |")


main()
