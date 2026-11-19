"""242: configs/three_seat.json, the baseline at 3 countries (Builder, Protector, Researcher) and difficulty 0:
the viewer's 3-seat game (its player view's done-when game). Derived: rerun after editing the baseline (regen.py does).

    python scripts/configs/three_seat.py
"""
import json
import os
import sys

root = os.path.join(os.path.dirname(__file__), '..', '..')
base = json.load(open(os.path.join(root, 'configs', 'baseline.json'), encoding='utf-8'))
base['name'] = 'three_seat'
base['countries'] = 3
adv = [m for m in base['modules'] if m['module'] == 'adversary']
assert len(adv) == 1 and 'difficulty' in adv[0]['params'], 'baseline has no adversary.difficulty'
adv[0]['params']['difficulty'] = 0
out = os.path.join(root, 'configs', 'three_seat.json')
text = json.dumps(base, indent=2) + '\n'
if '--check' in sys.argv[1:]:  # 284: regen.py --check; writes nothing
    sys.exit(0 if os.path.exists(out) and open(out, encoding='utf-8').read() == text else f'{out} is stale: run regen.py')
with open(out, 'w', encoding='utf-8', newline='\n') as f:
    f.write(text)
print('wrote', out)
