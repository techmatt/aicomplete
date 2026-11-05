"""117: write the read specs (run from the repo root; the specs land in experiments/).

    python experiments/results/117_big_cards_and_income/specs.py

screen-117-r45.json: the three pairs at d2, strategist and greedy, 48 a cell, fresh seeds 34224-34271.
screen-117-income.json: the same with r46's printed incomes as overlays, for the r45 binary: the income step alone.
Run each from a folder holding the binary and its configs (docs/experiments.md, *A before/after read is paired*).
The full r46 read is screen-117-r45.json run by the r46 binary.
"""

import json

INCOME = {"builder": 18, "protector": 16, "researcher": 14}
PAIRS = [("bp", []), ("br", ["ai.boards=0,2"]), ("pr", ["ai.boards=1,2"])]


def spec(name, note, extra):
    return {
        "name": name,
        "note": note,
        "configs": ["configs/baseline.json"],
        "variants": [{"name": p, "set": s + extra} for p, s in PAIRS],
        "difficulties": [2],
        "agents": ["strategist", "greedy"],
        "games": 48,
        "first_seed": 34224,
        "records": 0,
    }


def write(path, obj):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(obj, f, indent=2)
        f.write("\n")


write("experiments/screen-117-r45.json", spec(
    "screen-117-r45",
    "117: the ballpark read on fresh seeds 34224-34271: the three pairs at d2, the strategist and greedy, 48 a cell. "
    "Run by r45's binary (b34797c) for the base and by r46's for the full r46 read.", []))
write("experiments/screen-117-income.json", spec(
    "screen-117-income",
    "117: r46's printed incomes alone (Builder 18, Protector 16, Researcher 14) as overlays on r45's binary "
    "(b34797c), paired against screen-117-r45 on fresh seeds 34224-34271.",
    [f"ais.{b}.credits={c}" for b, c in INCOME.items()]))
