"""146: write the screen and read specs (experiments/screen-146-*.json, read-146-*.json).

    python experiments/results/146_threat_standard/specs.py screen NAME CELL[,CELL...] [--diffs 0,1,2,3] [--games 32]
    python experiments/results/146_threat_standard/specs.py read|confirm|final TAG

Cells name levers in LEVERS joined by '+' ('base' is the build as is). Every cell runs the three seat pairs
(Builder + Protector, Builder + Researcher, Protector + Researcher), the strategist. Screens run on the dev seeds
(36800-36831); the read is two independent 48-seed sets from the ledger's next free (36952-36999, 37000-37047), the
confirming read of the final build the next two (37048-37095, 37096-37143).
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
LEVERS = json.loads(Path(__file__).with_name("levers.json").read_text(encoding="utf-8"))
DEV = 36800
READ = (("a", 36952, 48), ("b", 37000, 48))
# The confirming read of the final build (the first read picked L1, the split counter ladder): the next free seeds.
CONFIRM = (("a", 37048, 48), ("b", 37096, 48))
# Matt's message 6254 (the Discord cards' two tier-2s) moved d2 out of band on CONFIRM: the final build's read.
FINAL = (("a", 37144, 48), ("b", 37192, 48))


def variants(cells):
    out = []
    for cell in cells:
        levers = [x for x in cell.split("+") if x and x != "base"]
        for pair, sets in PAIRS.items():
            name = pair + ("_" + "+".join(levers) if levers else "")
            extra = [s for lever in levers for s in LEVERS[lever]]
            out.append({"name": name, "set": sets + extra})
    return out


def spec(name, note, cells, diffs, games, seed):
    return {"name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants(cells),
            "difficulties": diffs, "agents": ["strategist"], "games": games, "first_seed": seed, "records": 0}


def write(name, doc):
    p = ROOT / "experiments" / f"{name}.json"
    p.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(p.relative_to(ROOT))


def main():
    a = sys.argv[1:]
    opt = lambda k, d: a[a.index(k) + 1] if k in a else d
    if a[0] == "screen":
        name, cells = a[1], a[2].split(",")
        diffs = [int(x) for x in opt("--diffs", "0,1,2,3").split(",")]
        games = int(opt("--games", "32"))
        write(f"screen-146-{name}", spec(f"screen-146-{name}", f"146 screen {name} on the dev seeds: {', '.join(cells)}",
                                         cells, diffs, games, DEV))
    elif a[0] in ("read", "confirm", "final"):
        tag = a[1]
        for part, seed, games in {"read": READ, "confirm": CONFIRM, "final": FINAL}[a[0]]:
            write(f"read-146-{tag}-{part}", spec(f"read-146-{tag}-{part}",
                                                 f"146 independent paired read ({tag}), set {part}: seeds {seed}-{seed + games - 1}, the three pairs, d0-d3.",
                                                 ["base"], [0, 1, 2, 3], games, seed))


if __name__ == "__main__":
    main()
