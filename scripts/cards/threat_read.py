"""Who counters what (070), per cell of experiment DIRs, read from games.jsonl: threats
dealt, countered by source type, paid abroad or by ambassador alone, sources made, rounds with every threat countered, and the
typed counters and capstones played.

    python scripts/cards/threat_read.py DIR [DIR ...] [--cards]
"""
import argparse
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
ROOT = Path(__file__).resolve().parents[2]


def typed_and_capstones():
    typed, cap = [], []
    for c in json.loads((ROOT / 'content/cards.json').read_text(encoding='utf-8')):
        ops = c.get('effects', []) + c.get('ability', {}).get('effects', [])
        if any(o['op'] == 'jam' and 'source' in o for o in ops):
            typed.append(c['id'])
        if c['tier'] == 2 and c['kind'] == 'building' and any(
                o['op'] in ('income_per_tag', 'auto_counter', 'project_drain') for o in c.get('passives', [])):
            cap.append(c['id'])
    return typed, cap


def read(d):
    head, cells = None, defaultdict(list)
    with open(Path(d) / 'games.jsonl', encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if head is None:
                head = g
                continue
            g.pop('actions', None)
            cells[g.get('cell', 0)].append(g)
    return head, cells


def label(head, cell, games):
    spec = (head or {}).get('cells') or []
    if isinstance(cell, int) and cell < len(spec):
        c = spec[cell]
        return f"{c.get('variant', '?')} {c.get('agent', '?')}" if isinstance(c, dict) else str(c)
    return str(cell)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--cards', action='store_true')
    a = ap.parse_args()
    typed, cap = typed_and_capstones()
    keys = [('dealt', 'threats.dealt'), ('counter', 'threats.countered'), ('inf', 'threats.countered.influence'),
            ('cyb', 'threats.countered.cyber'), ('eco', 'threats.countered.economic'),
            ('mil', 'threats.countered.military'), ('exec', 'threats.executed'), ('abroad', 'threats.paid_abroad'),
            ('byamb', 'threats.paid_by_ambassador'), ('made', 'threats.sources_made'), ('clean', 'threats.rounds_all_countered'),
            ('typed', 'threats.typed_counters'), ('auto', 'threats.auto_countered')]
    print('| cell | n | win | ' + ' | '.join(k for k, _ in keys) + ' | typed played | capstones played |')
    print('|---|' * 1 + '---|' * (len(keys) + 4))
    for d in a.dirs:
        head, cells = read(d)
        for cell, gs in cells.items():
            n = len(gs)
            cnt = lambda k: sum(g.get('counters', {}).get(k, 0) for g in gs) / n
            pl = lambda ids: sum(cnt(f'cards.played.{i}') for i in ids)
            won = sum(1 for g in gs if g.get('won') or g.get('outcome', {}).get('win')) / n * 100
            print(f"| {label(head, cell, gs)} | {n} | {won:.0f} | " + ' | '.join(f'{cnt(k):.1f}' for _, k in keys)
                  + f' | {pl(typed):.2f} | {pl(cap):.2f} |')
            if a.cards:
                for i in typed + cap:
                    if cnt(f'cards.played.{i}'):
                        print(f"  {i}: {cnt(f'cards.played.{i}'):.2f}")


if __name__ == '__main__':
    main()
