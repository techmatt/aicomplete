"""162: agents against one another in one run (a screen or a read with several agents), paired on pair and seed.

python experiments/results/162_judge_purge/judges162.py DIR[,DIR...] --ref AGENT_OR_NAME [--name SPEC=NAME]... [--md]

Each DIR is an experiment or assembled farm output (games.jsonl); several DIRs are a read's sets, pooled. Per
difficulty and agent: games, win %, SE, the paired gap to --ref (same variant and seed, across engines too) and its SE, each
set's win %, and a game's cities freed, ambassadors bought (the Ambassador action) and Liberates. Also prints the
shards by engine (native / wasm) of each DIR.
"""
import argparse
import collections
import math
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cells_of, engine, gap, stream  # noqa: E402

MEASURES = (("freed", "board.liberated.city"), ("amb", "board.std.ambassador"), ("lib", "board.std.liberate"))


def load(d):
    it = stream(d)
    head = next(it)
    cells = cells_of(head['spec'])
    out = []
    engines = collections.Counter()
    for g in it:
        c = cells[g['cell']]
        engines[engine(g)] += 1
        cnt = g.get('counters', {})
        out.append({'variant': c['variant'], 'difficulty': c['difficulty'], 'agent': c['agent'], 'seed': int(g['seed']),
                    'won': bool(g['won']), 'engine': engine(g),
                    **{k: cnt.get(key, 0) for k, key in MEASURES}})
    return out, engines


def se(p, n):
    return 100 * math.sqrt(p * (1 - p) / n) if n else 0.0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs')
    ap.add_argument('--ref', required=True)
    ap.add_argument('--name', action='append', default=[], help='SPEC=NAME, a short name for an agent spec')
    a = ap.parse_args()
    names = {}
    for x in a.name:
        spec, name = x.rsplit('=', 1)  # a spec holds '=' itself: the name is after the last one
        names[spec] = name
    sets = []
    for d in a.dirs.split(','):
        games, engines = load(d)
        print(f"{d}: {sum(engines.values())} games, by engine: {dict(engines)}")
        sets.append(games)
    nm = lambda s: names.get(s, s)
    agents = []
    for gs in sets:
        for g in gs:
            if nm(g['agent']) not in agents:
                agents.append(nm(g['agent']))
    ref = a.ref
    by = collections.defaultdict(dict)  # (set, difficulty, agent) -> {(variant, seed): game}
    for i, gs in enumerate(sets):
        for g in gs:
            by[(i, g['difficulty'], nm(g['agent']))][(g['variant'], g['seed'])] = g
    diffs = sorted({g['difficulty'] for gs in sets for g in gs})
    print()
    print(f"| Level | Agent | Games | Win % | SE | Gap to {ref} | Gap SE | Sets | Freed | Amb bought | Liberates |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for d in diffs:
        for ag in agents:
            games, pairs, per_set, mixed = [], [], [], 0
            for i in range(len(sets)):
                mine = by.get((i, d, ag), {})
                if not mine:
                    continue
                games += mine.values()
                per_set.append(100 * sum(g['won'] for g in mine.values()) / len(mine))
                theirs = by.get((i, d, ref), {})
                for k, g in mine.items():
                    t = theirs.get(k)
                    if t is None:
                        continue
                    # Native and wasm play identical games (160, 161): a pair across engines counts (Matt, 7325).
                    mixed += t['engine'] != g['engine']
                    pairs.append(int(g['won']) - int(t['won']))
            if not games:
                continue
            n = len(games)
            p = sum(g['won'] for g in games) / n
            m, s, _ = gap(pairs)
            mean = lambda k: sum(g[k] for g in games) / n
            gap_txt = "-" if ag == ref else f"{m:+.1f}"
            se_txt = "-" if ag == ref else f"{s:.1f}" + (f" ({mixed} across engines)" if mixed else "")
            print(f"| d{d} | {ag} | {n} | {100 * p:.1f} | {se(p, n):.1f} | {gap_txt} | {se_txt} | "
                  f"{' / '.join(f'{x:.1f}' for x in per_set)} | {mean('freed'):.2f} | {mean('amb'):.2f} | {mean('lib'):.2f} |")


if __name__ == '__main__':
    main()
