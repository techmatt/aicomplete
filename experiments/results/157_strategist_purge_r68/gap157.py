"""157: what player strength would close the gap: per level, the win % if the team freed k more cities in the last
three rounds (every loss at Unity step 2, 6+ tips, with k or fewer corrupted cities standing at the end flips), and
the ambassadors that takes (the Liberate bill at the end of those games, less what the team held at the snapshot).

python gap157.py DIR[,DIR...]   (a read with the end-game probe on every loss)

A loss below step 2 (troops or ambassadors still count) does not flip by liberations alone: its row says how many.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
PURGE_AT = (3, 6)


def load(dirs):
    for d in dirs.split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                yield cells[g["cell"]], g


games = defaultdict(int)
wins = defaultdict(int)
low = defaultdict(int)  # losses below step 2
flip = defaultdict(lambda: defaultdict(list))  # level -> cities left -> [(bill_end, held_snap)]
for c, g in load(sys.argv[1]):
    lv = c["difficulty"]
    games[lv] += 1
    if g["won"]:
        wins[lv] += 1
        continue
    tips = g["counters"].get("alignment.tips_unity", 0)
    step = sum(tips >= a for a in PURGE_AT)
    e = g.get("endgame")
    if step < 2 or not e:
        low[lv] += 1
        continue
    k = e["end"]["cities"]
    held = e["team"]["ambassadors_supply"] + e["team"]["ambassadors_board"]
    flip[lv][k].append((e["end"]["liberate_cost"], held))

print("| Level | Games | Win % | Losses below step 2 | Step-2 losses by cities left (1 / 2 / 3 / 4+) | Win % with +1 / +2 / +3 cities freed late | "
      "Bill (end) / held (snapshot) for a 1-city loss | for a 2-city loss |")
print("|---|---|---|---|---|---|---|---|")
for lv in sorted(games):
    n = games[lv]
    by = flip[lv]
    counts = [len(by.get(1, [])), len(by.get(2, [])), len(by.get(3, [])), sum(len(v) for k, v in by.items() if k >= 4)]
    zero = len(by.get(0, []))
    w = wins[lv] + zero
    cum = []
    for kk in (1, 2, 3):
        w += len(by.get(kk, []))
        cum.append(100 * w / n)

    def bill(kk):
        v = by.get(kk, [])
        if not v:
            return "-"
        return f"{sum(b for b, _ in v) / len(v):.1f} / {sum(h for _, h in v) / len(v):.1f}"

    print(f"| d{lv} | {n} | {100 * wins[lv] / n:.1f} | {low[lv]} | {' / '.join(map(str, counts))} | "
          f"{' / '.join(f'{x:.1f}' for x in cum)} | {bill(1)} | {bill(2)} |")
