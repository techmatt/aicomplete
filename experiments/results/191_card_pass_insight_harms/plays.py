"""191: plays a game by insight cost and tier, and the repriced node cards' plays, new against old.

    python plays.py NEW_DIR OLD_DIR [--old-rev de6b0aa] > plays.md

Each DIR holds a games.jsonl. A play is `counters['cards.played.<id>']`; a card's insight to play is read from
content/cards.json for NEW and from `git show REV:content/cards.json` for OLD, so each side counts at its own price.
Per game (both seats), pooled over every cell of the run.
"""
import argparse
import json
import os
import subprocess
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
NODE = ['national_grid_buildout', 'beamed_power_constellation', 'relay_mast_rollout', 'aegis_grid_lockdown']


def cards_at(rev):
    if rev is None:
        return json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))
    text = subprocess.run(['git', 'show', f'{rev}:content/cards.json'], cwd=ROOT, capture_output=True,
                          check=True).stdout.decode('utf-8')
    return json.loads(text)


def read(d, cards):
    price = {c['id']: (c['tier'], c['insight']) for c in cards}
    games = 0
    by = defaultdict(float)  # (tier, insight) -> plays
    node = defaultdict(float)
    insight_played = 0.0
    with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
        f.readline()
        for line in f:
            g = json.loads(line)
            games += 1
            cn = g.get('counters', {})
            insight_played += cn.get('cards.insight_played', 0)
            for k, v in cn.items():
                if k.startswith('cards.played.'):
                    cid = k[len('cards.played.'):]
                    if cid in price:
                        by[price[cid]] += v
                    if cid in NODE:
                        node[cid] += v
    return games, by, node, insight_played


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('new')
    ap.add_argument('old')
    ap.add_argument('--old-rev', default='de6b0aa')
    a = ap.parse_args()
    new_cards, old_cards = cards_at(None), cards_at(a.old_rev)
    gn, bn, nn, ins_n = read(a.new, new_cards)
    go, bo, no, ins_o = read(a.old, old_cards)
    deck = lambda cards, t, i: sum(1 for c in cards if c['tier'] == t and c['insight'] == i)
    print(f'Games: new {gn}, old {go}. Plays a game (both seats), by the card\'s insight to play on that side.\n')
    print('| tier | insight | cards new | old | plays a game new | old | share of plays new | old |')
    print('|---|---|---|---|---|---|---|---|')
    tn, to = sum(bn.values()), sum(bo.values())
    for t in (1, 2):
        for i in (0, 1, 2):
            pn, po = bn[(t, i)] / gn, bo[(t, i)] / go
            print(f'| {t} | {i} | {deck(new_cards, t, i)} | {deck(old_cards, t, i)} | {pn:.2f} | {po:.2f} | '
                  f'{100 * bn[(t, i)] / tn:.0f}% | {100 * bo[(t, i)] / to:.0f}% |')
    print(f'| all | | | | {tn / gn:.2f} | {to / go:.2f} | | |')
    print(f'\nInsight paid to play a game: new {ins_n / gn:.1f}, old {ins_o / go:.1f}.\n')
    print('| node card | plays a game new | old |')
    print('|---|---|---|')
    for c in NODE:
        print(f'| {c} | {nn[c] / gn:.2f} | {no[c] / go:.2f} |')


if __name__ == '__main__':
    main()
