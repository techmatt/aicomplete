"""234: the converted cards in a read: games that kept each, games that played it, plays a game, per agent; and the
same for the base read when given. Done-when 3: every converted card is played.

    python experiments/results/234_card_targets/card_plays.py NEW_DIR [OLD_GAMES_FILE]
"""
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))


def converted():
    faces = json.load(open(os.path.join(ROOT, 'web', 'card', 'faces.json'), encoding='utf-8'))
    deck = open(os.path.join(ROOT, 'configs', 'baseline.json'), encoding='utf-8').read()
    shelter = ['aegis_grid_lockdown']  # converted to the targetless reach Shelter
    return sorted([k for k, v in faces.items() if v.get('target') and f'"{k}"' in deck] + shelter)


sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_for  # noqa: E402


def read(d):
    """Per agent: games, and per card (kept games, played games, plays). `d`: a read's folder (games.jsonl with
    its header)."""
    cells = cells_for(d)
    with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
        lines = f.read().splitlines()
    n = collections.Counter()
    stats = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0]))
    for line in lines[1:]:
        if not line.startswith('{"cell"'):
            continue
        g = json.loads(line)
        agent = cells[g['cell']]['agent'].split(':')[0]
        n[agent] += 1
        for cid, c in g['cards'].items():
            s = stats[agent][cid]
            s[0] += c.get('kept', 0) > 0
            s[1] += c.get('played', 0) > 0
            s[2] += c.get('played', 0)
    return n, stats


def main():
    ids = converted()
    new_n, new = read(sys.argv[1])
    old_n, old = read(sys.argv[2]) if len(sys.argv) > 2 else (None, None)
    for agent in sorted(new_n):
        print(f'## {agent} ({new_n[agent]} games)\n')
        print('| card | kept % | played % | plays a game | r103 played % | r103 plays a game |')
        print('|---|---|---|---|---|---|')
        for cid in ids:
            k, p, plays = new[agent][cid]
            row = f'| {cid} | {100 * k / new_n[agent]:.1f} | {100 * p / new_n[agent]:.1f} | {plays / new_n[agent]:.2f} |'
            if old is not None:
                tot = sum(old_n.values())
                ok, op, oplays = [sum(old[a][cid][i] for a in old) for i in range(3)]
                row += f' {100 * op / tot:.1f} | {oplays / tot:.2f} |'
            else:
                row += ' | |'
            print(row)
        never = [c for c in ids if new[agent][c][1] == 0]
        print(f'\nnever played: {", ".join(never) or "none"}\n')


main()
