"""107: an agent's keeps and plays a game, all cards, per read (cards.csv), and the kept-never-played gap.

    python experiments/results/107_agents_play_engines/totals.py DIR[:AGENT] ...
"""
import csv
import sys

for arg in sys.argv[1:]:
    d, _, agent = arg.partition(':')
    agent = agent or 'engine:strategy=1'
    n = sum(1 for _ in open(f'{d}/games.jsonl', encoding='utf-8'))
    agents = {r['agent'] for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8'))}
    games = n / len(agents)
    k = p = 0
    for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8')):
        if r['agent'] == agent:
            k += int(r['kept'])
            p += int(r['played'])
    print(f'{d} {agent}: kept {k / games:.2f} a game, played {p / games:.2f}, kept - played {(k - p) / games:.2f}')
