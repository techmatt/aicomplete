"""112: set r45 prices from out/price_list.json (written by price_list.py) and rewrite content/cards.json.

Run from the repo root after price_list.py. Prices only: credits and play insight, no effect touched.

The rule (112's prompt: the margins are a maximum; a fair card reads about 1.0-1.5 at the anchors):
- A card 111 lowered (credits or play insight) goes back to its r43 price.
  - If it reads under 1.0 at the r43 price with its r44 effect, it takes the highest price between r44 and r43 that
    reads at least 1.0 (r43's insight first, then r44's): 111's cut was partly fair, not only to reach the band.
  - If it reads over its maximum (1.5 / 2.5) at the r43 price, the credits rise to read the maximum. Tier 2 is
    efficiency (guideline 23), so a tier-2 card at 1.5-2.5 stays at its r43 price.
- Any other card over its maximum margin (tier 1 over 1.5, tier 2 over 2.5): credits rise to read 1.5 / 2.5.
"""
import json

FAIR_LO, FAIR_HI = 1.0, 1.5
MAX = {1: 1.5, 2: 2.5}


def ratio(r, cr, ins):
    keep = 1 if r['tier'] == 1 else 2
    return r['get_lo'] / (cr + 4 * (keep + ins) + r['other'])


def propose(r):
    r44 = tuple(r['r44'])
    if 'get_lo' not in r:
        if r['r43'] and (r44[0] < r['r43'][0] or r44[1] < r['r43'][1]):
            return tuple(r['r43']), 'restored (no anchor parsed)'
        return r44, None
    lowered = r['r43'] and (r44[0] < r['r43'][0] or r44[1] < r['r43'][1])
    if lowered:
        cr43, ins43 = r['r43']
        if ratio(r, cr43, ins43) >= FAIR_LO - 0.005:
            cr = cr43
            while ratio(r, cr, ins43) > MAX[r['tier']] + 0.005:
                cr += 1
            return (cr, ins43), 'restored' if cr == cr43 else 'restored, raised to its maximum'
        for ins in sorted({ins43, r44[1]}, reverse=True):
            for cr in range(cr43, r44[0] - 1, -1):
                if ratio(r, cr, ins) >= FAIR_LO - 0.005 and (cr, ins) >= r44:
                    return (cr, ins), 'partly restored, to 1.0'
        return r44, 'kept at r44 (under 1.0 at any higher price)'
    cr, ins = r44
    if ratio(r, cr, ins) > MAX[r['tier']] + 0.005:
        while ratio(r, cr, ins) > MAX[r['tier']] + 0.005:
            cr += 1
        return (cr, ins), 'over its maximum, raised'
    return r44, None


def main():
    with open('out/price_list.json', encoding='utf-8') as f:
        rows = json.load(f)
    new = {}
    for r in rows:
        p, why = propose(r)
        if why and p != tuple(r['r44']):
            new[r['id']] = (p, why)
    with open('content/cards.json', encoding='utf-8') as f:
        cards = json.load(f)
    for c in cards:
        if c['id'] in new:
            (cr, ins), _ = new[c['id']]
            c['cost'] = cr
            c['insight'] = ins
    with open('content/cards.json', 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(cards, indent=1, ensure_ascii=False) + '\n')
    with open('out/r45_changes.json', 'w', encoding='utf-8', newline='\n') as f:
        json.dump({k: [list(v[0]), v[1]] for k, v in new.items()}, f, indent=1)
    print(len(new), 'cards repriced')


main()
