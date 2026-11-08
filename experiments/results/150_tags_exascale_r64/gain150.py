"""150: the play eval's one-ply read of each kept pick (the `plan_eval` trace) against how often its card is played.

python gain150.py TRACE DIR

TRACE holds `DBGK seed round seat card kept gain` lines (stderr of a run of the traced build in plan_eval.patch's
era with AIC_DBG150 set; `seed` is the game's internal seed, not the spec's, so the join is per card, not per game);
DIR is that run's output. Per card kept in at least 8 games: games kept, the share of those that played it (from
games.jsonl), and the eval's median gain at the keep, sorted by the gain. Played cards sit all along the gain
column: the one-ply gain at a keep doesn't tell a played card from an unplayed one.
"""
import collections
import json
import os
import statistics
import sys

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def main():
    trace, d = sys.argv[1], sys.argv[2]
    kept, played = collections.Counter(), collections.Counter()
    with open(os.path.join(d, "games.jsonl"), encoding="utf-8") as f:
        f.readline()
        for line in f:
            for k, v in json.loads(line)["cards"].items():
                if v.get("kept", 0) > 0:
                    kept[k] += 1
                    played[k] += v.get("played", 0) > 0
    gains, seen = collections.defaultdict(list), set()
    for line in open(trace, encoding="utf-8", errors="replace"):
        if not line.startswith("DBGK "):
            continue
        _, seed, rnd, seat, card, k, gain = line.split()
        if k != "1" or gain == "NaN" or (seed, rnd, seat, card) in seen:
            continue
        seen.add((seed, rnd, seat, card))
        gains[card].append(float(gain))
    print("| card | games kept | played, % of kept | median one-ply gain at the keep |")
    print("|---|---|---|---|")
    rows = sorted((statistics.median(xs), c) for c, xs in gains.items() if kept[c] >= 8)
    for med, c in rows:
        print(f"| {c} | {kept[c]} | {100.0 * played[c] / kept[c]:.0f} | {med:.1f} |")


if __name__ == "__main__":
    main()
