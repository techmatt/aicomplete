"""Proves a refactor left play unchanged: same games, state by state where it can tell.

    python scripts/bench/same_play.py run SIM DIR [--threads N]   # play the matrix on binary SIM into DIR
    python scripts/bench/same_play.py compare SIM BASE NEW [--map OLD=NEW]...
        # SIM: the new binary

The matrix covers both configs, 1-4 countries, three difficulties, the off and still
variants, with the random, greedy, engine, rollout and planner agents (030), and one
dominance check under the planner, saving every game. `compare` checks:

- summaries match, timing and the content hash aside (renaming ops moves the hash);
- first-game traces (actions and events) match, wall time and the content hash aside;
- every game saved by the old binary replays on the new one to the same fingerprint, so
  option labels, counters and vars all survived.

--map is for a rename (026), as in `same_results.py`: each OLD becomes NEW in BASE's text.
The state's JSON holds the name, so fingerprints move and old records can't replay; each
record is compared with the new one instead, field by field, hashes and fingerprint aside.

Exits 1 on any difference. Run times are in the README; build the old binary first and
copy it aside. For experiment specs use `same_results.py`.
"""

import glob
import json
import os
import re
import subprocess
import sys

from same_results import read, take_maps

CONFIGS = ["baseline", "shared_row"]
# Fields and trace lines that differ between runs of the same play.
IGNORED = ("content_hash", "ms_per_game", "paths")
TRACE_NOISE = ("wall ms/game", "seeds ")
# `run --threads N`, to leave a shared machine room: results do not depend on it.
THREADS = []
# And what a rename moves in a record.
RENAME_IGNORED = ("config_hash", "final_fingerprint")


def sim_to(f, cmd):
    """Runs `cmd` with stdout to `f`; a refused config or a crash stops the matrix, named."""
    done = subprocess.run(cmd, stdout=f, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace")
    if done.returncode != 0:
        sys.exit(f"{' '.join(cmd)}\nfailed: {done.stderr.strip()}")


def play(sim, out, label, agent, games, config, sets):
    base = [os.path.abspath(sim), "run", f"configs/{config}.json", "--seed", "1000", "--agent", agent]
    extra = [x for s in sets for x in ("--set", s)]
    rec = os.path.join(out, "rec", label)
    with open(os.path.join(out, label + ".json"), "w", encoding="utf-8") as f:
        sim_to(f, base + extra + THREADS + ["--games", str(games), "--json", "--save", rec,
                                            "--save-when", "all", "--save-max", "1000"])
    with open(os.path.join(out, label + ".trace"), "wb") as f:
        sim_to(f, base + extra + ["--games", "1", "--threads", "1", "--trace"])


def run(sim, out):
    os.makedirs(out, exist_ok=True)
    for cfg in CONFIGS:
        for c in (1, 2, 3, 4):
            for d in (0, 2, 4):
                sets = [f"countries={c}", f"adversary.difficulty={d}"]
                play(sim, out, f"{cfg}-c{c}-d{d}-random", "random", 12, cfg, sets)
                play(sim, out, f"{cfg}-c{c}-d{d}-greedy", "greedy", 8, cfg, sets)
        play(sim, out, f"{cfg}-c2-d2-engine", "engine", 6, cfg, ["countries=2", "adversary.difficulty=2"])
        play(sim, out, f"{cfg}-c3-d1-engine", "engine", 4, cfg, ["countries=3", "adversary.difficulty=1"])
        play(sim, out, f"{cfg}-c2-d2-rollout", "rollout", 2, cfg, ["countries=2", "adversary.difficulty=2"])
        play(sim, out, f"{cfg}-c2-d2-planner", "planner", 6, cfg, ["countries=2", "adversary.difficulty=2"])
        play(sim, out, f"{cfg}-c3-d2-planner", "planner", 3, cfg, ["countries=3", "adversary.difficulty=2"])
    play(sim, out, "v-planner-never_tag", "planner:never_tag=2", 3, "baseline", ["adversary.difficulty=2"])
    for v in ("threats.variant=off", "alignment.variant=still", "alignment.variant=off", "cards.variant=off"):
        play(sim, out, "v-" + v, "greedy", 6, "baseline", [v])


def clean(v, ignored=IGNORED):
    if isinstance(v, dict):
        return {k: clean(x, ignored) for k, x in v.items()
                if k not in ignored}
    if isinstance(v, list):
        return [clean(x, ignored) for x in v]
    return v


def same_records(records, base, new, maps):
    """The renamed records against the new binary's, by parsed fields."""
    bad = 0
    ignored = IGNORED + RENAME_IGNORED
    for f in records:
        g = os.path.join(new, os.path.relpath(f, base))
        if not os.path.exists(g):
            print("record missing:", g)
            bad += 1
        elif clean(json.loads(read(f, maps)), ignored) != clean(json.loads(read(g)), ignored):
            print("record differs:", os.path.relpath(f, base))
            bad += 1
    return bad


def replays(sim, records):
    """The old binary's records on the new binary."""
    bad = 0
    for i in range(0, len(records), 40):
        p = subprocess.run([os.path.abspath(sim), "replay", *records[i:i + 40]], capture_output=True, text=True)
        lines = (p.stdout + p.stderr).splitlines()
        for line in lines:
            if ("WARNING" in line and "content hash" not in line) or "panicked" in line or "FAIL" in line:
                print(line[:200])
                bad += 1
        # Exit 1 is expected for content-hash warnings alone; anything else is a crash.
        warned = sum("content hash" in line for line in lines)
        if p.returncode not in (0, 1) or (p.returncode == 1 and warned == 0):
            print(f"replay exited {p.returncode}: {records[i]}...")
            bad += 1
    return bad


def trace_lines(text, maps):
    """A trace's comparable lines. Actions and events stay in order; under a rename the
    config hash goes and the closing table is compared as a set of rows, since a renamed
    counter sorts to another row and its padding moves."""
    lines = [x for x in text.splitlines() if not x.startswith(TRACE_NOISE)]
    if not maps:
        return lines
    events = [re.sub(r'"config_hash":"\w+"', "", x) for x in lines if x.startswith("{")]
    table = [" ".join(x.split()) for x in lines if not x.startswith(("{", "config hash"))]
    return events + sorted(table)


def compare(sim, base, new, maps=()):
    bad = 0
    ignored = IGNORED + RENAME_IGNORED if maps else IGNORED
    summaries = sorted(glob.glob(os.path.join(base, "*.json")))
    for f in summaries:
        g = os.path.join(new, os.path.basename(f))
        if clean(json.loads(read(f, maps)), ignored) != clean(json.loads(read(g)), ignored):
            print("summary differs:", os.path.basename(f))
            bad += 1
    traces = sorted(glob.glob(os.path.join(base, "*.trace")))
    for f in traces:
        g = os.path.join(new, os.path.basename(f))
        if trace_lines(read(f, maps), maps) != trace_lines(read(g), maps):
            print("trace differs:", os.path.basename(f))
            bad += 1
    records = sorted(glob.glob(os.path.join(base, "rec", "*", "*.json")))
    bad += same_records(records, base, new, maps) if maps else replays(sim, records)
    print(f"{len(summaries)} summaries, {len(traces)} traces, {len(records)} records: {bad} difference(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    argv = sys.argv[1:]
    pairs = take_maps(argv)
    if "--threads" in argv:
        i = argv.index("--threads")
        THREADS.extend(argv[i:i + 2])
        del argv[i:i + 2]
    if argv[:1] == ["run"] and len(argv) == 3:
        run(argv[1], argv[2])
    elif argv[:1] == ["compare"] and len(argv) == 4:
        sys.exit(compare(*argv[1:4], pairs))
    else:
        sys.exit(__doc__)
