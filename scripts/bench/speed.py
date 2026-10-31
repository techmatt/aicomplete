"""Games per minute for each agent, and a standing check that agents have not slowed.

    python scripts/bench/speed.py table SIM [--out FILE] [--only "AGENT[ cN]"]...
        # the full table: every agent at c2 and c3, d2, one thread and --threads;
        # about 20 min, mostly `rollout`, so `--only` and an `--out` that keeps its rows
    python scripts/bench/speed.py pair OLD NEW [--old-config FILE] [--out FILE] [--only "AGENT[ cN]"]...
        # the same rows on two binaries in turn: use this for a before and after, because
        # the ratio holds on a busy machine and two `table` runs at different times don't
    python scripts/bench/speed.py compare BEFORE.json AFTER.json   # two `table` files
    python scripts/bench/speed.py check SIM                        # about 2 min, see below
    python scripts/bench/speed.py check SIM --update               # after a change meant to cost time

`--threads N` caps `table` and `pair`'s many-thread column: the default leaves 4 hardware
threads free. `check` always runs on one.

`check` runs a small fixed set on one thread, best of three (the planner: four seeds, best of
two), and fails if an agent's ms/game is more than TOLERANCE over
`scripts/bench/speed_baseline.json`. It needs a quiet machine: anything heavy alongside reads
as a slowdown. It only holds within one rules revision: another revision plays other games, so
ms/game moves with their length, and `check` then gives no verdict (read `per_decision.py`).

`pair` plays whole games, so it has the same limit: across a rules revision or a config change
its ratio mixes speed with game length, and it says so. An OLD that refuses today's config
takes the one it was built with through `--old-config`. `same_play.py` is the check that play held.
"""

import json
import os
import re
import subprocess
import sys
import time

CONFIG = "configs/baseline.json"
SEED = "1000"
# Agent, then games on one thread and on every thread. Enough to fill 10-60 s each;
# rollout takes minutes, see the README.
TABLE = [
    ("random", 400, 4000),
    ("greedy", 16, 200),
    ("engine", 4, 48),
    ("rollout", 2, 12),
    ("planner", 2, 24),
]
# Agent, games, countries, runs. One planner game is too few: a rules change that moves its
# length reads as slowness (042: r18 to r20 read 6.7 x on one seed and 1.75 x on 16).
CHECK = [("random", 200, 2, 3), ("greedy", 6, 2, 3), ("engine", 2, 2, 3), ("planner", 4, 2, 2)]
TOLERANCE = 1.3
BASELINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "speed_baseline.json")


def default_threads():
    """Every hardware thread but 4, for what else the box is running."""
    return max(1, (os.cpu_count() or 1) - 4)


def timed(sim, agent, games, countries, threads, config=CONFIG):
    """Wall seconds for `games` games."""
    cmd = [os.path.abspath(sim), "run", config, "--seed", SEED, "--agent", agent, "--games", str(games),
           "--threads", str(threads), "--json", "--set", f"countries={countries}",
           "--set", "adversary.difficulty=2"]
    t = time.perf_counter()
    r = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, encoding="utf-8", errors="replace")
    if r.returncode:
        sys.exit(f"{sim} failed on {config}: {r.stderr.strip()[:500]}")
    return time.perf_counter() - t


def plays(sim, config=CONFIG):
    """What fixes the games `sim` plays on `config`: its rules revision and the config's hash.
    Content moves without a revision bump only in words, so its hash stays out."""
    r = subprocess.run([os.path.abspath(sim), "show", config], capture_output=True, encoding="utf-8", errors="replace")
    m = re.search(r"config hash (\w+)\s+content hash \w+\s+rules revision (\d+)", r.stdout)
    if not m:
        why = r.stderr.strip().split("\n")[0]
        sys.exit(f"{sim} cannot read {config}: {why}\n"
                 f"An older binary reads the config it was built with: git show <rev>:{CONFIG} > out/old-baseline.json, "
                 f"then pair --old-config out/old-baseline.json.")
    return int(m.group(2)), m.group(1)


def rows_to_run(only, threads):
    """Each (agent, countries, games on one thread, games on `threads`) `only` lets through."""
    for agent, one, many in TABLE:
        for c in (2, 3):
            if only and agent not in only and f"{agent} c{c}" not in only:
                continue
            # Three countries take about three times as long: fewer games, a like wait.
            yield agent, c, max(1, one * 2 // c // (c - 1)), max(threads, many * 2 // c // (c - 1))


def saved(out):
    """The rows `out` already holds."""
    if out and os.path.exists(out):
        with open(out, encoding="utf-8") as f:
            return json.load(f)
    return {}


def save(out, rows):
    if out:
        with open(out, "w", encoding="utf-8") as f:
            json.dump(rows, f, indent=1)


def table(sim, out, only, threads):
    """Saves after each row, into what `out` already holds, so a stopped run keeps its rows."""
    rows = saved(out)
    for agent, c, n1, nn in rows_to_run(only, threads):
        s1 = timed(sim, agent, n1, c, 1)
        sn = timed(sim, agent, nn, c, threads)
        rows[f"{agent} c{c}"] = {"gpm_1": 60 * n1 / s1, "gpm_all": 60 * nn / sn, "games_1": n1, "games_all": nn}
        print(f"{agent} c{c}: {60 * n1 / s1:.1f} games/min on 1 thread ({n1} in {s1:.1f} s), "
              f"{60 * nn / sn:.1f} on {threads} ({nn} in {sn:.1f} s)", flush=True)
        save(out, rows)


def pair(old, new, out, only, threads, old_config):
    """Each row on both binaries in turn, old-new-new-old, so both see the same machine: a
    busy box moves the rates but not the ratio. Saves after each row, as `table`."""
    rows = saved(out)
    before, after = plays(old, old_config), plays(new)
    if before != after:
        print(f"NOT LIKE FOR LIKE: r{before[0]} on {old_config} against r{after[0]} on {CONFIG}. The two play different "
              "games, so a ratio below is speed times game length. per_decision.py splits them.", flush=True)
    for agent, c, n1, nn in rows_to_run(only, threads):
        r = {}
        for label, games, t in (("1", n1, 1), ("all", nn, threads)):
            first = timed(old, agent, games, c, t, old_config)
            b = timed(new, agent, games, c, t) + timed(new, agent, games, c, t)
            a = first + timed(old, agent, games, c, t, old_config)
            r[f"old_{label}"], r[f"new_{label}"] = 120 * games / a, 120 * games / b
        rows[f"{agent} c{c}"] = r
        print(f"{agent} c{c}: 1 thread {r['old_1']:.1f} to {r['new_1']:.1f} ({r['new_1'] / r['old_1']:.2f} x), "
              f"{threads} threads {r['old_all']:.1f} to {r['new_all']:.1f} "
              f"({r['new_all'] / r['old_all']:.2f} x)", flush=True)
        save(out, rows)


def compare(before, after):
    with open(before, encoding="utf-8") as f:
        a = json.load(f)
    with open(after, encoding="utf-8") as f:
        b = json.load(f)
    print("| agent | 1 thread before | after | x | all threads before | after | x |")
    print("|---|---|---|---|---|---|---|")
    for k in a:
        if k in b:
            x, y = a[k], b[k]
            print(f"| {k} | {x['gpm_1']:.1f} | {y['gpm_1']:.1f} | {y['gpm_1'] / x['gpm_1']:.2f} "
                  f"| {x['gpm_all']:.1f} | {y['gpm_all']:.1f} | {y['gpm_all'] / x['gpm_all']:.2f} |")


def check(sim, update):
    now = {}
    for agent, games, c, runs in CHECK:
        now[agent] = 1000 * min(timed(sim, agent, games, c, 1) for _ in range(runs)) / games
    rev = plays(sim)[0]
    if update:
        with open(BASELINE, "w", encoding="utf-8", newline="\n") as f:
            json.dump({"rules_revision": rev, **{k: round(v, 1) for k, v in now.items()}}, f, indent=1)
            f.write("\n")
    with open(BASELINE, encoding="utf-8") as f:
        base = json.load(f)
    held = base.get("rules_revision") == rev
    bad = 0
    for agent, ms in now.items():
        ratio = ms / base[agent]
        slow = held and ratio > TOLERANCE
        bad += slow
        print(f"{agent}: {ms:.1f} ms/game, {ratio:.2f} x baseline{'  SLOWER' if slow else ''}")
    if not held:
        print(f"NO VERDICT: the baseline is r{base.get('rules_revision', '?')} and {sim} is r{rev}, so these are other "
              "games. Read per_decision.py for the cost, then --update.")
    return 1 if bad else 0


def take(argv, name):
    """Every value of `name`, out of `argv`, so flags can go anywhere."""
    values = []
    while name in argv:
        i = argv.index(name)
        if i + 1 == len(argv):
            sys.exit(f"{name} needs a value")
        values.append(argv[i + 1])
        del argv[i:i + 2]
    return values


if __name__ == "__main__":
    argv = sys.argv[1:]
    out = (take(argv, "--out") or [None])[-1]
    only = take(argv, "--only")
    threads = int((take(argv, "--threads") or [default_threads()])[-1])
    old_config = (take(argv, "--old-config") or [CONFIG])[-1]
    update = "--update" in argv
    argv = [a for a in argv if a != "--update"]
    if threads < 1 or any(a.startswith("--") for a in argv):
        sys.exit(__doc__)
    if argv[:1] == ["table"] and len(argv) == 2:
        table(argv[1], out, only, threads)
    elif argv[:1] == ["pair"] and len(argv) == 3:
        pair(argv[1], argv[2], out, only, threads, old_config)
    elif argv[:1] == ["compare"] and len(argv) == 3:
        compare(argv[1], argv[2])
    elif argv[:1] == ["check"] and len(argv) == 2:
        sys.exit(check(argv[1], update))
    else:
        sys.exit(__doc__)
