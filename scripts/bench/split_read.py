"""Splits a lever screen into one plain three-pair read per lever, for ladder.py and design_check.py.

    python scripts/bench/split_read.py READ_DIR OUT_PREFIX          # a read folder (games.jsonl with its header)
    python scripts/bench/split_read.py FILE=SPEC.json OUT_PREFIX    # a farm's out file (no header) and its spec

A screen's variants are named `PAIR` (the base) or `PAIR_LEVER` (bp, br, pr). Each lever's games go to
OUT_PREFIX-LEVER/games.jsonl (the base to OUT_PREFIX-base), under a header whose spec holds only that lever's
variants renamed to the bare pairs, with cells renumbered to match, so each folder reads as a plain read.
221 promoted it from 215's and 219's item copies (`experiments/results/{215,219}_*/split_read.py`).
"""
import json
import os
import sys


def read(arg):
    """(header, game rows) from a read folder or `FILE=SPEC`; a farm file's header is made from its spec."""
    if '=' in arg:
        path, spec_path = arg.split('=', 1)
        with open(spec_path, encoding='utf-8') as f:
            head = {'format': 'aic-games/2', 'spec': json.load(f)}
    else:
        path, head = os.path.join(arg, 'games.jsonl'), None
    rows = []
    with open(path, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            if head is None:
                head = json.loads(line)
                continue
            rows.append(json.loads(line))
    return head, rows


def split(head, rows, prefix):
    """Writes one folder per lever; returns their paths."""
    spec = head['spec']
    per = len(spec['difficulties']) * len(spec['agents'])
    levers = {}
    for vi, v in enumerate(spec['variants']):
        pair, _, lever = v['name'].partition('_')
        levers.setdefault(lever or 'base', []).append((vi, pair, v))
    hashes = head.get('config_hashes') or []
    outs = []
    for lever, vs in levers.items():
        old_to_new = {vi * per + k: ni * per + k for ni, (vi, _, _) in enumerate(vs) for k in range(per)}
        sub = dict(spec, name=f"{spec.get('name', 'read')}-{lever}", variants=[dict(v, name=pair) for _, pair, v in vs])
        h = dict(head, spec=sub)
        if hashes:
            h['config_hashes'] = [hashes[o] for o in sorted(old_to_new, key=old_to_new.get)]
        out = f'{prefix}-{lever}'
        os.makedirs(out, exist_ok=True)
        with open(os.path.join(out, 'games.jsonl'), 'w', encoding='utf-8', newline='\n') as f:
            f.write(json.dumps(h) + '\n')
            for g in rows:
                if g['cell'] in old_to_new:
                    f.write(json.dumps(dict(g, cell=old_to_new[g['cell']])) + '\n')
        outs.append(out)
        print(out, len(vs), 'variants')
    return outs


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    if len(sys.argv) != 3 or sys.argv[1] in ('-h', '--help'):
        print(__doc__)
        return 0 if len(sys.argv) == 2 else 2
    head, rows = read(sys.argv[1])
    split(head, rows, sys.argv[2])
    return 0


if __name__ == '__main__':
    sys.exit(main())
