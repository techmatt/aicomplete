"""157 (156's endgame156.py, plus the stacks on the last cities): the end-game probe (154's `"endgame"`, 155's `all_losses`) by Unity step: what remains when a game is lost.

python endgame156.py DIR[,DIR...]

Per level and Unity step at the end (0: 0-2 tips, every piece counts; 1: 3-5, no ambassadors; 2: 6+, cities only),
for each loss kind: games, and at the end the corrupted cities, botnets, troops and adversary ambassadors standing,
the purge's own count at that step, Discord cards left, and the team's ambassadors (supply + board) at the snapshot
(the start of the third-last round) against the Liberate bill for every corrupted city then.
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


if __name__ == "__main__":
    by = defaultdict(list)
    for c, g in load(sys.argv[1]):
        e = g.get("endgame")
        if not e or g["won"]:
            continue
        tips = g["counters"].get("alignment.tips_unity", 0)
        step = sum(tips >= a for a in PURGE_AT)
        by[(c["difficulty"], step, e.get("reason") or g["reason"])].append(e)
    print("| Level | Unity step | Loss | Games | Cities | Botnets | Troops | Adv. ambassadors | Purge count | Discord left | "
          "Team ambassadors at snapshot | Liberate bill at snapshot | Adv. ambassadors on corrupted cities (end) | "
          "Corrupted cities in reach (end) | Liberate bill (end) |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for k in sorted(by):
        es = by[k]
        n = len(es)
        m = lambda f: sum(f(e) for e in es) / n
        print(f"| d{k[0]} | {k[1]} | {k[2]} | {n} | {m(lambda e: e['end']['cities']):.1f} | {m(lambda e: e['end']['botnets']):.1f} | "
              f"{m(lambda e: e['end']['troops']):.1f} | {m(lambda e: e['end']['ambassadors']):.1f} | {m(lambda e: e['end']['purge']):.1f} | "
              f"{m(lambda e: e['discord_left']):.1f} | "
              f"{m(lambda e: e['team']['ambassadors_supply'] + e['team']['ambassadors_board']):.1f} | "
              f"{m(lambda e: e['snap']['liberate_cost']):.1f} | {m(lambda e: e['end']['ambassadors_on_corrupted']):.1f} | "
              f"{m(lambda e: e['end']['cities_in_reach']):.1f} | {m(lambda e: e['end']['liberate_cost']):.1f} |")
