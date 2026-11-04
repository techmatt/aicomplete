"""A sampling profile of one aic-sim run on Windows, with no admin rights (103).

    python scripts/bench/profile_win.py SIM.exe OUT_DIR -- run configs/baseline.json --agent strategist ...

`samply` needs an elevated session on Windows (it raises a UAC prompt and waits). Visual
Studio's Standard Collector does not: its service samples for any user. This script launches
SIM.exe with the arguments after `--` under a CPU-sampling session (`VSDiagnostics.exe`,
`CpuUsageBase.json`, about 1 kHz), stops the session when it exits, unpacks the
`.diagsession`, dumps its ETL with `xperf -symbols` (the Windows Performance Toolkit; symbols
from the `.pdb` beside SIM.exe only, no symbol server), and tallies the samples of that process.

Build SIM.exe with symbols, in its own target dir so the release binary stays as it is:
`CARGO_PROFILE_RELEASE_DEBUG=true CARGO_TARGET_DIR=out/target-prof cargo build --release -p aic-sim`.
Debug info does not change the code. Inlined functions count toward the function they were
inlined into: `self` is where the sample landed, `total` any frame on the stack (each
function once a sample, so recursion does not double it).

The collector walks stacks only for a process it launches (`/launch`), and its output is
not captured: the run's own report is lost, so time it apart. File arguments are made
absolute (the launched process's directory is not this one). Self shares are over all
samples, totals and callers over those with stacks (the count is printed; it should be
nearly all). `--runs K` profiles K runs, each on the next `--games` seeds. A session's
trace holds about 5.6 s whatever the rate, so keep a launched run under that, or give
`--attach S1,S2,...`: the script starts the run itself, attaches 5 s windows from those
offsets (run i at S(i mod n)) and stops it, so a planner game is read early to late.

Writes OUT_DIR/self.csv, total.csv, callers.csv (a leaf by its nearest caller in aic code)
and edges.csv (each function's total by its nearest aic caller: `grep 'format_inner <-'`
says who formats), each as function, samples, share, and prints the top `--top` of the first three.
"""
import argparse
import collections
import csv
import glob
import json
import os
import re
import subprocess
import sys
import time
import zipfile

VS = r"C:\Program Files\Microsoft Visual Studio\18\Professional\Team Tools\DiagnosticsHub\Collector"
XPERF = r"C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe"
SESSION = "23"  # a collector session id; one killed mid-run can stay stuck, so use another
RATE = 1000  # samples a second; `--rate`
WINDOW = 5.0  # seconds profiled from each `--attach` offset (the trace holds about 5.6)


def pids(name):
    """The pids of the running processes named `name`."""
    out = subprocess.run(["tasklist", "/FI", f"IMAGENAME eq {name}", "/FO", "CSV", "/NH"],
                         capture_output=True, text=True).stdout
    return {int(row[1]) for row in csv.reader(out.splitlines()) if len(row) > 1 and row[0] == name}


def record(sim, out, args, at=None):
    """Profiles `sim args`, whole or (`at`) a window of it; returns (xperf dump path, pid)."""
    exe = os.path.join(VS, "VSDiagnostics.exe")
    config = os.path.join(out, "cpu.json")
    with open(config, "w", encoding="utf-8", newline="\n") as f:
        json.dump({"Agents": [{"CLSID": "4EA90761-2248-496C-B854-3C0399A591A4", "Name": "DiagnosticsHub.CpuAgent.dll",
                               "Config": {"cpuSampleRate": RATE}}]}, f)
    subprocess.run([exe, "stop", SESSION, "/output:" + os.path.join(out, "stale.diagsession")],
                   capture_output=True)
    # The collector walks stacks only for a process it launched (or attached to): a
    # session with no target samples everything but walks no stacks.
    args = [os.path.abspath(x) if os.path.isfile(x) else x for x in args]
    name = os.path.basename(sim)
    if at is not None:
        return attach(exe, config, sim, out, args, at)
    before = pids(name)
    subprocess.run([exe, "start", SESSION, "/launch:" + os.path.abspath(sim),
                    "/launchArgs:" + subprocess.list2cmdline(args), "/loadConfig:" + config],
                   check=True, capture_output=True)
    try:
        started = time.time()
        new = set()
        while not new and time.time() - started < 30:
            new = pids(name) - before
            time.sleep(0.05)
        if not new:
            sys.exit(f"no {name} started")
        pid = min(new)
        while pids(name) & new:
            time.sleep(0.5)
    finally:
        session = os.path.join(out, "run.diagsession")
        if os.path.exists(session):
            os.remove(session)
        subprocess.run([exe, "stop", SESSION, "/output:" + session], check=True, capture_output=True)
    return dumped(sim, out, session), pid


def attach(exe, config, sim, out, args, at):
    """Runs `sim args`, profiles WINDOW seconds of it from `at` seconds in, and stops it."""
    run = subprocess.Popen([os.path.abspath(sim)] + args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    session = os.path.join(out, "run.diagsession")
    try:
        time.sleep(at)
        if run.poll() is not None:
            sys.exit(f"the run ended before {at} s")
        subprocess.run([exe, "start", SESSION, f"/attach:{run.pid}", "/loadConfig:" + config],
                       check=True, capture_output=True)
        time.sleep(WINDOW)
    finally:
        if os.path.exists(session):
            os.remove(session)
        subprocess.run([exe, "stop", SESSION, "/output:" + session], capture_output=True)
        run.kill()
        run.wait()
    return dumped(sim, out, session), run.pid


def dumped(sim, out, session):
    """The xperf dump of a `.diagsession`, with `sim`'s symbols."""
    unpacked = os.path.join(out, "session")
    with zipfile.ZipFile(session) as z:
        z.extractall(unpacked)
    etls = glob.glob(os.path.join(unpacked, "**", "*.etl"), recursive=True)
    if not etls:
        sys.exit("no .etl in the session")
    etl = max(etls, key=os.path.getsize)
    dump = os.path.join(out, "dump.txt")
    env = dict(os.environ, _NT_SYMBOL_PATH=os.path.dirname(os.path.abspath(sim)))
    subprocess.run([XPERF, "-i", etl, "-symbols", "-o", dump, "-a", "dumper"], check=True, env=env,
                   capture_output=True)
    return dump


GENERIC = re.compile(r"<[^<>]*>")


def clean(name):
    """`aic-sim.exe!aic_sim::agents::eval::Eval::value` -> `agents::eval::Eval::value`."""
    image, _, fn = name.strip().partition("!")
    if not fn or fn.startswith("0x"):
        return image.strip() + "!?"
    while GENERIC.search(fn):
        fn = GENERIC.sub("", fn)
    for crate in ("aic_sim::", "aic_core::"):
        if fn.startswith(crate):
            fn = fn[len(crate):]
    return fn


def tally(dump, pid, run_pid=None):
    """The samples of the busiest process named `pid` (an image name, `aic-sim.exe`):
    (count, self counts, stacked count, total counts, caller counts, edges).

    Self is read off every sample. The collector keeps stack walks in a ring buffer, so only
    about the last second of a run has stacks: totals and callers are over those samples
    alone, so a profile is several short runs (`--runs`), each ending at another point.
    `callers` counts, for each function, the nearest frame below it that is in `aic`
    code (the caller of a `memcpy` or an allocation)."""
    by_process = collections.defaultdict(dict)  # process -> (timestamp, thread) -> leaf
    stacks = collections.defaultdict(list)
    with open(dump, encoding="utf-8", errors="replace") as f:
        for line in f:
            s = line.lstrip()
            if s.startswith("SampledProfile,"):
                p = [x.strip() for x in s.split(",")]
                if len(p) > 7 and p[2].startswith(pid + " ("):
                    by_process[p[2]][(p[1], p[3])] = clean(p[7])
            elif s.startswith("Stack,"):
                p = [x.strip() for x in s.split(",")]
                if len(p) > 5 and p[1] != "TimeStamp":
                    stacks[(p[1], p[2])].append(clean(p[5]))
    # The run: by its pid when known (attached), else the busiest process of that name
    # (the pid a launch shows tasklist is not reliably the one sampled).
    mine = f"{pid} ({run_pid})"
    samples = by_process.get(mine) or max(by_process.values(), key=len, default={})
    selfs = collections.Counter()
    totals = collections.Counter()
    callers = collections.Counter()
    edges = collections.Counter()
    stacked = 0
    for k, leaf in samples.items():
        selfs[leaf] += 1
        frames = stacks.get(k)
        if not frames:
            continue
        stacked += 1
        for fn in set(frames):
            totals[fn] += 1
        ours = [f for f in frames[1:] if "!" not in f and not f.startswith(("core::", "alloc::", "std::"))]
        callers[(frames[0], ours[0] if ours else "?")] += 1
        # Each function's nearest caller in aic code, once a sample: who spends a total.
        seen = set()
        for i, fn in enumerate(frames):
            up = next((f for f in frames[i + 1:] if is_ours(f) and f != fn), "?")
            if (up, fn) not in seen:
                seen.add((up, fn))
                edges[(up, fn)] += 1
    return len(samples), selfs, stacked, totals, callers, edges


def is_ours(fn):
    return "!" not in fn and not fn.startswith(("core::", "alloc::", "std::"))


def write(path, n, counts):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        w = csv.writer(f, lineterminator="\n")
        w.writerow(["function", "samples", "share"])
        for fn, c in counts.most_common():
            w.writerow([fn, c, f"{c / n:.4f}"])


def main():
    if "--" in sys.argv:
        i = sys.argv.index("--")
        own, run_args = sys.argv[1:i], sys.argv[i + 1:]
    else:
        own, run_args = sys.argv[1:], []
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("sim")
    ap.add_argument("out")
    ap.add_argument("--top", type=int, default=40)
    ap.add_argument("--runs", type=int, default=1,
                    help="profile this many runs, the run's --seed moved on by --games each time")
    ap.add_argument("--rate", type=int, default=RATE,
                    help="samples a second (the trace cap holds about 4000 samples a run)")
    ap.add_argument("--attach", help="S1,S2,...: run i is profiled for 5 s from offset S(i mod n) "
                                     "seconds in, then stopped (for runs longer than the trace holds)")
    a = ap.parse_args(own)
    globals()["RATE"] = a.rate
    offsets = [float(x) for x in a.attach.split(",")] if a.attach else None
    os.makedirs(a.out, exist_ok=True)
    n = stacked = 0
    selfs, totals, callers = collections.Counter(), collections.Counter(), collections.Counter()
    edges = collections.Counter()
    for i in range(a.runs):
        args = list(run_args)
        if i and "--seed" in args and "--games" in args:
            j = args.index("--seed") + 1
            args[j] = str(int(args[j]) + i * int(args[args.index("--games") + 1]))
        part = os.path.join(a.out, f"run{i}")
        os.makedirs(part, exist_ok=True)
        dump, pid = record(a.sim, part, args, offsets[i % len(offsets)] if offsets else None)
        k, s, st, t, c, e = tally(dump, os.path.basename(a.sim), pid if offsets else None)
        os.remove(dump)
        n, stacked = n + k, stacked + st
        selfs.update(s)
        totals.update(t)
        callers.update(c)
        edges.update(e)
        print(f"run {i}: {k} samples of pid {pid}, {st} with stacks", flush=True)
    if not n:
        sys.exit("no samples")
    write(os.path.join(a.out, "self.csv"), n, selfs)
    write(os.path.join(a.out, "total.csv"), stacked, totals)
    write(os.path.join(a.out, "callers.csv"), stacked,
          collections.Counter({f"{leaf} <- {up}": c for (leaf, up), c in callers.items()}))
    write(os.path.join(a.out, "edges.csv"), stacked,
          collections.Counter({f"{fn} <- {up}": c for (up, fn), c in edges.items()}))
    print(f"{n} samples, {stacked} with stacks")
    for title, counts, of in (("self", selfs, n), ("total (stacked samples)", totals, stacked)):
        print(f"\n| {title} | share | samples |\n|---|---|---|")
        for fn, c in counts.most_common(a.top):
            print(f"| `{fn}` | {100 * c / of:.1f}% | {c} |")
    print("\n| leaf <- nearest aic caller (stacked samples) | share |\n|---|---|")
    for (leaf, up), c in callers.most_common(a.top):
        print(f"| `{leaf}` <- `{up}` | {100 * c / stacked:.1f}% |")


if __name__ == "__main__":
    main()
