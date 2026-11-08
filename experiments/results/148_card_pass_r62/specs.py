"""148: write the screen and read specs (experiments/screen-148-*.json, read-148-*.json).

    python experiments/results/148_card_pass_r62/specs.py screen NAME AGENT[;AGENT...] [--diffs 0,1,2,3] [--games 32] [--seed 36800] [--lever NAME=K=V,K=V]
    python experiments/results/148_card_pass_r62/specs.py final|final2 TAG [AGENT;...]

Every cell runs the three seat pairs (Builder + Protector, Builder + Researcher, Protector + Researcher). Screens run
on the dev seeds (36800-36831) unless --seed says otherwise. The final read (F1) is two 48-seed sets from the ledger's
next free (37528-37575, 37576-37623); a re-read after the d2 step (F2) is the next two (37624-37671, 37672-37719).
Each plays the r62 build and the r61 binary (its own configs, run from its own folder) on the same seeds.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
DEV = 36800
FINAL = (("a", 37528, 48), ("b", 37576, 48))
FINAL2 = (("a", 37624, 48), ("b", 37672, 48))


def spec(name, note, agents, diffs, games, seed, levers=None):
    """`levers`: name -> config sets, each run on every pair as its own variant (`<pair>_<name>`)."""
    variants = [{"name": p, "set": s} for p, s in PAIRS.items()]
    for lever, sets in (levers or {}).items():
        variants += [{"name": f"{p}_{lever}", "set": s + sets} for p, s in PAIRS.items()]
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
            "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0}


def write(name, doc, out=None):
    p = Path(out) if out else ROOT / "experiments" / f"{name}.json"
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(p)


def main():
    a = sys.argv[1:]
    opt = lambda k, d: a[a.index(k) + 1] if k in a else d
    out = opt("--out", None)
    if a[0] == "screen":
        name, agents = a[1], a[2].split(";")
        diffs = [int(x) for x in opt("--diffs", "0,1,2,3").split(",")]
        games, seed = int(opt("--games", "32")), int(opt("--seed", str(DEV)))
        levers = {}
        for i, x in enumerate(a):
            if x == "--lever":
                lever, _, sets = a[i + 1].partition("=")
                levers[lever] = sets.split(",")
        write(f"screen-148-{name}", spec(f"screen-148-{name}", f"148 screen {name}: {'; '.join(agents)}, seeds from {seed}",
                                         agents, diffs, games, seed, levers), out)
    elif a[0] in ("final", "final2"):
        tag = a[1]
        agents = a[2].split(";") if len(a) > 2 and not a[2].startswith("--") else ["strategist"]
        for part, seed, games in {"final": FINAL, "final2": FINAL2}[a[0]]:
            write(f"read-148-{tag}-{part}", spec(f"read-148-{tag}-{part}",
                                                 f"148 final paired read ({tag}), set {part}: seeds {seed}-{seed + games - 1}, the three pairs, d0-d3.",
                                                 agents, [0, 1, 2, 3], games, seed),
                  (Path(out) / f"read-148-{tag}-{part}.json") if out else None)


if __name__ == "__main__":
    main()
