"""cardcmp.py DIR... [--out CSV]: per card, the oracle against the strategist (and the engine) on the same seeds and cells.

DIR: experiment outputs (results.json maps a cell to its agent; a DIR named eng-* holds the engine,
`engine:strategy=1`: the oracle's line policy without its search). Per agent and card: offered, kept
(drafted keeps) and played per game, the share of games it was played in, and the share of the agent's
wins it was played in. The oracle plays more of everything (longer games, more resources), so a card's
`lift` is its oracle/strategist play-rate ratio over the median ratio of cards both play in 5%+ of games;
`e_lift` the same for the engine. Flags: `oracle-only` (lift >= 2 and the oracle plays it in 10%+ of its
games; `(valuation)` when the engine's lift is also >= 2, `(search)` when it is under 1.5); `unused` (both
under 3%); `kept-unplayed` (the strategist keeps it in 10%+ of games, plays it in under 5%).
"""
import collections
import csv
import json
import os
import sys

dirs = [a for a in sys.argv[1:] if not a.startswith("--")]
out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else None
if out in dirs:
    dirs.remove(out)

stat = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0, 0, 0]))  # kind -> card -> offered, kept, played, played_in_win, games_played_in
games = collections.Counter()
wins = collections.Counter()
for d in dirs:
    res = json.load(open(os.path.join(d, "results.json"), encoding="utf-8"))
    eng = os.path.basename(d.rstrip("/")).startswith("eng-")
    kind = {c["cell"]: "engine" if eng else "oracle" if c["agent"].startswith("oracle") else "strategist" for c in res["cells"]}
    for line in open(os.path.join(d, "games.jsonl"), encoding="utf-8"):
        g = json.loads(line)
        if "seed" not in g:
            continue
        k = kind[g["cell"]]
        games[k] += 1
        wins[k] += g["won"]
        for card, c in g["cards"].items():
            s = stat[k][card]
            s[0] += c["offered"]
            s[1] += c["kept"]
            s[2] += c["played"]
            s[3] += 1 if (c["played"] and g["won"]) else 0
            s[4] += 1 if c["played"] else 0

kinds = [k for k in ["strategist", "engine", "oracle"] if games[k]]
cards = sorted(set().union(*[stat[k].keys() for k in kinds]))


def share(k, c):
    return stat[k][c][4] / max(games[k], 1)


def median_ratio(k):
    rs = sorted(share(k, c) / share("strategist", c) for c in cards if share(k, c) >= 0.05 and share("strategist", c) >= 0.05)
    return rs[len(rs) // 2] if rs else 1.0


med = {k: median_ratio(k) for k in kinds if k != "strategist"}
print("median play-rate ratio over the strategist (cards both play in 5%+ of games): "
      + ", ".join(f"{k} {v:.2f}" for k, v in med.items()))
rows = []
for card in cards:
    r = {"card": card}
    for k in kinds:
        s = stat[k][card]
        n = max(games[k], 1)
        r[f"{k}_offered"] = s[0] / n
        r[f"{k}_kept"] = s[1] / n
        r[f"{k}_played"] = s[2] / n
        r[f"{k}_in_games"] = s[4] / n
        r[f"{k}_win_share"] = s[3] / max(wins[k], 1)
    so, oo = r["strategist_in_games"], r["oracle_in_games"]
    r["lift"] = oo / max(so, 0.005) / med["oracle"]
    if "engine" in kinds:
        r["e_lift"] = r["engine_in_games"] / max(so, 0.005) / med["engine"]
    flag = ""
    if oo >= 0.10 and r["lift"] >= 2:
        flag = "oracle-only"
        if "engine" in kinds:
            flag += " (valuation)" if r["e_lift"] >= 2 else " (search)" if r["e_lift"] < 1.5 else ""
    elif oo < 0.03 and so < 0.03:
        flag = "unused"
    elif r["strategist_kept"] >= 0.10 and so < 0.05:
        flag = "kept-unplayed"
    r["flag"] = flag
    rows.append(r)

print(f"games: {dict(games)}; wins: {dict(wins)}")
rows.sort(key=lambda r: -r["lift"])
e = "engine" in kinds
hdr = (f"{'card':30s} {'s.kept':>6s} {'o.kept':>6s} {'s.play%':>7s}" + (f" {'e.play%':>7s}" if e else "")
       + f" {'o.play%':>7s} {'s.winsh':>7s} {'o.winsh':>7s} {'lift':>5s}" + (f" {'e.lift':>6s}" if e else "") + " flag")
lines = [hdr]
for r in rows:
    lines.append(f"{r['card']:30s} {r['strategist_kept']:6.2f} {r['oracle_kept']:6.2f} {100*r['strategist_in_games']:7.1f}"
                 + (f" {100*r['engine_in_games']:7.1f}" if e else "")
                 + f" {100*r['oracle_in_games']:7.1f} {100*r['strategist_win_share']:7.1f} {100*r['oracle_win_share']:7.1f} {r['lift']:5.2f}"
                 + (f" {r['e_lift']:6.2f}" if e else "") + f" {r['flag']}")
sys.stdout.reconfigure(encoding="utf-8", newline="\n")
print("\n".join(lines))
if out:
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        w.writeheader()
        for r in rows:
            w.writerow({k: (round(v, 4) if isinstance(v, float) else v) for k, v in r.items()})
