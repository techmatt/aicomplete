"""155: whether two runs played the same games (python same155.py DIR_A AGENT_A DIR_B AGENT_B).

Matches games on (variant, difficulty, seed) between agent AGENT_A in DIR_A and AGENT_B in DIR_B (a label may
differ only in a switch that should not change play, `threads`) and compares their actions; prints the games
compared, how many differ, and each run's agent seconds a game.
"""
import json
import os
import sys


def games(d, agent):
    lines = [json.loads(x) for x in open(os.path.join(d, "games.jsonl"), encoding="utf-8")]
    spec = lines[0]["spec"]
    cells = [(v["name"], lv, a) for c in spec["configs"] for v in spec["variants"]
             for lv in spec["difficulties"] for a in spec["agents"]]
    out = {}
    for g in lines[1:]:
        v, lv, a = cells[g["cell"]]
        if a == agent:
            out[(v, lv, int(g["seed"]))] = g
    return out


def main(da, aa, db, ab):
    a, b = games(da, aa), games(db, ab)
    both = sorted(set(a) & set(b))
    diff = [k for k in both if a[k]["actions"] != b[k]["actions"]]
    ta = sum(a[k]["agent_ms"] for k in both) / max(len(both), 1) / 1000
    tb = sum(b[k]["agent_ms"] for k in both) / max(len(both), 1) / 1000
    print(f"{len(both)} games compared, {len(diff)} differ{': ' + str(diff) if diff else ''}; "
          f"agent s a game {ta:.1f} vs {tb:.1f}")


if __name__ == "__main__":
    main(*sys.argv[1:5])
