"""149: the boards' Movement and revealed tags (Matt's board direction, design doc rev 24).

python experiments/results/149_board_movement_r63/boards149.py [--check]

Rerunnable from r62's content/ais.json: it sets each board's printed Movement, its
Movement spaces and its 1-2 revealed tags, and takes every other revealed tag off.
Spaces are 1-based as docs/rules/ai-boards.md prints them.
"""
import json
import sys

PATH = "content/ais.json"

# board id -> (printed Movement, {(track, space): moves}, {(track, space): [tags]})
# Every revealed tag not listed here is removed.
LAYOUT = {
    # Builder: few ambassador plays (4.2 moves a game at r62); 1 printed, 1 more
    # with the third economy node (the old Power-tag space, cost 1).
    "builder": (1, {("economy", 4): 1}, {("economy", 6): ["power"], ("play", 6): ["cyber"]}),
    # Protector: 1 printed, 1 more on its fourth economy space (the old Diplomacy tag).
    "protector": (1, {("economy", 4): 1}, {("economy", 6): ["cyber"], ("play", 6): ["diplomacy"]}),
    # Researcher: the movement-heavy board (6.4 moves a game at r62, the most): its
    # stations across the map are sites its own ambassador armors and an adversary
    # ambassador blocks. 2 printed, and more on two spaces.
    "researcher": (2, {("economy", 4): 1, ("play", 2): 1}, {("economy", 6): ["power"], ("play", 6): ["industry"]}),
}

TRACK = {"economy": "economy_track", "play": "play_track"}


def apply(ais):
    for b in ais:
        printed, moves, tags = LAYOUT[b["id"]]
        b["moves"] = printed
        for track, field in TRACK.items():
            for i, u in enumerate(b[field], start=1):
                u.pop("moves", None)
                if (track, i) in moves:
                    u["moves"] = moves[(track, i)]
                u.pop("tags", None)
                if (track, i) in tags:
                    u["tags"] = tags[(track, i)]
    return ais


def main():
    ais = json.load(open(PATH, encoding="utf-8"))
    out = apply(json.loads(json.dumps(ais)))
    if "--check" in sys.argv:
        sys.exit(0 if out == ais else 1)
    with open(PATH, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    for b in out:
        print(b["id"], "moves", b["moves"])
        for field in TRACK.values():
            for i, u in enumerate(b[field], start=1):
                if u.get("moves") or u.get("tags"):
                    print(f"  {field} {i}: cost {u.get('cost', 0)} moves {u.get('moves', 0)} tags {u.get('tags', [])}")


if __name__ == "__main__":
    main()
