"""Committed-seat payback for every engine building (108): rounds a seat built around the card's tag
takes to earn back its price.

    python scripts/cards/payback.py                      # content/cards.json, one column
    python scripts/cards/payback.py --old OLD_CARDS.json [--old-keep-tier2 3] [--cards NEW.json]
                                                         # an old deck beside the current one

Prints a Markdown table to stdout; writes nothing. From 108's
experiments/results/108_engines_worth_playing/payback.py, which compared r42 with r43.

Give: credits + insight to play and to keep (the keep's insight from configs/baseline.json's cards
params, `keep_tier1` / `keep_tier2`; `--old-keep-tier2` for an older deck's) + energy, chips, antimatter,
research cubes at configs/agents/values.json's `resources`.
Get: the when-built gain, the card's own first fire (a play trigger counts the card itself), and a
round's yield at the rate a seat built around the tag sees (106's anchor, card-pricing.md *Engines*):
1 card of the tag a round, 3 tags of any kind, 1.5 cards printed at 10+, 1 troop removed, 0.6 compute
removals, 0.8 ambassador removals, 2 Datasets placed, 0.5 teammate plays of the tag. Rounds to pay
back = (give - when built - own fire) / a round. Ops it does not price (auto_counter, say) count 0.
"""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


VALUES = read(ROOT / 'configs/agents/values.json')
RES = VALUES['resources']
DRAW = VALUES['rates']['drawn_card']
SWAY = VALUES['rates']['sway']
FEED = {k[len('feed_'):]: v for k, v in VALUES['tokens'].items() if k.startswith('feed_')}
TROOP = 11  # one troop removed, at a committed seat (not values.json's 0.65 share for being there)
RATES = {'play': 1.0, 'play_tag': 3.0, 'play_cost': 1.5, 'team_play': 0.5, 'remove_troop': 1.0,
         'remove_compute': 0.6, 'remove_ambassadors': 0.8, 'place_tokens': 2.0}
TAGS_HELD = 4  # a committed seat's tags of its own kind when the building is out
TROOPS_REMOVED = {1: 1.0, 2: 1.48, 3: 1.75, 4: 2.37, 6: 3.6}
HORIZON = 4  # rounds of production


def keep_params():
    mod = next(m for m in read(ROOT / 'configs/baseline.json')['modules'] if m['module'] == 'cards')
    return mod['params']['keep_tier1'], mod['params']['keep_tier2']


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
        return RES[o['resource']] * o['amount'] * HORIZON
    if k == 'add_tokens':
        if o.get('here') and threshold:
            at, then = threshold
            return o['amount'] / at * sum(op_worth(t) for t in then)
        return FEED.get(o['kind'], 1) * o['amount']
    return 0.0


def give(c, keep_t1, keep_t2):
    keep = keep_t1 if c['tier'] == 1 else keep_t2
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


def cell(d, keep_t1, keep_t2):
    if d is None:
        return 'new'
    th = next(((p['at'], p['then']) for p in d.get('passives', []) if p['op'] == 'threshold'), None)
    g = give(d, keep_t1, keep_t2)
    up = sum(op_worth(e, th) for e in d.get('effects', []))
    per, own = engine_rows(d)
    rounds = (g - up - own) / per if per else float('inf')
    return f'{g:.0f} / {up:.0f} + {own:.0f} / {per:.1f} / **{max(rounds, 0):.1f}**'


def main():
    ap = argparse.ArgumentParser(allow_abbrev=False, description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--cards', default=str(ROOT / 'content/cards.json'), help='the deck to read (default: content/cards.json)')
    ap.add_argument('--old', help='an older cards.json, shown in a column before it')
    ap.add_argument('--old-keep-tier2', type=float, help="the old deck's tier-2 keep in insight (default: the current one)")
    a = ap.parse_args()
    keep_t1, keep_t2 = keep_params()
    new = {c['id']: c for c in read(a.cards)}
    sides = [(new, keep_t2)]
    head = 'give / when built + own fire / a round / rounds'
    if a.old:
        old = {c['id']: c for c in read(a.old)}
        sides.insert(0, (old, keep_t2 if a.old_keep_tier2 is None else a.old_keep_tier2))
        print(f'| card | old: {head} | new: {head} |')
        print('|---|---|---|')
    else:
        print(f'| card | {head} |')
        print('|---|---|')
    for cid, c in new.items():
        if c['kind'] != 'building' or not c.get('passives'):
            continue
        if not any(p['op'] in ('when', 'income', 'income_per_tag', 'card_discount') for p in c['passives']):
            continue
        cols = [cell(cards.get(cid), keep_t1, kt2) for cards, kt2 in sides]
        print(f'| {c["name"]} | ' + ' | '.join(cols) + ' |')


if __name__ == '__main__':
    main()
