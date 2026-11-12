"""189, amendment 8615's watch list, new against old, per game: how often a Rally share is blocked and how often
players lay firewalls or no-Rally tokens; how often the opening's tier-2 cards are kept and played; how often
Patience carries insight and how much.

    python watch.py NEW_DIR OLD_DIR [CONTENT_CARDS_JSON]

Opening tier-2s: each seat is dealt 2 (r81 and r82). A tier-2 card never offered in a draft (`offered` excludes the
opening deal; no draw takes tier 2) but kept (r81: the opening keep, 1 insight) or played is an opening card; r82
deals them into the hand, so all are kept. Shares are of the 2 a seat was dealt. Patience: the Researcher's carried insight
(`economy.insight_carried`, a game's sum; `.rNN` per round), read only in cells with the Researcher.
"""
import json
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
CARDS = sys.argv[3] if len(sys.argv) > 3 else 'content/cards.json'
TIER = {c['id']: c['tier'] for c in json.load(open(CARDS, encoding='utf-8'))}


def load(d, new_rules):
    s = defaultdict(float)
    n = researcher_games = 0
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        first = json.loads(f.readline())
        cells = first.get('cells') or []
        for line in f:
            g = json.loads(line)
            n += 1
            c = g.get('counters', {})
            s['rallied'] += c.get('adversary.rallied', 0)
            s['rally agents blocked (r82 counter)'] += c.get('adversary.rally_blocked', 0)
            s['no-Rally tokens blocked'] += sum(v for k, v in c.items() if re.match(r'^prohibit\.rally\..*\.blocked$', k))
            s['no-Rally prohibitions made'] += sum(v for k, v in c.items() if re.match(r'^prohibit\.rally\..*\.made$', k))
            s['firewalls laid (cards)'] += c.get('effects.firewalls', 0)
            s['agents a firewall turned away'] += c.get('adversary.firewalled', 0)
            s['games with a blocked Rally share'] += c.get('adversary.rally_blocked', 0) > 0
            seats = len(g.get('spent') or [])
            s['opening tier-2 dealt'] += 2 * seats
            for card, v in (g.get('cards') or {}).items():
                if TIER.get(card) == 2 and v.get('offered', 0) == 0:
                    played = v.get('played', 0) > 0
                    s['opening tier-2 kept'] += (v.get('kept', 0) > 0) and not new_rules
                    s['opening tier-2 played'] += played
            if new_rules:
                s['opening tier-2 kept'] += 2 * seats  # r82: dealt into the hand, kept by rule
            if any(k.startswith('ai.power.researcher') for k in c):
                researcher_games += 1
                s['Patience: insight carried a game'] += c.get('economy.insight_carried', 0)
                per = [v for k, v in c.items() if k.startswith('economy.insight_carried.r')]
                s['Patience: rounds carrying any'] += sum(1 for v in per if v > 0)
                s['Patience: rounds'] += g.get('rounds', 0)
    out = {k: v / n for k, v in s.items() if not k.startswith('Patience')}
    if researcher_games:
        out['Patience: insight carried a game'] = s['Patience: insight carried a game'] / researcher_games
        out['Patience: share of rounds carrying any'] = s['Patience: rounds carrying any'] / max(1, s['Patience: rounds'])
    if s['opening tier-2 dealt']:
        out['opening tier-2: share kept'] = s['opening tier-2 kept'] / s['opening tier-2 dealt']
        out['opening tier-2: share played'] = s['opening tier-2 played'] / s['opening tier-2 dealt']
    return n, researcher_games, out


new = load(sys.argv[1], True)
old = load(sys.argv[2], False)
print(f'New ({new[0]} games, {new[1]} with the Researcher), old ({old[0]}, {old[1]}). Per game unless a share.\n')
print('| measure | new | old |')
print('|---|---|---|')
for k in sorted(set(new[2]) | set(old[2])):
    a, b = new[2].get(k, 0), old[2].get(k, 0)
    f = (lambda x: f'{x:.0%}') if 'share' in k else (lambda x: f'{x:.2f}')
    print(f'| {k} | {f(a)} | {f(b)} |')
