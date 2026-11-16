"""Compare a screen's arms against a base read on the same seeds, by whole-game resource use and wins.

usage: python arms.py SCREEN.jsonl BASE.jsonl [ARM NAMES...]
The screen's variants are arm-major, three pairs each (bp, br, pr), d0-d3: cell = (arm*3 + pair)*4 + level.
The base read is one arm (pair*4 + level), restricted to the screen's seeds."""
import json, sys, math
from collections import defaultdict
sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PEAKS = ["credits", "insight", "energy", "chips", "antimatter", "research_cubes", "card_tokens", "ambassadors"]
LEFT = ["credits", "chips", "energy", "antimatter", "ambassadors"]
args = sys.argv[1:]
A = 1
if args[0] == '-a':
    A, args = int(args[1]), args[2:]
screen, base = args[0], args[1]
names = args[2:]


def rows(path):
    with open(path, encoding='utf-8') as f:
        for l in f:
            if l.startswith('{"cell"'):
                yield json.loads(l)


def summarize(g):
    """Per-game numbers summed over seats."""
    boards = [b.split(':')[0] for b in g['build'].split('|')]
    d = defaultdict(float)
    d['won'] = 1.0 if g['won'] else 0.0
    d['rounds'] = g['rounds']
    for s, b in enumerate(boards):
        for i, r in enumerate(PEAKS):
            d[f'gain.{r}'] += g['gained'][s][i]
            d[f'spent.{r}'] += g['spent'][s][i]
        for i, r in enumerate(LEFT):
            d[f'held.{r}'] += g['margin']['held_after_actions'][s][i]
            d[f'held.{b}.{r}'] += g['margin']['held_after_actions'][s][i]
        d[f'seats.{b}'] += 1
    d['seats'] = len(boards)
    c = g['counters']
    d['tidal_chip'] = c.get('board.chip_converter.0', 0)
    d['conv.chips.all'] = sum(v for k, v in c.items() if k.startswith('board.chip_converter.') and k[-1].isdigit())
    for a in g['actions']:
        t = a.get('type')
        if t == 'standard' and a.get('act') in ('liberate', 'liberate_am', 'node', 'buy_energy', 'antimatter'):
            d['std.' + a['act']] += 1
        elif t == 'power':
            d[f"power.{boards[a['seat']]}.{a['power']}"] += 1
        elif t == 'use':
            d['use.' + a['card']] += 1
    cd = g['cards'].get('dyson_swarm_tranche', {})
    d['dyson.kept'] = cd.get('kept', 0)
    d['dyson.played'] = cd.get('played', 0)
    return d


new = defaultdict(dict)  # arm -> (seed, pair, level) -> summary
seeds = set()
for g in rows(screen):
    vl, ag = divmod(g['cell'], A)
    v, lv = divmod(vl, 4)
    arm, pair = divmod(v, 3)
    new[arm * A + ag][(g['seed'], pair, lv)] = summarize(g)
    seeds.add(g['seed'])
old = {}
for g in rows(base):
    if g['seed'] in seeds:
        pair, lv = divmod(g['cell'], 4)
        old[(g['seed'], pair, lv)] = summarize(g)
arms = [('base', old)] + [(names[a] if a < len(names) else f'arm{a}', new[a]) for a in sorted(new)]


def mean(xs):
    return sum(xs) / len(xs) if xs else float('nan')


print(f"seeds {len(seeds)}; games per arm {len(old)}")
print("\n## Wins by level (paired vs base: gap ± SE)")
print("| arm | d0 | d1 | d2 | d3 | all | rounds |")
print("|---|---|---|---|---|---|---|")
for name, arm in arms:
    cells = []
    for lv in range(4):
        keys = [k for k in arm if k[2] == lv and k in old]
        w = mean([arm[k]['won'] for k in keys]) * 100
        diffs = [arm[k]['won'] - old[k]['won'] for k in keys]
        m = mean(diffs) * 100
        sd = math.sqrt(sum((x * 100 - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)) / math.sqrt(max(1, len(diffs)))
        cells.append(f"{w:.1f} ({m:+.1f}±{sd:.1f})" if name != 'base' else f"{w:.1f}")
    keys = [k for k in arm if k in old]
    diffs = [arm[k]['won'] - old[k]['won'] for k in keys]
    m = mean(diffs) * 100
    sd = math.sqrt(sum((x * 100 - m) ** 2 for x in diffs) / max(1, len(diffs) - 1)) / math.sqrt(max(1, len(diffs)))
    allw = mean([arm[k]['won'] for k in keys]) * 100
    print(f"| {name} | " + " | ".join(cells) + f" | {allw:.1f} ({m:+.1f}±{sd:.1f}) | {mean([arm[k]['rounds'] for k in keys]):.2f} |")

print("\n## Wins by pair, all levels (paired gap)")
for name, arm in arms:
    out = []
    for p, pn in enumerate(['bp', 'br', 'pr']):
        keys = [k for k in arm if k[1] == p and k in old]
        out.append(f"{pn} {mean([arm[k]['won'] for k in keys]) * 100:.1f} ({mean([arm[k]['won'] - old[k]['won'] for k in keys]) * 100:+.1f})")
    print(f"- {name}: " + ", ".join(out))

print("\n## Resource use, per seat a game (all levels)")
print("| arm | energy gained / spent (share) | energy held after last actions | builder / protector / researcher held | antimatter gained / spent (share) | chips gained / spent (share) | Tidal chip conv / seat | all chip conv / seat | Dyson kept / played a game |")
print("|---|---|---|---|---|---|---|---|---|")
for name, arm in arms:
    ks = list(arm)
    S = sum(arm[k]['seats'] for k in ks)
    def per(f):
        return sum(arm[k][f] for k in ks) / S
    def perb(b, r):
        n = sum(arm[k][f'seats.{b}'] for k in ks)
        return sum(arm[k][f'held.{b}.{r}'] for k in ks) / n if n else float('nan')
    eg, es = per('gain.energy'), per('spent.energy')
    ag, as_ = per('gain.antimatter'), per('spent.antimatter')
    cg, cs = per('gain.chips'), per('spent.chips')
    print(f"| {name} | {eg:.1f} / {es:.1f} ({es / eg:.0%}) | {per('held.energy'):.2f} | {perb('builder', 'energy'):.1f} / {perb('protector', 'energy'):.1f} / {perb('researcher', 'energy'):.1f} | {ag:.2f} / {as_:.2f} ({as_ / ag if ag else 0:.0%}) | {cg:.1f} / {cs:.1f} ({cs / cg:.0%}) | {per('tidal_chip'):.2f} | {per('conv.chips.all'):.2f} | {sum(arm[k]['dyson.kept'] for k in ks) / len(ks):.2f} / {sum(arm[k]['dyson.played'] for k in ks) / len(ks):.2f} |")

print("\n## Energy held after the last action phase, by level (per seat)")
for name, arm in arms:
    out = []
    for lv in range(4):
        ks = [k for k in arm if k[2] == lv]
        out.append(f"d{lv} {sum(arm[k]['held.energy'] for k in ks) / sum(arm[k]['seats'] for k in ks):.2f}")
    print(f"- {name}: " + ", ".join(out))

print("\n## Actions per seat a game")
keys = sorted({f for _, arm in arms for k in arm for f in arm[k] if f.startswith(('std.', 'power.', 'use.'))})
want = [k for k in keys if k.startswith('std.') or k.startswith('power.builder') or k.startswith('use.') and any(x in k for x in ('patent', 'orbital', 'bug_bounty', 'positron', 'wafer', 'antiproton', 'cyclotron', 'containment'))]
print("| action | " + " | ".join(n for n, _ in arms) + " |")
print("|---|" + "---|" * len(arms))
for f in want:
    vals = []
    for _, arm in arms:
        S = sum(arm[k]['seats'] for k in arm)
        vals.append(f"{sum(arm[k].get(f, 0) for k in arm) / S:.2f}")
    print(f"| {f} | " + " | ".join(vals) + " |")
