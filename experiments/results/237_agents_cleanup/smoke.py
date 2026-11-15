"""237: the agent-valuation smoke, each arm against the arm before it and against base, paired on (cell, seed).

    python experiments/results/237_agents_cleanup/smoke.py BASE.jsonl ARM1.jsonl [ARM2.jsonl ...]

Each file is a games file with its header (`shards.sh` output) of one spec (experiments/farm/s237.json). Per agent:
win % per file, paired gap ± one SE against the previous file and against the first, by difficulty and pooled; and
the share of games that played differently (decisions, rounds or result moved). Last, the `cards.unofferable.<id>`
tally of the last file, per game.
"""
import json
import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'scripts'))
from screens import cells_of  # noqa: E402


def load(path):
    with open(path, encoding='utf-8') as f:
        cells = cells_of(json.loads(f.readline())['spec'])
        out = {}
        for line in f:
            if line.strip():
                g = json.loads(line)
                out[(g['cell'], g['seed'])] = g
    return cells, out


def gap(a, b, keys):
    d = [float(b[k]['won']) - float(a[k]['won']) for k in keys]
    n = len(d)
    if n < 2:
        return 0.0, 0.0
    m = sum(d) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1))
    return 100 * m, 100 * sd / math.sqrt(n)


def moved(a, b, keys):
    f = lambda g: (g['won'], g['rounds'], g.get('decisions'))  # noqa: E731
    return sum(f(a[k]) != f(b[k]) for k in keys) / max(len(keys), 1)


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    paths = sys.argv[1:]
    runs = [load(p) for p in paths]
    cells = runs[0][0]
    names = [Path(p).stem for p in paths]
    groups = defaultdict(list)
    for key in runs[0][1]:
        c = cells[key[0]]
        groups[(c['agent'], c['difficulty'])].append(key)
        groups[(c['agent'], 'all')].append(key)
    print('| agent | d | games | ' + ' | '.join(f'{n} win %' for n in names) + ' | '
          + ' | '.join(f'{names[i]} - {names[i - 1]}' for i in range(1, len(names)))
          + (f' | {names[-1]} - {names[0]}' if len(names) > 2 else '') + ' | games moved (last vs base) |')
    print('|---|---|---|' + '---|' * (len(names) * 2 - 1 + (len(names) > 2)) + '---|')
    for (agent, d), keys in sorted(groups.items(), key=lambda x: (x[0][0], str(x[0][1]))):
        keys = [k for k in keys if all(k in r[1] for r in runs)]
        wins = [100 * sum(r[1][k]['won'] for k in keys) / len(keys) for r in runs]
        gaps = [gap(runs[i - 1][1], runs[i][1], keys) for i in range(1, len(runs))]
        if len(runs) > 2:
            gaps.append(gap(runs[0][1], runs[-1][1], keys))
        cols = ' | '.join(f'{w:.1f}' for w in wins) + ' | ' + ' | '.join(f'{m:+.1f} ±{s:.1f}' for m, s in gaps)
        print(f'| {agent} | {d} | {len(keys)} | {cols} | {100 * moved(runs[0][1], runs[-1][1], keys):.0f}% |')
    last = runs[-1][1]
    stuck = defaultdict(int)
    for g in last.values():
        for k, v in (g.get('counters') or {}).items():
            if k.startswith('cards.unofferable.') and '.r' not in k[len('cards.unofferable.'):]:
                stuck[k[len('cards.unofferable.'):]] += v
    n = len(last)
    print(f'\ncards.unofferable in {names[-1]} ({n} games), per game:')
    for cid, v in sorted(stuck.items(), key=lambda x: -x[1]):
        print(f'- {cid}: {v / n:.3f}')


if __name__ == '__main__':
    main()
