"""162: a farmed run's ms a game by agent and level, from the main box's shard log (farm lines carry no timing).

python experiments/results/162_judge_purge/cost162.py SPEC MAIN_LOG
MAIN_LOG: the job's `main.log` under conductor's remote jobs folder (lines `farm: cell C seed S MS ms`).
"""
import collections
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cells_of  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))


def main():
    spec, log = sys.argv[1:3]
    exe = os.path.join(ROOT, 'target', 'release', 'aic-sim.exe')
    head = json.loads(subprocess.run([exe, 'farm', spec, '--header', '--seed', '0', '--games', '1'], cwd=ROOT,
                                     check=True, capture_output=True, text=True, encoding='utf-8').stdout)
    cells = cells_of(head['spec'])
    t = collections.defaultdict(list)
    for line in open(log, encoding='utf-8', errors='replace'):
        m = re.match(r'farm: cell (\d+) seed (\d+) ([\d.]+) ms', line.strip())
        if m:
            c = cells[int(m.group(1))]
            t[(c['agent'], c['difficulty'])].append(float(m.group(3)))
    print('| Agent | Level | Games timed | s a game (mean) | s a game (max) |')
    print('|---|---|---|---|---|')
    for (a, d), v in sorted(t.items()):
        short = a if len(a) <= 40 else a[:24] + '...' + a[-16:]
        print(f'| {short} | d{d} | {len(v)} | {sum(v) / len(v) / 1000:.1f} | {max(v) / 1000:.1f} |')


if __name__ == '__main__':
    main()
