"""195: whether two reads of the same spec and seeds played the same games, from games.jsonl alone (a farmed read has
no games.csv or results.json for `scripts/bench/same_results.py`): per (cell, seed), the outcome, end reason, rounds
and decisions. Prints how many differ, and the win % per difficulty on each side. Usage:

    python experiments/results/195_project_overhaul/same_games.py A/games.jsonl B/games.jsonl
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
from screens import cells_of  # noqa: E402


def read(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        cells = cells_of(json.loads(f.readline())['spec'])
        for line in f:
            g = json.loads(line)
            c = cells[g['cell']]
            out[(c['variant'], c['difficulty'], c['agent'], str(g['seed']))] = (g['won'], g['reason'], g['rounds'],
                                                                              g['decisions'])
    return out


def main():
    a, b = read(sys.argv[1]), read(sys.argv[2])
    keys = sorted(set(a) & set(b))
    diff = [k for k in keys if a[k] != b[k]]
    print(f'{len(keys)} shared games, {len(diff)} differ ({len(set(a) ^ set(b))} unpaired)')
    for side, data in (('A', a), ('B', b)):
        wins, n = defaultdict(int), defaultdict(int)
        for k in keys:
            n[k[1]] += 1
            wins[k[1]] += int(data[k][0])
        print(side, ' '.join(f'd{d} {100 * wins[d] / n[d]:.1f}' for d in sorted(n)))


if __name__ == '__main__':
    main()
