"""313: rank the judge's disagreements with a recorded oracle by decision kind, from `aic-sim agree` outputs split by
the judge's result on the same seed (split.py): per kind, decisions a game, the judge's miss rate (picked otherwise,
not even later this round) in the games it lost and in those it won, and misses a game in its lost games beyond the
won-game rate (`excess`: where disagreeing goes with losing). `play:<card>` rows come from the `--cards` flag.

    python rank.py LOST.jsonl WON.jsonl [--agent strategist] [--cards N] [--group all]
"""
import argparse
import json


def load(path, agent, group):
    for line in open(path, encoding="utf-8"):
        j = json.loads(line)
        if j["agent"] == agent:
            return j["by"][group], j.get("games", 0)
    raise SystemExit(f"{path}: no {agent}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("lost")
    ap.add_argument("won")
    ap.add_argument("--agent", default="strategist")
    ap.add_argument("--group", default="all")
    ap.add_argument("--cards", type=int, default=0)
    ap.add_argument("--games", default="", help="games in LOST,WON (when the agree line has no count)")
    a = ap.parse_args()
    lost, gl = load(a.lost, a.agent, a.group)
    won, gw = load(a.won, a.agent, a.group)
    if a.games:
        gl, gw = map(int, a.games.split(","))
    rows = []
    for k in sorted(set(lost) | set(won)):
        if k == "any" or (k.startswith("play:") and not a.cards):
            continue
        l = lost.get(k, [0, 0, 0])
        w = won.get(k, [0, 0, 0])
        miss_l = (l[2] - l[1]) / l[2] if l[2] else 0.0
        miss_w = (w[2] - w[1]) / w[2] if w[2] else 0.0
        per_l = (l[2] - l[1]) / max(gl, 1)
        excess = per_l - miss_w * l[2] / max(gl, 1)
        rows.append((excess, k, l[2] / max(gl, 1), miss_l, miss_w, per_l))
    rows.sort(reverse=True)
    if a.cards:
        rows = [r for r in rows if r[1].startswith("play:")][: a.cards] + [r for r in rows if not r[1].startswith("play:")]
    print(f"games: judge lost {gl}, won {gw}")
    print("| kind | decisions a lost game | miss % (judge lost) | miss % (judge won) | misses a lost game | excess a lost game |")
    print("|---|---|---|---|---|---|")
    for excess, k, n, ml, mw, pl in rows:
        print(f"| {k} | {n:.1f} | {100 * ml:.0f} | {100 * mw:.0f} | {pl:.1f} | {excess:+.1f} |")


if __name__ == "__main__":
    main()
