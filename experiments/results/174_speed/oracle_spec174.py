"""One oracle game for the `profile` feature's split (174): out/p174-oracle.json, bp d3 seed 36800, the
oracle row's agent string (skilled_rows.py), threads 1.

    python experiments/results/174_speed/oracle_spec174.py [LEVEL] [SEED]
"""
import json
import os
import sys

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))


def exact():
    """The oracle's identical-games check: its row's agent at d3, 2 seeds a pair, no probe; the new binary's
    `discards=99` (no cap) against the base binary's agent."""
    s = json.load(open(os.path.join(TOP, "out", "skilled-174.json"), encoding="utf-8"))
    oracle = [a for a in s["agents"] if a.startswith("oracle")][0]
    s.pop("endgame", None)
    s["difficulties"] = [3]
    s["games"] = 2
    for side, agent in (("base", oracle), ("final", oracle + ",discards=99")):
        s["name"] = f"x174-oracle-{side}"
        s["agents"] = [agent]
        p = os.path.join(TOP, "out", s["name"] + ".json")
        with open(p, "w", encoding="utf-8", newline="\n") as f:
            json.dump(s, f, indent=2)
        print(p)


def main():
    if sys.argv[1:] == ["exact"]:
        return exact()
    level = int(sys.argv[1]) if len(sys.argv) > 1 else 3
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 36800
    s = json.load(open(os.path.join(TOP, "out", "skilled-174.json"), encoding="utf-8"))
    s["name"] = f"p174-oracle-d{level}"
    s["agents"] = [a for a in s["agents"] if a.startswith("oracle")]
    s["variants"] = [v for v in s["variants"] if v["name"] == "bp"]
    s["difficulties"] = [level]
    s["games"] = 1
    s["first_seed"] = seed
    s.pop("endgame", None)
    p = os.path.join(TOP, "out", s["name"] + ".json")
    with open(p, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
    print(p)


if __name__ == "__main__":
    main()
