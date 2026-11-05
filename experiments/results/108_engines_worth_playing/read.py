"""108: the ballpark read, r42 against r43 on the same seeds (experiments/screen-108-r43.json, 33692-33739).

    python read.py R42_DIR R43_DIR

Per pair and agent: win rate r42 → r43 and the paired gap (game by game on the same seed) ± its SE; rounds; then
credits (and chips) left at game end per seat, by board (`margin.unspent`, what a seat held when its last action phase
ended).
"""
import io
import json
import math
import sys
from collections import defaultdict

BOARDS = {'bp': ('Builder', 'Protector'), 'br': ('Builder', 'Researcher'), 'pr': ('Protector', 'Researcher')}


def load(d):
    lines = open(f'{d}/games.jsonl', encoding='utf-8').read().splitlines()
    spec = json.loads(lines[0])['spec']
    cells = [(v['name'], a) for v in spec['variants'] for _ in spec['difficulties'] for a in spec['agents']]
    out = {}
    for ln in lines[1:]:
        g = json.loads(ln)
        if 'cell' in g:
            v, a = cells[g['cell']]
            out[(v, a, int(g['seed']))] = g
    return out


def main():
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    a, b = load(sys.argv[1]), load(sys.argv[2])
    agents = sorted({k[1] for k in a})
    print('| agent | pair | win r42 → r43 | gap ± SE | rounds r42 → r43 |')
    print('|---|---|---|---|---|')
    for ag in agents:
        for pair in ('bp', 'br', 'pr', None):
            keys = [k for k in a if k[1] == ag and (pair is None or k[0] == pair) and k in b]
            d = [float(b[k]['won']) - float(a[k]['won']) for k in keys]
            n = len(d)
            m = sum(d) / n
            se = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1) / n)
            wa = 100 * sum(a[k]['won'] for k in keys) / n
            wb = 100 * sum(b[k]['won'] for k in keys) / n
            ra = sum(a[k]['rounds'] for k in keys) / n
            rb = sum(b[k]['rounds'] for k in keys) / n
            label = pair or '**pooled**'
            print(f'| {ag} | {label} | {wa:.1f} → {wb:.1f} | {100 * m:+.1f} ±{100 * se:.1f} | {ra:.2f} → {rb:.2f} |')
    print()
    print('Left at game end, a seat (credits / chips), r42 → r43:')
    print()
    print('| agent | pair | board | credits | chips |')
    print('|---|---|---|---|---|')
    for ag in agents:
        for pair in ('bp', 'br', 'pr'):
            for seat in (0, 1):
                vals = []
                for run in (a, b):
                    xs = [g['margin']['unspent'][seat] for k, g in run.items() if k[1] == ag and k[0] == pair]
                    vals.append((sum(x[0] for x in xs) / len(xs), sum(x[1] for x in xs) / len(xs)))
                print(f'| {ag} | {pair} | {BOARDS[pair][seat]} | {vals[0][0]:.1f} → {vals[1][0]:.1f} | '
                      f'{vals[0][1]:.1f} → {vals[1][1]:.1f} |')


if __name__ == '__main__':
    main()
