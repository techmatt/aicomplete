"""294: the troops-and-winning smoke table from farmed games (a `farm-seeds` out file, one game a line).

    python smoke_table.py LABEL=FILE [LABEL=FILE ...] [--names bp,br,pr] [--levels 4] [--level 0]

Each file's cells are `variant * levels + level` (the farm spec's grid order). Prints, a variant and overall: games,
wins (and how many by round 5), rounds, how each loss happened, the research slot reached by game end, and the
piles' peaks; then, for a build with troops that garrison (r141 on), the troop table: games with a troop, the round
the first one arrived, troops at their peak and by source, Strike icons and hits a game, Redeploys, and how often a
troop stood between the players and a liberation.
"""
import collections
import json
import sys


def load(path):
    with open(path, encoding="utf-8") as f:
        return [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def c(g, k):
    return g["counters"].get(k, 0)


def placed(g):
    """Troops placed, as (source, round, count): the per-round table `map.troops_placed.<source>.rNN`."""
    out = []
    for k, v in g["counters"].items():
        if k.startswith("map.troops_placed.") and v > 0:
            source, r = k[len("map.troops_placed."):].rsplit(".r", 1)
            out.append((source, int(r), v))
    return out


def first_troop(g):
    rounds = [r for _, r, _ in placed(g)]
    return min(rounds) if rounds else None


def row(label, games):
    n = len(games)
    wins = [g for g in games if g["won"]]
    early = sum(1 for g in wins if g["rounds"] <= 5)
    rounds = [g["rounds"] for g in games]
    losses = collections.Counter(g["reason"] for g in games if not g["won"])
    slots = collections.Counter(c(g, "adversary.research_fired") for g in games)
    peak = [max([v for k, v in g["counters"].items() if k.startswith("adversary.ambassadors_r")] or [0]) for g in games]
    discord = [c(g, "alignment.sway_discord") for g in games]
    cubes = [c(g, "adversary.research_made") for g in games]
    how = ", ".join(f"{k} {v}" for k, v in sorted(losses.items())) or "-"
    slot = " ".join(f"{k}:{v}" for k, v in sorted(slots.items()))
    return (f"| {label} | {n} | {len(wins)} ({100 * len(wins) / n:.0f}%), {early} by r5 | {mean(rounds):.1f} ({min(rounds)}-{max(rounds)}) "
            f"| {how} | {slot} | {mean(peak):.1f} / {max(peak)} | {mean(discord):.1f} / {max(discord)} "
            f"| {mean(cubes):.1f} / {max(cubes)} | {mean([g['margin']['troops_end'] for g in games]):.2f} / {max(g['margin']['troops_end'] for g in games)} |")


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
    return (f"| {label} | {len(seen)} / {n} | {when}; {early} by r3 | {src} | {mean([c(g, 'adversary.troops_peak') for g in games]):.2f} / {max(c(g, 'adversary.troops_peak') for g in games)} "
            f"| {mean([c(g, 'board.strikes') for g in games]):.2f} | {mean([c(g, 'board.troop_strikes') for g in games]):.2f} "
            f"| {mean([c(g, 'board.troop_destroyed.node') for g in games]):.2f} | {mean([c(g, 'board.redeploys') for g in games]):.2f} / {mean([c(g, 'board.troops_redeployed') for g in games]):.2f} "
            f"| {mean([c(g, 'board.guarded_city_rounds') for g in games]):.2f} | {mean([c(g, 'board.guard_blocked_rounds') for g in games]):.2f}, in {sum(1 for g in games if c(g, 'board.guard_blocked_rounds'))} games "
            f"| {mean([c(g, 'board.std.troop') for g in games]):.2f} |")


def main():
    args = sys.argv[1:]
    opts = {"--names": "bp,br,pr", "--level": "0", "--levels": "4"}
    files = []
    i = 0
    while i < len(args):
        if args[i] in opts:
            opts[args[i]] = args[i + 1]
            i += 2
        else:
            files.append(args[i].split("=", 1))
            i += 1
    names = opts["--names"].split(",")
    levels, level = int(opts["--levels"]), int(opts["--level"])
    groups = []
    for label, path in files:
        games = [g for g in load(path) if g["difficulty"] == level]
        for v, name in enumerate(names):
            mine = [g for g in games if g["cell"] // levels == v]
            if mine:
                groups.append((f"{label} {name}", mine))
        if len(names) > 1 and len({g["cell"] // levels for g in games}) <= 3:
            groups.append((f"{label} all", games))
    print("| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max "
          "| Discord swayed: mean / max | Research cubes made: mean / max | Troops at the end: mean / max |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for label, games in groups:
        print(row(label, games))
    print()
    print("| Read | Games with a troop | First troop, round: mean (min-max) | Troops placed a game, by source | Troops, peak: mean / max "
          "| Strike icons a game | Strike hits a game | Nodes lost to strikes a game | Redeploy icons / troops moved a game "
          "| Corrupted-city rounds under guard a game | Rounds a troop alone blocked a Liberate, a game | Remove a troop actions a game |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for label, games in groups:
        print(troop_row(label, games))


if __name__ == "__main__":
    main()
