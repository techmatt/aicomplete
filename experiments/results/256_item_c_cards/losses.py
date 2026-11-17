"""256: losses by reason at each level, per arm and agent (the d3 drop's split).

    python experiments/results/256_item_c_cards/losses.py READ
"""
import os
import sys
from collections import Counter, defaultdict

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts', 'bench'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from read_lib import games  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
L = defaultdict(Counter)
N = Counter()
for G in games(sys.argv[1]):
    k = (G.agent, G.arm, G.level)
    N[k] += 1
    if not G.g['won']:
        L[k][G.g['reason']] += 1
reasons = sorted({r for c in L.values() for r in c})
print('| agent | arm | level | games | lost | ' + ' | '.join(reasons) + ' |')
print('|---|---|---|---|---|' + '---|' * len(reasons))
for k in sorted(N):
    print(f'| {k[0]} | {k[1]} | d{k[2]} | {N[k]} | {sum(L[k].values())} | ' + ' | '.join(str(L[k][r]) for r in reasons) + ' |')
