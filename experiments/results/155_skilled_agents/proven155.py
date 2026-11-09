"""155: seeds proved winnable (python proven155.py DIR [DIR...]): a seed any oracle row won is winnable (the game is
deterministic from seed and choices, and the oracle plays on the true state). Prints, per level, the seeds proved, of
those played, and the seeds no oracle row won.
"""
import csv
import glob
import os
import sys
from collections import defaultdict


def main(dirs):
    won, seen = defaultdict(set), defaultdict(set)
    for d in dirs:
        for path in glob.glob(os.path.join(d, "games.csv")):
            for r in csv.DictReader(open(path, encoding="utf-8")):
                if not r["agent"].startswith("oracle"):
                    continue
                k = (r["variant"], int(r["seed"]))
                seen[int(r["difficulty"])].add(k)
                if r["won"] == "1":
                    won[int(r["difficulty"])].add(k)
    for lv in sorted(seen):
        left = sorted(seen[lv] - won[lv])
        print(f"d{lv}: {len(won[lv])} of {len(seen[lv])} proved winnable ({100 * len(won[lv]) / len(seen[lv]):.1f}%); "
              f"unproved: {', '.join(f'{v} {s}' for v, s in left) or 'none'}")


if __name__ == "__main__":
    main(sys.argv[1:])
