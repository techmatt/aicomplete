"""293: the smoke table from farmed games (a `farm-seeds` out file, one game a line).

    python smoke_table.py LABEL=FILE [LABEL=FILE ...] [--cells N_PER_VARIANT] [--names bp,br,pr] [--level D]

Each file's cells are `variant * levels + level` (the farm spec's grid order). Prints, a variant and overall: games,
wins, rounds (mean, min-max), how each loss happened, the research slot reached by game end (its distribution), and
the piles: the peak of agents on the board, Discord swayed, research cubes made, the agent supply's low point.
"""
import collections
import json
import sys


def load(path):
    with open(path, encoding="utf-8") as f:
        return [r for r in (json.loads(line) for line in f if line.strip()) if "cell" in r]


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def row(label, games):
    n = len(games)
    wins = sum(1 for g in games if g["won"])
    rounds = [g["rounds"] for g in games]
    losses = collections.Counter(g["reason"] for g in games if not g["won"])
    c = lambda g, k: g["counters"].get(k, 0)
    slots = collections.Counter(c(g, "adversary.research_fired") for g in games)
    peak = [max([v for k, v in g["counters"].items() if k.startswith("adversary.ambassadors_r")] or [0]) for g in games]
    discord = [c(g, "alignment.sway_discord") for g in games]
    cubes = [c(g, "adversary.research_made") for g in games]
    supply = [g["margin"]["supply_left"] for g in games]
    how = ", ".join(f"{k} {v}" for k, v in sorted(losses.items())) or "-"
    slot = " ".join(f"{k}:{v}" for k, v in sorted(slots.items()))
    return (f"| {label} | {n} | {wins} ({100 * wins / n:.0f}%) | {mean(rounds):.1f} ({min(rounds)}-{max(rounds)}) | {how} "
            f"| {slot} | {mean(peak):.1f} / {max(peak)} | {mean(discord):.1f} / {max(discord)} "
            f"| {mean(cubes):.1f} / {max(cubes)} | {min(supply)} |")


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
    print("| Read | Games | Wins | Rounds: mean (min-max) | Losses | Slot reached (slot:games) | Agents on board, peak: mean / max "
          "| Discord swayed: mean / max | Research cubes made: mean / max | Supply low |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for label, path in files:
        games = [g for g in load(path) if g["difficulty"] == level]
        for v, name in enumerate(names):
            mine = [g for g in games if g["cell"] // levels == v]
            if mine:
                print(row(f"{label} {name}", mine))
        print(row(f"{label} all", games))


if __name__ == "__main__":
    main()
