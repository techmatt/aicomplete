"""149: Movement used and lost a game per board (`board.amb_moves.<seat>` counts every
ambassador move, cards' too; `board.moves_lost.<seat>` the Movement left at each round's end).

python lost149.py DIR[,DIR...]
"""
import json
import sys
from collections import defaultdict

BOARDS = ["Builder", "Protector", "Researcher"]


def main():
    agg = defaultdict(lambda: defaultdict(float))
    for d in sys.argv[1].split(","):
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        with open(f"{d}/games.jsonl", encoding="utf-8") as f:
            sets = {v["name"]: v["set"] for v in json.loads(f.readline())["spec"]["variants"]}
            for line in f:
                g = json.loads(line)
                c = cells[g["cell"]]
                boards = [0, 1]
                for s in sets[c["variant"]]:
                    if s.startswith("ai.boards="):
                        boards = [int(x) for x in s.split("=", 1)[1].split(",")]
                k = g["counters"]
                for s, b in enumerate(boards):
                    a = agg[b]
                    a["seats"] += 1
                    a["moves"] += k.get(f"board.amb_moves.{s}", 0)
                    a["lost"] += k.get(f"board.moves_lost.{s}", 0)
                    a["rounds"] += g["rounds"]
    print("| board | moves a game | Movement lost a game | rounds | lost a round |")
    print("|---|---|---|---|---|")
    for b, a in sorted(agg.items()):
        n = a["seats"]
        print(f"| {BOARDS[b]} | {a['moves'] / n:.1f} | {a['lost'] / n:.1f} | {a['rounds'] / n:.1f} | {a['lost'] / a['rounds']:.2f} |")


if __name__ == "__main__":
    main()
