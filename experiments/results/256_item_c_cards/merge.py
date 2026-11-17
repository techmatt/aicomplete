"""256: one read from r115's native run and the pass's farmed run of specs with the same configs, levels and agents.

    python experiments/results/256_item_c_cards/merge.py OLD.jsonl OLD_SPEC NEW.jsonl NEW_SPEC OUT

OLD's variants (r115's bare `bp`, `br`, `pr`: read_lib's `base` arm) go first, NEW's after them; NEW's cell indexes
move up by OLD's cell count. Writes OUT.jsonl (bare, as farmed) and OUT.spec.json: read it as OUT.jsonl=OUT.spec.json.
"""
import json
import sys

old, old_spec, new, new_spec, out = sys.argv[1:6]
a = json.load(open(old_spec, encoding='utf-8'))
b = json.load(open(new_spec, encoding='utf-8'))
for k in ('configs', 'difficulties', 'agents'):
    assert a[k] == b[k], k
per_variant = len(a['configs']) * len(a['difficulties']) * len(a['agents'])
shift = len(a['variants']) * per_variant
# A NEW variant named as an OLD one (both bare `bp`: the fresh read) becomes arm `r116` of its pair.
old_names = {v['name'] for v in a['variants']}
new_vs = [dict(v, name=f"{v['name']}-r116") if v['name'] in old_names else v for v in b['variants']]
spec = dict(b, name=f"{a['name']}+{b['name']}", variants=a['variants'] + new_vs,
            note=f"256: {old} (r115, native) then {new}, merged by merge.py")
with open(out + '.spec.json', 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec, f, indent=1)
n = 0
with open(out + '.jsonl', 'w', encoding='utf-8', newline='\n') as f:
    for path, add in ((old, 0), (new, shift)):
        for line in open(path, encoding='utf-8'):
            g = json.loads(line)
            g['cell'] += add
            f.write(json.dumps(g, separators=(',', ':')) + '\n')
            n += 1
print(n, 'games ->', out + '.jsonl')
