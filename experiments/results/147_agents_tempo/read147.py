"""147's read: per cell (difficulty, agent), the three seat pairs pooled.

    python experiments/results/147_agents_tempo/read147.py DIR [DIR ...] [--vs OLD_DIR] [--stages]

Win % (+-SE), rounds, ends, nodes lost a game by cause (threats, corruption), threats a game by tier (dealt /
countered / uncountered, the counter rate). `--stages`: the counter rate by stage (I-III) and tier
(`threats.countered.s<stage>.t<tier>`, `threats.uncountered.s<stage>.t<tier>`, new in 147). `--vs OLD_DIR`: the
paired gap (same pair, difficulty and seed) against OLD's cell of the same difficulty whose agent has the same kind
(the part before ':'), so the final agents read against r61's on the same games.
"""

import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


def load(d):
    games, cells = [], None
    with open(Path(d) / "games.jsonl", encoding="utf-8") as f:
        for line in f:
            g = json.loads(line)
            if cells is None:
                spec = g["spec"]
                cells = [(v["name"], diff, a) for _cfg in spec["configs"] for v in spec.get("variants") or [{"name": "base"}]
                         for diff in spec.get("difficulties", [0]) for a in spec["agents"]]
                continue
            g["_pair"], g["_diff"], g["_agent"] = cells[g["cell"]]
            games.append(g)
    return games


def c(g, k):
    return g["counters"].get(k, 0)


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def win(gs):
    p = mean([1.0 if g["won"] else 0.0 for g in gs])
    return 100 * p, 100 * math.sqrt(p * (1 - p) / max(1, len(gs)))


def kind(agent):
    return agent.split(":")[0]


def paired(gs, old):
    base = {(g["_pair"], g["_diff"], kind(g["_agent"]), g["seed"]): g["won"] for g in old}
    d = [float(g["won"]) - float(base[k]) for g in gs if (k := (g["_pair"], g["_diff"], kind(g["_agent"]), g["seed"])) in base]
    if not d:
        return None
    m = mean(d)
    sd = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1))
    return 100 * m, 100 * sd / math.sqrt(len(d)), len(d)


def rate(cnt, unc):
    return f"{100 * cnt / (cnt + unc):.0f}%" if cnt + unc else "-"


def row(gs, old=None, stages=False):
    w, se = win(gs)
    ends = Counter(g["reason"] for g in gs)
    out = (f"win {w:5.1f} ±{se:3.1f} (n {len(gs)}); rounds {mean([g['rounds'] for g in gs]):.2f}; "
           f"nodes lost: threats {mean([sum(v for k, v in g['counters'].items() if k.startswith('board.node_lost.threat')) for g in gs]):.2f}, "
           f"corruption {mean([c(g, 'board.node_lost.corruption') for g in gs]):.2f}; "
           f"countered {mean([c(g, 'threats.countered') for g in gs]):.2f} / uncountered {mean([c(g, 'threats.uncountered') for g in gs]):.2f}")
    if old is not None:
        p = paired(gs, old)
        if p:
            out += f"; paired vs old {p[0]:+.1f} ±{p[1]:.1f} ({p[2]})"
    parts = []
    for t in (1, 2):
        cnt = mean([c(g, f"threats.countered.t{t}") for g in gs])
        unc = mean([c(g, f"threats.uncountered.t{t}") for g in gs])
        parts.append(f"t{t} {cnt:.1f}/{unc:.1f} ({rate(cnt, unc)})")
    out += "\n    countered/uncountered a game: " + "; ".join(parts)
    if stages:
        st = []
        for s in (1, 2, 3):
            for t in (1, 2):
                cnt = mean([c(g, f"threats.countered.s{s}.t{t}") for g in gs])
                unc = mean([c(g, f"threats.uncountered.s{s}.t{t}") for g in gs])
                if cnt + unc:
                    st.append(f"{'I' * s} t{t} {rate(cnt, unc)} ({cnt + unc:.1f})")
        out += "\n    by stage: " + "; ".join(st)
    out += "\n    ends: " + ", ".join(f"{k} {v}" for k, v in ends.most_common())
    return out


def main():
    argv = sys.argv[1:]
    vs = argv[argv.index("--vs") + 1] if "--vs" in argv else None
    dirs = [a for a in argv if not a.startswith("--") and a != vs]
    old = []
    if vs:
        for d in vs.split(","):
            old += load(d)
    games = []
    for d in dirs:
        games += load(d)
    cells = defaultdict(list)
    for g in games:
        cells[(g["_diff"], g["_agent"])].append(g)
    for (diff, agent), gs in sorted(cells.items()):
        print(f"- d{diff} {agent}: {row(gs, old if vs else None, '--stages' in argv)}")
    if vs:
        ocells = defaultdict(list)
        for g in old:
            ocells[(g["_diff"], g["_agent"])].append(g)
        for (diff, agent), gs in sorted(ocells.items()):
            print(f"- OLD d{diff} {agent}: {row(gs, None, '--stages' in argv)}")


if __name__ == "__main__":
    main()
