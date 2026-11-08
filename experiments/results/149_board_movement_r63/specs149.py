"""149: writes the screen and read specs (python specs149.py; specs land beside this file).

Screens run on the dev seeds (36800-), 32 a cell, the three pairs. Cells:
- base: 149's layout as built (the prompt's numbers).
- tags62: r62's revealed tags restored (4 a board, early), 149's Movement kept.
- mv9: Movement 9 printed on every board (the cap off, to see if it binds).
Agent cells: the strategist, and the strategist with the Movement term off
(`w.movement=0`).
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}

# r62's revealed tags (0-based space index), and 149's to clear.
TAGS62 = [
    'ais.builder.economy_track.3.tags=["power"]', 'ais.builder.play_track.1.tags=["industry"]',
    'ais.builder.play_track.3.tags=["science"]', 'ais.builder.economy_track.5.tags=[]',
    'ais.protector.economy_track.3.tags=["diplomacy"]', 'ais.protector.play_track.1.tags=["security"]',
    'ais.protector.play_track.3.tags=["cyber"]', 'ais.protector.economy_track.5.tags=[]',
    'ais.researcher.economy_track.3.tags=["power"]', 'ais.researcher.play_track.2.tags=["science"]',
    'ais.researcher.play_track.3.tags=["cyber"]', 'ais.researcher.economy_track.5.tags=[]',
]
MV9 = ["ais.builder.moves=9", "ais.protector.moves=9", "ais.researcher.moves=9"]
# Screen 2: the boards' own kinds on space 6 (Builder Industry + Science, Protector
# Security + Diplomacy, Researcher Science + Industry) ...
KINDS = [
    'ais.builder.economy_track.5.tags=["industry"]', 'ais.builder.play_track.5.tags=["science"]',
    'ais.protector.economy_track.5.tags=["security"]', 'ais.protector.play_track.5.tags=["diplomacy"]',
    'ais.researcher.economy_track.5.tags=["science"]', 'ais.researcher.play_track.5.tags=["industry"]',
]
# ... and 149's kinds a space earlier (space 5, cost 2).
SPACE5 = [
    'ais.builder.economy_track.5.tags=[]', 'ais.builder.play_track.5.tags=[]',
    'ais.builder.economy_track.4.tags=["power"]', 'ais.builder.play_track.4.tags=["cyber"]',
    'ais.protector.economy_track.5.tags=[]', 'ais.protector.play_track.5.tags=[]',
    'ais.protector.economy_track.4.tags=["cyber"]', 'ais.protector.play_track.4.tags=["diplomacy"]',
    'ais.researcher.economy_track.5.tags=[]', 'ais.researcher.play_track.5.tags=[]',
    'ais.researcher.economy_track.4.tags=["power"]', 'ais.researcher.play_track.4.tags=["industry"]',
]
LEVERS = {"base": [], "tags62": TAGS62, "mv9": MV9, "kinds": KINDS, "space5": SPACE5,
          # Screen 3, the d2 dial (d2 read 70.8 on F1, band 55-65): counters dearer from d2.
          "cmc6": ["adversary.counter_more_credits=6"], "cmc9": ["adversary.counter_more_credits=9"],
          "cma2": ["adversary.counter_more_all=2"],
          # Screen 4, d2-only dials (d3 already has them): the adversary board's line, or the extra threat, from d2.
          "bda2": ["adversary.board_difficulty_at=2"], "et2": ["adversary.extra_threats_at=2"],
          # Screen 5, d3-only dials (d3 read 54.2 on F2, band 40-50): larger builds, a shorter round cap.
          "bb3": ["adversary.build_bonus=3"], "s2r2": ["adversary.stage2_removed=2"]}


def spec(name, note, levers, diffs, agents, games, seed):
    variants = []
    for lv in levers:
        for p, s in PAIRS.items():
            variants.append({"name": p if lv == "base" else f"{p}_{lv}", "set": s + LEVERS[lv]})
    return {
        "name": name, "note": note, "configs": ["configs/baseline.json"], "variants": variants,
        "difficulties": diffs, "agents": agents, "games": games, "first_seed": seed, "records": 0,
    }


def read_spec(tag, half, seed, set_extra=()):
    """A paired ladder read set: the three pairs, d0-d3, the strategist, 48 seeds."""
    return {
        "name": f"read-149-{tag}-{half}",
        "note": f"149 ladder read ({tag}), set {half}: seeds {seed}-{seed + 47}, the three pairs, d0-d3; run the r62 binary on the same spec for the pair.",
        "configs": ["configs/baseline.json"],
        "variants": [{"name": p, "set": s + list(set_extra)} for p, s in PAIRS.items()],
        "difficulties": [0, 1, 2, 3], "agents": ["strategist"], "games": 48, "first_seed": seed, "records": 0,
    }


def write_top(s):
    top = os.path.normpath(os.path.join(HERE, "..", ".."))
    with open(os.path.join(top, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


def write(s):
    with open(os.path.join(HERE, s["name"] + ".json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    write(spec("screen149-s1", "149 screen 1: r62 tags back, Movement uncapped, the Movement term off (dev seeds).",
               ["base", "tags62", "mv9"], [0, 2], ["strategist", "strategist:w.movement=0"], 32, 36800))
    write(spec("screen149-s2", "149 screen 2: the revealed tags' kinds (the boards' own) and space (5, cost 2), dev seeds 36800-36863.",
               ["base", "kinds", "space5"], [0, 2], ["strategist"], 64, 36800))
    write(spec("screen149-s3", "149 screen 3: the d2 counter step (counter_more_credits 3 -> 6 / 9, counter_more_all 1 -> 2), d2-d3, dev seeds 36800-36863.",
               ["base", "cmc6", "cmc9", "cma2"], [2, 3], ["strategist"], 64, 36800))
    write(spec("screen149-s4", "149 screen 4: d2-only dials (board_difficulty_at 3 -> 2, extra_threats_at 4 -> 2), d2, dev seeds 36800-36863.",
               ["base", "bda2", "et2"], [2], ["strategist"], 64, 36800))
    write(spec("screen149-s5", "149 screen 5: d3-only dials (build_bonus 2 -> 3, stage2_removed 1 -> 2) on the dialed baseline, d3, dev seeds 36800-36863.",
               ["base", "bb3", "s2r2"], [3], ["strategist"], 64, 36800))
    # F1: the first fresh paired read, seeds 37624-37719 (the ledger's next free at 148).
    write_top(read_spec("f1", "a", 37624))
    write_top(read_spec("f1", "b", 37672))
    # F2: the final read on the dialed build (board_difficulty_at 2 in the baseline), fresh seeds 37720-37815;
    # the r62 binary runs the same seeds at its own dials.
    write_top(read_spec("f2", "a", 37720))
    write_top(read_spec("f2", "b", 37768))
    # F3: the confirming final read with build_bonus 3 as well (d3 read 54.2 on F2), fresh seeds 37816-37911.
    write_top(read_spec("f3", "a", 37816))
    write_top(read_spec("f3", "b", 37864))
