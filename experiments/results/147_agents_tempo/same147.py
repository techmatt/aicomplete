"""147: whether an agent cell of one run plays the same games as one of another (won, reason, rounds, decisions,
options), by pair, difficulty and seed. For checking that the r61-equivalent spec on 147's binary is r61's agents.

    python experiments/results/147_agents_tempo/same147.py DIR_A AGENT_A DIR_B AGENT_B
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from read147 import load  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def cell(d, agent):
    return {(g["_pair"], g["_diff"], g["seed"]): (g["won"], g["reason"], g["rounds"], g["decisions"], g["options_total"])
            for g in load(d) if g["_agent"] == agent}


def main():
    a, b = cell(sys.argv[1], sys.argv[2]), cell(sys.argv[3], sys.argv[4])
    keys = sorted(set(a) & set(b))
    diff = [k for k in keys if a[k] != b[k]]
    print(f"{len(keys)} games in both; {len(diff)} differ")
    for k in diff[:10]:
        print(k, a[k], b[k])
    sys.exit(1 if diff or not keys else 0)


if __name__ == "__main__":
    main()
