"""Writes a rules item's screen and read specs into out/ (158's specs158.py, promoted in 159; run from the repo root).

    python scripts/bench/read_specs.py offs --tag NNN --cards a,b,c
    python scripts/bench/read_specs.py screen NAME AGENTS LEVELS SEEDS --tag NNN [--offs --cards a,b,c] [--first N] [LEVER...]
    python scripts/bench/read_specs.py read TAG FIRST_SEED --tag NNN [AGENT] [--probe] [--levels 0,1,2,3] [SET...]
    any mode: [--out-dir DIR]

`--tag NNN` is the item's number: it names the read specs (`out/read-NNN-TAG-{a,b,c}.json`, what
`bench/run_read.sh` runs), the card-off folder (`out/cNNN/`) and leads each spec's note. `offs` writes
`out/cNNN/<id>_off.json` for each card of `--cards` (the baseline less that card) and `all_off.json` (less all of
them), and prints their paths. `screen` runs the three pairs on the dev seeds (36800-, or `--first N`); AGENTS is
';'-separated, LEVELS ','-separated, and a LEVER is `tag:a=1;b=2` (its `--set` entries, ';'-separated), screened beside
base. `--offs` adds every card-off config beside the baseline (a card on and off on the same seeds). `read` writes
three 48-seed sets, the three pairs, d0-d3 (or `--levels`, ','-separated), for one agent (default `strategist`), each
SET (a `--set` entry) on every variant; run them on the new and the previous revision's binary on the same fresh seeds,
and read the pair with `bench/ladder.py`. `--probe` adds 154's end-game probe on every loss. `--out-dir DIR` (from the
repo root, default `out`) writes the specs and the card-off folder there instead, e.g. `experiments/farm` for specs
`farm-seeds` runs (its FIRST overrides a spec's `first_seed`).
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", ".."))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
PROBE = {"back": 3, "agent": "", "rollouts": 0, "all_losses": True}


def offs(tag, cards, out_dir="out"):
    base = json.load(open(os.path.join(TOP, "configs", "baseline.json"), encoding="utf-8"))
    os.makedirs(os.path.join(TOP, out_dir, f"c{tag}"), exist_ok=True)
    paths = []
    for cid in cards + ["all"]:
        c = json.loads(json.dumps(base))
        drop = cards if cid == "all" else [cid]
        for m in c["modules"]:
            if m.get("module") == "cards":
                m["cards"] = [x for x in m["cards"] if x not in drop]
        c["name"] = f"{cid}_off"
        p = os.path.join(out_dir, f"c{tag}", f"{cid}_off.json")
        with open(os.path.join(TOP, p), "w", encoding="utf-8", newline="\n") as f:
            json.dump(c, f, indent=2)
            f.write("\n")
        paths.append(p.replace("\\", "/"))
    return paths


def spec(name, note, levers, diffs, agents, games, seed, probe=False, configs=None):
    variants = []
    for tag, sets in levers:
        for p, s in PAIRS.items():
            variants.append({"name": p if tag == "base" else f"{p}_{tag}", "set": s + sets})
    out = {"name": name, "note": note, "configs": configs or ["configs/baseline.json"], "variants": variants,
           "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0}
    if probe:
        out["endgame"] = PROBE
    return out


def dump(name, s, out_dir="out"):
    path = os.path.join(TOP, out_dir, name + ".json")
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")
    print(path)


def levers(args):
    out = [("base", [])]
    for a in args:
        tag, sets = a.split(":", 1)
        out.append((tag, sets.split(";")))
    return out


def take(args, flag):
    """Removes `flag VALUE` from args and returns VALUE (None if absent)."""
    if flag not in args:
        return None
    i = args.index(flag)
    if i + 1 >= len(args):
        sys.exit(f"{flag} needs a value")
    value = args[i + 1]
    del args[i:i + 2]
    return value


def span(levels):
    """`d0-d3` for a run of levels, `d1,d3` otherwise (a read spec's note)."""
    if levels == list(range(levels[0], levels[-1] + 1)) and len(levels) > 1:
        return f"d{levels[0]}-d{levels[-1]}"
    return ",".join(f"d{x}" for x in levels)


def main():
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        return
    tag = take(args, "--tag")
    cards = take(args, "--cards")
    cards = [c for c in cards.split(",") if c] if cards else None
    out_dir = take(args, "--out-dir") or "out"
    lv_read = take(args, "--levels")
    if tag is None:
        sys.exit("--tag NNN is required (the item's number: read-NNN-*, out/cNNN/)")
    os.makedirs(os.path.join(TOP, out_dir), exist_ok=True)
    mode = args[0]
    if lv_read is not None and mode != "read":
        sys.exit("--levels is for read (screen takes LEVELS)")
    if mode == "offs":
        if not cards:
            sys.exit("offs needs --cards a,b,c")
        print("\n".join(offs(tag, cards, out_dir)))
    elif mode == "screen":
        name, agents, lv, seeds = args[1], args[2], args[3], int(args[4])
        rest = args[5:]
        configs = None
        if "--offs" in rest:
            rest.remove("--offs")
            if not cards:
                sys.exit("--offs needs --cards a,b,c")
            configs = ["configs/baseline.json"] + offs(tag, cards, out_dir)
        first = 36800
        if "--first" in rest:
            i = rest.index("--first")
            first = int(rest[i + 1])
            del rest[i:i + 2]
        diffs = [int(x) for x in lv.split(",")]
        dump(name, spec(name, f"{tag} screen {name} (seeds {first}-{first + seeds - 1}).", levers(rest), diffs,
                        agents.split(";"), seeds, first, configs=configs), out_dir)
    elif mode == "read":
        rtag, seed = args[1], int(args[2])
        rest = args[3:]
        probe = "--probe" in rest
        rest = [a for a in rest if a != "--probe"]
        agent = "strategist"
        if rest and "=" not in rest[0]:
            agent = rest.pop(0)
        diffs = [int(x) for x in lv_read.split(",")] if lv_read else [0, 1, 2, 3]
        for i, half in enumerate("abc"):
            s = spec(f"read-{tag}-{rtag}-{half}", f"{tag} read ({rtag}), set {half}: seeds {seed + 48 * i}-{seed + 48 * i + 47}, the three pairs, {span(diffs)}.",
                     [("base", rest)], diffs, [agent], 48, seed + 48 * i, probe)
            dump(f"read-{tag}-{rtag}-{half}", s, out_dir)
    else:
        sys.exit(f"unknown mode {mode}: offs, screen or read")


if __name__ == "__main__":
    main()
