"""191: one current pricing note (docs/card-pricing-notes.json) for every card this pass repriced.

    python experiments/results/191_card_pass_insight_harms/notes.py

Reads give-get.md (spread.py) for the insight moves and NOTES below for the hand-priced cards; rerunnable.
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
PATH = os.path.join(ROOT, 'docs', 'card-pricing-notes.json')

D82 = {
    'rally_the_allies': 'r83 (191, D82): the 3-Diplomacy rider cut to fit 7.5 pt; 18 to 12 credits, 0 insight.',
    'mass_defection': 'r83 (191, D82): the 3-Diplomacy rider cut to fit 7.5 pt; 15 to 9 credits, 0 insight.',
    'surgical_strike': 'r83 (191, D82): the 3-Security botnet rider cut to fit 7.5 pt; 19 to 14 credits, 0 insight.',
    'double_agents': 'r83 (191, D82): Sway 1 cut to fit 7.5 pt; 12 to 8 credits, 0 insight.',
    'air_gap': 'r83 (191, D82): Sway 1 cut to fit 7.5 pt; 10 to 6 credits, 0 insight.',
}


def main():
    notes = json.load(open(PATH, encoding='utf-8'))
    cards = {c['name']: c['id'] for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
    n = 0
    for line in open(os.path.join(HERE, 'give-get.md'), encoding='utf-8'):
        if not line.startswith('| 1 ') and not line.startswith('| 2 '):
            continue
        f = [x.strip() for x in line.split('|')[1:-1]]
        _, name, was, now, _, give, give_anchor, get, net_t, net_a, why = f
        cid = cards[name]
        lead = f'node card, {why}; ' if why else ''
        notes[cid] = (f'r83 (191): {lead}insight to play spread, {was} to {now}; values.json: pay {give}; get {get} '
                      f'({net_t}); anchors: pay {give_anchor} ({net_a}).')
        n += 1
    notes.update(D82)
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(dict(sorted(notes.items())), f, indent=2, ensure_ascii=False)
        f.write('\n')
    print(f'{n} spread notes, {len(D82)} D82 notes')


if __name__ == '__main__':
    main()
