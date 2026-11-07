"""142's read: per cell (lever, difficulty), the three seat pairs pooled.

    python experiments/results/142_threat_intensity/read142.py DIR [DIR ...] [--vs OLD_DIR] [--full] [--md OUT]

Win % (±SE), rounds, ends, nodes lost a game by cause, threats by tier (dealt / countered / uncountered), counters
paid by currency (all tiers, and the share of all counters paid), credits peak a seat (by board) and spent a seat,
claims a game by site type and route. `--vs` adds the paired win gap (same pair, difficulty and seed) against an
older run's base cell. `--full` prints the long rows; the default is one line a cell.
"""

import json
import math
import statistics
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8", newline="\n")
BOARDS = {"bp": ("builder", "protector"), "br": ("builder", "researcher"), "pr": ("protector", "researcher")}
KINDS = ["credits", "energy", "insight", "ambassadors", "research_cubes", "chips", "tags", "nodes", "card", "standing"]


def load(d):
    head, games = None, []
    with open(Path(d) / "games.jsonl", encoding="utf-8") as f:
        for line in f:
            g = json.loads(line)
            if head is None:
                head = g
                cells = []
                spec = g["spec"]
                for _cfg in spec["configs"]:
                    for v in spec.get("variants") or [{"name": "base"}]:
                        for diff in spec.get("difficulties", [0]):
                            for a in spec["agents"]:
                                cells.append((v["name"], diff, a))
                continue
            name, diff, _a = cells[g["cell"]]
            pair, _, lever = name.partition("_")
            g["_pair"], g["_lever"], g["_diff"] = pair, lever or "base", diff
            games.append(g)
    return head, games


def c(g, k):
    return g["counters"].get(k, 0)


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def win(gs):
    p = mean([1.0 if g["won"] else 0.0 for g in gs])
    return 100 * p, 100 * math.sqrt(p * (1 - p) / max(1, len(gs)))


def cell_rows(gs):
    n = len(gs)
    w, se = win(gs)
    rounds = [g["rounds"] for g in gs]
    ends = Counter(g["reason"] for g in gs)
    lost = defaultdict(float)
    for g in gs:
        for k, v in g["counters"].items():
            if k.startswith("board.node_lost.") and k.count(".") == 2:
                lost[k.split(".")[2]] += v / n
    tiers = {t: (mean([c(g, f"threats.dealt.t{t}") for g in gs]), mean([c(g, f"threats.countered.t{t}") for g in gs]),
                 mean([c(g, f"threats.uncountered.t{t}") for g in gs])) for t in (1, 2, 3)}
    paid = {k: mean([c(g, f"threats.countered_by.{k}") for g in gs]) for k in KINDS}
    total = sum(paid.values()) or 1
    paid_t = {t: {k: mean([c(g, f"threats.countered_by.t{t}.{k}") for g in gs]) for k in KINDS} for t in (1, 2, 3)}
    peaks, spent = defaultdict(list), []
    for g in gs:
        for s, b in enumerate(BOARDS[g["_pair"]]):
            peaks[b].append(g["peaks"][s][0])
            spent.append(g["spent"][s][0])
    claims = {}
    for kind in ("city", "station", "factory"):
        for route in ("ambassadors", "card"):
            # r58 has no route key: every claim was the ambassador route.
            claims[f"{kind}.{route}"] = mean([
                c(g, f"board.claimed_by.{route}.{kind}") if any(k.startswith("board.claimed_by.") for k in g["counters"])
                else (c(g, f"board.claimed.{kind}") if route == "ambassadors" else 0) for g in gs])
    return {
        "n": n, "win": w, "se": se, "mean_rounds": mean(rounds), "median": statistics.median(rounds) if rounds else 0,
        "ends": dict(ends), "lost": dict(lost), "tiers": tiers, "paid": paid, "share": {k: 100 * v / total for k, v in paid.items()},
        "paid_t": paid_t, "peaks": {b: mean(v) for b, v in peaks.items()}, "peak": mean([x for v in peaks.values() for x in v]),
        "spent": mean(spent),
        "claims": claims,
    }


def paired(new, old):
    key = lambda g: (g["_pair"], g["_diff"], g["seed"])
    o = {key(g): g for g in old}
    d = [(1.0 if g["won"] else 0.0) - (1.0 if o[key(g)]["won"] else 0.0) for g in new if key(g) in o]
    if not d:
        return None
    m = mean(d)
    sd = statistics.pstdev(d) if len(d) > 1 else 0
    return 100 * m, 100 * sd / math.sqrt(len(d)), len(d)


def main():
    args = sys.argv[1:]
    vs = None
    md = None
    if "--vs" in args:
        i = args.index("--vs")
        vs = args[i + 1]
        del args[i:i + 2]
    if "--md" in args:
        i = args.index("--md")
        md = args[i + 1]
        del args[i:i + 2]
    full = "--full" in args
    args = [a for a in args if a != "--full"]
    old = load(vs)[1] if vs else None
    out = []
    for d in args:
        head, games = load(d)
        out.append(f"## {d} (r{head['rules_revision']})")
        by = defaultdict(list)
        for g in games:
            by[(g["_lever"], g["_diff"])].append(g)
        for (lever, diff), gs in sorted(by.items()):
            r = cell_rows(gs)
            gap = ""
            if old:
                ob = [g for g in old if g["_lever"] == "base"]
                p = paired(gs, ob)
                if p:
                    gap = f", paired vs old {p[0]:+.1f} ±{p[1]:.1f}"
            lost = r["lost"]
            threat_nodes = sum(v for k, v in lost.items() if k.startswith("threat"))
            share = ", ".join(f"{k} {v:.0f}%" for k, v in sorted(r["share"].items(), key=lambda x: -x[1]) if v >= 0.5)
            out.append(
                f"- **{lever} d{diff}** ({r['n']}): win {r['win']:.1f} ±{r['se']:.1f}{gap}; rounds {r['mean_rounds']:.2f} "
                f"(median {r['median']:.0f}); nodes lost: threats {threat_nodes:.2f} / corruption {lost.get('corruption', 0):.2f}; "
                f"credits peak {r['peak']:.0f}, spent {r['spent']:.0f} a seat; counters paid: {share}")
            if full:
                out.append(f"    ends {r['ends']}")
                out.append("    nodes lost " + ", ".join(f"{k} {v:.2f}" for k, v in sorted(lost.items())))
                out.append("    threats by tier (dealt / countered / uncountered): " + "; ".join(
                    f"t{t} {a:.1f} / {b:.1f} / {u:.2f}" for t, (a, b, u) in r["tiers"].items()))
                for t in (1, 2, 3):
                    out.append(f"    t{t} paid: " + ", ".join(f"{k} {v:.2f}" for k, v in r["paid_t"][t].items() if v > 0))
                out.append("    credits peak by board: " + ", ".join(f"{b} {v:.0f}" for b, v in r["peaks"].items()))
                out.append("    claims a game: " + ", ".join(f"{k} {v:.2f}" for k, v in r["claims"].items() if v > 0))
    text = "\n".join(out)
    print(text)
    if md:
        Path(md).write_text(text + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
