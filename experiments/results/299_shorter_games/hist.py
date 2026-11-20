"""299: round histograms and the length bars from farmed games (one game a line).

    python hist.py SPEC.json LABEL=FILE[,FILE...] [LABEL=FILE ...] [--by-arm] [--md]

A row a label (and arm, `--by-arm`: the variant name after its pair prefix) and level: games, wins, mean rounds,
the share ending in rounds 6-8, the share at 10 or more, losses at the round cap (`deck_empty`; such a game records
the deck's cards + 1 rounds) of all losses, wins by round 3, and the histogram (rounds <=5, 6, ..., 11, 12+), in
percent. Bars marked "(miss)": wins in the level's band; over half in 6-8; under ~10% at 10+ (Matt, 2026-11-20: cap
losses are fine at any level). "Won by r3" is a count, read against r144's (the bar is "rare", no number).
"""
import collections
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")  # make_tables.sh redirects into committed Markdown

# The judge's win bands (rulings.md, *Difficulty*): d0 at least 90, d1 80-95, d2 75-90, d3 55-70 (no ceiling).
BANDS = {0: (90, 101), 1: (80, 95), 2: (75, 90), 3: (55, 101)}

BINS = ["<=5", "6", "7", "8", "9", "10", "11", "12+"]


def load(path):
    with open(path, encoding="utf-8") as f:
        return [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]


def bin_of(r):
    if r <= 5:
        return "<=5"
    if r >= 12:
        return "12+"
    return str(r)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    by_arm = "--by-arm" in sys.argv
    spec = json.load(open(args[0], encoding="utf-8"))
    names = [v["name"] for v in spec["variants"]]
    levels = spec["difficulties"]
    groups = collections.OrderedDict()
    for a in args[1:]:
        label, _, files = a.partition("=")
        for p in files.split(","):
            for g in load(p):
                v = names[g["cell"] // len(levels)]
                arm = v.split("-", 1)[1] if "-" in v else "base"
                lvl = levels[g["cell"] % len(levels)]
                key = (label, arm if by_arm else "", lvl)
                groups.setdefault(key, []).append(g)
    print("| Read | Level | Games | Wins | Rounds | 6-8 | 10+ | Cap losses of losses | Won by r3 | " + " | ".join(BINS) + " |")
    print("|" + "---|" * (9 + len(BINS)))
    for (label, arm, lvl), gs in sorted(groups.items(), key=lambda kv: (list(groups).index(kv[0]),)):
        n = len(gs)
        h = collections.Counter(bin_of(g["rounds"]) for g in gs)
        mid = sum(1 for g in gs if 6 <= g["rounds"] <= 8) / n
        long_ = sum(1 for g in gs if g["rounds"] >= 10) / n
        cap = sum(1 for g in gs if g["reason"] == "deck_empty")
        lost = sum(1 for g in gs if not g["won"])
        early = sum(1 for g in gs if g["won"] and g["rounds"] <= 3)
        mk = lambda ok: "" if ok else " (miss)"
        name = f"{label} {arm}".strip()
        win = 100 * sum(g['won'] for g in gs) / n
        lo, hi = BANDS.get(lvl, (0, 101))
        print(f"| {name} | d{lvl} | {n} | {win:.0f}%{mk(lo <= round(win) <= hi)} | "
              f"{sum(g['rounds'] for g in gs) / n:.1f} | {100 * mid:.0f}%{mk(mid > 0.5)} | "
              f"{100 * long_:.0f}%{mk(long_ < 0.10)} | {cap} of {lost} | {early} | "
              + " | ".join(f"{100 * h[b] / n:.0f}" for b in BINS) + " |")


if __name__ == "__main__":
    main()
