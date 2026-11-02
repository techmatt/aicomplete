"""Close a gate: carry every check within 2 SE of its agent on to the cap, reusing its games (068).

    python scripts/bench/gate_close.py plan GATE_DIR SPEC OUT_DIR [--z 2] [--cap 96] [--fill 48]
    python scripts/bench/gate_close.py read GATE_DIR CLOSE_DIR... [--z 2]

`plan` reads a gate's results.json, keeps each row with |gap| under z paired SEs whatever its
`stopped` reason (a check stopped on a settled sign can still be close), and writes specs to
OUT_DIR, with no early stop: first each kept check from where it stopped up to --fill seeds,
grouped by that count (one spec per variant and count), then --fill to --cap for those checks and
the agent. Its `run.txt` lists each spec with its `--seed`, in order; `first_seed` in a spec is that
seed. `read` merges games.jsonl of the gate and the close runs by (variant, agent, seed) and prints
each check's gap to its agent on the seeds both played, with the paired SE.
"""
import argparse
import collections
import json
import math
import os
import sys


def rows(gate_dir):
    with open(os.path.join(gate_dir, 'results.json'), encoding='utf-8') as f:
        return json.load(f)['gate']['rows']


def spec_of(gate_dir):
    with open(os.path.join(gate_dir, 'games.jsonl'), encoding='utf-8') as f:
        return json.loads(f.readline())['spec']


def plan(a):
    spec = spec_of(a.gate)
    os.makedirs(a.out, exist_ok=True)
    keep = [r for r in rows(a.gate) if abs(r['gap']) < a.z * max(r['paired_se'], 1e-9)]
    print(f'{len(keep)} of {len(rows(a.gate))} checks within {a.z} SE:')
    for r in sorted(keep, key=lambda r: (r['variant'], r['games'])):
        print(f"  {r['variant']} {r['check']}: {r['games']} seeds, gap {100 * r['gap']:+.0f}, paired se {100 * r['paired_se']:.0f}, {r['decided']}")
    first = int(spec['first_seed'])
    variants = {v['name']: v for v in spec['variants']}
    run = []

    def write(name, variant, agents, start, games):
        s = dict(spec)
        s.pop('gate', None)
        s.update({'name': name, 'note': f'gate close (068): {variant}, seeds {start}+{games}', 'variants': [variants[variant]],
                  'agents': agents, 'games': games, 'first_seed': str(start), 'records': 0})
        path = os.path.join(a.out, f'{name}.json')
        with open(path, 'w', encoding='utf-8') as f:
            json.dump(s, f, indent=1)
        run.append(f'{name} {start} {games}')

    groups = collections.defaultdict(list)
    for r in keep:
        if r['games'] < a.fill:
            groups[(r['variant'], r['games'])].append(r['check'])
    for (v, n), checks in sorted(groups.items()):
        write(f'fill-{v}-{n}', v, checks, first + n, a.fill - n)
    for v in sorted({r['variant'] for r in keep}):
        checks = [r['check'] for r in keep if r['variant'] == v]
        agent = sorted({r['agent'] for r in keep if r['variant'] == v})
        write(f'next-{v}', v, agent + checks, first + a.fill, a.cap - a.fill)
    with open(os.path.join(a.out, 'run.txt'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(run) + '\n')
    print(f'{len(run)} specs in {a.out}')


def read(a):
    games = {}
    for d in [a.gate] + a.close:
        path = os.path.join(d, 'games.jsonl')
        if not os.path.exists(path):
            continue
        with open(path, encoding='utf-8') as f:
            head = json.loads(f.readline())
            spec = head['spec']
            cells = [(v['name'], ag) for v in spec['variants'] for ag in spec['agents']]
            for line in f:
                try:
                    g = json.loads(line)
                except json.JSONDecodeError:
                    continue
                v, ag = cells[g['cell']]
                games.setdefault((v, ag), {})[g['seed']] = g['won']
    print('| variant | check | seeds | win % | gap to its agent | paired se | gap / se |')
    print('|---|---|---|---|---|---|---|')
    for (v, ag), won in sorted(games.items()):
        if ':' not in ag:
            continue
        base = games.get((v, ag.split(':')[0]), {})
        seeds = set(won) & set(base)
        if len(seeds) < 2:
            continue
        d = [base[s] - won[s] for s in seeds]
        n = len(d)
        m = sum(d) / n
        se = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1) / n)
        q = sum(won[s] for s in seeds) / n
        flag = ' X' if m <= 0 else ''
        print(f'| {v} | {ag} | {n} | {100 * q:.0f} | {100 * m:+.1f}{flag} | {100 * se:.1f} | {m / se if se else 0:+.1f} |')


def main():
    sys.stdout.reconfigure(encoding='utf-8')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest='cmd', required=True)
    p = sub.add_parser('plan')
    p.add_argument('gate')
    p.add_argument('out')
    p.add_argument('--z', type=float, default=2.0)
    p.add_argument('--cap', type=int, default=96)
    p.add_argument('--fill', type=int, default=48)
    r = sub.add_parser('read')
    r.add_argument('gate')
    r.add_argument('close', nargs='*')
    a = ap.parse_args()
    (plan if a.cmd == 'plan' else read)(a)


if __name__ == '__main__':
    main()
