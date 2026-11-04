"""107: the strategist's lead over greedy, before and after, and its change ± one SE, paired by pair and seed.

    python experiments/results/107_agents_play_engines/lead.py BEFORE_DIR AFTER_DIR
"""
import json
import math
import sys


def wins(d):
    out = {}
    lines = open(f'{d}/games.jsonl', encoding='utf-8').read().splitlines()
    spec = json.loads(lines[0])['spec']
    cells = [(v['name'], a) for v in spec['variants'] for _ in spec['difficulties'] for a in spec['agents']]
    for line in lines[1:]:
        g = json.loads(line)
        v, a = cells[g['cell']]
        out[(v, a, g['seed'])] = 1.0 if g['won'] else 0.0
    return out


b, a = wins(sys.argv[1]), wins(sys.argv[2])
keys = sorted({(v, s) for (v, ag, s) in b})
diffs = []
for v, s in keys:
    lead_b = b[(v, 'strategist', s)] - b[(v, 'greedy', s)]
    lead_a = a[(v, 'strategist', s)] - a[(v, 'greedy', s)]
    diffs.append((lead_b, lead_a))
n = len(diffs)
for name, i in (('before', 0), ('after', 1)):
    x = [d[i] for d in diffs]
    m = sum(x) / n
    se = math.sqrt(sum((y - m) ** 2 for y in x) / (n - 1) / n)
    print(f'lead {name}: {100 * m:+.1f} ±{100 * se:.1f}')
x = [d[1] - d[0] for d in diffs]
m = sum(x) / n
se = math.sqrt(sum((y - m) ** 2 for y in x) / (n - 1) / n)
print(f'lead change: {100 * m:+.1f} ±{100 * se:.1f} on {n} paired seeds')
