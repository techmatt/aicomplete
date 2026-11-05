"""117: paired reads of win rate and credits left at game end, per cell and per seat.

    python experiments/results/117_big_cards_and_income/read.py levels DIR
        one run whose variants are named <pair>_<level>: each level against <pair>_base, same seeds
    python experiments/results/117_big_cards_and_income/read.py pair OLD_DIR NEW_DIR [LABEL_OLD LABEL_NEW]
        two runs of the same spec (old and new binary), paired on cell and seed

Credits left are `margin.unspent[seat][0]` (what a seat held when its last action phase ended). Seat 0 and seat 1
are the pair's boards in order: bp = Builder, Protector; br = Builder, Researcher; pr = Protector, Researcher.
"""

import json
import statistics as st
import sys

BOARDS = {"bp": ("B", "P"), "br": ("B", "R"), "pr": ("P", "R")}


def load(d):
    with open(f"{d}/games.jsonl", encoding="utf-8") as f:
        lines = f.read().splitlines()
    head = json.loads(lines[0])
    spec = head["spec"]
    cells = []
    for v in spec["variants"]:
        for diff in spec["difficulties"]:
            for a in spec["agents"]:
                cells.append((v["name"], a))
    games = {}
    for ln in lines[1:]:
        g = json.loads(ln)
        games[(cells[g["cell"]], g["seed"])] = g
    return head.get("rules_revision"), cells, games


def se_diff(d):
    return st.stdev(d) / len(d) ** 0.5 if len(d) > 1 else 0.0


def cr(g, s):
    return g["margin"]["unspent"][s][0]


def row(label, pair, keys_old, keys_new, go, gn):
    wo = [int(go[k]["won"]) for k in keys_old]
    wn = [int(gn[k]["won"]) for k in keys_new]
    d = [b - a for a, b in zip(wo, wn)]
    out = [label, str(len(d)), f"{100 * st.mean(wo):.0f}", f"{100 * st.mean(wn):.0f}",
           f"{100 * st.mean(d):+.0f} ± {100 * se_diff(d):.0f}",
           f"{st.mean(go[k]['rounds'] for k in keys_old):.1f} → {st.mean(gn[k]['rounds'] for k in keys_new):.1f}"]
    for s in (0, 1):
        co = [cr(go[k], s) for k in keys_old]
        cn = [cr(gn[k], s) for k in keys_new]
        dd = [b - a for a, b in zip(co, cn)]
        out.append(f"{BOARDS[pair][s]} {st.mean(co):.0f} → {st.mean(cn):.0f} ({st.mean(dd):+.0f} ± {se_diff(dd):.0f})")
    return "| " + " | ".join(out) + " |", wo, wn, d


HEAD = ("| cell | games | win % {a} | win % {b} | diff ± SE | rounds | credits left, seat 0 | credits left, seat 1 |\n"
        "|---|---|---|---|---|---|---|---|")


def summary(acc, a, b):
    lines = []
    for agent, (cro, crn, wo, wn, d) in sorted(acc.items()):
        dd = [y - x for x, y in zip(cro, crn)]
        lines.append(f"- {agent}: won {100 * st.mean(wo):.0f}% → {100 * st.mean(wn):.0f}% "
                     f"({100 * st.mean(d):+.1f} ± {100 * se_diff(d):.1f}); credits left a seat "
                     f"{st.mean(cro):.1f} → {st.mean(crn):.1f} ({st.mean(dd):+.1f} ± {se_diff(dd):.1f}), {a} → {b}")
    return "\n".join(lines)


def per_board(rows):
    """rows: list of (board letter, old credits, new credits)."""
    out = {}
    for bl, o, n in rows:
        out.setdefault(bl, ([], []))
        out[bl][0].append(o)
        out[bl][1].append(n)
    return "; ".join(f"{bl} {st.mean(o):.0f} → {st.mean(n):.0f}" for bl, (o, n) in sorted(out.items()))


def levels(d):
    _, cells, g = load(d)
    lv = sorted({c[0].split("_", 1)[1] for c in cells} - {"base"})
    agents = sorted({c[1] for c in cells})
    for level in lv:
        print(f"\n### level {level} against base\n")
        print(HEAD.format(a="base", b=level))
        acc = {}
        boards = {a: [] for a in agents}
        for pair in BOARDS:
            for a in agents:
                ko = sorted(k for k in g if k[0] == (f"{pair}_base", a))
                seeds = [k[1] for k in ko]
                kn = [((f"{pair}_{level}", a), s) for s in seeds]
                if not ko or any(k not in g for k in kn):
                    continue
                line, wo, wn, dl = row(f"{pair} {a}", pair, ko, kn, g, g)
                print(line)
                e = acc.setdefault(a, ([], [], [], [], []))
                for s in (0, 1):
                    e[0].extend(cr(g[k], s) for k in ko)
                    e[1].extend(cr(g[k], s) for k in kn)
                    boards[a].extend((BOARDS[pair][s], cr(g[x], s), cr(g[y], s)) for x, y in zip(ko, kn))
                e[2].extend(wo)
                e[3].extend(wn)
                e[4].extend(dl)
        print()
        print(summary(acc, "base", level))
        for a in agents:
            print(f"- {a} credits left by board: {per_board(boards[a])}")


def pair(old, new, la="old", lb="new"):
    ro, cells, go = load(old)
    rn, _, gn = load(new)
    print(f"{la} (r{ro}) → {lb} (r{rn}), paired on the same seeds\n")
    print(HEAD.format(a=la, b=lb))
    acc = {}
    boards = {}
    for cell in cells:
        keys = sorted(k for k in go if k[0] == cell and k in gn)
        if not keys:
            continue
        pr = cell[0].split("_")[0]
        line, wo, wn, dl = row(f"{cell[0]} {cell[1]}", pr, keys, keys, go, gn)
        print(line)
        e = acc.setdefault(cell[1], ([], [], [], [], []))
        for s in (0, 1):
            e[0].extend(cr(go[k], s) for k in keys)
            e[1].extend(cr(gn[k], s) for k in keys)
            boards.setdefault(cell[1], []).extend((BOARDS[pr][s], cr(go[k], s), cr(gn[k], s)) for k in keys)
        e[2].extend(wo)
        e[3].extend(wn)
        e[4].extend(dl)
    print()
    print(summary(acc, la, lb))
    for a, rows_ in sorted(boards.items()):
        print(f"- {a} credits left by board: {per_board(rows_)}")
    for label, gs in ((la, go), (lb, gn)):
        reasons = {}
        for gg in gs.values():
            reasons[gg["reason"]] = reasons.get(gg["reason"], 0) + 1
        print(f"- {label} ended by {dict(sorted(reasons.items()))}")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    if sys.argv[1] == "levels":
        levels(sys.argv[2])
    else:
        pair(*sys.argv[2:])
