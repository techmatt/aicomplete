"""234: what targeted removals find, per area, from a read's counters: agents removed per agent printed (on the
target, on or next to it), troops the same, and how often a card's target found a hex. values.json's
`rates.area_on` / `area_near` are these shares.

    python experiments/results/234_card_targets/finds.py DIR_OR_GAMES [...]
"""
import json
import os
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
KEYS = ['cards.remove_agents.on', 'cards.agents_removed.on', 'cards.remove_agents.near', 'cards.agents_removed.near',
        'cards.remove_troops_at.on', 'cards.troops_removed_at.on', 'cards.remove_troops_at.near',
        'cards.troops_removed_at.near', 'effects.targets', 'effects.targets_missed', 'effects.land_lifted',
        'cards.agents_in_reach', 'cards.troops_in_reach']


def games(path):
    if os.path.isdir(path):
        path = os.path.join(path, 'games.jsonl')
    with open(path, encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line.startswith('{') and '"counters"' in line:
                yield json.loads(line)


def counters(g):
    c = g.get('counters') or g.get('stats', {}).get('counters') or {}
    return c


def main():
    tot = Counter()
    n = 0
    for p in sys.argv[1:]:
        for g in games(p):
            n += 1
            for k, v in counters(g).items():
                if k in KEYS or k.count('.') == 3 or k.startswith(('cards.agents_in_reach.', 'cards.troops_in_reach.')):
                    tot[k] += v
    print(f'{n} games')
    # Per printed number: plays, found on / next to the target, and what r103's reach held (up to the number).
    print('| removal | N | plays | found a play | share of N | reach would find | share |')
    print('|---|---|---|---|---|---|---|')
    for kind, ask, got, reach in (('agents', 'cards.remove_agents', 'cards.agents_removed', 'cards.agents_in_reach'),
                                  ('troops', 'cards.remove_troops_at', 'cards.troops_removed_at', 'cards.troops_in_reach')):
        for at in ('on', 'near'):
            for big in range(1, 9):
                plays = tot[f'{ask}.{at}.{big}']
                if not plays:
                    continue
                found = tot[f'{got}.{at}.{big}'] / plays
                print(f'| {kind} {at} | {big} | {plays} | {found:.2f} | {found / big:.2f} | | |')
        for big in range(1, 9):
            r = tot[f'{reach}.{big}']
            plays = tot[f'{ask}.on.{big}'] + tot[f'{ask}.near.{big}']
            if plays:
                print(f'| {kind} reach (r103) | {big} | {plays} | | | {r / plays:.2f} | {r / plays / big:.2f} |')
    for at in ('on', 'near'):
        a, b = tot[f'cards.remove_agents.{at}'], tot[f'cards.agents_removed.{at}']
        c, d = tot[f'cards.remove_troops_at.{at}'], tot[f'cards.troops_removed_at.{at}']
        print(f'{at}: agents {b}/{a} = {b / a if a else 0:.2f}; troops {d}/{c} = {d / c if c else 0:.2f}')
    asked_a = tot['cards.remove_agents.on'] + tot['cards.remove_agents.near']
    asked_t = tot['cards.remove_troops_at.on'] + tot['cards.remove_troops_at.near']
    print(f"r103's reach would have found: agents {tot['cards.agents_in_reach']}/{asked_a} = "
          f"{tot['cards.agents_in_reach'] / max(asked_a, 1):.2f}; troops {tot['cards.troops_in_reach']}/{asked_t} = "
          f"{tot['cards.troops_in_reach'] / max(asked_t, 1):.2f}")
    t, m = tot['effects.targets'], tot['effects.targets_missed']
    print(f'targets found {t}, missed {m} ({t / max(n, 1):.2f} a game)')


main()
