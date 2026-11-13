"""213: each resource gained and spent a seat-game, per arm (read213.py's loader).

    python experiments/results/213_tier2_design/resources.py LABEL=PATH[:LEVER] ...
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
src = open(os.path.join(HERE, 'read213.py'), encoding='utf-8').read().replace('\nmain()\n', '\n')
ns = {'__file__': os.path.join(HERE, 'read213.py')}
exec(src, ns)
RES = ns['RES']
print('| arm | ' + ' | '.join(RES) + ' |')
print('|---|' + '---|' * len(RES))
for a in sys.argv[1:]:
    label, _, rest = a.partition('=')
    i = rest.rfind(':')
    path, lever = (rest[:i], rest[i + 1:]) if i > 1 else (rest, None)
    gs = ns['load'](path, lever)
    seats = sum(len(g['gained']) for g in gs)
    cells = []
    for k in range(len(RES)):
        gained = sum(s[k] for g in gs for s in g['gained']) / seats
        spent = sum(s[k] for g in gs for s in g['spent']) / seats
        cells.append(f'{gained:.1f} / {spent:.1f}')
    print(f'| {label} | ' + ' | '.join(cells) + ' |')
