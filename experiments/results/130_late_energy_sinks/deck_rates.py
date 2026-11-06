"""130: every deck card's keep and play rate a game (both seats), and its plays per card reaching a hand (kept or
drawn), for one agent; the energy-cost cards marked, against the medians of their tier and kind.

    python experiments/results/130_late_energy_sinks/deck_rates.py DIR [--agent strategist] [--all]

Reads each game's `cards` record (offered / drafted / kept / played / drawn, summed over the seats).
"""
import argparse
import collections
import json
import os
import statistics
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '128_draft_and_price'))
import read128 as R  # noqa: E402

ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
DECK = json.load(open(os.path.join(ROOT, 'configs', 'baseline.json'), encoding='utf-8'))


def deck_ids():
    for m in DECK['modules'] if isinstance(DECK.get('modules'), list) else []:
        pass
    text = json.dumps(DECK)
    return [c for c in R.CARDS if f'"{c}"' in text]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--all', action='store_true', help='print every card, not only the energy-cost ones')
    a = ap.parse_args()
    tot = collections.defaultdict(collections.Counter)
    ng = 0
    for label, pair, g in R.load(a.dir):
        if not label.endswith('/' + a.agent):
            continue
        ng += 1
        for c, v in g['cards'].items():
            tot[c].update(v)
    deck = deck_ids()
    rows = {}
    for c in deck:
        v = tot[c]
        reach = v['kept'] + v['drawn']
        rows[c] = (v['kept'] / ng, v['played'] / ng, v['played'] / reach if reach else 0)
    groups = collections.defaultdict(list)
    for c in deck:
        d = R.CARDS[c]
        groups[(d['tier'], d['kind'])].append(rows[c][2])
    print(f'{ng} games, agent {a.agent}. Medians of plays per card reaching a hand, by tier and kind:')
    for k, xs in sorted(groups.items()):
        print(f'- tier {k[0]} {k[1]}: {statistics.median(xs):.2f} ({len(xs)} cards)')
    print('\n| card | tier kind | energy | kept a game | played a game | plays per card in hand | group median |')
    print('|---|---|---|---|---|---|---|')
    for c in sorted(deck, key=lambda c: (-R.CARDS[c].get('energy', 0), c)):
        d = R.CARDS[c]
        if not a.all and not d.get('energy'):
            continue
        k, p, r = rows[c]
        print(f"| {c} | t{d['tier']} {d['kind']} | {d.get('energy', 0)} | {k:.2f} | {p:.2f} | {r:.2f} | "
              f"{statistics.median(groups[(d['tier'], d['kind'])]):.2f} |")


if __name__ == '__main__':
    main()
