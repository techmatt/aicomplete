"""314: the screen of the real oracle's new options (lone, phases), d4 on 313's spent seeds: win % per arm and its
gap to base, paired on (pair, seed).

    python experiments/results/314_strong_fair_player/screen.py [OUT_DIR]
"""
import glob
import json
import math
import os
import sys
from collections import defaultdict


def main():
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join("out", "314_strong_fair_player")
    won = defaultdict(dict)
    for p in glob.glob(os.path.join(out, "s314-*.jsonl")):
        arm, pair = os.path.basename(p)[5:-6].rsplit("-", 1)
        for line in open(p, encoding="utf-8"):
            g = json.loads(line)
            if "format" not in g:
                won[arm][(pair, int(g["seed"]))] = int(g["won"])
    print("| arm | n | win % | gap to base, paired | SE |")
    print("|---|---|---|---|---|")
    for arm in ["base", "lone", "phases", "both"]:
        w = won.get(arm, {})
        if not w:
            continue
        d = [v - won["base"][k] for k, v in w.items() if k in won.get("base", {})]
        m = sum(d) / len(d) if d else 0
        se = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1) / len(d)) if d else 0
        print(f"| {arm} | {len(w)} | {100 * sum(w.values()) / len(w):.1f} | {100 * m:+.1f} ({len(d)}) | {100 * se:.1f} |")


if __name__ == "__main__":
    main()
