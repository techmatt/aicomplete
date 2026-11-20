"""295: the early-win table from farmed games (a `farm-seeds` out file, one game a line).

    python early_table.py LABEL=FILE [LABEL=FILE ...] [--pool] [--by-level] [--spec=SPEC.json]

Each file's cells are `variant * levels + level` (the farm spec's grid order; the variant names and the levels come
from the file's header, or from `--spec` for a read farmed with no header: `experiments/farm/read.json`). A row a variant and level (`--pool`: the pairs of one cell pooled, a variant named
`<pair>_<cell>` pooling under `<cell>`; `--by-level`: one row a level, every variant pooled): games, wins, rounds,
the share of games won by round 3 and by round 5, the fastest win, how each game ended, and a game's liberations
(cities by the Liberate action and by a card, stations and factories), ambassador moves and Liberate actions.
"""
import collections
import json
import sys


def load(path):
    head, games = None, []
    with open(path, encoding="utf-8") as f:
        for line in f:
            if not line.strip():
                continue
            r = json.loads(line)
            if "cell" in r:
                games.append(r)
            elif head is None and "spec" in r:
                head = r
    return head, games


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def c(g, k):
    return g["counters"].get(k, 0)


def moves(g):
    return sum(v for k, v in g["counters"].items() if k.startswith("board.amb_moves."))


def row(label, games):
    n = len(games)
    wins = [g for g in games if g["won"]]
    by = lambda r: sum(1 for g in wins if g["rounds"] <= r)
    rounds = [g["rounds"] for g in games]
    ends = collections.Counter(g["reason"] for g in games)
    how = ", ".join(f"{k} {v}" for k, v in sorted(ends.items(), key=lambda kv: (-kv[1], kv[0])))
    fastest = min((g["rounds"] for g in wins), default=None)
    cities = mean([c(g, "board.liberated.city") for g in games])
    card = mean([c(g, "board.liberated_card.city") for g in games])
    sites = mean([c(g, "board.liberated.station") + c(g, "board.liberated.factory") for g in games])
    return (f"| {label} | {n} | {len(wins)} ({100 * len(wins) / n:.0f}%) | {mean(rounds):.1f} ({min(rounds)}-{max(rounds)}) "
            f"| {by(3)} ({100 * by(3) / n:.0f}%) | {by(5)} ({100 * by(5) / n:.0f}%) | {fastest if fastest is not None else '-'} "
            f"| {how} | {cities:.2f} ({card:.2f} by card) | {sites:.2f} "
            f"| {mean([c(g, 'board.std.liberate') for g in games]):.2f} | {mean([moves(g) for g in games]):.1f} |")


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    pool, by_level = "--pool" in sys.argv, "--by-level" in sys.argv
    specs = [a.split("=", 1)[1] for a in sys.argv[1:] if a.startswith("--spec=")]
    print("| Read | Games | Wins | Rounds: mean (min-max) | Won by round 3 | Won by round 5 | Fastest win | How games ended "
          "| Cities liberated a game | Stations and factories liberated a game | Liberate actions a game | Ambassador moves a game |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for a in args:
        label, path = a.split("=", 1)
        head, games = load(path)
        # A farmed read whose spec names no `first_seed` leads with no header: `--spec=FILE` names its spec.
        spec = head["spec"] if head else json.load(open(specs[0], encoding="utf-8"))
        names = [v["name"] for v in spec["variants"]]
        levels = spec["difficulties"]
        groups = collections.OrderedDict()
        for g in games:
            v, lv = divmod(g["cell"], len(levels))
            name = names[v]
            if by_level:
                key = f"d{levels[lv]}"
            elif pool:
                cell = name.split("_", 1)[1] if "_" in name else "asked"
                key = cell if len(levels) == 1 else f"{cell} d{levels[lv]}"
            else:
                key = name if len(levels) == 1 else f"{name} d{levels[lv]}"
            groups.setdefault(key, []).append(g)
        for key, gs in groups.items():
            print(row(f"{label} {key}", gs))


if __name__ == "__main__":
    main()
