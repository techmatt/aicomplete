"""139's read helpers.

    python experiments/results/139_discord_and_band/read139.py merge OUT_DIR DIR_A DIR_B
        # one games.jsonl from two runs of the same cells (read-139-*-a and -b: 48 seeds in two ranges)
    python experiments/results/139_discord_and_band/read139.py discord DIR [DIR ...]
        # per difficulty: Discord cards drawn a game (each card), threats Offensive dealt onto a node hex, adversary
        # ambassadors Discord cards created, nodes lost a game by cause, uncountered threats against corruption

Pairs (bp, br, pr) pool; a variant `<pair>_<lever>` keeps its lever in the row label.
"""
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
DISCORD = ['incursion', 'blight', 'propaganda', 'offensive', 'two_fronts', 'firestorm']


def merge(out, a, b):
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    with open(out / 'games.jsonl', 'w', encoding='utf-8', newline='\n') as f:
        heads = []
        for i, d in enumerate((a, b)):
            with open(Path(d) / 'games.jsonl', encoding='utf-8') as g:
                head = g.readline()
                heads.append(json.loads(head)['spec'])
                if i == 0:
                    f.write(head)
                for line in g:
                    f.write(line)
        key = lambda s: (s['variants'], s['difficulties'], s['agents'], s['configs'])  # noqa: E731
        assert key(heads[0]) == key(heads[1]), 'the two runs must have the same cells'
    print('merged into', out / 'games.jsonl')


def load(d):
    cells, games = None, defaultdict(list)
    with open(Path(d) / 'games.jsonl', encoding='utf-8') as f:
        for i, line in enumerate(f):
            g = json.loads(line)
            if i == 0:
                sp = g['spec']
                cells = [(v['name'], df) for v in sp['variants'] for df in sp['difficulties'] for _ in sp['agents']]
                continue
            var, df = cells[g['cell']]
            lever = var.split('_', 1)[1] if '_' in var else 'base'
            games[(lever, df)].append(g)
    return games


def c(g, k):
    return g['counters'].get(k, 0)


def discord(dirs):
    for d in dirs:
        print(f'## {d}')
        for (lever, df), gs in sorted(load(d).items(), key=lambda x: (x[0][1], x[0][0])):
            n = len(gs)
            mean = lambda k: sum(c(g, k) for g in gs) / n  # noqa: E731
            cards = ', '.join(f'{k} {mean("alignment.card." + k):.2f}' for k in DISCORD)
            lost = defaultdict(float)
            for g in gs:
                for k, v in g['counters'].items():
                    if k.startswith('board.node_lost.'):
                        lost[k[len('board.node_lost.'):]] += v / n
            threat = sum(v for k, v in lost.items() if k.startswith('threat'))
            print(f'- {lever} d{df}: {n} games, win {100 * sum(g["won"] for g in gs) / n:.1f}%, median rounds '
                  f'{statistics.median(g["rounds"] for g in gs)}, Discord tips {mean("alignment.tips_discord"):.2f}')
            print(f'    Discord cards drawn a game: {cards}')
            print(f'    Offensive threats onto a node hex {mean("threats.dealt_at_node"):.2f}; ambassadors created by '
                  f'Discord cards {mean("adversary.created.discord"):.2f}')
            print(f'    nodes lost a game {mean("board.node_lost"):.2f}: '
                  + ', '.join(f'{k} {v:.2f}' for k, v in sorted(lost.items()))
                  + f'; uncountered threats {threat:.2f} vs corruption {lost.get("corruption", 0):.2f}')


if __name__ == '__main__':
    if sys.argv[1] == 'merge':
        merge(*sys.argv[2:5])
    else:
        discord(sys.argv[2:])
