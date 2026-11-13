"""214: a farmed out file as a read folder (header line + games), for design_check.py, play_read.py and out/keep.

    python experiments/results/214_tier2_pricing_insight/keep_read.py OUT.jsonl SPEC.json DIR REVISION CONTENT_HASH
"""
import json
import os
import sys

src, spec_path, out_dir, rev, chash = sys.argv[1:6]
spec = json.load(open(spec_path, encoding='utf-8'))
spec['first_seed'] = None
os.makedirs(out_dir, exist_ok=True)
with open(os.path.join(out_dir, 'games.jsonl'), 'w', encoding='utf-8', newline='\n') as f:
    first = None
    lines = open(src, encoding='utf-8').read().splitlines()
    first = min(int(json.loads(ln)['seed']) for ln in lines)
    spec['first_seed'] = str(first)
    f.write(json.dumps({'format': 'aic-games/2', 'spec': spec, 'rules_revision': int(rev), 'content_hash': chash}) + '\n')
    for ln in lines:
        f.write(ln + '\n')
print(f'{len(lines)} games -> {out_dir}/games.jsonl')
