"""206: insight by board, a seat-round: cards kept, insight available (the track's total plus income, and the
Researcher's Patience bank), insight spent (keeps and plays), its unspent share, and the share of seat-rounds where
insight bound (`cards.insight_bound.<seat>.rNN`); the table's keep and play insight split (`cards.insight_kept`,
`cards.insight_played`, all seats).

    python experiments/results/206_pricing_powers_insight/insight206.py LABEL=DIR [LABEL=DIR ...]
"""
import json
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
INSIGHT = 1


def main():
    print('| read | board | seat-rounds | kept a seat-round | insight available | spent (keeps + plays) | unspent share | bound share |')
    print('|---|---|---|---|---|---|---|---|')
    split = []
    for arg in sys.argv[1:]:
        label, d = arg.split('=', 1)
        acc = defaultdict(lambda: defaultdict(float))
        kept_i = played_i = 0
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                boards = [p.split(':')[0] for p in g['build'].split('|')]
                c = g['counters']
                kept_i += c.get('cards.insight_kept', 0)
                played_i += c.get('cards.insight_played', 0)
                for s, b in enumerate(boards):
                    if s >= len(g['keeps']):
                        continue
                    k = g['keeps'][s]
                    rounds = len(k['insight'])
                    a = acc[b]
                    a['sr'] += rounds
                    a['kept'] += k['kept']
                    a['avail'] += sum(k['insight'])
                    a['spent'] += g['spent'][s][INSIGHT]
                    a['bound'] += sum(v for kk, v in c.items() if kk.startswith(f'cards.insight_bound.{s}.r'))
        for b in sorted(acc):
            a = acc[b]
            sr = a['sr'] or 1
            print(f"| {label} | {b} | {int(a['sr'])} | {a['kept'] / sr:.2f} | {a['avail'] / sr:.2f} | {a['spent'] / sr:.2f} | "
                  f"{1 - a['spent'] / a['avail'] if a['avail'] else 0:.2f} | {a['bound'] / sr:.2f} |")
        split.append((label, kept_i, played_i))
    print('\n| read | insight to keeps | insight to plays |')
    print('|---|---|---|')
    for label, k, p in split:
        t = (k + p) or 1
        print(f'| {label} | {100 * k / t:.0f}% | {100 * p / t:.0f}% |')


main()
