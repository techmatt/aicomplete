"""283's fresh read (282's read.py, unchanged but this line), both sides: per agent and level, win %, rounds, cities corrupted by round 4 and at the end, node
endings, research troops, node losses by march, card plays and non-card turns a seat-round.

python read.py NEW.jsonl NEW_SPEC.json OLD.jsonl OLD_SPEC.json
"""
import json
import sys
from collections import defaultdict


def cnt(g, k):
    v = g['counters'].get(k, 0)
    return v if isinstance(v, (int, float)) else 0


GROWTH = ('sprawl', 'buy_in', 'annex', 'levy', 'outpost', 'accord', 'field_survey', 'peer_review', 'field_station')


def side(path, spec_path):
    spec = json.load(open(spec_path, encoding='utf-8'))
    agents = spec['agents']
    rows = defaultdict(list)
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            rows[(agents[g['cell'] % len(agents)], g['difficulty'])].append(g)
    out = {}
    for key, gs in rows.items():
        n = len(gs)
        sr = sum(len(g['spent']) * g['rounds'] for g in gs)
        plays = std = powers = uses = 0
        for g in gs:
            for k, v in g['counters'].items():
                p = k.split('.')
                if not isinstance(v, (int, float)):
                    continue
                if k.startswith('cards.played.') and len(p) == 3:
                    plays += v
                elif k.startswith('board.std.') and len(p) == 3:
                    std += v
                elif k.startswith('ai.power.') and len(p) == 4 and not p[3].startswith('r') and p[3] not in GROWTH:
                    powers += v
                elif k.startswith('cards.used.') and len(p) == 3:
                    uses += v
        out[key] = dict(
            n=n, win=100 * sum(g['won'] for g in gs) / n, rounds=sum(g['rounds'] for g in gs) / n,
            r4=sum(sum(cnt(g, f'map.cities_corrupted.r0{r}') for r in range(1, 5)) for g in gs) / n,
            end=sum(g['margin'].get('cities_corrupted_end', 0) for g in gs) / n,
            node=sum(g['reason'] == 'node' for g in gs), rt=sum(cnt(g, 'adversary.research_troops') for g in gs) / n,
            march=sum(cnt(g, 'board.node_lost.march') for g in gs) / n, plays=plays / sr, noncard=(std + powers + uses) / sr)
    return out


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    new, old = side(sys.argv[1], sys.argv[2]), side(sys.argv[3], sys.argv[4])
    print('| agent | d | win % old → new | rounds | cities corrupted by r4 | at the end | node endings | research troops | nodes lost to the march | plays / non-card turns a seat-round |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for key in sorted(new):
        a, b = old[key], new[key]
        print(f"| {key[0]} | d{key[1]} | {a['win']:.1f} → **{b['win']:.1f}** | {a['rounds']:.2f} → {b['rounds']:.2f} | "
              f"{a['r4']:.2f} → {b['r4']:.2f} | {a['end']:.2f} → {b['end']:.2f} | {a['node']} → {b['node']} | "
              f"{a['rt']:.1f} → {b['rt']:.1f} | {a['march']:.1f} → {b['march']:.1f} | "
              f"{a['plays']:.2f} / {a['noncard']:.2f} → {b['plays']:.2f} / {b['noncard']:.2f} |")


if __name__ == '__main__':
    main()
