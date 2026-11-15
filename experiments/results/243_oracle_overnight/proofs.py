"""proofs.py plan CARDS K DIFF DIR... | proofs.py read

plan: for each card id (comma list), up to K games of the read at level DIFF the oracle won with the card
offered or drawn in its game; writes one spec per game in out/243_oracle_overnight/pf/ and pf/jobs.txt.
read: per card, games proven (won with the card played), won without it, lost.
"""
import glob
import json
import os
import sys

D = os.path.dirname(os.path.abspath(__file__))
PF = os.path.join(D, "pf")
ORACLE = "oracle:strategy=1,alts=32,budget=192,reseeds=64,answers=1"
SETS = {"bp": [], "br": ["ai.boards=0,2"], "pr": ["ai.boards=1,2"]}


def plan(cards, k, diff, dirs):
    rows = [l.split(",")[0] for l in open(os.path.join(D, "cards.csv"), encoding="utf-8").read().splitlines()[1:]]
    os.makedirs(PF, exist_ok=True)
    jobs = []
    for card in cards:
        idx = rows.index(card) + 1
        picked = []
        for d in dirs:
            res = json.load(open(os.path.join(D, d, "results.json"), encoding="utf-8"))
            cell = {c["cell"]: c for c in res["cells"]}
            for line in open(os.path.join(D, d, "games.jsonl"), encoding="utf-8"):
                g = json.loads(line)
                if "seed" not in g:
                    continue
                c = cell[g["cell"]]
                if not c["agent"].startswith("oracle") or c["difficulty"] != diff or not g["won"]:
                    continue
                cc = g["cards"].get(card, {})
                # Offered or drawn, and not already played: a game the read did not prove.
                if cc.get("offered", 0) + cc.get("drawn", 0) == 0 or cc.get("played", 0) > 0:
                    continue
                picked.append((c["variant"], int(g["seed"]), cc.get("played", 0)))
                if len(picked) >= k:
                    break
            if len(picked) >= k:
                break
        for v, seed, played in picked:
            name = f"pf-{card}-{v}-{seed}"
            spec = {"name": name, "note": f"243 card proof: {card} (row {idx}), already played in the read: {played}",
                    "configs": ["configs/baseline.json"], "variants": [{"name": v, "set": SETS[v]}],
                    "difficulties": [diff], "agents": [f"{ORACLE},prove={idx}"], "games": 1, "first_seed": seed, "records": 0}
            with open(os.path.join(PF, name + ".json"), "w", encoding="utf-8", newline="\n") as f:
                json.dump(spec, f)
            jobs.append(name)
        print(card, idx, len(picked), "games")
    with open(os.path.join(PF, "jobs.txt"), "a", encoding="utf-8", newline="\n") as f:
        f.write("".join(j + "\n" for j in jobs))


def read():
    out = {}
    for p in sorted(glob.glob(os.path.join(PF, "pf-*", "games.jsonl"))):
        name = os.path.basename(os.path.dirname(p))
        card = name[3:].rsplit("-", 2)[0]
        for line in open(p, encoding="utf-8"):
            g = json.loads(line)
            if "seed" not in g:
                continue
            played = g["cards"].get(card, {}).get("played", 0) > 0
            o = out.setdefault(f"{card} d{g['difficulty']}", [0, 0, 0, 0.0])
            if g["won"] and played:
                o[0] += 1
            elif g["won"]:
                o[1] += 1
            else:
                o[2] += 1
            o[3] += g["agent_ms"] / 1000
    print(f"{'card':32s} proven  won-without  lost  s/game")
    for card, (a, b, c, s) in sorted(out.items()):
        n = a + b + c
        print(f"{card:32s} {a:3d}/{n:<3d} {b:5d}      {c:5d} {s / max(n, 1):7.0f}")


if __name__ == "__main__":
    if sys.argv[1] == "plan":
        plan(sys.argv[2].split(","), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5:])
    else:
        read()
