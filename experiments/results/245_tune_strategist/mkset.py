"""245: an oracle game set for `aic-sim agree`: the oracle's won games from 243's kept reads (or any reads sharing
one grid), filtered by level, every Nth game, written as one games.jsonl under the first file's header.

python mkset.py OUT --levels 2,3 --every 4 [--max N] FILE...
"""
import argparse
import json

ap = argparse.ArgumentParser()
ap.add_argument("out")
ap.add_argument("files", nargs="+")
ap.add_argument("--levels", default="0,1,2,3")
ap.add_argument("--every", type=int, default=1)
ap.add_argument("--offset", type=int, default=0)
ap.add_argument("--max", type=int, default=0)
a = ap.parse_args()
levels = {int(x) for x in a.levels.split(",")}
head = None
picked = []
k = 0
for f in a.files:
    with open(f, encoding="utf-8") as fh:
        h = fh.readline()
        spec = json.loads(h)["spec"]
        if head is None:
            head = h
        nag = len(spec["agents"])
        oracle = [i for i, s in enumerate(spec["agents"]) if s.startswith("oracle")]
        for line in fh:
            if not line.strip():
                continue
            g = json.loads(line)
            if g["cell"] % nag not in oracle or not g["won"] or g["difficulty"] not in levels:
                continue
            if k % a.every == a.offset:
                picked.append(line if line.endswith("\n") else line + "\n")
            k += 1
if a.max:
    picked = picked[: a.max]
with open(a.out, "w", encoding="utf-8", newline="\n") as fh:
    fh.write(head)
    fh.writelines(picked)
print(f"{len(picked)} games -> {a.out}")
