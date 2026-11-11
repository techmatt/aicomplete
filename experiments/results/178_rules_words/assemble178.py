"""Assemble a farm-seeds output file as an experiment output dir (games.jsonl with its header).

    python experiments/results/178_rules_words/assemble178.py SPEC FIRST COUNT FILE OUT_DIR

The header comes from `aic-sim farm SPEC --header` (target/release, the repo root's configs): run it on the r77
build whose games FILE holds.
"""
import os
import sys

sys.path.insert(0, os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts')))
from screens import assemble  # noqa: E402

if __name__ == '__main__':
    spec, first, count, path, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
    assemble(spec, first, count, [path], out)
    print(os.path.join(out, 'games.jsonl'))
