"""Win %, rounds and ends by difficulty and lever for a screen whose variants are `<pair>` or `<pair>_<lever>`.

    python experiments/results/137_sites_and_war/levers.py DIR [--md OUT.md]

Pools the three pairs (bp, br, pr) of each lever; the paired gap of each lever against the base (no suffix) on the
same (pair, seed).
"""
import math
import statistics
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / '135_adversary_model'))
import read135  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def main():
    args = sys.argv[1:]
    md = None
    if '--md' in args:
        i = args.index('--md')
        md = args[i + 1]
        args = args[:i] + args[i + 2:]
    _, games = read135.load(args[0])
    cells = defaultdict(dict)
    for (diff, var, agent), gs in games.items():
        pair, _, lever = var.partition('_')
        key = (lever or 'base') if len({a for (_, _, a) in games}) == 1 else f'{lever or "base"} {agent}'
        for g in gs:
            cells[(diff, key)][(pair, g['seed'])] = g
    out = ['| Difficulty | Lever | Games | Win % | Gap vs base | Rounds | Ends | Purge | Nodes lost |',
           '|---|---|---|---|---|---|---|---|---|']
    for (diff, key), gs in sorted(cells.items()):
        n = len(gs)
        w = sum(g['won'] for g in gs.values()) / n
        base = cells.get((diff, 'base'), {})
        ks = sorted(set(gs) & set(base))
        d = [int(gs[k]['won']) - int(base[k]['won']) for k in ks]
        gap = f'{100 * statistics.mean(d):+.1f} ±{statistics.pstdev(d) / math.sqrt(len(d)) * 100:.1f}' if d and key != 'base' else ''
        ends = Counter(g['reason'] for g in gs.values())
        r = statistics.mean(g['rounds'] for g in gs.values())
        lost = statistics.mean(g['counters'].get('board.node_lost', 0) for g in gs.values())
        out.append(f'| d{diff} | {key} | {n} | {100 * w:.1f} ±{read135.se(w, n):.1f} | {gap} | {r:.2f} | '
                   + ', '.join(f'{k} {v}' for k, v in ends.most_common(4)) + f' | {ends.get("purge", 0)} | {lost:.2f} |')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
