"""210: how each resource is spent in a read: per resource, gained and spent a seat-game and its five biggest uses
(games.jsonl `uses` and `sources`, a list per seat of {source: amounts in PEAKS order}; own and other pooled),
levels pooled.

    python experiments/results/210_settle_r91/spend210.py DIR
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
path = sys.argv[1] if sys.argv[1].endswith('.jsonl') else os.path.join(sys.argv[1], 'games.jsonl')
uses = defaultdict(lambda: defaultdict(float))
gained = defaultdict(float)
seats = 0
with open(path, encoding='utf-8') as f:
    f.readline()
    for line in f:
        g = json.loads(line)
        seats += len(g['uses'])
        for per_seat, acc in ((g['uses'], uses), (g['sources'], None)):
            for seat in per_seat:
                for k, amounts in seat.items():
                    k = k.split(':', 1)[1] if k.startswith(('own:', 'other:')) else k
                    for r, v in zip(PEAKS, amounts):
                        if acc is None:
                            gained[r] += v
                        else:
                            acc[r][k] += v
print('| resource | gained a seat-game | spent a seat-game | by kind (share of spent) | biggest single uses |')
print('|---|---|---|---|---|')
for r in PEAKS:
    tot = sum(uses[r].values())
    if not tot:
        continue
    kinds = defaultdict(float)
    for k, v in uses[r].items():
        kinds[k.split(':', 1)[0]] += v
    kind = sorted(kinds.items(), key=lambda kv: -kv[1])[:5]
    top = sorted(uses[r].items(), key=lambda kv: -kv[1])[:4]
    print(f"| {r} | {gained[r] / seats:.2f} | {tot / seats:.2f} | "
          + ', '.join(f'{k} {100 * v / tot:.0f}%' for k, v in kind) + ' | '
          + ', '.join(f'{k} {100 * v / tot:.0f}%' for k, v in top) + ' |')
