"""148: the final read, pooled over its sets, paired against another build on the same seeds.

    python experiments/results/148_card_pass_r62/pairs148.py --new DIR [DIR ...] --old DIR [DIR ...]

Per difficulty: the new build's win % (± SE), the old's, the paired gap (± SE, same pair, difficulty and seed), and
each set's win % for the new build.
"""

import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "147_agents_tempo"))
from read147 import load, paired, win  # noqa: E402

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    a = sys.argv[1:]
    i, j = a.index("--new"), a.index("--old")
    new_dirs = a[i + 1:j] if i < j else a[i + 1:]
    old_dirs = a[j + 1:i] if j < i else a[j + 1:]
    new = defaultdict(list)
    sets = defaultdict(lambda: defaultdict(list))
    for d in new_dirs:
        for g in load(d):
            new[g["_diff"]].append(g)
            sets[g["_diff"]][d].append(g)
    old = defaultdict(list)
    for d in old_dirs:
        for g in load(d):
            old[g["_diff"]].append(g)
    print("| d | new | old | paired gap | new by set |")
    print("|---|---|---|---|---|")
    for diff in sorted(new):
        w, se = win(new[diff])
        ow, ose = win(old[diff]) if old[diff] else (float("nan"), 0)
        p = paired(new[diff], old[diff])
        gap = f"{p[0]:+.1f} ±{p[1]:.1f} ({p[2]})" if p else "-"
        by = " / ".join(f"{win(gs)[0]:.1f}" for gs in sets[diff].values())
        print(f"| d{diff} | {w:.1f} ±{se:.1f} | {ow:.1f} ±{ose:.1f} | {gap} | {by} |")


if __name__ == "__main__":
    main()
