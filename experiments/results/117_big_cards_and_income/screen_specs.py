"""Write income screen specs: base and income levels as variants per pair."""
import json
import sys

PAIRS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}
LEVELS = {
    "base": None,
    "a": (21, 17, 15),
    "b": (18, 15, 13),
    "c": (15, 12, 10),
    "m": (20, 16, 14),
}


def income_sets(lv):
    if lv is None:
        return []
    b, p, r = lv
    return [f"ais.builder.credits={b}", f"ais.protector.credits={p}", f"ais.researcher.credits={r}"]


def main(out, first_seed, games, levels):
    variants = []
    for lvn in levels:
        for pn, ps in PAIRS.items():
            variants.append({"name": f"{pn}_{lvn}", "set": ps + income_sets(LEVELS[lvn])})
    spec = {
        "name": out.split("/")[-1].replace(".json", ""),
        "note": "117: income screen",
        "configs": ["configs/baseline.json"],
        "variants": variants,
        "difficulties": [2],
        "agents": ["strategist", "greedy"],
        "games": games,
        "first_seed": first_seed,
        "records": 0,
    }
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        json.dump(spec, f, indent=2)


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4].split(","))
