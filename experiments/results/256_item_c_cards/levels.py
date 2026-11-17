"""256: a read cut to some levels (r115's d0-d4 screen run as the d3-d4 probe's base arm).

    python experiments/results/256_item_c_cards/levels.py GAMES.jsonl SPEC.json 3,4 OUT

Writes OUT.jsonl and OUT.spec.json (one config assumed), cells renumbered.
"""
import json
import sys

src, spec_path, lv, out = sys.argv[1:5]
keep = [int(x) for x in lv.split(',')]
spec = json.load(open(spec_path, encoding='utf-8'))
ds, ags = spec['difficulties'], spec['agents']
spec2 = dict(spec, difficulties=keep)
with open(out + '.spec.json', 'w', encoding='utf-8', newline='\n') as f:
    json.dump(spec2, f, indent=1)
n = 0
with open(out + '.jsonl', 'w', encoding='utf-8', newline='\n') as f:
    for line in open(src, encoding='utf-8'):
        g = json.loads(line)
        v, r = divmod(g['cell'], len(ds) * len(ags))
        d, a = divmod(r, len(ags))
        if ds[d] in keep:
            g['cell'] = v * len(keep) * len(ags) + keep.index(ds[d]) * len(ags) + a
            f.write(json.dumps(g, separators=(',', ':')) + '\n')
            n += 1
print(n, 'games ->', out + '.jsonl')
