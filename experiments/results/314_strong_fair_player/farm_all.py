"""314: farms every spec named in a jobs file (lines `name first count`), all at once through `farm-seeds`, each
to out/314_strong_fair_player/<name>.jsonl, printing a line as each ends.

    python experiments/results/314_strong_fair_player/farm_all.py out/314_strong_fair_player/jobs.txt [--only PREFIX,...]
"""
import os
import subprocess
import sys
import time

OUT = os.path.join("out", "314_strong_fair_player")


def main():
    path = sys.argv[1]
    only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
    jobs = []
    for line in open(path, encoding="utf-8"):
        if not line.strip():
            continue
        name, first, count = line.split()
        if only and not any(name.startswith(p) for p in only):
            continue
        jobs.append((name, first, count))
    procs = []
    for name, first, count in jobs:
        out = os.path.join(OUT, name + ".jsonl")
        if os.path.exists(out) and os.path.getsize(out) > 0:
            print(f"{time.strftime('%H:%M:%S')} skip {name}: {out} exists", flush=True)
            continue
        log = open(os.path.join(OUT, name + ".log"), "w", encoding="utf-8")
        cmd = ["sh", "farm-seeds", first, count, out, f"experiments/farm/{name}.json"]
        exe = subprocess.run(["sh", "-c", "command -v farm-seeds"], capture_output=True, text=True).stdout.strip()
        cmd[1] = exe
        procs.append((name, subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT), log))
        print(f"{time.strftime('%H:%M:%S')} start {name} {first} {count}", flush=True)
        time.sleep(1)
    left = list(procs)
    while left:
        time.sleep(10)
        for item in list(left):
            name, p, log = item
            if p.poll() is not None:
                log.close()
                print(f"{time.strftime('%H:%M:%S')} done {name} exit {p.returncode}", flush=True)
                left.remove(item)
    print(f"{time.strftime('%H:%M:%S')} ALL DONE", flush=True)


if __name__ == "__main__":
    main()
