"""147: each cell's paired win gap against one base cell of the same run(s), same pair, difficulty and seed.

    python experiments/results/147_agents_tempo/pairs147.py BASE_AGENT DIR [DIR ...]

BASE_AGENT is the agent string as the spec writes it (or a unique substring of it).
"""

import math
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from read147 import load  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    base_name, dirs = sys.argv[1], sys.argv[2:]
    games = [g for d in dirs for g in load(d)]
    agents = sorted({g["_agent"] for g in games})
    base = [a for a in agents if a == base_name] or [a for a in agents if base_name in a]
    if len(base) != 1:
        sys.exit(f"base {base_name!r} matches {base}")
    base = base[0]
    won = {(g["_pair"], g["_diff"], g["seed"]): float(g["won"]) for g in games if g["_agent"] == base}
    cells = defaultdict(list)
    for g in games:
        k = (g["_pair"], g["_diff"], g["seed"])
        if g["_agent"] != base and k in won:
            cells[(g["_diff"], g["_agent"])].append(float(g["won"]) - won[k])
    print(f"base: {base}")
    for (diff, agent), d in sorted(cells.items()):
        m = sum(d) / len(d)
        sd = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1))
        print(f"- d{diff} {agent}: {100 * m:+.1f} ±{100 * sd / math.sqrt(len(d)):.1f} ({len(d)})")


if __name__ == "__main__":
    main()
