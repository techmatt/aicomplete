"""149: ambassador moves a game per board, from a run's games.jsonl.

python move_read.py DIR [DIR...]

Per cell (variant, difficulty) and board: win %, ambassador moves a game by that
seat (`board.amb_moves.<seat>`: the Move action and card moves), and the table's
Move actions, claims, liberations and Removes a game. Board of each seat comes
from the variant (`ai.boards`, default 0,1).
"""
import json
import sys
from collections import defaultdict

BOARDS = ["Builder", "Protector", "Researcher"]


def seat_boards(variant_set):
    for s in variant_set:
        if s.startswith("ai.boards="):
            return [int(x) for x in s.split("=", 1)[1].split(",")]
    return [0, 1]


def read(d):
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        head = json.loads(f.readline())
        spec = head["spec"]
        res = json.load(open(f"{d}/results.json", encoding="utf-8"))
        cells = {c["cell"]: c for c in res["cells"]}
        sets = {v["name"]: v["set"] for v in spec["variants"]}
        agg = defaultdict(lambda: defaultdict(float))
        for line in f:
            g = json.loads(line)
            c = cells[g["cell"]]
            key = (c["variant"], c["difficulty"], c["agent"])
            a = agg[key]
            a["games"] += 1
            a["won"] += 1 if g["won"] else 0
            k = g["counters"]
            for name in ["move", "claim", "liberate", "remove", "free_move"]:
                a[name] += k.get(f"board.std.{name}", 0)
            a["card_moves"] += k.get("effects.ambassador_moves", 0)
            for s, b in enumerate(seat_boards(sets[c["variant"]])):
                a[f"moves.{b}"] += k.get(f"board.amb_moves.{s}", 0)
                a[f"seats.{b}"] += 1
                a[f"lost_moves.{b}"] += k.get(f"board.moves_lost.{s}", 0)
    return agg


def main():
    for d in sys.argv[1:]:
        print(f"## {d}")
        print("| cell | d | win % | Builder moves | Protector | Researcher | Move acts | claims | liberates | removes | moves lost B/P/R |")
        print("|---|---|---|---|---|---|---|---|---|---|---|")
        for (v, diff, agent), a in sorted(read(d).items()):
            n = a["games"]
            mv = [f"{a[f'moves.{b}'] / a[f'seats.{b}']:.1f}" if a[f"seats.{b}"] else "" for b in range(3)]
            lost = "/".join(f"{a[f'lost_moves.{b}'] / a[f'seats.{b}']:.1f}" if a[f"seats.{b}"] else "-" for b in range(3))
            print(f"| {v} | {diff} | {100 * a['won'] / n:.1f} | {' | '.join(mv)} | {a['move'] / n:.1f} | {a['claim'] / n:.1f} | {a['liberate'] / n:.1f} | {a['remove'] / n:.1f} | {lost} |")


if __name__ == "__main__":
    main()
