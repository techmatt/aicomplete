"""189: the amendment build (absolute insight totals) against the farmed r82 build on the same seeds: every game
present in both must match in outcome, rounds, decisions, options and counters.

    python same_games.py A_DIR B_DIR
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KEYS = ['won', 'reason', 'rounds', 'decisions', 'options_total', 'counters', 'cards', 'spent', 'uses']
if '--play' in sys.argv:  # a stats-only patch (189's `limit.*` counters): compare play, not counters
    sys.argv.remove('--play')
    KEYS = ['won', 'reason', 'rounds', 'decisions', 'options_total', 'cards', 'spent', 'uses']


def load(d):
    out = {}
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            out[(g['cell'], str(g['seed']))] = g
    return out


a, b = load(sys.argv[1]), load(sys.argv[2])
both = sorted(set(a) & set(b))
diff = [k for k in both if any(a[k].get(x) != b[k].get(x) for x in KEYS)]
print(f'{len(both)} games in both, {len(diff)} differ')
for k in diff[:5]:
    print(k, [x for x in KEYS if a[k].get(x) != b[k].get(x)])
sys.exit(1 if diff or not both else 0)
