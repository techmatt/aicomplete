"""258: one read from r117's native run and the pass's farmed run (256's merge.py, agents allowed to differ).

    python experiments/results/258_card_followups/merge.py OLD.jsonl OLD_SPEC NEW.jsonl NEW_SPEC OUT [LABEL]

OLD's variants (r117's bare `bp`, `br`, `pr`: read_lib's `base` arm) go first, NEW's after them renamed
`<pair>-LABEL` (default r118; r119 for the confirming read). The merged spec's agents are OLD's (all among NEW's); an
agent only NEW has becomes its own arm (`bp-sink05`), each game's cell recomputed from its (config, variant,
difficulty, agent). Writes OUT.jsonl (bare, as farmed) and OUT.spec.json: read it as OUT.jsonl=OUT.spec.json. The
merged spec is for reading only: an option arm lists its bare agent, so rerunning the spec would drop the option.
"""
import json
import sys

old, old_spec, new, new_spec, out = sys.argv[1:6]
label = sys.argv[6] if len(sys.argv) > 6 else 'r118'
a = json.load(open(old_spec, encoding='utf-8'))
b = json.load(open(new_spec, encoding='utf-8'))
for k in ('configs', 'difficulties'):
    assert a[k] == b[k], k
assert all(x in b['agents'] for x in a['agents']), 'OLD agents must be among NEW agents'


def cells(spec):
    """Cell index -> (config, variant, difficulty, agent), the experiment's order."""
    return [(c, v['name'], d, ag) for c in spec['configs'] for v in spec['variants'] for d in spec['difficulties']
            for ag in spec['agents']]


old_names = {v['name'] for v in a['variants']}
# An agent OLD lacks (`strategist:w.sink=1`) becomes its own arm of the same pair (`bp-sink1`), run by OLD's first
# agent's name with that suffix: read_lib pairs arms, and reads an agent's options as one agent.
extra = {ag: ag.split(':')[0] for ag in b['agents'] if ag not in a['agents']}
tag = {ag: ag.split(':', 1)[1].replace('w.', '').replace('=', '').replace('.', '') for ag in extra}
new_vs = [dict(v, name=f"{v['name']}-{label}") if v['name'] in old_names else v for v in b['variants']]
new_vs += [dict(v, name=f"{v['name']}-{tag[ag]}", set=v['set']) for ag in extra for v in b['variants']]
spec = dict(b, name=f"{a['name']}+{b['name']}", variants=a['variants'] + new_vs, agents=a['agents'],
            note=f"258: {old} (r117, native) then {new}, merged by merge.py")
index = {cell: i for i, cell in enumerate(cells(spec))}
with open(out + '.spec.json', 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec, f, indent=1)
n = 0
with open(out + '.jsonl', 'w', encoding='utf-8', newline='\n') as f:
    for path, s, rename in ((old, a, False), (new, b, True)):
        cs = cells(s)
        for line in open(path, encoding='utf-8'):
            line = line.strip()
            if not line:
                continue
            g = json.loads(line)
            if 'cell' not in g:
                continue  # a header line
            c, v, d, ag = cs[g['cell']]
            if rename and ag in extra:
                v, ag = f'{v}-{tag[ag]}', extra[ag]
            elif rename and v in old_names:
                v = f'{v}-{label}'
            g['cell'] = index[(c, v, d, ag)]
            f.write(json.dumps(g, separators=(',', ':')) + '\n')
            n += 1
print(f'{n} games -> {out}.jsonl', file=sys.stderr)
