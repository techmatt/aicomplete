"""323: configs/solo.json, one-player mode (Matt, Nov 25): the baseline at 1 country, with d1's numbers baked in.
Derived: rerun after editing the baseline or the content texts it resolves (regen.py does).

    python scripts/configs/solo.py [--check]

What it changes from the baseline (each solo adjustment is listed in docs/rules/board.md, *Solo*):
- `countries` 1, `adversary.difficulty` 1: d1's numbers, resolved by the engine (city 3, K 3, the 10-round deck).
- `ai.boards` [0]: the Builder, the suggested first board; the board is the player's choice (the viewer's Board
  select beside New game, `--set ai.boards=1` or `2` in the sim).
- The draft hand is 4 cards, as every game's (Matt, Nov 25, 326): `cards.no_pass` 1, the hand you build (each card's
  tier chosen) is your draft, nothing passed; then the keep as usual.
- Every "(level 1+)" a content text prints is dropped (a content overlay on the text alone): solo shows its
  resolved rule, never a level condition. A "(level N+)" above 1 would print a line solo never plays: refused.
"""
import json
import os
import re
import sys

root = os.path.join(os.path.dirname(__file__), '..', '..')
base = json.load(open(os.path.join(root, 'configs', 'baseline.json'), encoding='utf-8'))
LEVEL = 1


def module(name):
    ms = [m for m in base['modules'] if m['module'] == name]
    assert len(ms) == 1, f'baseline has no single {name} module'
    return ms[0]['params']


base['name'] = 'solo'
base['countries'] = 1
adv = module('adversary')
assert 'difficulty' in adv, 'baseline has no adversary.difficulty'
adv['difficulty'] = LEVEL
module('ai')['boards'] = [0]
cards = module('cards')
assert 'no_pass' in cards and cards.get('deal') == 4, 'baseline has no cards.no_pass, or its draft hand is not 4'
cards['no_pass'] = 1

# Content texts that print a level condition: the resolved line, as solo plays it.
MARK = re.compile(r' ?\(level (\d)\+\)')
overlay = []


def resolve(table, ident, path, text):
    levels = [int(m) for m in MARK.findall(text)]
    if not levels:
        return
    assert max(levels) <= LEVEL, f'{table}.{ident}.{path}: "(level {max(levels)}+)" is a line solo never plays'
    overlay.append(f'{table}.{ident}.{path}=' + json.dumps(MARK.sub('', text), ensure_ascii=False))


def walk(table, ident, obj, path):
    if isinstance(obj, dict):
        for k, v in obj.items():
            walk(table, ident, v, f'{path}.{k}' if path else k)
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            walk(table, ident, v, f'{path}.{i}')
    elif isinstance(obj, str):
        resolve(table, ident, path, obj)


adversary = json.load(open(os.path.join(root, 'content', 'adversary.json'), encoding='utf-8'))
walk('adversary', adversary['board']['id'], {k: v for k, v in adversary['board'].items() if k != 'id'}, '')
# The research row's K, resolved: the level's cubes a country, times solo's one country.
K_WORDS = "Every K cubes on the track (the level line's cubes a country, times the countries)"
research = adversary['board']['research']['text']
assert K_WORDS in research, f"the research row's text changed: teach solo.py to resolve its K ({research!r})"
k = adv['research_at'][min(LEVEL, len(adv['research_at']) - 1)] * base['countries']
overlay.append(f"adversary.{adversary['board']['id']}.research.text="
               + json.dumps(research.replace(K_WORDS, f'Every {k} cubes on the track'), ensure_ascii=False))
for e in adversary['events']:
    walk('adversary', e['id'], {k: v for k, v in e.items() if k != 'id'}, '')
for e in json.load(open(os.path.join(root, 'content', 'projects.json'), encoding='utf-8')):
    walk('projects', e['id'], {k: v for k, v in e.items() if k != 'id'}, '')
for f in ('threats', 'alignment', 'cards', 'ais'):
    text = open(os.path.join(root, 'content', f'{f}.json'), encoding='utf-8').read()
    assert not MARK.search(text), f'content/{f}.json prints a level condition: teach solo.py to resolve it'
base['content'] = overlay

out = os.path.join(root, 'configs', 'solo.json')
text = json.dumps(base, indent=2, ensure_ascii=False) + '\n'
if '--check' in sys.argv[1:]:  # regen.py --check; writes nothing
    sys.exit(0 if os.path.exists(out) and open(out, encoding='utf-8').read() == text else f'{out} is stale: run regen.py')
with open(out, 'w', encoding='utf-8', newline='\n') as f:
    f.write(text)
print('wrote', out, f'({len(overlay)} level texts resolved)')
