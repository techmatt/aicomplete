"""128: why building Actions go unused. As a seat passes its action phase, each standing building with an Action
was offered and skipped, or not offered (used this round already, unaffordable, or no target); with the energy held.

    python experiments/results/128_draft_and_price/action_read.py DIR [--cards a,b]

Reads the `actions.skipped.<seat>.<card>` / `actions.blocked.<seat>.<card>` counters (and `_energy` sums) the
experiment writes, and the `use` actions in the stream. Per board: passes with the building standing, uses, skipped,
blocked less used (blocked for want of energy or a target), and the mean energy held at a skip or a block.
"""
import argparse
import collections
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import read128 as R  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--cards', default=','.join(R.SINKS))
    a = ap.parse_args()
    cards = a.cards.split(',')
    t = collections.defaultdict(lambda: collections.Counter())
    for label, pair, g in R.load(a.dir):
        c = g['counters']
        uses = collections.Counter((x['seat'], x['card']) for x in g['actions'] if x['type'] == 'use')
        for s in range(len(g['peaks'])):
            b = R.BOARD[pair[s]]
            for card in cards:
                k = t[(label, b, card)]
                sk, bl = c.get(f'actions.skipped.{s}.{card}', 0), c.get(f'actions.blocked.{s}.{card}', 0)
                k['passes'] += sk + bl
                k['skipped'] += sk
                k['blocked'] += bl
                k['used'] += uses[(s, card)]
                k['e_skip'] += c.get(f'actions.skipped_energy.{s}.{card}', 0)
                k['e_block'] += c.get(f'actions.blocked_energy.{s}.{card}', 0)
    print('| run | board | card | rounds standing | used | skipped (energy held) | blocked, not used (energy held) |')
    print('|---|---|---|---|---|---|---|')
    for (label, b, card), k in sorted(t.items()):
        if not k['passes']:
            continue
        es = k['e_skip'] / k['skipped'] if k['skipped'] else 0
        eb = k['e_block'] / k['blocked'] if k['blocked'] else 0
        print(f"| {label} | {b} | {card} | {k['passes']} | {k['used']} | {k['skipped']} ({es:.1f}) | "
              f"{k['blocked'] - k['used']} ({eb:.1f}) |")


if __name__ == '__main__':
    main()
