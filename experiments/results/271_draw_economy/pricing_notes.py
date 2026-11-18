"""271: write docs/card-pricing-notes.json's line for each card the draw overlay changed. Run from the repo root.

    python experiments/results/271_draw_economy/pricing_notes.py BASE_CARDS_JSON
"""
import json
import sys

OVERLAY = 'experiments/results/271_draw_economy/draw.overlay'
NOTES = 'docs/card-pricing-notes.json'


def draw_count(card):
    return sum(e.get('count', 0) for e in card.get('effects', []) if e.get('op') == 'draw')


def main():
    base = {c['id']: c for c in json.load(open(sys.argv[1], encoding='utf-8'))}
    now = {c['id']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    edited = sorted({l.split('.')[1] for l in open(OVERLAY, encoding='utf-8') if l.startswith('cards.')})
    notes = json.load(open(NOTES, encoding='utf-8'))
    for cid in edited:
        b, a = base[cid], now[cid]
        what = []
        if draw_count(b) != draw_count(a):
            what.append(f'draws {draw_count(a)} when played (was {draw_count(b)})')
        if b.get('ability') != a.get('ability'):
            what.append('its Action changed')
        if b.get('passives') != a.get('passives'):
            what.append('its trigger names a tag or pays no card')
        if b.get('needs') != a.get('needs'):
            what.append(f'needs {a.get("needs")}')
        price = f'{a["cost"]} cr (was {b["cost"]})' if a['cost'] != b['cost'] else f'{a["cost"]} cr'
        notes[cid] = (f'r127 (271): the draw economy: {"; ".join(what) or "reworded"}; {price}, moved by its change in '
                      f'the effect ledger\'s reasoned value (floor 3); the balance pass follows.')
    with open(NOTES, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(dict(sorted(notes.items())), f, indent=2, ensure_ascii=False)
        f.write('\n')
    print(len(edited), 'notes')


if __name__ == '__main__':
    main()
