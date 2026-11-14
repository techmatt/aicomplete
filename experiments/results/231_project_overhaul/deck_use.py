"""231: how much of the project deck a game uses, per difficulty: projects revealed (dealt into a slot), discarded by
the draw rule, resolved, and reshuffles; the most in any game. The deck never reshuffles while every game's revealed
plus discarded stays at or under the deck size.

python deck_use.py OUT.jsonl=SPEC.json [DECK_SIZE]
"""
import json
import sys
from collections import defaultdict


def main():
    path, spec_path = sys.argv[1].split("=", 1)
    size = int(sys.argv[2]) if len(sys.argv) > 2 else 22
    spec = json.loads(open(spec_path, encoding="utf-8").read())
    cells = [(v["name"], d, a) for v in spec["variants"] for d in spec["difficulties"] for a in spec["agents"]]
    rows = defaultdict(list)
    with open(path, encoding="utf-8") as f:
        for line in f:
            g = json.loads(line)
            if "cell" not in g:
                continue
            v, d, a = cells[g["cell"]]
            c = g["counters"]
            used = c.get("adversary.projects_revealed", 0)  # every draw, a draw-rule discard included
            rows[(v.split("_", 1)[1] if "_" in v else "", d)].append(
                (used, c.get("adversary.projects_resolved", 0), c.get("adversary.project_reshuffles", 0)))
    print(f"| arm | d | games | used mean | used max | over {size} | resolved max | reshuffles |")
    print("|---|---|---|---|---|---|---|---|")
    for key in sorted(rows):
        r = rows[key]
        print(f"| {key[0] or 'base'} | {key[1]} | {len(r)} | {sum(x[0] for x in r) / len(r):.1f} | {max(x[0] for x in r)} | "
              f"{sum(1 for x in r if x[0] > size)} | {max(x[1] for x in r)} | {sum(x[2] for x in r)} |")


if __name__ == "__main__":
    main()
