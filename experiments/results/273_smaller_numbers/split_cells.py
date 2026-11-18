"""273: split a farmed read of several three-pair arms into one games.jsonl an arm (cells renumbered from 0).

    python experiments/results/273_smaller_numbers/split_cells.py GAMES.jsonl ARM_CELLS OUT_PREFIX

ARM_CELLS: cells an arm (30 for three pairs x five levels x two agents). Writes OUT_PREFIX0.jsonl, OUT_PREFIX1.jsonl, ...
"""
import json
import sys

src, size, prefix = sys.argv[1], int(sys.argv[2]), sys.argv[3]
outs = {}
for line in open(src, encoding='utf-8'):
    g = json.loads(line)
    if not isinstance(g.get('cell'), int):
        continue
    arm, g['cell'] = divmod(g['cell'], size)
    if arm not in outs:
        outs[arm] = open(f'{prefix}{arm}.jsonl', 'w', encoding='utf-8', newline='\n')
    outs[arm].write(json.dumps(g) + '\n')
for f in outs.values():
    f.close()
print(sorted(outs))
