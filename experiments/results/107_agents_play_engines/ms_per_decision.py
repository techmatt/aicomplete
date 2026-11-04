"""107: agent ms per decision per cell from an experiment's log (agent ms/game) and games.jsonl (decisions).

    python experiments/results/107_agents_play_engines/ms_per_decision.py LOG DIR [LOG DIR]

With two runs, prints the second over the first, cell by cell and pooled by agent (decision-weighted).
Both runs on the same seeds and threads, one after the other on a quiet box.
"""
import json
import re
import sys
from collections import defaultdict


def read(log, d):
    ms = {}
    for line in open(log, encoding='utf-8'):
        m = re.match(r'\s+(\S+) d\d (\S+)\s+win .* agent\s+([\d.]+) ms/game', line)
        if m:
            ms[(m[1], m[2])] = float(m[3])
    lines = open(f'{d}/games.jsonl', encoding='utf-8').read().splitlines()
    spec = json.loads(lines[0])['spec']
    cells = [(v['name'], a) for v in spec['variants'] for _ in spec['difficulties'] for a in spec['agents']]
    dec = defaultdict(list)
    for line in lines[1:]:
        g = json.loads(line)
        dec[cells[g['cell']]].append(g['decisions'])
    out = {}
    for (v, a), ds in dec.items():
        label = next(k for k in ms if k[0] == v and (k[1] == a or k[1].startswith('engine:strategy=1') and a == 'strategist'))
        n = sum(ds) / len(ds)
        out[(v, a)] = (ms[label], n)
    return out


runs = [read(sys.argv[i], sys.argv[i + 1]) for i in range(1, len(sys.argv) - 1, 2)]
for (v, a), (ms, n) in sorted(runs[0].items()):
    line = f'{v} {a}: {ms / n:.3f} ms/decision ({n:.0f} decisions)'
    if len(runs) > 1:
        ms2, n2 = runs[1][(v, a)]
        line += f' -> {ms2 / n2:.3f} ({n2:.0f}); ratio {(ms2 / n2) / (ms / n):.2f}'
    print(line)
if len(runs) > 1:
    for a in sorted({a for _, a in runs[0]}):
        t = [sum(r[k][0] for k in r if k[1] == a) / sum(r[k][1] for k in r if k[1] == a) for r in runs]
        print(f'{a} pooled: {t[0]:.3f} -> {t[1]:.3f} ms/decision, ratio {t[1] / t[0]:.2f}')
