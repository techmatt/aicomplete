"""219: split a read of `PAIR` and `PAIR_LEVER` variants into one folder per lever, each named as plain pairs
(bp, br, pr), for ladder.py and design_check.py.

    python experiments/results/219_antimatter_sources/split_read.py READ_DIR OUT_PREFIX

Writes OUT_PREFIX-LEVER/games.jsonl (base: OUT_PREFIX-base), cells renumbered to the trimmed spec.
"""
import json
import os
import sys

src, prefix = sys.argv[1], sys.argv[2]
with open(os.path.join(src, 'games.jsonl'), encoding='utf-8') as f:
    head = json.loads(f.readline())
    rows = [json.loads(line) for line in f]
spec = head['spec']
per = len(spec['difficulties']) * len(spec['agents'])
levers = {}
for vi, v in enumerate(spec['variants']):
    pair, _, lv = v['name'].partition('_')
    levers.setdefault(lv or 'base', []).append((vi, pair, v))
for lv, vs in levers.items():
    out = f'{prefix}-{lv}'
    os.makedirs(out, exist_ok=True)
    s = dict(spec, variants=[dict(v, name=pair) for _, pair, v in vs])
    old_to_new = {}
    for ni, (vi, _, _) in enumerate(vs):
        for k in range(per):
            old_to_new[vi * per + k] = ni * per + k
    hashes = head.get('config_hashes') or []
    h = dict(head, spec=s, config_hashes=[hashes[o] for o in sorted(old_to_new, key=old_to_new.get)] if hashes else hashes)
    with open(os.path.join(out, 'games.jsonl'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps(h) + '\n')
        for g in rows:
            if g['cell'] in old_to_new:
                f.write(json.dumps(dict(g, cell=old_to_new[g['cell']])) + '\n')
    print(out, len(vs), 'variants')
