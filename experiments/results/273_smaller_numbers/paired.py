"""273: the paired read: win % and rounds, new against old on the same cell and seed, by level and agent (and pooled).

    python experiments/results/273_smaller_numbers/paired.py NEW.jsonl OLD.jsonl

Cells run variant-major (three pairs), then level, then agent (strategist, greedy): `cell % 2` is the agent,
`(cell // 2) % 5` the level. Prints new, old, the paired gap and its SE, in points; rounds likewise.
"""
import json
import math
import sys

AGENTS = ('strategist', 'greedy')


def load(path):
    out = {}
    for line in open(path, encoding='utf-8'):
        g = json.loads(line)
        if isinstance(g.get('cell'), int):
            out[(g['cell'], g['seed'])] = (1.0 if g['won'] else 0.0, g['rounds'])
    return out


def stats(xs):
    n = len(xs)
    m = sum(xs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1)) if n > 1 else 0.0
    return m, sd / math.sqrt(n)


new, old = load(sys.argv[1]), load(sys.argv[2])
keys = sorted(set(new) & set(old))
print(f'{len(keys)} paired games')
print('| Agent | Level | win % new | old | gap ± SE | rounds new | old | gap ± SE |')
print('|---|---|---|---|---|---|---|---|')
for agent in AGENTS:
    for lv in list(range(5)) + ['all']:
        ks = [k for k in keys if AGENTS[k[0] % 2] == agent and (lv == 'all' or (k[0] // 2) % 5 == lv)]
        wn = [new[k][0] for k in ks]
        wo = [old[k][0] for k in ks]
        rn = [new[k][1] for k in ks]
        ro = [old[k][1] for k in ks]
        gw, sw = stats([a - b for a, b in zip(wn, wo)])
        gr, sr = stats([a - b for a, b in zip(rn, ro)])
        name = f'd{lv}' if lv != 'all' else 'd0-d4'
        print(f'| {agent} | {name} | {100 * sum(wn) / len(ks):.0f} | {100 * sum(wo) / len(ks):.0f} | '
              f'{100 * gw:+.1f} ± {100 * sw:.1f} | {sum(rn) / len(ks):.2f} | {sum(ro) / len(ks):.2f} | {gr:+.2f} ± {sr:.2f} |')
