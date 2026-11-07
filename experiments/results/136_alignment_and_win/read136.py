"""136's read: the two Alignment tracks (r55) against r54 on the same seeds.

    python experiments/results/136_alignment_and_win/read136.py NEW_DIR OLD_DIR [--md OUT.md]

Per difficulty and agent of each DIR's games.jsonl: win %, game length (mean, median, 10th-90th percentile,
min-max), how games end, Unity and Discord tips a game, Unity deck wins before round 6, Envoy and Unity held off a
game (Discord moved off by both), then the paired win and length gaps NEW - OLD on the same (pair, seed) for each
agent of NEW against OLD's first agent. 135's tables (threats, counter kinds, nodes lost by cause, peak holdings)
follow, from `read135.py` on both dirs.
"""
import math
import statistics
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / '135_adversary_model'))
import read135  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def pct(xs, q):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))]


def c(g, k):
    return g['counters'].get(k, 0)


def table(d, out):
    head, games = read135.load(d)
    p = out.append
    p(f'## {d} (rules r{head.get("rules_revision")})')
    p('')
    p('| Difficulty | Agent | Games | Win % | Rounds mean | median | p10-p90 | min-max | Ends | Unity tips | '
      'Discord tips | Unity wins before r6 | Earliest Unity win | Envoy | Unity held off | Discord off |')
    p('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    by = defaultdict(list)
    for (diff, var, agent), gs in games.items():
        for g in gs:
            by[(diff, agent)].append(g)
    for (diff, agent), gs in sorted(by.items()):
        n = len(gs)
        w = sum(g['won'] for g in gs) / n
        r = [g['rounds'] for g in gs]
        ends = Counter(g['reason'] for g in gs)
        uw = [g['rounds'] for g in gs if g['won'] and g['reason'] == 'unity_deck']
        early = sum(1 for r_ in uw if r_ < 6)
        first = f'r{min(uw)}' if uw else 'none'
        m = lambda k: sum(c(g, k) for g in gs) / n  # noqa: E731
        p(f'| d{diff} | {agent} | {n} | {100 * w:.1f} ±{read135.se(w, n):.1f} | {statistics.mean(r):.2f} | '
          f'{statistics.median(r):g} | {pct(r, 0.1)}-{pct(r, 0.9)} | {min(r)}-{max(r)} | '
          + ', '.join(f'{k} {v}' for k, v in ends.most_common()) + f' | {m("alignment.tips_unity"):.2f} | '
          f'{m("alignment.tips_discord"):.2f} | {early} | {first} | {m("board.std.envoy"):.2f} | '
          f'{m("alignment.unity_held_off"):.2f} | {m("alignment.discord_held_off"):.2f} |')
    p('')
    return games


def paired(new, old, out):
    p = out.append
    base_agent = sorted({a for (_, _, a) in old})[0]
    old_by = {}
    for (diff, var, agent), gs in old.items():
        if agent != base_agent:
            continue
        for g in gs:
            old_by[(diff, var, g['seed'])] = g
    p(f'Paired NEW - OLD (OLD agent `{base_agent}`), same pair and seed:')
    p('')
    p('| Difficulty | NEW agent | Games | Win gap (points) | Rounds gap |')
    p('|---|---|---|---|---|')
    acc = defaultdict(list)
    for (diff, var, agent), gs in new.items():
        for g in gs:
            o = old_by.get((diff, var, g['seed']))
            if o:
                acc[(diff, agent)].append((int(g['won']) - int(o['won']), g['rounds'] - o['rounds']))
    for key, ds in sorted(acc.items()):
        n = len(ds)
        dw = [x for x, _ in ds]
        dr = [y for _, y in ds]
        sw = statistics.pstdev(dw) / math.sqrt(n) * 100 if n > 1 else 0
        sr = statistics.pstdev(dr) / math.sqrt(n) if n > 1 else 0
        p(f'| d{key[0]} | {key[1]} | {n} | {100 * statistics.mean(dw):+.1f} ±{sw:.1f} | '
          f'{statistics.mean(dr):+.2f} ±{sr:.2f} |')
    p('')
    # The on/off gap between NEW's agents, same seeds.
    agents = sorted({a for (_, _, a) in new})
    if len(agents) > 1:
        p(f'NEW agents on the same seeds: {agents[1]} - {agents[0]}:')
        p('')
        p('| Difficulty | Win gap | Rounds gap | Node losses (on / off) |')
        p('|---|---|---|---|')
        for diff in sorted({d for (d, _, _) in new}):
            a = {(v, g['seed']): g for (d, v, ag), gs in new.items() if d == diff and ag == agents[0] for g in gs}
            b = {(v, g['seed']): g for (d, v, ag), gs in new.items() if d == diff and ag == agents[1] for g in gs}
            ks = sorted(set(a) & set(b))
            dw = [int(b[k]['won']) - int(a[k]['won']) for k in ks]
            dr = [b[k]['rounds'] - a[k]['rounds'] for k in ks]
            nl = (sum(b[k]['reason'] == 'node' for k in ks), sum(a[k]['reason'] == 'node' for k in ks))
            n = len(ks)
            p(f'| d{diff} | {100 * statistics.mean(dw):+.1f} ±{statistics.pstdev(dw) / math.sqrt(n) * 100:.1f} | '
              f'{statistics.mean(dr):+.2f} | {nl[0]} / {nl[1]} |')
        p('')


def main():
    args = sys.argv[1:]
    md = None
    if '--md' in args:
        i = args.index('--md')
        md = args[i + 1]
        args = args[:i] + args[i + 2:]
    out = []
    new = table(args[0], out)
    old = table(args[1], out)
    paired(new, old, out)
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
