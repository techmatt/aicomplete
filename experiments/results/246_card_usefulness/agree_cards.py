"""246: per-card play agreement from `aic-sim agree` output (its `play:<card>` keys): for each agent, at the
decisions where the oracle played the card, the share where the agent played it too (same) or later this round.

python agree_cards.py AGREE.jsonl [--cards a,b] [--group all]
"""
import argparse
import json
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
ap = argparse.ArgumentParser()
ap.add_argument("file")
ap.add_argument("--cards", default="")
ap.add_argument("--group", default="all")
a = ap.parse_args()
rows = [json.loads(l) for l in open(a.file, encoding="utf-8") if l.strip()]
cards = a.cards.split(",") if a.cards else sorted(
    {k[5:] for r in rows for k in r["by"][a.group] if k.startswith("play:")})
print(f"oracle plays (n) and each agent's same / later %, group {a.group}")
print(f"{'card':30} {'n':>5} " + " ".join(f"{r['agent'][:30]:>30}" for r in rows))
for c in cards:
    k = "play:" + c
    n = max((r["by"][a.group].get(k, [0, 0, 0])[2] for r in rows), default=0)
    cells = []
    for r in rows:
        s, l, m = r["by"][a.group].get(k, [0, 0, 0])
        cells.append(f"{100 * s / m:5.1f} / {100 * l / m:5.1f}" if m else "-")
    print(f"{c:30} {n:5} " + " ".join(f"{x:>30}" for x in cells))
for kd in ["play", "any"]:
    cells = []
    for r in rows:
        s, l, m = r["by"][a.group].get(kd, [0, 0, 0])
        cells.append(f"{100 * s / m:5.1f} / {100 * l / m:5.1f}" if m else "-")
    print(f"{'(' + kd + ')':30} {'':5} " + " ".join(f"{x:>30}" for x in cells))
