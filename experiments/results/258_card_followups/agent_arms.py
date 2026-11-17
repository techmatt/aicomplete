"""258: a farmed read whose arms are agent options (`strategist:w.sink=1`) as arms read_lib can pair (`bp-sink1`).

    python experiments/results/258_card_followups/agent_arms.py IN.jsonl SPEC.json OUT

read_lib reads `strategist` and `strategist:w.sink=1` as one agent ("pooled reads overlap"); this moves each option
agent into its own variant of the same pair, named by the option (`sink1`, `sink05`), the agent its bare name, so the
bare agent's games are each pair's `base` arm. Writes OUT.jsonl and OUT.spec.json: read them as OUT.jsonl=OUT.spec.json.
"""
import json
import sys

src, spec_path, out = sys.argv[1:4]
s = json.load(open(spec_path, encoding='utf-8'))
bare = [a for a in s['agents'] if ':' not in a]
opts = {a: a.split(':')[0] for a in s['agents'] if ':' in a}
tag = {a: a.split(':', 1)[1].replace('w.', '').replace('=', '').replace('.', '') for a in opts}
variants = list(s['variants']) + [dict(v, name=f"{v['name']}-{tag[a]}") for a in opts for v in s['variants']]
agents = bare or sorted(set(opts.values()))
new = dict(s, name=s['name'] + '-arms', variants=variants, agents=agents)
cells = [(c, v['name'], d, a) for c in s['configs'] for v in s['variants'] for d in s['difficulties'] for a in s['agents']]
index = {k: i for i, k in enumerate((c, v['name'], d, a) for c in new['configs'] for v in variants
                                    for d in new['difficulties'] for a in agents)}
with open(out + '.spec.json', 'w', encoding='utf-8', newline='\n') as f:
    json.dump(new, f, indent=1)
n = 0
with open(out + '.jsonl', 'w', encoding='utf-8', newline='\n') as f:
    for line in open(src, encoding='utf-8'):
        line = line.strip()
        if not line or '"cell"' not in line:
            continue
        g = json.loads(line)
        c, v, d, a = cells[g['cell']]
        if a in opts:
            v, a = f'{v}-{tag[a]}', opts[a]
        g['cell'] = index[(c, v, d, a)]
        f.write(json.dumps(g, separators=(',', ':')) + '\n')
        n += 1
print(f'{n} games -> {out}.jsonl', file=sys.stderr)
