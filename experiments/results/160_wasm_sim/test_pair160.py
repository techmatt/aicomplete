"""160: a test pair for the engine-aware pairing (from the repo root).

    python experiments/results/160_wasm_sim/test_pair160.py NATIVE WASM SPEC FIRST COUNT OUT

NATIVE and WASM: farm outputs of SPEC over the same seeds FIRST .. FIRST+COUNT-1, one per engine.
Assembles two read directories with `screens.assemble`: OUT/a, every seed native; OUT/b, the first
half of the seeds from WASM and the rest native. Then `paired.py OUT/a OUT/b --pair ...` and
`ladder.py OUT/b OUT/a` pair only the native seeds and count the wasm half as mixed.
"""
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import assemble  # noqa: E402


def by_seed(path):
    out = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            if line.strip():
                out.setdefault(int(json.loads(line)['seed']), []).append(line)
    return out


def main():
    native, wasm, spec, first, count, out = sys.argv[1:7]
    first, count = int(first), int(count)
    n, w = by_seed(native), by_seed(wasm)
    half = first + count // 2
    os.makedirs(out, exist_ok=True)
    mixed = os.path.join(out, 'b-shards.jsonl')
    with open(mixed, 'w', encoding='utf-8', newline='\n') as f:
        for s in range(first, first + count):
            f.writelines((w if s < half else n)[s])
    assemble(spec, first, count, [native], os.path.join(out, 'a'))
    assemble(spec, first, count, [mixed], os.path.join(out, 'b'))
    print(f'{out}/a: all native; {out}/b: seeds {first}..{half - 1} wasm, {half}..{first + count - 1} native')


if __name__ == '__main__':
    main()
