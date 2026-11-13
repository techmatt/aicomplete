"""Split a farmed read by lever into read folders with a games.jsonl header each (for design_check.py, ladder.py).

    python split_read.py FILE SPEC OUT_PREFIX
Writes OUT_PREFIX-<lever>/games.jsonl for each lever (`<pair>_<lever>` variants; `bp` alone is `base`), its header a
spec holding that lever's variants renamed to the bare pairs, so each folder reads as a plain three-pair read.
"""
import json
import os
import sys


def main():
    path, spec_path, prefix = sys.argv[1:4]
    with open(spec_path, encoding='utf-8') as f:
        spec = json.load(f)
    nd, na = len(spec['difficulties']), len(spec['agents'])
    levers = {}
    for i, v in enumerate(spec['variants']):
        pair, _, lever = v['name'].partition('_')
        levers.setdefault(lever or 'base', []).append((i, pair, v))
    outs = {}
    for lever, vs in levers.items():
        sub = dict(spec, name=f"{spec['name']}-{lever}",
                   variants=[dict(v, name=pair) for _, pair, v in vs])
        d = f'{prefix}-{lever}'
        os.makedirs(d, exist_ok=True)
        f = open(os.path.join(d, 'games.jsonl'), 'w', encoding='utf-8', newline='\n')
        f.write(json.dumps({'format': 'aic-games/2', 'spec': sub}) + '\n')
        remap = {}
        for k, (vi, _, _) in enumerate(vs):
            for c in range(nd * na):
                remap[vi * nd * na + c] = k * nd * na + c
        outs[lever] = (f, remap)
    with open(path, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            g = json.loads(line)
            vi = g['cell'] // (nd * na)
            lever = spec['variants'][vi]['name'].partition('_')[2] or 'base'
            out, remap = outs[lever]
            g['cell'] = remap[g['cell']]
            out.write(json.dumps(g) + '\n')
    for out, _ in outs.values():
        out.close()
    print(' '.join(f'{prefix}-{lever}' for lever in outs))


if __name__ == '__main__':
    main()
