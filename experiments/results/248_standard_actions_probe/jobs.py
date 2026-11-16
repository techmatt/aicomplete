"""248: read the job screen (p248j): each standard action off alone from arm a (-X), and a few back alone into
arm b (b+X), paired with p248's a and b on the same seeds.

usage: python jobs.py P248J.jsonl P248.jsonl [--md OUT.md]
Cells: arm-major, three pairs, d0-d3, cell = (arm*3 + pair)*4 + level; arm names from mkspec.JOB_ARMS."""
import argparse
import json
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
LOO = ["node", "station", "remove", "troop", "lift", "dispatch", "ambassador", "claim", "insight", "botnet",
       "buy_energy", "antimatter", "sell"]
LOI = ["node", "station", "insight", "buy_energy", "botnet", "claim"]
JOB_ARMS = ["-" + j for j in LOO] + ["b+" + j for j in LOI]
ACT = {"dispatch": "move"}


def rows(path):
    with open(path, encoding='utf-8') as f:
        for line in f:
            if line.startswith('{"cell"'):
                yield json.loads(line)


def brief(g):
    c = g['counters']
    # The node powers, by their tallies (an action's `power` is an index, not a name).
    powers = {k: c.get('ai.power.' + k, 0) for k in ('builder.buy_in', 'protector.accord', 'researcher.peer_review')}
    return {'won': 1.0 if g['won'] else 0.0, 'rounds': g['rounds'], 'node_loss': 1.0 if g['reason'] == 'node' else 0.0,
            'std': {k[10:]: v for k, v in c.items() if k.startswith('board.std.')}, 'powers': powers,
            'played': sum(v.get('played', 0) for v in g['cards'].values())}


def mean(xs):
    return sum(xs) / len(xs) if xs else float('nan')


def se(xs):
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / max(1, len(xs) - 1) / max(1, len(xs)))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('jobs')
    ap.add_argument('probe')
    ap.add_argument('--md', default='')
    a = ap.parse_args()
    arms = defaultdict(dict)
    for g in rows(a.jobs):
        v, lv = divmod(g['cell'], 4)
        arm, pair = divmod(v, 3)
        arms[JOB_ARMS[arm]][(int(g['seed']), pair, lv)] = brief(g)
    seeds = {k[0] for arm in arms.values() for k in arm}
    base = {'a': {}, 'b': {}}
    for g in rows(a.probe):
        if int(g['seed']) not in seeds:
            continue
        v, lv = divmod(g['cell'], 4)
        arm, pair = divmod(v, 3)
        if arm < 2:
            base['ab'[arm]][(int(g['seed']), pair, lv)] = brief(g)
    out = []
    p = out.append
    p(f"Seeds {min(seeds)}-{max(seeds)} ({len(seeds)}), {len(base['a'])} games an arm, paired with p248's a and b.\n")
    for kind, ref, title in (('-', 'a', 'Each action off alone, against arm a'), ('b+', 'b', 'Each action back alone into arm b, against arm b')):
        r = base[ref]
        p(f"## {title}\n")
        p(f"Arm {ref}: wins {mean([x['won'] for x in r.values()]) * 100:.1f}% (d0-d3 " + " / ".join(
            f"{mean([x['won'] for k, x in r.items() if k[2] == lv]) * 100:.1f}" for lv in range(4)) +
          f"), node-loss ends {mean([x['node_loss'] for x in r.values()]) * 100:.1f}%, rounds {mean([x['rounds'] for x in r.values()]):.2f}\n")
        p("| arm | its uses a game in a | wins all (gap ± SE) | d0 | d1 | d2 | d3 | node-loss ends | rounds | cards played a game |")
        p("|---|---|---|---|---|---|---|---|---|---|")
        for name in JOB_ARMS:
            if not name.startswith(kind):
                continue
            arm = arms[name]
            ks = [k for k in arm if k in r]
            job = name[len(kind):]
            uses = mean([base['a'][k]['std'].get(ACT.get(job, job), 0) for k in ks if k in base['a']])
            d = [(arm[k]['won'] - r[k]['won']) * 100 for k in ks]
            lv_gaps = []
            for lv in range(4):
                dl = [(arm[k]['won'] - r[k]['won']) * 100 for k in ks if k[2] == lv]
                lv_gaps.append(f"{mean(dl):+.1f}")
            p(f"| {name} | {uses:.2f} | {mean([arm[k]['won'] for k in ks]) * 100:.1f} ({mean(d):+.1f} ± {se(d):.1f}) | " +
              " | ".join(lv_gaps) + f" | {mean([arm[k]['node_loss'] for k in ks]) * 100:.1f} | {mean([arm[k]['rounds'] for k in ks]):.2f} | "
              f"{mean([arm[k]['played'] for k in ks]):.1f} (ref {mean([r[k]['played'] for k in ks]):.1f}) |")
        p("")
    # What replaces Place a node when it is off: the boards' node powers, a game.
    if '-node' in arms:
        p("## With Place a node off: node powers a game (arm a → -node)\n")
        ks = [k for k in arms['-node'] if k in base['a']]
        names = sorted({n for k in ks for n in list(arms['-node'][k]['powers']) + list(base['a'][k]['powers'])})
        for n in names:
            x0 = mean([base['a'][k]['powers'].get(n, 0) for k in ks])
            x1 = mean([arms['-node'][k]['powers'].get(n, 0) for k in ks])
            p(f"- {n}: {x0:.2f} → {x1:.2f}")
    text = "\n".join(out)
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text + "\n")


main()
