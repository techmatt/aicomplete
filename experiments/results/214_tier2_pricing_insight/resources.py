"""214: each resource gained / spent a seat-game per read (213's RES order in `gained` and `spent`).

    python experiments/results/214_tier2_pricing_insight/resources.py LABEL=FILE[=SPEC] ...
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
print('| read | ' + ' | '.join(RES) + ' |')
print('|---|' + '---|' * len(RES))
for arg in sys.argv[1:]:
    label, path = arg.split('=')[:2]
    g_sum = [0.0] * len(RES)
    s_sum = [0.0] * len(RES)
    seats = 0
    for ln in open(path, encoding='utf-8'):
        g = json.loads(ln)
        if 'format' in g:
            continue
        for gs, ss in zip(g['gained'], g['spent']):
            seats += 1
            for k in range(len(RES)):
                g_sum[k] += gs[k]
                s_sum[k] += ss[k]
    print(f'| {label} | ' + ' | '.join(f'{g_sum[k] / seats:.1f} / {s_sum[k] / seats:.1f}' for k in range(len(RES))) + ' |')
