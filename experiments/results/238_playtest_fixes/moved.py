"""238: what moved between two farm outputs of one spec and seeds (base r107, new r108), game by game.

    python experiments/results/238_playtest_fixes/moved.py BASE.jsonl NEW.jsonl

Prints how many paired games differ in outcome, end reason and length, and the per-game means of the counters
238's fixes touch (and the largest other counter moves), each side and the difference.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

WATCH = [
    'threats.hit_ties', 'threats.hit_ties_unasked', 'board.march_cut_by_loss', 'effects.targets',
    'effects.targets_missed', 'effects.shelter_reach', 'alignment.unity_held_off', 'fallout.placed',
    'board.troop_destroyed.node', 'cards.played',
]


def games(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g:
                continue
            out[(g['cell'], g['seed'])] = g
    return out


def main():
    a, b = games(sys.argv[1]), games(sys.argv[2])
    keys = sorted(set(a) & set(b))
    n = len(keys)
    same = lambda f: sum(a[k].get(f) == b[k].get(f) for k in keys)
    print(f'{n} paired games')
    for f in ('won', 'reason', 'rounds', 'decisions'):
        print(f'  same {f}: {same(f)} ({100 * same(f) / n:.1f}%)')
    flips = [(k, a[k]['won'], b[k]['won']) for k in keys if a[k]['won'] != b[k]['won']]
    print(f'  outcome flips: {len(flips)} (base won, new lost: {sum(1 for _, x, _ in flips if x)})')
    tot = defaultdict(lambda: [0.0, 0.0])
    for k in keys:
        for side, g in ((0, a[k]), (1, b[k])):
            for c, v in (g.get('counters') or {}).items():
                if isinstance(v, (int, float)):
                    tot[c][side] += v
    print('\nwatched counters, a game (base / new / diff):')
    for c in WATCH:
        x, y = tot.get(c, [0, 0])
        print(f'  {c:34s} {x / n:8.3f} {y / n:8.3f} {(y - x) / n:+8.3f}')
    moved = sorted(tot.items(), key=lambda kv: -abs(kv[1][1] - kv[1][0]) / n)
    print('\nlargest other moves, a game:')
    shown = 0
    for c, (x, y) in moved:
        if c in WATCH or c.startswith(('agents.', 'time')):
            continue
        print(f'  {c:44s} {x / n:9.3f} {y / n:9.3f} {(y - x) / n:+8.3f}')
        shown += 1
        if shown == 15:
            break


main()
