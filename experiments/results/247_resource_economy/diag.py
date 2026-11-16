"""Diagnose resource use in a read's games.jsonl: per board, per round held after actions,
gained / spent, and top uses / sources of energy, antimatter, chips."""
import json, sys
from collections import defaultdict
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PEAKS = ["credits", "insight", "energy", "chips", "antimatter", "research_cubes", "card_tokens", "ambassadors"]
path = sys.argv[1]
want = sys.argv[2].split(',') if len(sys.argv) > 2 else ["energy", "antimatter", "chips"]
n = defaultdict(int)
held = defaultdict(float)  # (board, res, round) -> sum
heldn = defaultdict(int)
gained = defaultdict(float)
spent = defaultdict(float)
uses = defaultdict(float)
srcs = defaultdict(float)
seats = defaultdict(int)
ending = defaultdict(float)
with open(path, encoding='utf-8') as f:
    f.readline()
    for line in f:
        g = json.loads(line)
        boards = [b.split(':')[0] for b in g['build'].split('|')]
        c = g['counters']
        for s, b in enumerate(boards):
            seats[b] += 1
            seats['all'] += 1
            for i, r in enumerate(PEAKS):
                if r not in want:
                    continue
                for bb in (b, 'all'):
                    gained[(bb, r)] += g['gained'][s][i]
                    spent[(bb, r)] += g['spent'][s][i]
                for k, v in g['uses'][s].items():
                    if v[i]:
                        uses[(b, r, k)] += v[i]
                        uses[('all', r, k)] += v[i]
                for k, v in g['sources'][s].items():
                    if v[i]:
                        srcs[(b, r, k)] += v[i]
                        srcs[('all', r, k)] += v[i]
                for rd in range(1, 13):
                    k = f"unspent.{s}.{r}.r{rd:02d}"
                    if k in c:
                        for bb in (b, 'all'):
                            held[(bb, r, rd)] += c[k]
                            heldn[(bb, r, rd)] += 1
            lu = g['margin']['held_after_actions'][s]
            for i, r in enumerate(["credits", "chips", "energy", "antimatter", "ambassadors"]):
                if r in want:
                    ending[(b, r)] += lu[i]
                    ending[('all', r)] += lu[i]
for b in ['builder', 'protector', 'researcher', 'all']:
    if not seats[b]:
        continue
    print(f"\n## {b} ({seats[b]} seat-games)")
    for r in want:
        G = gained[(b, r)] / seats[b]
        S = spent[(b, r)] / seats[b]
        print(f"{r}: gained {G:.2f} spent {S:.2f} share {S / G if G else 0:.0%}; held after last action phase {ending[(b, r)] / seats[b]:.2f}")
        row = []
        for rd in range(1, 12):
            if heldn[(b, r, rd)]:
                row.append(f"r{rd}:{held[(b, r, rd)] / heldn[(b, r, rd)]:.1f}")
        print("  held after actions by round:", ' '.join(row))
        us = sorted(((v, k[2]) for k, v in uses.items() if k[0] == b and k[1] == r), reverse=True)[:14]
        print("  uses:", ', '.join(f"{k} {v / seats[b]:.2f}" for v, k in us))
        ss = sorted(((v, k[2]) for k, v in srcs.items() if k[0] == b and k[1] == r), reverse=True)[:14]
        print("  sources:", ', '.join(f"{k} {v / seats[b]:.2f}" for v, k in ss))
