"""297: the content sweep's smoke table from farmed games (`farm-seeds` out files, one game a line).

    python smoke_table.py LABEL=FILE [LABEL=FILE ...] [--names bp,br,pr] [--pairs]

Each file's cells are `variant * 4 + level` (the farm spec's grid order: `experiments/farm/read.json`). A row a
label and level, the pairs pooled (`--pairs`: a row a pair too): wins, rounds, how games ended, the research slot
reached, the piles, then the troop table: games with a troop, the round the first one came, troops by source,
Strike icons and hits, Redeploy icons and troops moved, guard blocks, Discord taken off by Unity cards.
"""
import collections
import json
import sys

LEVELS = 4


def load(path):
    with open(path, encoding="utf-8") as f:
        return [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def c(g, k):
    return g["counters"].get(k, 0)


def placed(g):
    out = []
    for k, v in g["counters"].items():
        if k.startswith("map.troops_placed.") and v > 0:
            source, r = k[len("map.troops_placed."):].rsplit(".r", 1)
            out.append((source, int(r), v))
    return out


def first_troop(g):
    rounds = [r for _, r, _ in placed(g)]
    return min(rounds) if rounds else None


def median(xs):
    xs = sorted(xs)
    return xs[len(xs) // 2] if xs else 0


def row(label, games):
    n = len(games)
    wins = [g for g in games if g["won"]]
    by3 = sum(1 for g in wins if g["rounds"] <= 3)
    by5 = sum(1 for g in wins if g["rounds"] <= 5)
    rounds = [g["rounds"] for g in games]
    ends = collections.Counter("liberated" if g["won"] else g["reason"] for g in games)
    order = ["liberated", "deck_empty", "research", "discord_deck", "no_nodes"]
    how = ", ".join(f"{k} {ends[k]}" for k in order + sorted(set(ends) - set(order)) if ends[k])
    slots = [c(g, "adversary.research_fired") for g in games]
    high = sum(1 for s in slots if s >= 4)
    peak = [max([v for k, v in g["counters"].items() if k.startswith("adversary.ambassadors_r")] or [0]) for g in games]
    discord = [c(g, "alignment.sway_discord") for g in games]
    cubes = [c(g, "adversary.research_made") for g in games]
    return (f"| {label} | {n} | {100 * len(wins) / n:.0f}% | {mean(rounds):.1f} ({min(rounds)}-{max(rounds)}) | {by3} / {by5} "
            f"| {how} | {median(slots)}; {high} | {mean(peak):.0f} / {max(peak)} | {mean(discord):.0f} / {max(discord)} "
            f"| {mean(cubes):.0f} / {max(cubes)} |")


def troop_row(label, games):
    n = len(games)
    firsts = [first_troop(g) for g in games]
    seen = [f for f in firsts if f is not None]
    by = collections.Counter()
    for g in games:
        for source, _, v in placed(g):
            by[source] += v
    src = ", ".join(f"{k} {v / n:.2f}" for k, v in sorted(by.items())) or "-"
    when = f"{mean(seen):.1f} ({min(seen)}-{max(seen)})" if seen else "-"
    early = sum(1 for f in seen if f <= 3)
    blocked = sum(1 for g in games if c(g, "board.guard_blocked_rounds"))
    return (f"| {label} | {len(seen)} | {when} | {early} | {src} | {mean([c(g, 'board.strikes') for g in games]):.2f} "
            f"| {mean([c(g, 'board.troop_strikes') for g in games]):.2f} | {mean([c(g, 'board.redeploys') for g in games]):.2f} "
            f"/ {mean([c(g, 'board.troops_redeployed') for g in games]):.2f} | {blocked} | {mean([c(g, 'board.std.troop') for g in games]):.2f} "
            f"| {mean([c(g, 'alignment.discord_pulled') for g in games]):.1f} |")


def main():
    args = sys.argv[1:]
    names, pairs, files = "bp,br,pr".split(","), False, []
    i = 0
    while i < len(args):
        if args[i] == "--names":
            names = args[i + 1].split(",")
            i += 2
        elif args[i] == "--pairs":
            pairs = True
            i += 1
        else:
            files.append(args[i].split("=", 1))
            i += 1
    groups = []
    for level in range(LEVELS):
        for label, path in files:
            games = [g for g in load(path) if g["difficulty"] == level]
            if pairs:
                for v, name in enumerate(names):
                    mine = [g for g in games if g["cell"] // LEVELS == v]
                    if mine:
                        groups.append((f"d{level} {label} {name}", mine))
            groups.append((f"d{level} {label}", games))
    print("| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 / 5 | How games ended | Slot reached: median; games at slot 4+ "
          "| Agents on board, peak: mean / max | Discord swayed: mean / max | Research cubes made: mean / max |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for label, games in groups:
        print(row(label, games))
    print()
    print("| Read | Games with a troop | First troop, round: mean (min-max) | Games with one by round 3 | Troops placed a game, by source "
          "| Strike icons a game | Strike hits a game | Redeploy icons / troops moved a game | Games a troop alone blocked a Liberate "
          "| Remove a troop a game | Discord taken off by Unity cards a game |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for label, games in groups:
        print(troop_row(label, games))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")  # 298: a redirected table was CRLF on Windows
    main()
