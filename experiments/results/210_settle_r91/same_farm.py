"""210: whether two farm outputs (games.jsonl lines) played the same games: cell, seed, level, result, rounds and
the action log, game by game. Stats and timings may differ.

    python experiments/results/210_settle_r91/same_farm.py A.jsonl B.jsonl
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KEYS = ('cell', 'seed', 'difficulty', 'won', 'reason', 'rounds', 'decisions', 'actions')


def games(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'seed' not in g or 'actions' not in g:
                continue
            out[(g['cell'], g['seed'], g['difficulty'])] = g
    return out


a, b = games(sys.argv[1]), games(sys.argv[2])
diff = [k for k in a if k not in b or any(a[k].get(x) != b[k].get(x) for x in KEYS)]
missing = [k for k in b if k not in a]
print(f'same: {len(a) - len(diff)} of {len(a)} games' + (f'; differ {len(diff)}: {diff[:5]}' if diff else '')
      + (f'; only in B {len(missing)}' if missing else ''))
sys.exit(1 if diff or missing else 0)
