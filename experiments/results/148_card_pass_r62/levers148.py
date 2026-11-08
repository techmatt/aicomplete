"""148: a screen's levers against its base, per difficulty, the three seat pairs pooled.

    python experiments/results/148_card_pass_r62/levers148.py DIR

Cells are `<pair>` (base) and `<pair>_<lever>`. Per lever and difficulty: win %, the gap to the base on the same seeds
(paired, ± SE), and the round cap's share of losses (`deck_empty`).
"""

import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "147_agents_tempo"))
from read147 import load, win  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    by = defaultdict(lambda: defaultdict(dict))
    for g in load(sys.argv[1]):
        pair, _, lever = g["_pair"].partition("_")
        by[lever or "base"][g["_diff"]][(pair, g["seed"])] = g
    print("| lever | d | win | gap to base | losses: round cap |")
    print("|---|---|---|---|---|")
    for lever, diffs in by.items():
        for d, games in sorted(diffs.items()):
            gs = list(games.values())
            w, se = win(gs)
            gap = "-"
            if lever != "base":
                base = by["base"][d]
                x = [float(g["won"]) - float(base[k]["won"]) for k, g in games.items() if k in base]
                m = sum(x) / len(x)
                sd = math.sqrt(sum((v - m) ** 2 for v in x) / max(1, len(x) - 1))
                gap = f"{100 * m:+.1f} ±{100 * sd / math.sqrt(len(x)):.1f}"
            losses = [g for g in gs if not g["won"]]
            cap = sum(1 for g in losses if g["reason"] == "deck_empty")
            print(f"| {lever} | d{d} | {w:.1f} ±{se:.1f} | {gap} | {cap} of {len(losses)} |")


if __name__ == "__main__":
    main()
