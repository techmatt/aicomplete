"""212: one lever's games of an experiment folder, as a games.jsonl other readers take (design_check.py, ladder.py).

    python experiments/results/212_tags_node_powers/cells212.py DIR LEVER OUT.jsonl

LEVER: `base` (the bp / br / pr cells) or a lever name (`prompt` keeps bp_prompt, br_prompt, pr_prompt).
"""
import json
import os
import sys

src, lever, out = sys.argv[1], sys.argv[2], sys.argv[3]
path = src if src.endswith('.jsonl') else os.path.join(src, 'games.jsonl')
with open(path, encoding='utf-8') as f, open(out, 'w', encoding='utf-8', newline='\n') as o:
    names, per = None, None
    for line in f:
        g = json.loads(line)
        if 'format' in g:
            spec = g['spec'] if isinstance(g['spec'], dict) else json.loads(g['spec'])
            names = [v['name'] for v in spec['variants']]
            per = len(spec['difficulties']) * len(spec['agents'])
            o.write(line)
            continue
        v = names[(g['cell'] // per) % len(names)]
        _, _, lv = v.partition('_')
        if (lv or 'base') == lever:
            o.write(line)
