"""273: every whole-game counter's mean a game-round, new read against old (paired cells and seeds), biggest moves first.

    python experiments/results/273_smaller_numbers/counters_diff.py NEW.jsonl OLD.jsonl [--grep TEXT] [--top N]

Counters are flattened (`a.b.c`); a node's own count sits at its path. Pooled over every cell, per game-round, so a
longer game doesn't inflate a count. Prints new, old, the ratio and the difference.
"""
import argparse
import collections
import json


def flat(d, pre, out):
    for k, v in d.items():
        key = f'{pre}.{k}' if pre and k != '' else (pre or k)
        if isinstance(v, dict):
            flat(v, key, out)
        elif isinstance(v, (int, float)):
            out[key] += v


def load(path):
    tot = collections.Counter()
    rounds = 0
    games = 0
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'counters' not in g and 'stats' not in g:
                continue
            c = g.get('counters') or g.get('stats', {}).get('counters', {})
            flat(c, '', tot)
            rounds += g.get('rounds') or g.get('round') or 0
            games += 1
    return tot, rounds, games


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('new')
    ap.add_argument('old')
    ap.add_argument('--grep', default='')
    ap.add_argument('--top', type=int, default=60)
    a = ap.parse_args()
    n, nr, ng = load(a.new)
    o, orr, og = load(a.old)
    print(f'new {ng} games {nr} rounds; old {og} games {orr} rounds')
    rows = []
    for k in set(n) | set(o):
        if a.grep and a.grep not in k:
            continue
        x, y = n[k] / max(nr, 1), o[k] / max(orr, 1)
        if x == y:
            continue
        rows.append((abs(x - y) / max(abs(y), 0.05), k, x, y))
    rows.sort(reverse=True)
    print('| counter | new / round | old / round | new ÷ old |')
    print('|---|---|---|---|')
    for _, k, x, y in rows[:a.top]:
        print(f'| {k} | {x:.3f} | {y:.3f} | {x / y if y else float("inf"):.2f} |')


if __name__ == '__main__':
    main()
