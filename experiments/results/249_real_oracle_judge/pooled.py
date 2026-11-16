"""249: the ship rule's read (245's): agent B against agent A on the seeds both played, pooled over the given levels,
and by level: gap ± one paired SE.

    python experiments/results/249_real_oracle_judge/pooled.py READ_DIR A B [--levels 2,3]
"""
import json
import math
import sys
from collections import defaultdict


def main():
    d, a, b = sys.argv[1:4]
    levels = [int(x) for x in sys.argv[sys.argv.index("--levels") + 1].split(",")] if "--levels" in sys.argv else None
    lines = open(d + "/games.jsonl", encoding="utf-8").read().splitlines()
    spec = json.loads(lines[0])["spec"]
    cells = [(v["name"], lv, ag) for _ in spec["configs"] for v in spec["variants"] for lv in spec["difficulties"]
             for ag in spec["agents"]]
    won = defaultdict(dict)
    for line in lines[1:]:
        g = json.loads(line)
        v, lv, ag = cells[g["cell"]]
        won[(v, lv, g["seed"])][ag] = g["won"]

    def gap(keep):
        ds = [int(r[b]) - int(r[a]) for k, r in won.items() if keep(k[1]) and a in r and b in r]
        n = len(ds)
        m = sum(ds) / n
        sd = math.sqrt(sum((x - m) ** 2 for x in ds) / (n - 1))
        return n, 100 * m, 100 * sd / math.sqrt(n)
    for lv in sorted({k[1] for k in won}):
        n, m, se = gap(lambda x, lv=lv: x == lv)
        print(f"d{lv}: n {n}, {m:+.2f} ± {se:.2f} ({m / se if se else 0:+.1f} SE)")
    if levels:
        n, m, se = gap(lambda x: x in levels)
        print(f"pooled d{'-d'.join(map(str, levels))}: n {n}, {m:+.2f} ± {se:.2f} ({m / se:+.1f} SE)")


if __name__ == "__main__":
    main()
