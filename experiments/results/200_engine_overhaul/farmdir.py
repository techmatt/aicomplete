"""200: a farmed out file as a read directory (DIR/games.jsonl: the header, then the games), for the readers here.

    python experiments/results/200_engine_overhaul/farmdir.py OUT.jsonl SPEC FIRST COUNT DIR [EXE]

The header comes from `EXE farm SPEC --header --seed FIRST --games COUNT` (EXE: target/release/aic-sim.exe).
"""
import os
import subprocess
import sys

out, spec, first, count, d = sys.argv[1:6]
exe = sys.argv[6] if len(sys.argv) > 6 else os.path.join('target', 'release', 'aic-sim.exe')
head = subprocess.check_output([exe, 'farm', spec, '--header', '--seed', first, '--games', count]).decode('utf-8')
os.makedirs(d, exist_ok=True)
with open(out, encoding='utf-8') as f:
    body = [line for line in f if line.strip() and '"format"' not in line[:40]]
with open(os.path.join(d, 'games.jsonl'), 'w', encoding='utf-8', newline='\n') as f:
    f.write(head.strip() + '\n')
    f.writelines(body)
print(f'{d}/games.jsonl: {len(body)} games')
