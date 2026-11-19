"""282: the hand-edited content (alignment.json and the adversary's drafted projects), rerunnable from r133.

python experiments/results/282_balance_pass/apply_drafts.py [--dry-run]

- Surge projects priced as captures (givget.md): drafts/adversary/projects.json costs (then content/load_adversary.py).
- New Deal and Rebuilding Fund: 1 credit a site -> 2 (no card prints 1 credit); Propaganda: 2 credits a corrupted
  city, at most 1 / 2 / 3 (was 1, at most 2 / 3 / 4).
"""
import json
import re
import sys

PROJECTS = {
    'lab_seizure': (13, 21),
    'coastal_raids': (9, 14),
    'plant_seizure': (9, 13),
    'mob_rule': (14, 21),
    'martial_law': (15, 29),
    'recruitment_drive': (20, 32),
    'grand_offensive': (22, 32),
}


def block(text, ident):
    """The span of the object whose "id" is ident: from its id line to the next "id" line."""
    m = re.search(r'"id": "%s"' % re.escape(ident), text)
    if not m:
        raise SystemExit(f'no id {ident}')
    n = re.search(r'"id": "', text[m.end():])
    end = m.end() + n.start() if n else len(text)
    return m.start(), end


def edit(text, ident, fn):
    a, b = block(text, ident)
    new = fn(text[a:b])
    if new == text[a:b]:
        raise SystemExit(f'{ident}: nothing changed')
    return text[:a] + new + text[b:]


def main():
    dry = '--dry-run' in sys.argv
    p = 'drafts/adversary/projects.json'
    t = open(p, encoding='utf-8', newline='').read()
    for ident, (old, new) in PROJECTS.items():
        def reprice(s, old=old, new=new):
            s = s.replace(f'"cost": {old},', f'"cost": {new},', 1)
            return s.replace(f'"value": "275: give {old} compute a board ({2 * old} at 2 countries);',
                             f'"value": "282: give {new} compute a board ({2 * new} at 2 countries), priced as its captures '
                             f'(experiments/results/282_balance_pass/givget.md; 275: {old});', 1)
        t = edit(t, ident, reprice)
    if not dry:
        open(p, 'w', encoding='utf-8', newline='\n').write(t)

    a = 'content/alignment.json'
    t = open(a, encoding='utf-8', newline='').read()

    def two_credits(s):
        s = s.replace('gains 1 credit and', 'gains 2 credits and')
        return s.replace('{"op": "gain", "resource": "credits", "amount": 1}', '{"op": "gain", "resource": "credits", "amount": 2}')

    t = edit(t, 'new_deal', two_credits)
    t = edit(t, 'rebuilding_fund', two_credits)

    def propaganda(s):
        s = s.replace('loses 1 credit for each corrupted city', 'loses 2 credits for each corrupted city')
        s = s.replace('{"op": "players_lose", "resource": "credits", "amount": 1}', '{"op": "players_lose", "resource": "credits", "amount": 2}')
        for old, new in ((2, 1), (3, 2), (4, 3)):
            s = s.replace(f'(at most {old}).', f'(at most {new}).', 1)
            s = s.replace(f'"max": {old},', f'"max": {new},', 1)
        return s

    t = edit(t, 'propaganda', propaganda)
    if not dry:
        open(a, 'w', encoding='utf-8', newline='\n').write(t)
    print('done' if not dry else 'dry run ok')


if __name__ == '__main__':
    main()
