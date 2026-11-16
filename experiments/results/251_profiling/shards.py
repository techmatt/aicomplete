"""251: plays a farm spec's seed range natively as parallel shards with a given binary, then joins them.

    python shards.py BIN SPEC FIRST COUNT PER_SHARD THREADS OUT

The joined file is what `aic-sim farm SPEC --seed FIRST --games COUNT` writes (shards in seed order).
Used for a side that must run on a fixed binary (the base copy), where farm-seeds builds the tree.
"""
import os
import subprocess
import sys
import time

bin_, spec, first, count, per, threads, out = sys.argv[1:8]
bin_ = os.path.abspath(bin_)
first, count, per, threads = int(first), int(count), int(per), int(threads)
ranges = [(s, min(per, first + count - s)) for s in range(first, first + count, per)]
pending = list(ranges)
running = []
t0 = time.time()
while pending or running:
    while pending and len(running) < threads:
        s, n = pending.pop(0)
        path = f"{out}.{s}"
        p = subprocess.Popen([bin_, "farm", spec, "--seed", str(s), "--games", str(n), "--out", path],
                             stderr=subprocess.DEVNULL)
        running.append(p)
    time.sleep(0.5)
    running = [p for p in running if p.poll() is None]
with open(out, "w", encoding="utf-8", newline="\n") as f:
    for s, _ in ranges:
        with open(f"{out}.{s}", encoding="utf-8") as g:
            f.write(g.read())
print(f"SHARDSDONE {spec} {count} seeds in {time.time() - t0:.0f} s")
