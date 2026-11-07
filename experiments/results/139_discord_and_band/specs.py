"""139: write the screen and read specs (experiments/screen-139-*.json, read-139*.json).

    python experiments/results/139_discord_and_band/specs.py

Levers (Matt's, from the prompt): I3 each board's printed credits -3 (24 / 22 / 20 to 21 / 19 / 17: half of 137's +6
back), C2 `alignment.carry` 2 (a Discord tip carries its surplus), L1 `adversary.corrupt_cut_at` 1 (the corrupt cut,
138's d2 step, moved to d1). Every cell runs the three seat pairs (Builder + Protector, Builder + Researcher,
Protector + Researcher), as 136-138 did.
"""

import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
LEVERS = {
    "I3": ["ais.builder.credits=21", "ais.protector.credits=19", "ais.researcher.credits=17"],
    "C2": ["alignment.carry=2"],
    "L1": ["adversary.corrupt_cut_at=1"],
    "L0": ["adversary.corrupt_cut_at=0"],
    "T1": ["adversary.counter_pct=200,150,150"],
    "X0": ["adversary.extra_threats_at=0"],
}
DEV = 36800  # the dev seeds 136-138 screened on (36800-36815)
FRESH = 36584  # the ledger's next free seed


def variants(cells):
    out = []
    for cell in cells:
        for pair, sets in PAIRS.items():
            name = pair + ("_" + cell if cell else "")
            extra = [s for lever in (cell.split("+") if cell else []) for s in LEVERS[lever]]
            out.append({"name": name, "set": sets + extra})
    return out


def spec(name, note, cells, diffs, games, seed, path=None):
    d = {
        "name": name,
        "note": note,
        "configs": ["configs/baseline.json"],
        "variants": variants(cells),
        "difficulties": diffs,
        "agents": ["strategist"],
        "games": games,
        "first_seed": seed,
        "records": 0,
    }
    p = ROOT / "experiments" / (path or f"{name}.json")
    p.write_text(json.dumps(d, indent=2) + "\n", encoding="utf-8", newline="\n")
    print("wrote", p.relative_to(ROOT), len(d["variants"]), "variants")


def read_specs(cell, root=None, tag="r58"):
    """The fresh paired read: 48 seeds from the ledger's next free, skipping 36600-36647 (two specs, a and b)."""
    for part, seed, games in (("a", FRESH, 16), ("b", 36648, 32)):
        d = {
            "name": f"read-139-{tag}-{part}",
            "note": f"139 fresh paired read ({tag}{', ' + cell if cell else ''}): seeds {seed}-{seed + games - 1} "
                    "(with the other part, 48 a cell: 36584-36599 and 36648-36679), the three pairs, d0-d2.",
            "configs": ["configs/baseline.json"],
            "variants": variants([cell]),
            "difficulties": [0, 1, 2],
            "agents": ["strategist"],
            "games": games,
            "first_seed": seed,
            "records": 0,
        }
        base = Path(root) if root else ROOT
        p = base / "experiments" / f"read-139-{tag}-{part}.json"
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(json.dumps(d, indent=2) + "\n", encoding="utf-8", newline="\n")
        print("wrote", p)


def main():
    import sys
    if len(sys.argv) > 1 and sys.argv[1] == "--read":
        # --read CELL [ROOT TAG]: e.g. --read I3+L1, or --read "" out/r57src r57
        read_specs(sys.argv[2], *(sys.argv[3:5]))
        return
    three = ["I3", "C2", "L1"]
    factorial = [""] + ["+".join(c) for k in (1, 2, 3) for c in itertools.combinations(three, k)]
    spec("screen-139-b",
         "139 screen B (dev seeds 36800-36815, 16 a cell, the three pairs, d0) on r58 as built: I3 and C2 as in "
         "screen A (L1 doesn't act at d0), and L0 (corrupt_cut_at 0: the cut at every level) as a factorial.",
         [""] + ["+".join(c) for k in (1, 2, 3) for c in itertools.combinations(["I3", "C2", "L0"], k)], [0], 16, DEV)
    spec("screen-139-d1",
         "139 screen D1 (Matt's message 5865; dev seeds 36816-36831, 16 more a cell, the three pairs, d0-d1): the "
         "corrupt cut at every level (L0) alone, with I3 and with C2. L0's d2 is L1's (the dial is one).",
         ["L0", "I3+L0", "C2+L0"], [0, 1], 16, DEV + 16)
    spec("screen-139-d2",
         "139 screen D2 (Matt's message 5865; dev seeds 36800-36831, 32 a cell, the three pairs, d0-d2): T1, tier-1 "
         "counters at 200% (d0 deals mostly tier 1), with L0 and with I3+L0.",
         ["L0+T1", "I3+L0+T1"], [0, 1, 2], 32, DEV)
    spec("screen-139-e",
         "139 screen E (dev seeds 36800-36831, 32 a cell, the three pairs, d0) on r58 frozen (I3 + L1 in the baseline): "
         "X0, the extra threats moved from d1 to d0 (extra_threats_at 0); d1 and d2 play as before.",
         ["", "X0"], [0], 32, DEV)
    spec("screen-139-c",
         "139 screen C (dev seeds 36816-36831, 16 more a cell, the three pairs, d0-d2) on r58 as built: the "
         "finalists I3+L1 and C2+L1 beside the asked cell I3+C2+L1, to cut screen A-B's noise.",
         ["I3+L1", "C2+L1", "I3+C2+L1"], [0, 1, 2], 16, DEV + 16)
    spec("screen-139-a",
         "139 screen A (dev seeds 36800-36815, 16 a cell, the three pairs, d1-d2) on r58 as built: Matt's three levers "
         "I3 (board credits -3), C2 (Discord carry 2), L1 (corrupt_cut_at 1) as a full factorial.",
         factorial, [1, 2], 16, DEV)


if __name__ == "__main__":
    main()
