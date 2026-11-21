"""309: 278's traces.sh at most 6 games at a time (the box's thread cap): 268's seeds 268000-268013 at d2 and d4,
the judge, traced into OUT/d<L>-<S>.out, and the same games as an experiment in OUT/exp (per-round counters).

    python experiments/results/309_rulebook_friction_refresh/traces.py OUT [k=v ...]
"""
import concurrent.futures
import json
import os
import subprocess
import sys

EXE = os.path.join('target', 'release', 'aic-sim.exe')
SEEDS = range(268000, 268014)


def run(level, seed, sets, out):
    cmd = [EXE, 'run', 'configs/baseline.json', '--games', '1', '--seed', str(seed), '--agent', 'strategist',
           '--set', f'adversary.difficulty={level}'] + sets + ['--trace']
    with open(os.path.join(out, f'd{level}-{seed}.out'), 'w', encoding='utf-8', newline='\n') as f:
        subprocess.run(cmd, stdout=f, stderr=subprocess.STDOUT, check=True)


def main():
    out = sys.argv[1]
    kvs = [kv for kv in sys.argv[2:] if kv]
    sets = [x for kv in kvs for x in ('--set', kv)]
    os.makedirs(out, exist_ok=True)
    with concurrent.futures.ThreadPoolExecutor(6) as ex:
        list(ex.map(lambda a: run(*a, sets, out), [(lv, s) for lv in (2, 4) for s in SEEDS]))
    spec = {'name': 's309-trace', 'configs': ['configs/baseline.json'], 'variants': [{'name': 'v', 'set': kvs}],
            'difficulties': [2, 4], 'agents': ['strategist'], 'games': len(SEEDS), 'first_seed': SEEDS[0], 'records': 0}
    with open(os.path.join(out, 'spec.json'), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f)
    with open(os.path.join(out, 'exp.log'), 'w', encoding='utf-8', newline='\n') as f:
        subprocess.run([EXE, 'experiment', os.path.join(out, 'spec.json'), '--out', os.path.join(out, 'exp'),
                        '--threads', '6'], stdout=f, stderr=subprocess.STDOUT, check=True)
    print('traced', out)


if __name__ == '__main__':
    main()
