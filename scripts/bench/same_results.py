"""Checks that two experiment output directories hold the same games.

Usage: python scripts/bench/same_results.py DIR_A DIR_B [--ignore-content-hash]
           [--map OLD=NEW]...

Compares games.csv row by row and results.json field by field, ignoring
what is timing (any `ms` field or column, wall seconds, threads), record
paths and how many games a resumed run read back. Exits 1 on any
difference. For checking that a speed change left play unchanged, or that
a stopped and resumed run matches an unbroken one (012).
--ignore-content-hash is for a content refactor that renames ops but must
play the same (025).
--map is for a rename (026): each OLD becomes NEW in DIR_A's text before
parsing, in the order given, so the longest words go first. Implies
ignoring the content and config hashes, since the names are in them.
An agent string that differs (a renamed or respelled agent, 092) is no
difference when every game is otherwise identical: it is listed, and the
exit is 0.
"""

import csv
import io
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

TIMING = ("ms", "wall", "threads", "seconds")
# Run bookkeeping, not play (012 added the games_* counts); `countries` is a label 052 added,
# so an output from before it still compares. A spec's `note` is prose (145: 144 reworded
# use.json's note and the same games read as one difference).
IGNORED = ["path", "games_resumed", "games_played", "games_planned", "countries", "note"]


def timing(key):
    k = key.lower()
    return any(t in k.split("_") or k.startswith(t) or k.endswith(t) for t in TIMING)


def strip(v):
    if isinstance(v, dict):
        return {
            k: strip(x)
            for k, x in v.items()
            if not timing(k) and k not in IGNORED
        }
    if isinstance(v, list):
        return [strip(x) for x in v]
    return v


def take_maps(args):
    """Removes each `--map OLD=NEW` from `args`; returns the (OLD, NEW) pairs."""
    maps = []
    while "--map" in args:
        i = args.index("--map")
        maps.append(tuple(args[i + 1].split("=", 1)))
        del args[i:i + 2]
    return maps


def read(path, maps=()):
    with open(path, encoding="utf-8", newline="") as f:
        text = f.read()
    for old, new in maps:
        text = text.replace(old, new)
    return text


def rows(path, maps=()):
    return [
        {k: v for k, v in r.items() if not timing(k) and k not in IGNORED}
        for r in csv.DictReader(io.StringIO(read(path, maps), newline=""))
    ]


def diff(a, b, where=""):
    if type(a) is not type(b):
        return [f"{where}: {a!r} != {b!r}"]
    if isinstance(a, dict):
        out = []
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append(f"{where}.{k}: only in {'B' if k not in a else 'A'}")
            else:
                out += diff(a[k], b[k], f"{where}.{k}")
        return out
    if isinstance(a, list):
        if len(a) != len(b):
            return [f"{where}: length {len(a)} != {len(b)}"]
        out = []
        for i, (x, y) in enumerate(zip(a, b)):
            out += diff(x, y, f"{where}[{i}]")
        return out
    return [] if a == b else [f"{where}: {a!r} != {b!r}"]


def agent_names(path, maps=()):
    """Each cell's agent string, from results.json."""
    cells = json.loads(read(path, maps)).get("cells", [])
    return [c.get("agent") for c in cells]


def main():
    args = sys.argv[1:]
    if any(x in ("-h", "--help") for x in args):
        print(__doc__)
        return 0
    maps = take_maps(args)
    flagged = "--ignore-content-hash" in args
    args = [x for x in args if x != "--ignore-content-hash"]
    if flagged or maps:
        IGNORED.append("content_hash")
    if maps:
        IGNORED.append("config_hash")
    if len(args) != 2:
        sys.exit(f"needs two output directories, got {args}\n{__doc__.splitlines()[2]}")
    a, b = args[0], args[1]
    # 096: a cell whose agent string differs (renamed, respelled) compares as the other's
    # name, quoted, so a longer name holding it is untouched; the rename is listed.
    na, nb = agent_names(f"{a}/results.json", maps), agent_names(f"{b}/results.json")
    renamed = sorted({(x, y) for x, y in zip(na, nb) if x != y and x and y})
    maps = list(maps) + [(f'"{x}"', f'"{y}"') for x, y in renamed]
    problems = []
    ra, rb = rows(f"{a}/games.csv", maps), rows(f"{b}/games.csv")
    if len(ra) != len(rb):
        problems.append(f"games.csv: {len(ra)} rows != {len(rb)}")
    for i, (x, y) in enumerate(zip(ra, rb)):
        if x != y:
            problems.append(f"games.csv row {i + 2}: {x} != {y}")
    ja = strip(json.loads(read(f"{a}/results.json", maps)))
    jb = strip(json.loads(read(f"{b}/results.json")))
    problems += diff(ja, jb, "results.json")
    for p in problems[:20]:
        print(p)
    if problems:
        print(f"{len(problems)} difference(s)")
        sys.exit(1)
    for x, y in renamed:
        print(f"agent renamed, games identical: {x} -> {y}")
    print(f"same: {len(ra)} games")


if __name__ == "__main__":
    main()
