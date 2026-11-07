"""142: write the screen and read specs (experiments/screen-142-*.json, read-142-*.json).

    python experiments/results/142_threat_intensity/specs.py screen NAME CELL[,CELL...] [--diffs 0,1,2,3] [--games 16] [--seed 36800]
    python experiments/results/142_threat_intensity/specs.py read|confirm|confirm2|confirm3 TAG [--root DIR] [--cell CELL]

Cells name levers in LEVERS joined by '+' ('' or 'base' is the build as is). Every cell runs the three seat pairs
(Builder + Protector, Builder + Researcher, Protector + Researcher), as 136-139 did. The fresh read is 48 seeds from
the ledger's next free: 36680-36699 (part a, 20) and 36748-36775 (part b, 28), d0-d3; the confirming read 36776-36799 and 36832-36855.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
# Levers screened on and off; each is a list of --set overrides.
LEVERS = {
    "base": [],
}
LEVERS_FILE = Path(__file__).with_name("levers.json")
if LEVERS_FILE.exists():
    LEVERS.update(json.loads(LEVERS_FILE.read_text(encoding="utf-8")))
DEV = 36800
FRESH = (("a", 36680, 20), ("b", 36748, 28))
# The confirming read (Matt's message 6016: the first fresh read picked B2 and F1): the ledger's next free seeds.
CONFIRM = (("a", 36776, 24), ("b", 36832, 24))
# A second independent read of the final build (no screen or pick used these): 36856-36903.
CONFIRM2 = (("a", 36856, 24), ("b", 36880, 24))
# The d3 confirm of the second counter step (M13, picked on the dev seeds after CONFIRM and CONFIRM2 read d3 58.7).
CONFIRM3 = (("a", 36904, 24), ("b", 36928, 24))


def variants(cells):
    out = []
    for cell in cells:
        levers = [x for x in cell.split("+") if x and x != "base"]
        for pair, sets in PAIRS.items():
            name = pair + ("_" + "+".join(levers) if levers else "")
            extra = [s for lever in levers for s in LEVERS[lever]]
            out.append({"name": name, "set": sets + extra})
    return out


def write(d, base):
    p = base / "experiments" / f"{d['name']}.json"
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(d, indent=2) + "\n", encoding="utf-8", newline="\n")
    print("wrote", p)


def arg(name, default):
    if name in sys.argv:
        return sys.argv[sys.argv.index(name) + 1]
    return default


def main():
    kind = sys.argv[1]
    diffs = [int(x) for x in arg("--diffs", "0,1,2,3").split(",")]
    if kind == "screen":
        name, cells = sys.argv[2], sys.argv[3].split(",")
        games, seed = int(arg("--games", "16")), int(arg("--seed", str(DEV)))
        write({"name": f"screen-142-{name}", "note": f"142 screen {name}: {cells}, seeds {seed}+{games}",
               "configs": ["configs/baseline.json"], "variants": variants(cells), "difficulties": diffs,
               "agents": ["strategist"], "games": games, "first_seed": seed, "records": 0}, ROOT)
    elif kind in ("read", "confirm", "confirm2", "confirm3"):
        tag = sys.argv[2]
        base = Path(arg("--root", str(ROOT)))
        cell = arg("--cell", "")
        for part, seed, games in {"read": FRESH, "confirm": CONFIRM, "confirm2": CONFIRM2, "confirm3": CONFIRM3}[kind]:
            write({"name": f"read-142-{tag}-{part}",
                   "note": f"142 fresh paired read ({tag}{', ' + cell if cell else ''}): seeds {seed}-{seed + games - 1} "
                           f"(with the other part, 48 a cell), the three pairs, d0-d3.",
                   "configs": ["configs/baseline.json"], "variants": variants([cell]), "difficulties": diffs,
                   "agents": ["strategist"], "games": games, "first_seed": seed, "records": 0}, base)


if __name__ == "__main__":
    main()
