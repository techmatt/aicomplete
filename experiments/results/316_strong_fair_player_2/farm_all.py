"""316: farms every spec named in a jobs file (lines `name first count`), all at once through `farm-seeds`, each
to out/316_strong_fair_player_2/<name>.jsonl, printing a line as each ends (314's launcher, its out folder moved).
A spec whose out file already holds games is skipped, so a relaunch farms only what is missing.

    python experiments/results/316_strong_fair_player_2/farm_all.py JOBS_FILE [--only PREFIX,...]
"""
import os
import subprocess
import sys
import time

OUT = os.path.join("out", "316_strong_fair_player_2")


def main():
    path = sys.argv[1]
    only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
    exe = subprocess.run(["sh", "-c", "command -v farm-seeds"], capture_output=True, text=True).stdout.strip()
    procs = []
    for line in open(path, encoding="utf-8"):
        if not line.strip():
            continue
        name, first, count = line.split()
        if only and not any(name.startswith(p) for p in only):
            continue
        out = os.path.join(OUT, name + ".jsonl")
        if os.path.exists(out) and os.path.getsize(out) > 0:
            print(f"{time.strftime('%I:%M:%S%p')} skip {name}: {out} exists", flush=True)
            continue
        log = open(os.path.join(OUT, name + ".log"), "w", encoding="utf-8")
        cmd = ["sh", exe, first, count, out, f"experiments/farm/{name}.json"]
        procs.append((name, subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT), log))
        print(f"{time.strftime('%I:%M:%S%p')} start {name} {first} {count}", flush=True)
        time.sleep(1)
    left = list(procs)
    while left:
        time.sleep(10)
        for item in list(left):
            name, p, log = item
            if p.poll() is not None:
                log.close()
                print(f"{time.strftime('%I:%M:%S%p')} done {name} exit {p.returncode}", flush=True)
                left.remove(item)
    print(f"{time.strftime('%I:%M:%S%p')} ALL DONE", flush=True)


if __name__ == "__main__":
    main()
