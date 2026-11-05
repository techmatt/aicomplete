"""112: every deck card's price at r43 and r44, with 111's anchor check, and a proposed r45 price.

Run from the repo root: python experiments/results/112_price_correction/price_list.py R43_CARDS.json [R45_CARDS.json]
R43_CARDS.json is `git show ffefde2:content/cards.json`. Writes out/price_list.json (scratch) and prints a table.

Anchor check (docs/card-guidelines.md): pay = credits + 4 x (keep + play insight) + other costs; get = effects at the
anchors. 111's cards.md gives "pay A+B+...=P; get ... = R" per changed or new card; get = R x P, and "other" is
P - credits - 4 x (keep + insight). For a range "R1-R2" the low end is used (the seat that uses it least),
reported as the range.
"""
import json
import re
import sys

R44 = 'out/cards_r44.json'
CARDS_MD = 'experiments/results/111_card_pass/cards.md'


def load(path):
    with open(path, encoding='utf-8') as f:
        return {c['id']: c for c in json.load(f)}


def deck_ids():
    with open('configs/baseline.json', encoding='utf-8') as f:
        base = json.load(f)
    found = set()

    def walk(x):
        if isinstance(x, dict):
            for k, v in x.items():
                if k in ('deck', 'cards', 'tier1', 'tier2') and isinstance(v, list) and v and isinstance(v[0], str):
                    found.update(v)
                walk(v)
        elif isinstance(x, list):
            for v in x:
                walk(v)
    walk(base)
    return found


def anchor_rows():
    rows = {}
    with open(CARDS_MD, encoding='utf-8') as f:
        for line in f:
            if not line.startswith('| ') or line.startswith('| card') or line.startswith('|---'):
                continue
            cells = [c.strip() for c in line.strip().strip('|').split(' | ')]
            name = cells[0]
            rows[re.sub(r' \(was .*\)$', '', name)] = cells[-1]
    return rows


def parse_anchor(s):
    m = re.search(r'pay ([^;=]*=\s*)?([\d.]+)', s)
    if not m:
        return None
    pay = float(m.group(2))
    # last "= R" or "= R1-R2" on the line
    rs = re.findall(r'=\s*([\d.]+)(?:\s*-\s*([\d.]+))?', s)
    if not rs:
        return None
    lo, hi = rs[-1]
    lo = float(lo)
    hi = float(hi) if hi else lo
    return pay, lo, hi


def main():
    r43 = load(sys.argv[1])
    r44 = load(R44)
    r45 = load(sys.argv[2]) if len(sys.argv) > 2 else r44
    deck = deck_ids()
    anchors = anchor_rows()
    out = []
    for cid, c in r44.items():
        if deck and cid not in deck:
            continue
        o = r43.get(cid)
        tier = c['tier']
        keep = 1 if tier == 1 else 2
        row = {'id': cid, 'name': c['name'], 'tier': tier, 'kind': c['kind'],
               'r44': [c.get('cost', 0), c.get('insight', 0)],
               'r43': [o.get('cost', 0), o.get('insight', 0)] if o else None,
               'r45': [r45[cid].get('cost', 0), r45[cid].get('insight', 0)],
               'effect_same': bool(o) and {k: v for k, v in o.items() if k not in ('cost', 'insight', 'name')}
               == {k: v for k, v in c.items() if k not in ('cost', 'insight', 'name')},
               'anchor': anchors.get(c['name'])}
        p = parse_anchor(row['anchor']) if row['anchor'] else None
        if p:
            pay, lo, hi = p
            other = pay - c.get('cost', 0) - 4 * (keep + c.get('insight', 0))
            row['get_lo'] = (lo + hi) / 2 * pay
            row['get_hi'] = hi * pay
            row['other'] = other
            row['ratio44'] = [lo, hi]

            def ratio(cr, ins):
                pp = cr + 4 * (keep + ins) + other
                return [round(row['get_lo'] / pp, 2), round(row['get_hi'] / pp, 2)]
            if o:
                row['ratio_at_r43'] = ratio(*row['r43'])
            row['ratio45'] = ratio(*row['r45'])
        out.append(row)
    with open('out/price_list.json', 'w', encoding='utf-8', newline='\n') as f:
        json.dump(out, f, indent=1)
    lowered = [r for r in out if r['r43'] and (r['r44'][0] < r['r43'][0] or r['r44'][1] < r['r43'][1])]
    print(len(out), 'deck cards;', len(lowered), 'lowered by 111')
    for r in sorted(lowered, key=lambda r: r['name']):
        print(f"{r['name'][:28]:28} t{r['tier']} r43 {r['r43']} r44 {r['r44']} r45 {r['r45']} same={int(r['effect_same'])}"
              f" ratio44 {r.get('ratio44')} @r43 {r.get('ratio_at_r43')} r45 {r.get('ratio45')}")
    print('--- over the maximum at r45 (t1 > 1.5, t2 > 2.5), or no anchor')
    for r in sorted(out, key=lambda r: r['name']):
        rr = r.get('ratio45')
        if rr is None:
            print('  no anchor:', r['name'], r['r45'])
        elif rr[0] > (1.5 if r['tier'] == 1 else 2.5):
            print('  over:', r['name'], 't%d' % r['tier'], r['r45'], rr)
    costs = {1: [], 2: []}
    for r in out:
        costs[r['tier']].append(r['r45'][0])
    for t in costs:
        v = sorted(costs[t])
        print('tier', t, 'median', v[len(v) // 2], 'n', len(v))
    print('cost <= 5:', sum(1 for r in out if r['r45'][0] <= 5))


main()
