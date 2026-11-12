"""200 (r88): the boards' antimatter pieces, written into content/ais.json (rerunnable: an entry already there is left).

    python experiments/results/200_engine_overhaul/apply_boards.py

- Annihilate, a power on every board (last in its list, so earlier powers keep their indexes and counters):
  2 antimatter, once a round: remove every agent and troop in one hex in reach, and sway 3 toward Unity.
- Two antimatter capstones of six: the Researcher's Open Archive (economy) and the Protector's Sanctuary (economy)
  each add 1 antimatter a round to what they did.
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
PATH = os.path.join(ROOT, 'content', 'ais.json')
ANNIHILATE = {
    'name': 'Annihilate',
    'tags': [],
    'cost': [['antimatter', 2]],
    'text': 'remove every agent and troop in one hex in reach; sway 3 toward Unity',
    # Screen 5: Sway 3 (was 2) +5.3 +- 1.2 over 2 on 1440 games, d0 93.4 with no extra event cards.
    'effects': [{'op': 'destroy_all'}, {'op': 'sway', 'side': 'unity', 'amount': 3}],
}
AM = {'op': 'income', 'resource': 'antimatter', 'amount': 1}
CAPSTONES = {
    'Open Archive': 'Each round, 1 research cube and 1 antimatter onto your board.',
    'Sanctuary': 'Corruption never destroys your Nodes. Each round, 1 antimatter.',
}

with open(PATH, encoding='utf-8') as f:
    boards = json.load(f)
for b in boards:
    b['powers'] = [p for p in b['powers'] if p['name'] != 'Annihilate'] + [ANNIHILATE]
    for space in b['economy_track'] + b['play_track']:
        cap = space.get('capstone')
        if cap and cap['name'] in CAPSTONES:
            cap['text'] = CAPSTONES[cap['name']]
            if AM not in cap['passives']:
                cap['passives'].append(AM)
with open(PATH, 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(boards, indent=1, ensure_ascii=True) + '\n')
print('wrote', PATH)
