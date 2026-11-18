"""273: the credit numbers the design checks hold by hand, ÷ 1.5 with the content (run once, from r127's files).

    python experiments/results/273_smaller_numbers/scripts_scale.py

scripts/design_check.json `extra_anchors`, scripts/design_check_jobs.json `units` and the rows that copy a card's
credit numbers by hand. Writes in place, keeping each file's layout (text edits).
"""
import json
import os
import re
import subprocess

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
BASE = 'aea5535805'


def base_text(path):
    return subprocess.run(['git', 'show', f'{BASE}:{path}'], cwd=ROOT, capture_output=True, check=True,
                          encoding='utf-8').stdout


def num(x):
    v = round(x / 1.5, 2)
    return str(int(v)) if v == int(v) else repr(v)


def sub_key(text, key, start, scale=True, new=None):
    pat = re.compile(rf'("{re.escape(key)}": )(-?[\d.]+)')
    m = pat.search(text, start)
    assert m, key
    val = new if new is not None else num(float(m.group(2)))
    return text[:m.start(2)] + val + text[m.end(2):]


def write(path, text):
    json.loads(text)
    open(os.path.join(ROOT, path), 'w', encoding='utf-8', newline='\n').write(text)


# design_check.json: extra_anchors, all credits.
path = 'scripts/design_check.json'
text = base_text(path)
a = text.index('"extra_anchors": {')
for k in ('harden', 'botnet', 'station_freed', 'factory_freed', 'city_liberated', 'card', 'compute', 'agent', 'sway',
          'node', 'factory', 'land_token', 'station_owned'):
    text = sub_key(text, k, a)
write(path, text)

# design_check_jobs.json: units in credits (cr_inc is +1 credit income in credits: 4 rounds, unchanged).
path = 'scripts/design_check_jobs.json'
text = base_text(path)
a = text.index('"units": {')
for k in ('move', 'project', 'firewall', 'jam', 'shelter', 'follower', 'ins_inc', 'en_inc1', 'en_inc2', 'chip_inc',
          'station_freed', 'factory_freed'):
    text = sub_key(text, k, a)
# Rows copying a card's numbers (each its new content value).
ROWS = [  # (card, old, new): replaced inside that card's row only
    ('Honeypot Relay', '"other": "card + 2 * 3"', '"other": "card + 2 * 2"'),   # 3 credits a removal -> 2
    ('Counterinsurgency Campaign', '"other": "2 * cr_inc"', '"other": "1 * cr_inc"'),  # +2 credit income -> 1
    ('Counterinsurgency Campaign', '+2 credit income 8', '+1 credit income 4'),
    ('Fusion Startup Stake', '"other": "en_inc1 + 3 * cr_inc"', '"other": "en_inc1 + 2 * cr_inc"'),  # 3 -> 2
    ('Pumped Hydro Storage', '"other": "5 * cr_inc"', '"other": "3 * cr_inc"'),   # 5 -> 3
    ('Pumped Hydro Storage', '+5 credit income 20', '+3 credit income 12'),
    ('Trade Delegation', '"other": "3 * cr_inc"', '"other": "2 * cr_inc"'),       # 3 -> 2
    ('Frontier Lab Charter', '"other": "2 * cr_inc"', '"other": "1 * cr_inc"'),   # 2 -> 1
    ('Power to the People', '"other": "uses * tags + 0.5 * station_owned"',
     '"other": "uses * tags / 2 + 0.5 * station_owned"'),                          # 1 per Power tag -> per 2
    ('Treaty Isotope Bank', '"use": 8', '"use": 5'),                              # its Action's 8 credits -> 5
    ('Teardown (Builder)', '"other": "-(10 + 12)"', '"other": "-(6.67 + 8)"'),    # burn 10, Fallout 12 at the table
    ('Teardown (Builder)', '(-10) and a Fallout (-12)', '(-6.67) and a Fallout (-8)'),
    ('Meltdown (Researcher)', '"other": -15', '"other": -10'),                    # 15 at the table
]
for name, old, new in ROWS:
    a = text.index(f'"name": "{name}"')
    b = text.index('}', a)
    block = text[a:b]
    assert block.count(old) == 1, (name, old)
    text = text[:a] + block.replace(old, new) + text[b:]
write(path, text)
print('ok')
