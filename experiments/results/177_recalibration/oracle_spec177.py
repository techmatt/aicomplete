"""177: the oracle row's spec beside the fresh read: `skilled_rows.py`'s companion of s177-fa at its first seed.

    python experiments/results/177_recalibration/oracle_spec177.py FIRST_SEED

Writes experiments/results/177_recalibration/oracle177.json (d3 and d4, 8 seeds a pair from FIRST_SEED, the
strategist and the oracle, the end-game probe on every loss). Run it locally:
`aic-sim experiment experiments/results/177_recalibration/oracle177.json --threads 3 --out out/oracle177`.
"""
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))


def main():
    first = int(sys.argv[1])
    with open(os.path.join(ROOT, 'experiments', 'farm', 's177-fa.json'), encoding='utf-8') as f:
        read = json.load(f)
    read['first_seed'] = first
    read['name'] = 's177-oracle'
    tmp = os.path.join(ROOT, 'out', 's177-oracle-read.json')
    with open(tmp, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(read, f, indent=2)
    out = os.path.join(HERE, 'oracle177.json')
    subprocess.run([sys.executable, os.path.join(ROOT, 'scripts', 'bench', 'skilled_rows.py'), tmp, '--out', out],
                   check=True, cwd=ROOT)


if __name__ == '__main__':
    main()
