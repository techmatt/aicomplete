"""108: the value check at a committed seat's fire rate, for every engine building (r42 vs r43).

usage: python payback.py OLD_CARDS NEW_CARDS > payback.md

Give: credits + insight to play and to keep (2.5 an insight; the keep at 1 insight for tier 1, 2 for tier 2 at r43,
3 at r42) + energy 4.5, chips 1.5, antimatter 7 (configs/agents/values.json).
Get: the when-built gain, the card's own first fire (a play trigger counts the card itself), and a round's yield at
the rate a seat built around the tag sees (106's anchor, card-pricing.md *Engines*): 1 card of the tag a round,
3 tags of any kind, 1.5 cards printed at 10+, 1 troop removed, 0.6 compute removals, 0.8 ambassador removals, 2
Datasets placed, 0.5 teammate plays of the tag. Rounds to pay back = (give - when built - own fire) / a round.
"""
import json
import sys

RES = {'credits': 1, 'chips': 1.5, 'energy': 4.5, 'antimatter': 7, 'insight': 2.5, 'research_cubes': 3, 'ambassadors': 4}
DRAW = 4
SWAY = 4
TROOP = 11  # one troop removed
FEED = {'datasets': 3, 'capital': 3, 'followers': 2.5, 'agents': 2}
RATES = {'play': 1.0, 'play_tag': 3.0, 'play_cost': 1.5, 'team_play': 0.5, 'remove_troop': 1.0,
         'remove_compute': 0.6, 'remove_ambassadors': 0.8, 'place_tokens': 2.0}
TAGS_HELD = 4  # a committed seat's tags of its own kind when the building is out
TROOPS_REMOVED = {1: 1.0, 2: 1.48, 3: 1.75, 4: 2.37, 6: 3.6}


def op_worth(o, threshold=None):
    k = o['op']
    if k == 'gain':
        return RES[o['resource']] * o['amount']
    if k == 'draw':
        return DRAW * o['count']
    if k == 'remove_troops':
        return TROOP * TROOPS_REMOVED.get(o['count'], o['count'])
    if k == 'gain_ambassadors':
        return RES['ambassadors'] * o['count']
    if k == 'sway':
        return SWAY * o['amount']
    if k == 'production':
        return RES[o['resource']] * o['amount'] * 4  # 4 rounds of horizon
    if k == 'add_tokens':
        if o.get('here') and threshold:
            at, then = threshold
            return o['amount'] / at * sum(op_worth(t) for t in then)
        return FEED.get(o['kind'], 1) * o['amount']
    return 0.0


def give(c, keep_t2):
    keep = 1 if c['tier'] == 1 else keep_t2
    g = c.get('cost', 0) + RES['insight'] * (c.get('insight', 0) + keep)
    for r in ('energy', 'chips', 'antimatter', 'research_cubes'):
        g += RES[r] * c.get(r, 0)
    return g


def engine_rows(c):
    """(per round, own first fire) for a building's standing parts."""
    per_round = 0.0
    own = 0.0
    ps = c.get('passives', [])
    th = next(((p['at'], p['then']) for p in ps if p['op'] == 'threshold'), None)
    for p in ps:
        if p['op'] == 'when':
            ev = p['on']['event']
            rate = RATES.get(ev, 0.5)
            w = sum(op_worth(t, th) for t in p['then'])
            per_round += rate * w
            if ev == 'play' and (p['on'].get('tag') is None or p['on']['tag'] in c['tags']):
                own += w
            if ev == 'play_tag':
                own += w * len(c['tags'])
            if ev == 'play_cost' and c['cost'] >= p['on']['at']:
                own += w
        elif p['op'] == 'income':
            per_round += RES[p['resource']] * p['amount']
        elif p['op'] == 'income_per_tag':
            per_round += RES[p['resource']] * p.get('amount', 1) * TAGS_HELD / p.get('per', 1)
        elif p['op'] == 'card_discount':
            n = RATES['play'] if p.get('tag') else 2.0
            per_round += n * p['amount'] * (1 if p['resource'] == 'credits' else RES['insight'])
    return per_round, own


def main():
    old = {c['id']: c for c in json.load(open(sys.argv[1], encoding='utf-8'))}
    new = {c['id']: c for c in json.load(open(sys.argv[2], encoding='utf-8'))}
    print('| card | r42 give / when built + own fire / a round / rounds | r43 give / when built + own fire / a round / rounds |')
    print('|---|---|---|')
    for cid, c in new.items():
        if c['kind'] != 'building' or not c.get('passives'):
            continue
        if not any(p['op'] in ('when', 'income', 'income_per_tag', 'card_discount') for p in c['passives']):
            continue
        cols = []
        for cards, kt2 in ((old, 3), (new, 2)):
            d = cards.get(cid)
            if d is None:
                cols.append('new')
                continue
            th = next(((p['at'], p['then']) for p in d.get('passives', []) if p['op'] == 'threshold'), None)
            g = give(d, kt2)
            up = sum(op_worth(e, th) for e in d.get('effects', []))
            per, own = engine_rows(d)
            rounds = (g - up - own) / per if per else float('inf')
            cols.append(f'{g:.0f} / {up:.0f} + {own:.0f} / {per:.1f} / **{max(rounds, 0):.1f}**')
        print(f'| {c["name"]} | {cols[0]} | {cols[1]} |')


if __name__ == '__main__':
    main()
