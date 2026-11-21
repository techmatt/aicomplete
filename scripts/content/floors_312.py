"""312 (r151, Matt's picks on 309's friction table): content edits, rerunnable from any revision.

- Row 5: every Unity card's tiers print a floor after their one fear-card op: when it counts none (no site, nothing
  removed, no hex covered), every player gains 2 credits (`floor`). The text gets the matching sentence.
- Row 11: Incursion I-II name the extra threats' tier and country (`threats::deal_extra`: the tier of this round's
  adversary card's first threat, one country piece at a time from the first).

Run from the repo root: python scripts/content/floors_312.py. Then python scripts/regen.py.
"""
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'cards'))
from apply_overlay import dump_alignment  # noqa: E402

PATH = 'content/alignment.json'
FLOOR = {"op": "floor", "then": [{"op": "gain", "resource": "credits", "amount": 2}]}
SAYS = {
    'per_site': ' If there are none, every player gains 2 credits instead.',
    'remove_where': ' If nothing comes off, every player gains 2 credits instead.',
    'prohibit': ' If no city, research station or factory qualifies, every player gains 2 credits instead.',
}
# The first wording, before the diff review: the engine counts sites, not hexes, for a prohibition's floor.
OLD_SAYS = {'prohibit': ' If no hex qualifies, every player gains 2 credits instead.'}
INCURSION = {
    0: ("Deal 1 extra threat for each corrupted city (at most 2).",
        "Deal 1 extra threat for each corrupted city (at most 2), of the tier of this round's adversary card's first "
        "threat, one country piece at a time from the first."),
    1: ("Deal 1 extra threat for each corrupted city (at most 4).",
        "Deal 1 extra threat for each corrupted city (at most 4), of the tier of this round's adversary card's first "
        "threat, one country piece at a time from the first."),
}


def main():
    with open(PATH, encoding='utf-8') as f:
        data = json.load(f)
    edits = 0
    for card in data:
        if card['side'] == 'unity':
            for t in card['tiers']:
                ops = t['effects']
                if any(o['op'] == 'floor' for o in ops):
                    old = OLD_SAYS.get(ops[0]['op'])
                    if old and t['text'].endswith(old):
                        t['text'] = t['text'][:-len(old)] + SAYS[ops[0]['op']]
                        edits += 1
                    continue
                if len(ops) != 1 or ops[0]['op'] not in SAYS:
                    sys.exit(f"{card['id']}: a tier is not one fear-card op: {[o['op'] for o in ops]}")
                ops.append(json.loads(json.dumps(FLOOR)))
                t['text'] = t['text'].rstrip() + SAYS[ops[0]['op']]
                edits += 1
        if card['id'] == 'incursion':
            for i, (old, new) in INCURSION.items():
                text = card['tiers'][i]['text']
                if text == new:
                    continue
                if text != old:
                    sys.exit(f"incursion tier {i + 1}: unexpected text {text!r}")
                card['tiers'][i]['text'] = new
                edits += 1
    with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
        f.write(dump_alignment(data))
    print(f"{edits} edits")


if __name__ == '__main__':
    main()
