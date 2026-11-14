"""234: the effect ledger's reasoned value (values.json) of each converted card, before (git REV) and after (the
working tree's experiments/results/effect-ledger/cards.csv): what the conversion took or gave, at the table.

    python experiments/results/234_card_targets/ledger_diff.py [--rev aeced1d321] [--out FILE]
"""
import argparse
import csv
import io
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
LEDGER = 'experiments/results/effect-ledger/cards.csv'


def rows(text):
    return {r['card']: r for r in csv.DictReader(io.StringIO(text))}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--rev', default='aeced1d321')
    ap.add_argument('--out', default='')
    a = ap.parse_args()
    old = rows(subprocess.run(['git', 'show', f'{a.rev}:{LEDGER}'], cwd=ROOT, capture_output=True, text=True,
                              encoding='utf-8', check=True).stdout)
    new = rows(open(os.path.join(ROOT, LEDGER), encoding='utf-8').read())
    cards = json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))
    targeted = [c['id'] for c in cards if '"op": "target"' in json.dumps(c) or '"reach": true' in json.dumps(c)
                and '"shelter"' in json.dumps(c)]
    lines = ['| card | tier | cost | reasoned before | after | change | ratio before | after |', '|---|---|---|---|---|---|---|---|']
    for cid in targeted:
        o, n = old.get(cid), new.get(cid)
        if not o or not n:
            continue
        rb, ra = float(o['reasoned']), float(n['reasoned'])
        lines.append(f"| {cid} | {n['tier']} | {n['cost']} | {rb:.1f} | {ra:.1f} | {ra - rb:+.1f} | {o['ratio']} | {n['ratio']} |")
    text = '\n'.join(lines) + '\n'
    if a.out:
        with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)
    print(text)


main()
