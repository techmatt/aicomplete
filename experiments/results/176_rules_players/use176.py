"""176: each new or changed mechanic's use a game, per agent, from one or more runs' games.jsonl.

    python experiments/results/176_rules_players/use176.py DIR[,DIR...] [--by-level]

Prints, per agent (and difficulty with --by-level), the mean per game of the counters below: the standard actions
(`board.std.<name>`), Claim and Liberate by site, the Buy energy action's energy, botnets taken down, protections
spent or declined, cancel choices, corp-troop stops, converters, and Unity held off.
"""
import collections
import json
import os
import sys

KEYS = [
    ("remove", "board.std.remove"),
    ("claim", "board.std.claim"),
    ("liberate", "board.std.liberate"),
    ("lib.city", "board.liberated.city"),
    ("lib.station", "board.liberated.station"),
    ("lib.factory", "board.liberated.factory"),
    ("claim.city", "board.claimed.city"),
    ("claim.station", "board.claimed.station"),
    ("claim.factory", "board.claimed.factory"),
    ("own_on_site", "board.site_standing_spent"),
    ("botnet_std", "board.std.botnet"),
    ("botnets_down", "adversary.botnets_removed"),
    ("buy_energy", "board.std.buy_energy"),
    ("energy_bought", "board.city_energy_bought"),
    ("move", "board.std.move"),
    ("armor", "board.ambassador_shield"),
    ("harden_used", "board.hardening_used"),
    ("base_absorbed", "board.base_absorbed"),
    ("aegis", "board.aegis_used"),
    ("declined", "board.protection_declined"),
    ("factory_amb", "board.factory_ambassador_used"),
    ("base_amb", "board.base_ambassador_used"),
    ("held_off", "alignment.unity_held_off"),
    ("canceled", "adversary.canceled"),
    ("fac.neutral", "board.factories_built.neutral"),
    ("fac.player", "board.factories_built.player"),
    ("fac.adv", "board.factories_built.adversary"),
    ("cubes_conv", "board.cubes_converted"),
    ("tips_discord", "alignment.tips_discord"),
    ("tips_unity", "alignment.tips_unity"),
]


def main():
    dirs = sys.argv[1].split(",")
    by_level = "--by-level" in sys.argv
    sums = collections.defaultdict(lambda: collections.defaultdict(float))
    games = collections.Counter()
    for d in dirs:
        with open(os.path.join(d, "games.jsonl"), encoding="utf-8") as f:
            s = json.loads(f.readline())["spec"]
            cells = [(v["name"], diff, a) for _ in s["configs"] for v in s.get("variants", [{"name": "base"}])
                     for diff in s["difficulties"] for a in s["agents"]]
            for line in f:
                g = json.loads(line)
                if "cell" not in g:
                    continue
                _, diff, agent = cells[g["cell"]]
                key = (agent[:24], diff if by_level else "all")
                games[key] += 1
                c = g.get("counters", {})
                for name, k in KEYS:
                    sums[key][name] += c.get(k, 0)
                # Converters: any board.converter.<c>.
                sums[key]["converter"] += sum(v for k, v in c.items() if k.startswith("board.converter."))
    for key in sorted(games):
        n = games[key]
        print(f"{key[0]} d{key[1]} ({n} games)")
        print("  " + ", ".join(f"{name} {sums[key][name] / n:.2f}" for name, _ in KEYS + [("converter", "")]))


if __name__ == "__main__":
    main()
