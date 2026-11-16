"""248: read the standard-actions probe: per arm, wins and rounds by level (paired against arm a), each standard
action's uses a game, resource use, liberations by source, how games end.

usage: python probe.py PROBE.jsonl [--arms a,b,bd,c] [--extra READ.jsonl] [--md OUT.md]
PROBE.jsonl: p248's games (arm-major variants, three pairs, d0-d3: cell = (arm*3 + pair)*4 + level, one agent).
--extra: more arm-a games from read.json (cell = pair*4 + level), pooled into arm a's wins and rounds only
(the bands), never into the paired gaps."""
import argparse
import json
import math
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
RES = ["credits", "insight", "energy", "chips", "antimatter", "research_cubes", "card_tokens", "ambassadors"]
STD = ["node", "station", "remove", "troop", "lift", "move", "ambassador", "claim", "liberate", "botnet",
       "buy_energy", "sell", "insight", "envoy", "antimatter", "factory"]
NAMES = {"node": "Place a node", "station": "Build a research station", "remove": "Remove agents",
         "troop": "Remove a troop", "lift": "Lift Fallout", "move": "Dispatch", "ambassador": "Gain an ambassador",
         "claim": "Claim a site", "liberate": "Liberate a site", "botnet": "Take down a botnet",
         "buy_energy": "Buy a city's energy", "sell": "Sell a card", "insight": "Turn a cube into insight",
         "envoy": "Envoy", "antimatter": "Make antimatter", "factory": "Build a factory"}
BANDS = [(90, 101), (80, 95), (75, 90), (55, 70)]


def rows(path):
    with open(path, encoding='utf-8') as f:
        for line in f:
            if line.startswith('{"cell"'):
                yield json.loads(line)


def summarize(g):
    d = defaultdict(float)
    d['won'] = 1.0 if g['won'] else 0.0
    d['rounds'] = g['rounds']
    d['reason.' + g['reason']] = 1
    seats = len(g['gained'])
    d['seats'] = seats
    for s in range(seats):
        for i, r in enumerate(RES):
            d['gain.' + r] += g['gained'][s][i]
            d['spent.' + r] += g['spent'][s][i]
        uses = g['uses'][s]
        for k, v in uses.items():
            parts = k.split(':')
            if parts[0] != 'own':
                continue
            src = 'std' if parts[1] == 'standard' else 'sell' if parts[1] == 'sell' else parts[1]
            for i, r in enumerate(RES):
                d[f'use.{src}.{r}'] += v[i]
    boards = [b.split(':')[0] for b in g['build'].split('|')]
    for b in boards:
        d['seats.' + b] += 1
    rnd, prev = 0, None
    for act in g['actions']:
        t = act.get('type')
        # Each round's draft opens with the tier picks: a new round.
        if t == 'tier' and prev != 'tier':
            rnd += 1
        prev = t
        if t in ('standard', 'sell') and act.get('act') not in ('move', 'liberate'):
            d[f'stdr.{min(rnd, 12)}'] += 1
            d[f'stdlate.{"late" if rnd >= g["rounds"] - 1 else "early"}'] += 1
        if t == 'standard':
            d[f"bstd.{boards[act['seat']]}.{act['act']}"] += 1
        elif act.get('type') == 'sell':
            d[f"bstd.{boards[act['seat']]}.sell"] += 1
    c = g['counters']
    for a in STD:
        d['std.' + a] = c.get('board.std.' + a, 0)
    d['lib.card'] = c.get('board.liberated_card', 0)
    d['lib.all'] = sum(c.get('board.liberated.' + k, 0) for k in ('city', 'station', 'factory'))
    d['freed'] = c.get('board.cities_freed', 0)
    return d


def mean(xs):
    return sum(xs) / len(xs) if xs else float('nan')


def se(xs):
    if len(xs) < 2:
        return float('nan')
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1) / len(xs))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('probe')
    ap.add_argument('--arms', default='a,b,bd,c')
    ap.add_argument('--extra', default='')
    ap.add_argument('--md', default='')
    a = ap.parse_args()
    names = a.arms.split(',')
    arms = defaultdict(dict)
    for g in rows(a.probe):
        v, lv = divmod(g['cell'], 4)
        arm, pair = divmod(v, 3)
        arms[names[arm]][(g['seed'], pair, lv)] = summarize(g)
    extra = {}
    if a.extra:
        for g in rows(a.extra):
            pair, lv = divmod(g['cell'], 4)
            extra[(g['seed'], pair, lv)] = summarize(g)
    base = arms[names[0]]
    out = []
    p = out.append
    seeds = sorted({k[0] for k in base})
    p(f"Seeds {seeds[0]}-{seeds[-1]} ({len(seeds)}), {len(base)} games an arm" +
      (f"; arm {names[0]}'s bands pool {len(extra)} more ({min(k[0] for k in extra)}-{max(k[0] for k in extra)})" if extra else ""))
    p("\n## Wins by level (paired gap against arm a, ± SE)\n")
    p("| arm | d0 | d1 | d2 | d3 | all |")
    p("|---|---|---|---|---|---|")
    for n in names:
        arm = arms[n]
        cells = []
        for lv in list(range(4)) + [None]:
            ks = [k for k in arm if lv is None or k[2] == lv]
            w = mean([arm[k]['won'] for k in ks]) * 100
            if n == names[0]:
                cells.append(f"{w:.1f} ± {se([arm[k]['won'] * 100 for k in ks]):.1f}")
            else:
                diffs = [(arm[k]['won'] - base[k]['won']) * 100 for k in ks if k in base]
                cells.append(f"{w:.1f} ({mean(diffs):+.1f} ± {se(diffs):.1f})")
        p(f"| {n} | " + " | ".join(cells) + " |")
    if extra:
        pooled = dict(base)
        pooled.update(extra)
        p(f"\n**Bands, arm {names[0]} pooled ({len(pooled)} games):** " + ", ".join(
            f"d{lv} {mean([pooled[k]['won'] for k in pooled if k[2] == lv]) * 100:.1f} ± "
            f"{se([pooled[k]['won'] * 100 for k in pooled if k[2] == lv]):.1f} "
            f"({'met' if BANDS[lv][0] <= mean([pooled[k]['won'] for k in pooled if k[2] == lv]) * 100 <= BANDS[lv][1] else 'unmet'})"
            for lv in range(4)))
        p("Its second half alone (the fresh read on read.json): " + ", ".join(
            f"d{lv} {mean([extra[k]['won'] for k in extra if k[2] == lv]) * 100:.1f}" for lv in range(4)))
    p("\n## Wins by pair, all levels\n")
    for n in names:
        arm = arms[n]
        p(f"- {n}: " + ", ".join(
            f"{pn} {mean([arm[k]['won'] for k in arm if k[1] == i]) * 100:.1f}" for i, pn in enumerate(['bp', 'br', 'pr'])))
    p("\n## Rounds by level (all games / won games)\n")
    p("| arm | d0 | d1 | d2 | d3 |")
    p("|---|---|---|---|---|")
    for n in names:
        arm = arms[n]
        cells = []
        for lv in range(4):
            ks = [k for k in arm if k[2] == lv]
            cells.append(f"{mean([arm[k]['rounds'] for k in ks]):.2f} / {mean([arm[k]['rounds'] for k in ks if arm[k]['won']]):.2f}")
        p(f"| {n} | " + " | ".join(cells) + " |")
    p("\n## How games end (share of games)\n")
    reasons = sorted({f for n in names for k in arms[n] for f in arms[n][k] if f.startswith('reason.')})
    p("| arm | " + " | ".join(r[7:] for r in reasons) + " |")
    p("|---|" + "---|" * len(reasons))
    for n in names:
        arm = arms[n]
        p(f"| {n} | " + " | ".join(f"{mean([arm[k].get(r, 0) for k in arm]) * 100:.1f}" for r in reasons) + " |")
    p("\n## Standard actions, uses a game (all seats; all levels)\n")
    p("| action | " + " | ".join(names) + " |")
    p("|---|" + "---|" * len(names))
    for s in STD:
        vals = [mean([arms[n][k]['std.' + s] for k in arms[n]]) for n in names]
        if any(vals):
            p(f"| {NAMES[s]} | " + " | ".join(f"{x:.2f}" for x in vals) + " |")
    p("| **all standard actions** | " + " | ".join(
        f"{mean([sum(arms[n][k]['std.' + s] for s in STD) for k in arms[n]]):.2f}" for n in names) + " |")
    p(f"\n## Arm {names[0]}: standard actions per seat a game, by board\n")
    arm = base
    bl = ['builder', 'protector', 'researcher']
    p("| action | " + " | ".join(bl) + " |")
    p("|---|---|---|---|")
    for s in STD:
        vals = []
        for b in bl:
            n = sum(arm[k].get('seats.' + b, 0) for k in arm)
            vals.append(sum(arm[k].get(f'bstd.{b}.{s}', 0) for k in arm) / n if n else 0)
        if any(vals):
            p(f"| {NAMES[s]} | " + " | ".join(f"{x:.2f}" for x in vals) + " |")
    p(f"\n## Arm {names[0]}: when the standard actions come (Dispatch and Liberate aside), uses a game by round\n")
    p(", ".join(f"r{r} {mean([arm[k].get(f'stdr.{r}', 0) for k in arm]):.2f}" for r in range(1, 13)))
    late = mean([arm[k].get('stdlate.late', 0) for k in arm])
    early = mean([arm[k].get('stdlate.early', 0) for k in arm])
    p(f"\nIn a game's last two rounds: {late:.2f} a game; before them: {early:.2f} ({late / (late + early):.0%} late).")
    p("\n## Liberations a game\n")
    p("| arm | sites liberated | by the Liberate action | by a card | cities freed | won games: cities freed |")
    p("|---|---|---|---|---|---|")
    for n in names:
        arm = arms[n]
        ks = list(arm)
        p(f"| {n} | {mean([arm[k]['lib.all'] for k in ks]):.2f} | {mean([arm[k]['std.liberate'] for k in ks]):.2f} | "
          f"{mean([arm[k]['lib.card'] for k in ks]):.2f} | "
          f"{mean([arm[k]['freed'] for k in ks]):.2f} | {mean([arm[k]['freed'] for k in ks if arm[k]['won']]):.2f} |")
    p("\n## Resource use, per seat a game: gained / spent (spent share); standard actions' share of the spend\n")
    p("| resource | " + " | ".join(names) + " |")
    p("|---|" + "---|" * len(names))
    for r in RES:
        if r == 'card_tokens':
            continue
        cells = []
        for n in names:
            arm = arms[n]
            S = sum(arm[k]['seats'] for k in arm)
            gn = sum(arm[k]['gain.' + r] for k in arm) / S
            sp = sum(arm[k]['spent.' + r] for k in arm) / S
            st = sum(arm[k][f'use.std.{r}'] + arm[k][f'use.sell.{r}'] for k in arm) / S
            cells.append(f"{gn:.1f} / {sp:.1f} ({sp / gn if gn else 0:.0%}); std {st / sp if sp else 0:.0%}")
        p(f"| {r} | " + " | ".join(cells) + " |")
    p("\n## Spend by use, per seat a game (credits, energy, ambassadors, antimatter)\n")
    for r in ['credits', 'energy', 'ambassadors', 'antimatter', 'research_cubes', 'chips']:
        srcs = sorted({f.split('.')[1] for n in names for k in arms[n] for f in arms[n][k] if f.startswith('use.') and f.endswith('.' + r)})
        line = []
        for n in names:
            arm = arms[n]
            S = sum(arm[k]['seats'] for k in arm)
            vals = {s: sum(arm[k].get(f'use.{s}.{r}', 0) for k in arm) / S for s in srcs}
            line.append(f"{n}: " + ", ".join(f"{s} {v:.1f}" for s, v in sorted(vals.items(), key=lambda x: -x[1]) if v >= 0.05))
        p(f"- **{r}** — " + "; ".join(line))
    text = "\n".join(out)
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text + "\n")


main()
