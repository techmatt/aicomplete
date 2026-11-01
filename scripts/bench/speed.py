"""Games per minute for each agent, and a standing check that agents have not slowed.

    python scripts/bench/speed.py table SIM [--out FILE] [--only "AGENT[ cN]"]... [--games N]
        # the full table: every agent at c2 and c3, d2, one thread and --threads;
        # about 20 min, mostly `rollout`, so `--only` and an `--out` that keeps its rows
    python scripts/bench/speed.py pair OLD NEW [--old-config FILE] [--out FILE] [--only "AGENT[ cN]"]... [--games N]
        # the same rows on two binaries in turn: use this for a before and after, because
        # the ratio holds on a busy machine and two `table` runs at different times don't
    python scripts/bench/speed.py specs SIM SPEC_A SPEC_B [--countries 2,3] [--games N] [--out FILE]
        # two agent specs on one binary, in turn on the same seeds (planner:deals=1,lead=0 against
        # planner): games a minute on one thread and on --threads, and B's cost in A's games;
        # four planner runs at c3 on one thread are long
    python scripts/bench/speed.py compare BEFORE.json AFTER.json   # two `table` files
    python scripts/bench/speed.py check SIM                        # about 2 min, see below
    python scripts/bench/speed.py check SIM --update               # after a change meant to cost time

`--threads N` caps `table` and `pair`'s many-thread column: the default leaves 4 hardware
threads free. `check` always runs on one. `--games N` sets the games of the --threads column in
`table`, `pair` and `specs`; the one-thread column keeps its games. Each row prints its games
first, with a rough time when the baseline has the agent.

`check` runs a small fixed set on one thread, best of three (the planner: four seeds, best of
two), and fails if an agent's ms/game is more than TOLERANCE over
`scripts/bench/speed_baseline.json`. It needs a quiet machine: anything heavy alongside reads
as a slowdown. It only holds on the same games: the baseline records the rules revision and what
each agent's check games came to (`PLAY_FIELDS`), so a new revision, or a changed agent default,
weight or config at the same revision, moves ms/game with game length and `check` then gives no
verdict on that agent and names why (read `per_decision.py`). A baseline without these records
gives no verdict until `--update`.

`pair` plays whole games, so it has the same limit: across a rules revision or a config change
its ratio mixes speed with game length, and it says so. An OLD that refuses today's config
takes the one it was built with through `--old-config`. A row whose games came out differently
on the same revision and config says so too. `same_play.py` is the check that play held.
"""

import hashlib
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
# What a `run --json` result says about the games, timing aside. The seeds and the agent are
# fixed, so these move only if play did. Counters stay out: adding one is no change of play.
PLAY_FIELDS = ("wins", "rounds_mean", "decisions_mean", "options_per_decision", "forced_share",
               "max_options", "ended", "end_round")
BASELINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "speed_baseline.json")


def default_threads():
    """Every hardware thread but 4, for what else the box is running."""
    return max(1, (os.cpu_count() or 1) - 4)


def played(sim, agent, games, countries, threads, config=CONFIG):
    """Wall seconds for `games` games, and a hash of what they came to."""
    cmd = [os.path.abspath(sim), "run", config, "--seed", SEED, "--agent", agent, "--games", str(games),
           "--threads", str(threads), "--json", "--set", f"countries={countries}",
           "--set", "adversary.difficulty=2"]
    t = time.perf_counter()
    r = subprocess.run(cmd, capture_output=True, encoding="utf-8", errors="replace")
    secs = time.perf_counter() - t
    if r.returncode:
        sys.exit(f"{sim} failed on {config}: {r.stderr.strip()[:500]}")
    result = json.loads(r.stdout)["results"][0]
    text = json.dumps([result[k] for k in PLAY_FIELDS], sort_keys=True)
    return secs, hashlib.sha256(text.encode()).hexdigest()[:12]


def timed(sim, agent, games, countries, threads, config=CONFIG):
    """Wall seconds for `games` games."""
    return played(sim, agent, games, countries, threads, config)[0]


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


def rows_to_run(only, threads, games=0):
    """Each (agent, countries, games on one thread, games on `threads`) `only` lets through.
    `games`, if set, is the games on `threads`."""
    for agent, one, many in TABLE:
        for c in (2, 3):
            if only and agent not in only and f"{agent} c{c}" not in only:
                continue
            # Three countries take about three times as long: fewer games, a like wait.
            yield agent, c, max(1, one * 2 // c // (c - 1)), games or max(threads, many * 2 // c // (c - 1))


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


def announce(name, agent, c, n1, nn, threads, runs):
    """What a row will play, before it starts, and a rough time if the baseline has `agent`:
    its ms/game at c2, about three times that at c3, with `runs` passes over both columns."""
    ms = saved(BASELINE).get(agent.split(":")[0])
    secs = ms * (n1 + nn / threads) * runs * (3 if c >= 3 else 1) / 1000 if ms else 0
    rough = f", roughly {secs / 60:.0f} min" if secs >= 90 else f", roughly {secs:.0f} s" if secs else ""
    print(f"{name}: {n1} on 1 thread, {nn} on {threads}{rough}", flush=True)


def table(sim, out, only, threads, games=0):
    """Saves after each row, into what `out` already holds, so a stopped run keeps its rows."""
    rows = saved(out)
    for agent, c, n1, nn in rows_to_run(only, threads, games):
        announce(f"{agent} c{c}", agent, c, n1, nn, threads, 1)
        s1 = timed(sim, agent, n1, c, 1)
        sn = timed(sim, agent, nn, c, threads)
        rows[f"{agent} c{c}"] = {"gpm_1": 60 * n1 / s1, "gpm_all": 60 * nn / sn, "games_1": n1, "games_all": nn}
        print(f"{agent} c{c}: {60 * n1 / s1:.1f} games/min on 1 thread ({n1} in {s1:.1f} s), "
              f"{60 * nn / sn:.1f} on {threads} ({nn} in {sn:.1f} s)", flush=True)
        save(out, rows)


def pair(old, new, out, only, threads, old_config, games=0):
    """Each row on both binaries in turn, old-new-new-old, so both see the same machine: a
    busy box moves the rates but not the ratio. Saves after each row, as `table`."""
    rows = saved(out)
    before, after = plays(old, old_config), plays(new)
    if before != after:
        print(f"NOT LIKE FOR LIKE: r{before[0]} on {old_config} against r{after[0]} on {CONFIG}. The two play different "
              "games, so a ratio below is speed times game length. per_decision.py splits them.", flush=True)
    for agent, c, n1, nn in rows_to_run(only, threads, games):
        announce(f"{agent} c{c}", agent, c, n1, nn, threads, 4)
        r = {}
        same = True
        for label, n, t in (("1", n1, 1), ("all", nn, threads)):
            first, was = played(old, agent, n, c, t, old_config)
            b, now = played(new, agent, n, c, t)
            b += timed(new, agent, n, c, t)
            a = first + timed(old, agent, n, c, t, old_config)
            same = same and was == now
            r[f"old_{label}"], r[f"new_{label}"] = 120 * n / a, 120 * n / b
        rows[f"{agent} c{c}"] = r
        if before == after and not same:
            print(f"NOT LIKE FOR LIKE: {agent} c{c} played other games on the two binaries at the same revision and "
                  "config, so a default or weight moved. Its ratio is speed times game length.", flush=True)
        print(f"{agent} c{c}: 1 thread {r['old_1']:.1f} to {r['new_1']:.1f} ({r['new_1'] / r['old_1']:.2f} x), "
              f"{threads} threads {r['old_all']:.1f} to {r['new_all']:.1f} "
              f"({r['new_all'] / r['old_all']:.2f} x)", flush=True)
        save(out, rows)


def specs(sim, a, b, out, threads, countries, games):
    """Two specs on one binary, a-b-b-a as `pair`, on the same seeds, so the ratio holds under
    load. The games differ with the agent: the ratio is whole games, per_decision.py splits it."""
    rows = saved(out)
    kind = a.split(":")[0]
    one, many = next(((x, y) for k, x, y in TABLE if k == kind), (TABLE[-1][1], TABLE[-1][2]))
    for c in countries:
        n1, nn = max(1, one * 2 // c // (c - 1)), games or max(threads, many * 2 // c // (c - 1))
        announce(f"c{c}", kind, c, n1, nn, threads, 4)
        r = {}
        for label, n, t in (("1", n1, 1), ("all", nn, threads)):
            first = timed(sim, a, n, c, t)
            secs_b = timed(sim, b, n, c, t) + timed(sim, b, n, c, t)
            secs_a = first + timed(sim, a, n, c, t)
            r[f"a_{label}"], r[f"b_{label}"] = 120 * n / secs_a, 120 * n / secs_b
        rows[f"c{c}"] = {"a": a, "b": b, **r}
        print(f"c{c}: 1 thread {r['a_1']:.1f} to {r['b_1']:.1f} games/min ({r['a_1'] / r['b_1']:.2f} x the time), "
              f"{threads} threads {r['a_all']:.1f} to {r['b_all']:.1f} ({r['a_all'] / r['b_all']:.2f} x)", flush=True)
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
    now, came_to = {}, {}
    for agent, games, c, runs in CHECK:
        passes = [played(sim, agent, games, c, 1) for _ in range(runs)]
        now[agent] = 1000 * min(secs for secs, _ in passes) / games
        came_to[agent] = passes[0][1]
    rev = plays(sim)[0]
    if update:
        with open(BASELINE, "w", encoding="utf-8", newline="\n") as f:
            json.dump({"rules_revision": rev, "games": came_to, **{k: round(v, 1) for k, v in now.items()}}, f, indent=1)
            f.write("\n")
    with open(BASELINE, encoding="utf-8") as f:
        base = json.load(f)
    same_rev = base.get("rules_revision") == rev
    recorded = base.get("games", {})
    moved = [a for a in now if recorded.get(a) != came_to[a]]
    bad = 0
    for agent, ms in now.items():
        ratio = ms / base[agent]
        slow = same_rev and agent not in moved and ratio > TOLERANCE
        bad += slow
        print(f"{agent}: {ms:.1f} ms/game, {ratio:.2f} x baseline{'  SLOWER' if slow else ''}")
    if not same_rev:
        print(f"NO VERDICT: the baseline is r{base.get('rules_revision', '?')} and {sim} is r{rev}, so these are other "
              "games. Read per_decision.py for the cost, then --update.")
    elif not recorded:
        print("NO VERDICT: the baseline does not record its games. Run --update on a binary known to be fine.")
    elif moved:
        print(f"NO VERDICT on {', '.join(moved)}: the same revision, r{rev}, but other games than the baseline's, so "
              "an agent default, a weight or the config moved. Read per_decision.py for the cost, then --update.")
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
    countries = [int(c) for c in (take(argv, "--countries") or ["2,3"])[-1].split(",")]
    games = int((take(argv, "--games") or [0])[-1])
    update = "--update" in argv
    argv = [a for a in argv if a != "--update"]
    if threads < 1 or any(a.startswith("--") for a in argv):
        sys.exit(__doc__)
    if games and argv[:1] in (["check"], ["compare"]):
        sys.exit(f"--games does not apply to {argv[0]}")
    if argv[:1] == ["table"] and len(argv) == 2:
        table(argv[1], out, only, threads, games)
    elif argv[:1] == ["pair"] and len(argv) == 3:
        pair(argv[1], argv[2], out, only, threads, old_config, games)
    elif argv[:1] == ["specs"] and len(argv) == 4:
        specs(argv[1], argv[2], argv[3], out, threads, countries, games)
    elif argv[:1] == ["compare"] and len(argv) == 3:
        compare(argv[1], argv[2])
    elif argv[:1] == ["check"] and len(argv) == 2:
        sys.exit(check(argv[1], update))
    else:
        sys.exit(__doc__)
