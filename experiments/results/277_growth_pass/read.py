"""277: the paired read's win % and rounds by level and agent, from farmed JSONL.

    python read.py NEW=SPEC OLD=SPEC

Win % and mean rounds per (agent, level), new against old, with the gap's paired SE (same seed, pair and level).
Growth option use comes from `python scripts/design_check.py NEW=SPEC --usage`. Run from the repo root.
"""
import collections
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cell_labels, stream  # noqa: E402


def load(arg):
    it = stream(arg)
    header = next(it)
    labels = cell_labels(header['spec'])
    out = {}
    for g in it:
        label = labels[g['cell']]
        parts = label.split('/')
        agent, variant, level = parts[-1], parts[-2], g['difficulty']
        out[(agent, level, variant, g['seed'])] = (1.0 if g['won'] else 0.0, g['rounds'])
    return out


def main():
    new, old = load(sys.argv[1]), load(sys.argv[2])
    keys = sorted(set(new) & set(old))
    cells = collections.defaultdict(list)
    for k in keys:
        cells[(k[0], k[1])].append((new[k], old[k]))
    print('| Agent | Level | Win % old → new | Gap ± SE (paired) | Rounds old → new | Gap ± SE |')
    print('|---|---|---|---|---|---|')
    for agent in ('strategist', 'greedy'):
        for level in range(5):
            rows = cells.get((agent, level))
            if not rows:
                continue
            n = len(rows)

            def stat(i):
                d = [a[i] - b[i] for a, b in rows]
                m = sum(d) / n
                sd = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1)) if n > 1 else 0.0
                return m, sd / math.sqrt(n)
            wn = 100 * sum(a[0] for a, _ in rows) / n
            wo = 100 * sum(b[0] for _, b in rows) / n
            rn = sum(a[1] for a, _ in rows) / n
            ro = sum(b[1] for _, b in rows) / n
            gw, sw = stat(0)
            gr, sr = stat(1)
            name = 'judge' if agent == 'strategist' else agent
            print(f'| {name} | d{level} | {wo:.1f} → {wn:.1f} | {100 * gw:+.1f} ± {100 * sw:.1f} | {ro:.2f} → {rn:.2f} | '
                  f'{gr:+.2f} ± {sr:.2f} |')
    print(f'\n{len(keys)} paired games.')


if __name__ == '__main__':
    main()
