"""155: writes the skilled-agent cost-curve specs (python specs155.py STAGE AGENT [AGENT...]).

24 fresh seeds a level (39112-39135), d3 and d4, the three board pairs rotated: Builder + Protector on
39112-39119, Builder + Researcher on 39120-39127, Protector + Researcher on 39128-39135 (one spec a pair,
`out/s155-STAGE-PAIR.json`). Each game is played on the same seed by every agent, so rows pair.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]
FIRST, PER_PAIR = 39112, 8


def spec(stage, pair, sets, first, agents, endgame=False):
    out = {"name": f"s155-{stage}-{pair}",
           "note": f"155 cost curve ({stage}): pair {pair}, seeds {first}-{first + PER_PAIR - 1}, d3-d4.",
           "configs": ["configs/baseline.json"], "variants": [{"name": pair, "set": sets}],
           "difficulties": [3, 4], "agents": agents, "games": PER_PAIR, "first_seed": first, "records": 0}
    if endgame:
        out["endgame"] = {"back": 3, "agent": "", "rollouts": 1, "all_losses": True}
    return out


if __name__ == "__main__":
    stage, agents = sys.argv[1], [a for a in sys.argv[2:] if not a.startswith("--")]
    # --fresh: the confirming set, the next 24 seeds (39136-39159).
    FIRST += 24 * ("--fresh" in sys.argv)
    for k, (pair, sets) in enumerate(PAIRS):
        s = spec(stage, pair, sets, FIRST + PER_PAIR * k, agents, "--endgame" in sys.argv)
        if "--d4" in sys.argv:
            s["difficulties"] = [4]
        path = os.path.join(TOP, "out", f"s155-{stage}-{pair}.json")
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(s, f, indent=2)
            f.write("\n")
        print(path)
