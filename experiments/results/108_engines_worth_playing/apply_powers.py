"""108 (r43): the two card-throughput powers, applied to r42's content/ais.json (57469c6).

Run from the repo root. Idempotent: it starts from r42's file in git.
- Builder, Procure: 2 chips, draw 2 cards (spare chips into cards; Mint makes chips from credits).
- Protector, Debrief: sacrifice 1 Diplomacy, draw 2 cards (finished Diplomacy cards into fresh ones).
"""
import json
import subprocess

R42 = '57469c6'
ais = json.loads(subprocess.run(['git', 'show', f'{R42}:content/ais.json'], capture_output=True, check=True).stdout.decode('utf-8'))
by = {a['id']: a for a in ais}
by['builder']['powers'].append({
    'name': 'Procure',
    'tags': [],
    'cost': [['chips', 2]],
    'text': 'draw 2 cards',
    'effects': [{'op': 'draw', 'count': 2}],
})
by['protector']['powers'].append({
    'name': 'Debrief',
    'tags': [['diplomacy', 1]],
    'text': 'draw 2 cards',
    'effects': [{'op': 'draw', 'count': 2}],
})
with open('content/ais.json', 'w', encoding='utf-8', newline='\n') as f:
    f.write(json.dumps(ais, indent=1, ensure_ascii=False) + '\n')
print('ok')
