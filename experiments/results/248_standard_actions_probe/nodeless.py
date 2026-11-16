"""248: who runs out of nodes. For the probe's arms (p248) and the job screen's -node arm (p248j): wins by pair, and
in node-loss games the board whose nodes ran out (fewest nodes at the end) and the round.

usage: python nodeless.py P248.jsonl P248J.jsonl"""
import json
import sys
from collections import Counter, defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = ['bp', 'br', 'pr']


def rows(path):
    with open(path, encoding='utf-8') as f:
        for line in f:
            if line.startswith('{"cell"'):
                yield json.loads(line)


def read(path, names, want):
    wins = defaultdict(list)
    lost_by = defaultdict(Counter)
    rounds = defaultdict(list)
    for g in rows(path):
        v, lv = divmod(g['cell'], 4)
        arm, pair = divmod(v, 3)
        n = names[arm] if arm < len(names) else None
        if n not in want:
            continue
        wins[(n, PAIRS[pair])].append(g['won'])
        if g['reason'] == 'node':
            boards = [b.split(':')[0] for b in g['build'].split('|')]
            # `node_spread`: each seat's nodes at the end (by country); the seat at 0 lost them all.
            nodes = [sum(x) for x in g['margin'].get('node_spread', [])]
            who = '?'
            if isinstance(nodes, list) and len(nodes) == len(boards):
                who = boards[min(range(len(boards)), key=lambda s: nodes[s])]
            lost_by[n][who] += 1
            rounds[n].append(g['rounds'])
    for n in want:
        print(f"- {n}: wins " + ", ".join(f"{p} {sum(wins[(n, p)]) / len(wins[(n, p)]) * 100:.1f}" for p in PAIRS if wins[(n, p)]) +
              f"; node-loss games {sum(lost_by[n].values())}, by board {dict(lost_by[n])}, mean round "
              f"{sum(rounds[n]) / len(rounds[n]) if rounds[n] else 0:.1f}")


read(sys.argv[1], ['a', 'b', 'bd', 'c'], ['a', 'b', 'c'])
LOO = ["node", "station", "remove", "troop", "lift", "dispatch", "ambassador", "claim", "insight", "botnet",
       "buy_energy", "antimatter", "sell"]
read(sys.argv[2], ["-" + j for j in LOO] + ["b+" + j for j in ["node", "station", "insight", "buy_energy", "botnet", "claim"]],
     ['-node', '-station', 'b+node', 'b+station'])
