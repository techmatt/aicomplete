"""271: re-derive scripts/design_check_jobs.json's draw terms for the cards the overlay changed. Run from the repo root.

    python experiments/results/271_draw_economy/jobs_fix.py BASE_CARDS_JSON

A row's `other` holds its on-play draw as `N * card` and an Action's as `uses * card`: each is set to the card's
r127 count (dropped at 0), and the note gains the change.
"""
import json
import re
import sys

JOBS = 'scripts/design_check_jobs.json'


def on_play(c):
    return sum(e.get('count', 0) for e in c.get('effects', []) if e.get('op') == 'draw')


def action(c):
    return sum(e.get('count', 0) for e in json.loads(json.dumps(c.get('ability', {})), object_hook=lambda d: d)
               .get('effects', []) if e.get('op') == 'draw') if c.get('ability') else 0


def main():
    base = {c['name']: c for c in json.load(open(sys.argv[1], encoding='utf-8'))}
    now = {c['name']: c for c in json.load(open('content/cards.json', encoding='utf-8'))}
    text = open(JOBS, encoding='utf-8').read()
    d = json.loads(text)
    n = 0
    for job in d['jobs'].values():
        for row in job['cards']:
            name = row['name']
            if name not in now or name not in base:
                continue
            b, a = base[name], now[name]
            if (on_play(b), action(b)) == (on_play(a), action(a)):
                continue
            other = str(row.get('other', 0))
            terms = [t.strip() for t in other.split('+')] if other not in ('0', '') else []
            terms = [t for t in terms if not re.fullmatch(r'\d+ \* card', t) and t != 'uses * card']
            if on_play(a):
                terms.append(f'{on_play(a)} * card')
            if action(a):
                terms.append(f'{action(a)} * uses * card' if action(a) > 1 else 'uses * card')
            row['other'] = ' + '.join(terms) if terms else 0
            row['note'] = f'{row.get("note", "")} (271: draws {on_play(a)} on play, {action(a)} an Action)'.strip()
            n += 1
    with open(JOBS, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(d, f, indent=1, ensure_ascii=False)
        f.write('\n')
    print(n, 'rows')


if __name__ == '__main__':
    main()
