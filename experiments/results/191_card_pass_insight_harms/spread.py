"""191: the insight-to-play spread and the node cards, as one overlay, with each changed card's give / get.

    python experiments/results/191_card_pass_insight_harms/spread.py [--before content/cards.json]

Writes `insight.overlay` (apply with scripts/cards/apply_overlay.py) and `give-get.md` beside this file, from the
pre-pass cards (git show de6b0aa:content/cards.json, or --before) and the effect ledger's
`reasoned` value. Rules (Matt, 2026-11-11: "spread both tiers across 0, 1 and 2 insight, with far more 0s,
especially in tier-1"; a card moving to 0 gains value, so its credits move to keep give / get):

- A card that draws keeps at least 1 insight to play (card-guidelines 7: a round's chain of draws stops with its
  insight).
- Tier 1: every card that doesn't draw plays for 0; the 12 tier-1 draw cards worth most at the table play for 2.
- Tier 2: every card at 1 that doesn't draw plays for 0; the 10 cheapest non-drawing cards at 2 play for 1.
- Credits move by the anchor, 4 an insight (card-guidelines, *Unit anchors*: the anchor sets the printed price).
- The four node cards are priced by hand below (NODE), the spaces they fill counted at 1.5 energy a node.
"""
import argparse
import csv
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
ANCHOR = 4  # credits an insight, the printed price's anchor
TABLE = 2.5  # values.json's insight
SPACE = 1.5 * 4.5  # a node's space, at 1.5 energy
TIER1_TO_2 = 12
TIER2_TO_1 = 10

# id: (credits, insight, why)
NODE = {
    'national_grid_buildout': (22, 0, 'never played (0.01 a game): 3 nodes and +2 energy income'),
    'beamed_power_constellation': (14, 1, 'never played (0.00 a game): 3 nodes and 1 energy a node'),
    'relay_mast_rollout': (6, 1, 'draws, so 1 insight stays'),
    'aegis_grid_lockdown': (4, 0, '3 nodes and Shelter'),
}


def ops(o, acc):
    if isinstance(o, dict):
        if 'op' in o:
            acc.add(o['op'])
        for v in o.values():
            ops(v, acc)
    elif isinstance(o, list):
        for v in o:
            ops(v, acc)


def draws(card):
    acc = set()
    ops(card, acc)
    return bool(acc & {'draw', 'discard_hand'})


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--before', help='the pre-pass cards.json (default: git show d08124f)')
    a = ap.parse_args()
    if a.before:
        cards = json.load(open(a.before, encoding='utf-8'))
    else:
        text = subprocess.run(['git', 'show', 'de6b0aa:content/cards.json'], cwd=ROOT, capture_output=True,
                              check=True).stdout.decode('utf-8')
        cards = json.loads(text)
    led = {r['card']: r for r in csv.DictReader(open(os.path.join(ROOT, 'experiments', 'results', 'effect-ledger',
                                                                    'cards.csv'), encoding='utf-8'))}
    new = {c['id']: [c['cost'], c['insight']] for c in cards}
    t1_draw = sorted((c for c in cards if c['tier'] == 1 and draws(c) and c['id'] not in NODE),
                     key=lambda c: (-float(led[c['id']]['reasoned']), c['id']))
    def net(c, cost, ins):
        return float(led[c['id']]['reasoned']) - cost - TABLE * ins

    # A card moving to 2 keeps a gain at the table: 1 credit at least.
    to2 = {c['id'] for c in t1_draw if c['cost'] >= 2 + ANCHOR * (2 - c['insight'])
           and net(c, c['cost'] - ANCHOR * (2 - c['insight']), 2) >= 1}
    to2 = [c['id'] for c in t1_draw if c['id'] in to2][:TIER1_TO_2]
    t2_two = sorted((c for c in cards if c['tier'] == 2 and c['insight'] == 2 and not draws(c) and c['id'] not in NODE),
                    key=lambda c: (c['cost'], c['id']))[:TIER2_TO_1]
    for c in cards:
        i = c['id']
        if i in NODE:
            new[i] = list(NODE[i][:2])
            continue
        target = c['insight']
        if c['tier'] == 1:
            target = 2 if i in to2 else (max(1, min(c['insight'], 1)) if draws(c) else 0)
        elif c['tier'] == 2:
            if c['insight'] == 1 and not draws(c):
                target = 0
            elif c in t2_two:
                target = 1
        d = c['insight'] - target
        move = ANCHOR * d
        if d > 0:
            # The anchor's 4 an insight, less what would leave the card below a 1-credit gain at the table (2.5 an
            # insight), never under 2 an insight: a card the table already priced at a loss is flagged, not cut.
            short = 1 - net(c, c['cost'] + move, target)
            if short > 0:
                move = max(2 * d, move - int(-(-short // 1)))
        new[i] = [c['cost'] + move, target]
    lines, rows = [], []
    for c in cards:
        i = c['id']
        cost, ins = new[i]
        if [cost, ins] == [c['cost'], c['insight']]:
            continue
        lines.append(f'cards.{i}.cost={cost}')
        lines.append(f'cards.{i}.insight={ins}')
        get = float(led[i]['reasoned'])
        space = 0
        if i in NODE:
            n = {'national_grid_buildout': 3, 'beamed_power_constellation': 3, 'relay_mast_rollout': 2,
                 'aegis_grid_lockdown': 3}[i]
            space = n * SPACE
        give_old = c['cost'] + TABLE * c['insight'] + space
        give_new = cost + TABLE * ins + space
        anchor_old = c['cost'] + ANCHOR * c['insight'] + space
        anchor_new = cost + ANCHOR * ins + space
        rows.append((c['tier'], c['name'], c['cost'], c['insight'], cost, ins, give_old, give_new, anchor_new, get,
                     get - give_new, get - anchor_new, NODE.get(i, (0, 0, ''))[2]))
    with open(os.path.join(HERE, 'insight.overlay'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('# 191: insight to play spread (spread.py writes this file; rerun, then apply_overlay.py)\n')
        f.write('\n'.join(lines) + '\n')
    with open(os.path.join(HERE, 'give-get.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('# 191: give / get for every changed card\n\nWritten by `spread.py`. Get: the effect ledger\'s `reasoned` '
                'value at `values.json`. Give: credits + insight to play (table 2.5, anchor 4) + the node spaces '
                '(1.5 energy a node at 4.5). The keep (1 or 2 insight by tier) is unchanged by the pass and left out '
                'of both columns. Net at the table and at the anchor.\n\n')
        f.write('| tier | card | was | now | give (table) was | now | give (anchor) | get | net table | net anchor | note |\n')
        f.write('|---|---|---|---|---|---|---|---|---|---|---|\n')
        for r in sorted(rows, key=lambda r: (r[0], r[5], r[1])):
            f.write(f'| {r[0]} | {r[1]} | {r[2]} cr, {r[3]} ins | {r[4]} cr, {r[5]} ins | {r[6]:.1f} | {r[7]:.1f} | '
                    f'{r[8]:.1f} | {r[9]:.1f} | {r[10]:+.1f} | {r[11]:+.1f} | {r[12]} |\n')
    print(f'{len(rows)} cards changed; to 2 at tier 1: {to2}; to 1 at tier 2: {[c["id"] for c in t2_two]}')


if __name__ == '__main__':
    main()
