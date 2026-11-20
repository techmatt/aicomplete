"""298: a lever screen's table from a farmed file (one game a line) and its spec.

    python screen.py SPEC.json FILE [FILE ...] [--pairs]

A row an arm and level, the three pairs pooled (`--pairs`: a row a pair too): wins, rounds, games lost to the
research row, wins by round 3, Strike hits, ambassadors a Strike sent home, troops redeployed, games with a troop
by round 3, Martial Law's first troops' round, the hedges a seat-game (Gain a player ambassador, Remove agents),
Accord a Protector seat-round, Offensive's deals and idles. Cells are variant-major: `variant * levels + level`.
"""
import collections
import json
import sys


def load(path):
    with open(path, encoding="utf-8") as f:
        return [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]


def c(g, k):
    return g["counters"].get(k, 0)


def troop_round(g, source=None):
    rounds = []
    for k, v in g["counters"].items():
        if k.startswith("map.troops_placed.") and v > 0:
            src, r = k[len("map.troops_placed."):].rsplit(".r", 1)
            if source is None or src == source:
                rounds.append(int(r))
    return min(rounds) if rounds else None


def protectors(g, variant):
    """Protector seats: none in the Builder + Researcher pair, one otherwise (the 3-seat game included)."""
    return 1 if not variant.startswith("br") else 0


def row(games, variant_of):
    n = len(games)
    if not n:
        return None
    m = lambda f: sum(f(g) for g in games) / n
    seats = len(games[0]["margin"]["unspent"])  # one entry a seat
    prot_rounds = sum(g["rounds"] * protectors(g, variant_of(g)) for g in games) or 1
    early = sum(1 for g in games if (troop_round(g) or 99) <= 3)
    return [
        n,
        f"{100 * m(lambda g: g['won']):.0f}%",
        f"{m(lambda g: g['rounds']):.1f}",
        sum(1 for g in games if g["reason"] == "research"),
        sum(1 for g in games if g["won"] and g["rounds"] <= 3),
        f"{m(lambda g: c(g, 'board.troop_strikes')):.2f}",
        f"{m(lambda g: c(g, 'board.troop_sent_home')):.2f}",
        f"{m(lambda g: c(g, 'board.troops_redeployed')):.2f}",
        early,
        f"{m(lambda g: c(g, 'board.std.ambassador')) / seats:.2f}",
        f"{m(lambda g: c(g, 'board.std.remove')) / seats:.2f}",
        f"{sum(c(g, 'ai.power.protector.accord') for g in games) / prot_rounds:.2f}",
        f"{m(lambda g: c(g, 'threats.dealt_at_node')):.2f} / {m(lambda g: c(g, 'discord.offensive_idle')):.2f}",
    ]


HEAD = ["Arm", "Level", "Games", "Wins", "Rounds", "Research losses", "Won by r3", "Strike hits", "Sent home",
        "Troops redeployed", "Troop by r3", "Gain ambassador / seat-game", "Remove agents / seat-game",
        "Accord / Protector seat-round", "Offensive deals / idle"]


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    pairs = "--pairs" in sys.argv
    spec = json.load(open(args[0], encoding="utf-8"))
    names = [v["name"] for v in spec["variants"]]
    levels = spec["difficulties"]
    games = [g for p in args[1:] for g in load(p)]
    variant_of = lambda g: names[g["cell"] // len(levels)]
    groups = collections.defaultdict(list)
    for g in games:
        v = variant_of(g)
        pair, arm = v.split("-", 1)
        groups[(arm, g["difficulty"])].append(g)
        if pairs:
            groups[(f"{arm} {pair}", g["difficulty"])].append(g)
    print("| " + " | ".join(HEAD) + " |")
    print("|" + "---|" * len(HEAD))
    arms = list(dict.fromkeys(k[0] for k in groups))
    for arm in arms:
        for lv in levels:
            r = row(groups.get((arm, lv), []), variant_of)
            if r:
                print("| " + " | ".join([arm, f"d{lv}"] + [str(x) for x in r]) + " |")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    main()
