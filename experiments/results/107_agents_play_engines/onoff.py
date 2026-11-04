"""107: each fix's on - off gap, pooled over the three pairs, from scripts/bench/paired.py.

    python experiments/results/107_agents_play_engines/onoff.py ON_DIR/AGENT OFF_DIR/AGENT [...]

Each argument is DIR/AGENT; the first is the base (fix on), every other is read as base minus it, pooled
over bp, br and pr on the seeds both played.
"""
import os
import subprocess
import sys


def split(x):
    # DIR/AGENT, where AGENT may hold no '/' and DIR may.
    d, a = x.rsplit('/', 1)
    return d, a


base, *others = sys.argv[1:]
bdir, bagent = split(base)
for o in others:
    odir, oagent = split(o)
    args = ['python', 'scripts/bench/paired.py', bdir] + ([odir] if odir != bdir else [])
    bn, on = os.path.basename(bdir), os.path.basename(odir)
    for v in ('bp', 'br', 'pr'):
        args += ['--pair', f'{on}/{v}/{oagent}', f'{bn}/{v}/{bagent}']
    out = subprocess.run(args, capture_output=True, encoding='utf-8').stdout
    pooled = [line for line in out.splitlines() if line.startswith('pooled')]
    print(f'{base} minus {o}: {pooled[0] if pooled else out[-400:]}')
