"""List tier-2 (or all) cards from docs/cards.md with plays a game from a read's cards.csv.

python t2list.py CARDS_CSV [tier]   (tier default 2; 'all' for every card)
"""
import csv, json, re, sys, collections

cards = json.load(open('content/cards.json', encoding='utf-8'))
by_name = {c['name']: c for c in cards}
tier = sys.argv[2] if len(sys.argv) > 2 else '2'
rows = {}
for line in open('docs/cards.md', encoding='utf-8'):
    if not line.startswith('| ') or line.startswith('| Card') or line.startswith('|---'):
        continue
    cells = [x.strip() for x in line.strip().strip('|').split('|')]
    if cells[0] in by_name:
        rows[cells[0]] = cells
games = collections.Counter()
st = collections.defaultdict(lambda: [0, 0, 0, 0])
seen = set()
for r in csv.DictReader(open(sys.argv[1], encoding='utf-8')):
    key = (r['agent'], r['variant'], r['difficulty'])
    st[r['card']][0] += int(r['offered']); st[r['card']][1] += int(r['kept']); st[r['card']][2] += int(r['played'])
# game count from games.csv beside it
ng = sum(1 for _ in open(sys.argv[1].replace('cards.csv', 'games.csv'), encoding='utf-8')) - 1
print(f'games {ng}')
for c in cards:
    if tier != 'all' and str(c['tier']) != tier:
        continue
    s = st.get(c['id'], [0, 0, 0, 0])
    cells = rows.get(c['name'])
    text = cells[-1] if cells else '?'
    cost = cells[3] if cells else '?'
    print(f"{c['id']} | {c['name']} | t{c['tier']} {c['kind']} | {cost} | ins {c.get('insight',0)} | {'/'.join(c['tags'])} | kept {s[1]/ng:.2f} played {s[2]/ng:.2f} | {text}")
