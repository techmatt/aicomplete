"""231: compute made a game by source, and project throughput, per difficulty and agent, from a games.jsonl.

python compute_stats.py GAMES.jsonl [--by-agent]
Agents are read from the spec's cell order (variants x difficulties x agents, agents innermost).
"""
import json
import sys
from collections import defaultdict


def cells(spec):
    out = []
    for v in spec["variants"]:
        for d in spec["difficulties"]:
            for a in spec["agents"]:
                out.append((v["name"], d, a))
    return out


def main(path):
    rows = defaultdict(lambda: defaultdict(float))
    n = defaultdict(int)
    spec = None
    if "=" in path:  # a farm's out file has no header: OUT=SPEC.json
        path, spec_path = path.split("=", 1)
        spec = json.loads(open(spec_path, encoding="utf-8").read())
        table = cells(spec)
    with open(path, encoding="utf-8") as f:
        for line in f:
            g = json.loads(line)
            if "spec" in g:
                spec = g["spec"]
                table = cells(spec)
                continue
            if "cell" not in g:
                continue
            v, d, a = table[g["cell"]]
            a = a.split(":")[0]
            key = (a, d)
            c = g["counters"]
            n[key] += 1
            r = rows[key]
            r["won"] += 1 if g["won"] else 0
            r["rounds"] += g["rounds"]
            made = c.get("adversary.compute_made", 0)
            inc = c.get("adversary.compute_income", 0)
            bot = c.get("adversary.compute_botnets", 0)
            st = c.get("board.station_compute", 0)
            r["made"] += made
            r["income"] += inc
            r["botnets"] += bot
            r["stations"] += st
            r["other"] += made - inc - bot - st
            r["drained"] += c.get("adversary.compute_drained", 0)
            r["resolved"] += c.get("adversary.projects_resolved", 0)
            r["revealed"] += c.get("adversary.projects_revealed", 0)
            r["discarded"] += c.get("adversary.projects_discarded", 0)
            r["reshuffles"] += c.get("adversary.project_reshuffles", 0)
            r["seen_max"] = max(r["seen_max"], c.get("adversary.projects_revealed", 0) + c.get("adversary.projects_discarded", 0))
    cols = ["won", "rounds", "made", "income", "botnets", "stations", "other", "drained", "resolved", "revealed", "discarded", "reshuffles"]
    print("| agent | d | games | " + " | ".join(cols) + " | made a round | seen max |")
    print("|---" * (len(cols) + 5) + "|")
    for key in sorted(rows):
        r, k = rows[key], n[key]
        vals = [f"{100 * r['won'] / k:.1f}" if c == "won" else f"{r[c] / k:.2f}" for c in cols]
        print(f"| {key[0]} | {key[1]} | {k} | " + " | ".join(vals) + f" | {r['made'] / r['rounds']:.2f} | {int(r['seen_max'])} |")


if __name__ == "__main__":
    main(sys.argv[1])
