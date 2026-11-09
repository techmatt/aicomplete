"""156: each Unity and Discord card's get at each tier, priced at configs/agents/values.json.

python tiers156.py [--players N] > tiers.md

A team line ("every player gains") pays each player, so it counts N times (2 by default, the ballpark read's
seats); income counts `rates.income_rounds` rounds. `one_player` is the chosen player's ops at their own rows
(`ops.one_player` share). Rows mirror crates/aic-sim/src/agents/value.rs for the ops these cards print. A card
costs nothing to draw (a tip brings it), so its give is 0 and its get is the tier's worth; a Discord card's is a
cost (negative). Where in a game it has a place to matter is in the write-up.
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
V = json.load(open(os.path.join(TOP, "configs", "agents", "values.json"), encoding="utf-8"))
CARDS = json.load(open(os.path.join(TOP, "content", "alignment.json"), encoding="utf-8"))
RES, RATES, OPS = V["resources"], V["rates"], V["ops"]
PLAYERS = int(sys.argv[sys.argv.index("--players") + 1]) if "--players" in sys.argv else 2


def row(k):
    return OPS[k]["credits"]


def troops(n):
    return V["troops_removed"].get(str(n), n)


def worth(op, team):
    k = op["op"]
    each = PLAYERS if team else 1
    if k == "gain":
        return row("gain") * RES[op["resource"]] * op["amount"] * each
    if k == "team_income":
        return row("team_income") * RES[op["resource"]] * op["amount"] * RATES["income_rounds"] * PLAYERS
    if k == "free_purchases":
        return row("free_purchases") * RATES["keep_tier1"] * op["count"]
    if k == "counter_threat":
        return row("counter_threat")
    if k == "counter_all":
        return row("counter_all")
    if k == "pushback":
        return row("pushback") * RATES["countries"] * op["ambassadors"]
    if k == "create_site":
        return row("create_site")
    if k == "sway":
        return row("sway") * RATES["sway"] * op["amount"] * (1 if op["side"] == "unity" else -1)
    if k == "remove_botnets":
        return row("remove_botnets") * op["count"]
    if k == "one_player":
        return row("one_player") * sum(worth(o, False) for o in op["then"])
    if k == "clear_city":
        return row("clear_city") * (op["max"] or RATES["token_pile"])
    if k == "damage":
        return max(row("remove_ambassador") * op.get("ambassadors", 0), row("remove_troop") * op.get("troops", 0))
    if k == "remove_troops":
        return row("remove_troops") * troops(op["count"])
    if k == "remove_botnet":
        return row("remove_botnet")
    if k == "clear_markers":
        return row("clear_markers") * op["count"]
    if k == "liberate":
        return (row("liberate") + (row("liberate_align") if op.get("align") else 0)
                + row("gain_ambassadors") * RES["ambassadors"] * op.get("ambassadors", 0)
                + (row("liberate_botnet") if op.get("botnet") else 0))
    if k == "cleanse":
        return row("cleanse_station") * op["amount"]
    if k == "gain_ambassadors":
        return row("gain_ambassadors") * RES["ambassadors"] * op["count"]
    if k == "move_ambassadors":
        return row("move_ambassadors") * op["max"]
    if k == "remove_ambassadors_country":
        n = op.get("max") or RATES["token_pile"]
        up = op.get("unity_per", 0)
        return row("remove_ambassadors_country") * n + ((n // up) * RATES["sway"] if up else 0)
    # Discord's lines: the rows are costs already.
    if k == "deal_threats":
        n = op["count"] * (RATES["countries"] if op.get("tier") else 1)
        return row("deal_threats") * n * (1.5 if op.get("tier") == 2 else 1)
    if k == "create":
        return row("create") * op["count"] * RATES["countries"]
    if k == "corrupt":
        return row("corrupt")
    if k == "threat_at_node":
        return row("threat_at_node") * op["tier"]
    if k == "players_lose":
        return row("players_lose") * RES[op["resource"]] * op["amount"] * PLAYERS
    if k == "pull_unity":
        return row("pull_unity") * op["amount"]
    if k == "lose_ambassador":
        return row("lose_ambassador") * PLAYERS
    if k == "adversary_build":
        return row("adversary_build")
    raise SystemExit(f"no row for {k}")


if __name__ == "__main__":
    print(f"| Card | Side | Draft | Tier I | Tier II | Tier III |  ({PLAYERS} players, values.json)")
    print("|---|---|---|---|---|---|")
    sums = {}
    for c in CARDS:
        w = [sum(worth(o, True) for o in t["effects"]) for t in c["tiers"]]
        for i, x in enumerate(w):
            sums.setdefault((c["side"], i), []).append(x)
        print(f"| {c['name']} | {c['side']} | {'draft' if c.get('draft') else ''} | " + " | ".join(f"{x:.0f}" for x in w) + " |")
    for side in ("unity", "discord"):
        m = [sum(sums[(side, i)]) / len(sums[(side, i)]) for i in range(3)]
        print(f"| *mean* | {side} | | " + " | ".join(f"{x:.0f}" for x in m) + " |")
