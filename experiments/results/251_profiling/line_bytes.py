"""251: bytes a games.jsonl line spends by top-level key, summed over a file's games.

    python line_bytes.py FILE.jsonl
"""
import json
import sys
from collections import Counter

sizes = Counter()
games = 0
total = 0
with open(sys.argv[1], encoding="utf-8") as f:
    for line in f:
        g = json.loads(line)
        if "seed" not in g:
            continue
        games += 1
        total += len(line.encode("utf-8"))
        for k, v in g.items():
            sizes[k] += len(json.dumps(v, separators=(",", ":"), ensure_ascii=False).encode("utf-8")) + len(k) + 4
print(f"{games} games, {total / games / 1024:.1f} KB a line")
for k, n in sizes.most_common(15):
    print(f"{k:24s} {n / games / 1024:8.1f} KB  {n / total:6.1%}")
