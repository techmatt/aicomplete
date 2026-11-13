"""212: cards per tag before (a git revision's content/cards.json) and after (the tree's), events apart (their tags never
count in play), and the board powers paid in each tag.

    python experiments/results/212_tags_node_powers/tagcount.py [REV]     (REV default b5490fa, r92)
"""
import json
import os
import subprocess
import sys
from collections import Counter

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
TAGS = ['security', 'diplomacy', 'science', 'industry', 'power', 'cyber']


def at(rev, path):
    if rev is None:
        with open(os.path.join(ROOT, path), encoding='utf-8') as f:
            return json.load(f)
    return json.loads(subprocess.run(['git', 'show', f'{rev}:{path}'], cwd=ROOT, capture_output=True, check=True,
                                     encoding='utf-8').stdout)


def count(cards, boards):
    every, played, powers = Counter(), Counter(), Counter()
    for c in cards:
        for t in c['tags']:
            every[t] += 1
            if c['kind'] != 'event':
                played[t] += 1
    for b in boards:
        for p in b['powers']:
            for t, _ in p.get('tags', []):
                powers[(b['id'], t)] += 1
    return every, played, powers


rev = sys.argv[1] if len(sys.argv) > 1 else 'b5490fa'
old = count(at(rev, 'content/cards.json'), at(rev, 'content/ais.json'))
new = count(at(None, 'content/cards.json'), at(None, 'content/ais.json'))
print('| tag | cards (all) before → after | non-event cards before → after | Builder / Protector / Researcher powers paid in it, before → after |')
print('|---|---|---|---|')
for t in TAGS:
    pw = lambda c: ' / '.join(str(c[2][(b, t)]) for b in ('builder', 'protector', 'researcher'))
    print(f'| {t} | {old[0][t]} → {new[0][t]} | {old[1][t]} → {new[1][t]} | {pw(old)} → {pw(new)} |')
