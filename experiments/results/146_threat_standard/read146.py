"""146's read: per cell (lever, difficulty, agent), the three seat pairs pooled.

    python experiments/results/146_threat_standard/read146.py DIR [DIR ...] [--vs OLD_DIR] [--tiers] [--harms]

Win % (+-SE), rounds, ends, nodes lost a game by cause, threats a game by tier (dealt / countered / uncountered and
the counter rate), what paid. `--vs OLD_DIR`: the paired gap (same pair, difficulty, agent and seed) against OLD's
cell of the same difficulty and agent (its base lever). `--harms`: each threat's uncountered executions over the
runs (`threats.uncountered.id.<id>`), so every harm type can be seen landing.
"""

import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8", newline="\n")


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
            name, diff, a = cells[g["cell"]]
            pair, _, lever = name.partition("_")
            g["_pair"], g["_lever"], g["_diff"], g["_agent"] = pair, lever or "base", diff, a
            games.append(g)
    return head, games


def c(g, k):
    return g["counters"].get(k, 0)


def mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def win(gs):
    p = mean([1.0 if g["won"] else 0.0 for g in gs])
    return 100 * p, 100 * math.sqrt(p * (1 - p) / max(1, len(gs)))


def paired(gs, old):
    base = {(g["_pair"], g["_diff"], g["_agent"], g["seed"]): g["won"] for g in old if g["_lever"] == "base"}
    d = [float(g["won"]) - float(base[k]) for g in gs if (k := (g["_pair"], g["_diff"], g["_agent"], g["seed"])) in base]
    if not d:
        return None
    m = mean(d)
    sd = math.sqrt(sum((x - m) ** 2 for x in d) / max(1, len(d) - 1))
    return 100 * m, 100 * sd / math.sqrt(len(d)), len(d)


def row(gs, old=None, tiers=False):
    w, se = win(gs)
    ends = Counter(g["reason"] for g in gs)
    out = (f"win {w:5.1f} ±{se:3.1f} (n {len(gs)}); rounds {mean([g['rounds'] for g in gs]):.2f}; "
           f"nodes lost: threats {mean([sum(v for k, v in g['counters'].items() if k.startswith('board.node_lost.threat')) for g in gs]):.2f}, "
           f"corruption {mean([c(g, 'board.node_lost.corruption') for g in gs]):.2f}; "
           f"uncountered {mean([c(g, 'threats.uncountered') for g in gs]):.2f}/{mean([c(g, 'threats.dealt') for g in gs]):.1f}")
    if old is not None:
        p = paired(gs, old)
        if p:
            out += f"; paired vs old {p[0]:+.1f} ±{p[1]:.1f} ({p[2]})"
    if tiers:
        parts = []
        for t in (1, 2, 3):
            dealt = mean([c(g, f"threats.dealt.t{t}") for g in gs])
            if not dealt:
                continue
            cnt = mean([c(g, f"threats.countered.t{t}") for g in gs])
            unc = mean([c(g, f"threats.uncountered.t{t}") for g in gs])
            parts.append(f"t{t} {dealt:.1f}/{cnt:.1f}/{unc:.1f} ({100 * cnt / max(1e-9, cnt + unc):.0f}% countered)")
        out += "\n    threats dealt/countered/uncountered: " + "; ".join(parts)
        out += "\n    ends: " + ", ".join(f"{k} {v}" for k, v in ends.most_common())
    return out


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    vs = None
    if "--vs" in sys.argv:
        vs = sys.argv[sys.argv.index("--vs") + 1]
        args = [a for a in args if a != vs]
    old = load(vs)[1] if vs else None
    games = []
    for d in args:
        games += load(d)[1]
    cells = defaultdict(list)
    for g in games:
        cells[(g["_lever"], g["_diff"], g["_agent"])].append(g)
    for (lever, diff, agent), gs in sorted(cells.items(), key=lambda x: (x[0][0] != "base", x[0])):
        print(f"- {lever} d{diff} {agent}: {row(gs, old, '--tiers' in sys.argv)}")
    if old is not None and "--tiers" in sys.argv:
        ocells = defaultdict(list)
        for g in old:
            ocells[(g["_lever"], g["_diff"], g["_agent"])].append(g)
        for (lever, diff, agent), gs in sorted(ocells.items()):
            if lever == "base":
                print(f"- OLD {lever} d{diff} {agent}: {row(gs, None, True)}")
    if "--harms" in sys.argv:
        tot = Counter()
        for g in games:
            for k, v in g["counters"].items():
                if k.startswith("threats.uncountered.id."):
                    tot[k[len("threats.uncountered.id."):]] += v
        print(f"uncountered executions by threat ({len(games)} games):")
        print(", ".join(f"{k} {v}" for k, v in sorted(tot.items())))


if __name__ == "__main__":
    main()
