"""155: the skilled-player row beside a read (python scripts/bench/skilled_rows.py READ.json [--rows oracle]
[--games 24] [--levels 3,4] [--out PATH]).

Writes a companion spec to the read: its configs and variants, `--levels` (default 3 and 4), and the
rows below plus the strategist, on the read's first seeds, `--games` a level spread over its variants (24 over
three pairs: 8 seeds each), so each row pairs with the read's own strategist games. It runs on its own binary call
(about 10-90 x a strategist game: docs/experiments.md, *Skilled rows*). The end-game probe reads every loss's end
state (`all_losses`).

- `oracle`: the full-information search (`agents/oracle.rs`): a win it finds proves the seed winnable, so its rate
  is a floor on a perfect player's. Its budget is 155's top: rollouts answer a prompt one prompt deep (`answers=1`), more lines for the time.
  243: `tries=3` restarts a lost game twice on other policy seeds; the first try is the single oracle, so its wins
  are a superset of that one's, at up to 3x the time on a seed it loses.

- `real` (249): the real oracle, the fair-information player (Matt, 155, amendment 6814): the oracle's search on
  worlds sampled from what players can see (`worlds`), never the true table, and no restarts. The design target: very
  strong play that can't see the future. On 249's judge weights; about 135 x a strategist game (r112), and it read
  at the judge (80.0 vs 78.3% d0-d4, 249).
"""
import argparse
import json
import math
import os

ROWS = {
    "oracle": "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1,tries=3",
    # 249's measured spec: the search on the judge's weights as 249 read it (before `w.card_action` and `w.sink`).
    # 260: 249 left its first, discarded version here (engine weights, strategy=1, 3 worlds), which read 55% to the
    # judge's 70%.
    "real": "oracle:strategy=0,worlds=4,alts=3,budget=32,answers=1,truth=1,w.built=4,w.built_cost=0.75,w.clock=0,"
            "w.energy_cap=0,w.hand=1.5,w.hand_cost=0.15,w.horizon=0,w.insight_income=0,w.movement=0,w.packet_keep=1,"
            "w.production=0,w.strat_contents=0,w.table=0,w.tag_hand=0,w.purge=3.6,w.last_node=120",
}


def companion(read, rows, games, levels):
    variants = read.get("variants") or [{"name": "base", "set": []}]
    per = math.ceil(games / len(variants))
    diffs = levels or [3, 4]
    out = {"name": read["name"] + "-skilled",
           "note": f"Skilled rows beside {read['name']}: {', '.join(rows)}, {per} seeds a variant, d{diffs}.",
           "configs": read["configs"], "variants": variants, "difficulties": diffs,
           "agents": ["strategist"] + [ROWS[r] for r in rows], "games": per,
           "first_seed": read.get("first_seed", 0), "records": 0,
           "endgame": {"back": 3, "agent": "", "rollouts": 1, "all_losses": True}}
    if "max_actions" in read:
        out["max_actions"] = read["max_actions"]
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("read")
    ap.add_argument("--rows", default="oracle")
    ap.add_argument("--games", type=int, default=24)
    ap.add_argument("--levels", default="")
    ap.add_argument("--out", default="")
    a = ap.parse_args()
    with open(a.read, encoding="utf-8") as f:
        read = json.load(f)
    rows = [r for r in a.rows.split(",") if r]
    bad = [r for r in rows if r not in ROWS]
    if bad:
        raise SystemExit(f"unknown rows {bad}; known: {', '.join(ROWS)}")
    levels = [int(x) for x in a.levels.split(",") if x]
    s = companion(read, rows, a.games, levels)
    path = a.out or os.path.splitext(a.read)[0] + "-skilled.json"
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(s, f, indent=2)
        f.write("\n")
    print(path)


if __name__ == "__main__":
    main()
