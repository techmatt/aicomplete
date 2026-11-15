"""otry.py DIR...: today's oracle (try 1 of tries=3, the same game) against the new oracle, per level and pair.

DIR: an experiment output with DIR.trace beside it (OTRY lines: seed, config hash, try, won, ms). Today's
cost a game is try 1's ms; the new oracle's is every try's. Games an hour are per thread.
"""
import collections
import json
import os
import re
import sys

pat = re.compile(r"OTRY seed=(\d+) config=(\S+) try=(\d+) won=(\w+) ms=(\d+)")
games = {}  # (variant, diff, seed) -> list of (try, won, ms)
for d in sys.argv[1:]:
    d = d.rstrip("/")
    res = json.load(open(os.path.join(d, "results.json"), encoding="utf-8"))
    cfg = {c["config_hash"]: (c["variant"], c["difficulty"]) for c in res["cells"] if c["agent"].startswith("oracle")}
    for line in open(d + ".trace", encoding="utf-8", errors="replace"):
        m = pat.search(line)
        if not m:
            continue
        seed, h, t, won, ms = m.groups()
        v, df = cfg[h]
        games.setdefault((v, df, seed), []).append((int(t), won == "true", int(ms)))


def row(keys, label):
    n = len(keys)
    if not n:
        return
    today = sum(1 for k in keys if games[k][0][1])
    new = sum(1 for k in keys if any(w for _, w, _ in games[k]))
    t_ms = sum(games[k][0][2] for k in keys) / n
    n_ms = sum(sum(ms for _, _, ms in games[k]) for k in keys) / n
    print(f"{label:9s} n {n:4d} today {today:4d} ({100*today/n:5.1f}%) new {new:4d} ({100*new/n:5.1f}%) +{new-today:<3d}"
          f" s/game today {t_ms/1000:6.1f} new {n_ms/1000:6.1f}  games/thread-hour today {3600000/t_ms:6.0f} new {3600000/n_ms:6.0f}")


for df in sorted({k[1] for k in games}):
    for v in sorted({k[0] for k in games}):
        row([k for k in games if k[0] == v and k[1] == df], f"{v} d{df}")
    row([k for k in games if k[1] == df], f"all d{df}")
row(list(games), "all")
