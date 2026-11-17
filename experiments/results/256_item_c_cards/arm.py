"""256: one arm of a farmed read as its own read (for design_check.py and play_read.py, which pool every cell).

    python experiments/results/256_item_c_cards/arm.py GAMES.jsonl SPEC.json ARM OUT

Keeps the games of the variants named `PAIR-ARM` (`base`: a bare `PAIR`), renumbered, and writes OUT.jsonl and
OUT.spec.json: read it as OUT.jsonl=OUT.spec.json.
"""
import json
import sys

src, spec_path, arm, out = sys.argv[1:5]
spec = json.load(open(spec_path, encoding='utf-8'))
per = len(spec['configs']) * len(spec['difficulties']) * len(spec['agents'])


def arm_of(name):
    return name.split('-', 1)[1] if '-' in name else 'base'


keep = [i for i, v in enumerate(spec['variants']) if arm_of(v['name']) == arm]
new_index = {old: new for new, old in enumerate(keep)}
spec2 = dict(spec, name=f"{spec['name']}:{arm}", variants=[spec['variants'][i] for i in keep])
with open(out + '.spec.json', 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec2, f, indent=1)
n = 0
with open(out + '.jsonl', 'w', encoding='utf-8', newline='\n') as f:
    for line in open(src, encoding='utf-8'):
        g = json.loads(line)
        v, r = divmod(g['cell'], per)
        if v in new_index:
            g['cell'] = new_index[v] * per + r
            f.write(json.dumps(g, separators=(',', ':')) + '\n')
            n += 1
print(n, 'games ->', out + '.jsonl')
