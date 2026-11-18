"""271: each changed card's value check: its cost and the effect ledger's reasoned value (values.json's rates),
before and after the overlay. Run from the repo root.

    python experiments/results/271_draw_economy/ledger_diff.py BASE_CARDS_JSON OVERLAY > ledger.md

make_overlay.py moved each changed card's cost by its change in reasoned value (floor 3), so `cost - reasoned`
holds unless the floor or a hand price (the three engines) set it.
"""
import csv
import io
import subprocess
import sys


def ledger(args):
    out = subprocess.run([sys.executable, 'scripts/cards/effect_ledger.py', '--stdout'] + args,
                         capture_output=True, text=True, encoding='utf-8', check=True).stdout
    text = out[out.index('card,name'):]
    text = text[:text.index('\n\n')] if '\n\n' in text else text
    return {r['card']: r for r in csv.DictReader(io.StringIO(text))}


def main():
    base, overlay = sys.argv[1], sys.argv[2]
    before = ledger(['--cards', base])
    after = ledger(['--cards', base, '--overlay', overlay])
    changed = {line.split('.')[1] for line in open(overlay, encoding='utf-8') if line.startswith('cards.')}
    print('| Card | Cost before | Reasoned before | Cost after | Reasoned after | Cost - reasoned, before / after |')
    print('|---|---|---|---|---|---|')
    for cid in sorted(changed):
        b, a = before[cid], after[cid]
        gb = float(b['cost']) - float(b['reasoned'])
        ga = float(a['cost']) - float(a['reasoned'])
        print(f"| {a['name']} | {b['cost']} | {b['reasoned']} | {a['cost']} | {a['reasoned']} | {gb:+.1f} / {ga:+.1f} |")


if __name__ == '__main__':
    main()
