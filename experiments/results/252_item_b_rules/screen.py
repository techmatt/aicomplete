"""252: the paired direction screen. Reads two games.jsonl of the same spec and seeds (base r112, new r113: 251's
i251-judges, greedy and the judge, three pairs, d0-d3, dev seeds 36800-36831) and prints wins (paired gap and SE),
rounds, nodes by round and route, standard-action uses a seat-game, liberations and claims, and resource use.

    python screen.py BASE.jsonl NEW.jsonl BASE_AIS.json NEW_AIS.json [--md OUT.md]
"""
import json
import math
import sys
from collections import defaultdict

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
# Cells in spec order: variants (bp, br, pr) x difficulties (0-3) x agents (greedy, strategist).
VARIANTS, DIFFS, AGENTS = ['bp', 'br', 'pr'], [0, 1, 2, 3], ['greedy', 'strategist']


def cell_key(c):
    v, rest = divmod(c, len(DIFFS) * len(AGENTS))
    d, a = divmod(rest, len(AGENTS))
    return VARIANTS[v], DIFFS[d], AGENTS[a]


def games(path):
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' in g and 'won' in g:
                yield g


def load_powers(path):
    return {b['id']: [p['name'] for p in b['powers']] for b in json.load(open(path, encoding='utf-8'))}


def summarize(path, powers):
    out = {'wins': {}, 'rounds': defaultdict(list), 'nodes_r': defaultdict(float), 'route': defaultdict(float),
           'std': defaultdict(float), 'seatgames': defaultdict(int), 'games': defaultdict(int),
           'spent': defaultdict(lambda: [0] * 8), 'gained': defaultdict(lambda: [0] * 8), 'ctr': defaultdict(float)}
    for g in games(path):
        var, d, agent = cell_key(g['cell'])
        out['wins'][(var, d, agent, g['seed'])] = 1 if g['won'] else 0
        out['rounds'][agent].append(g['rounds'])
        out['games'][agent] += 1
        boards = PAIRS[var]
        for s, b in enumerate(boards):
            out['seatgames'][(agent, b)] += 1
            for i in range(8):
                out['spent'][agent][i] += g['spent'][s][i]
                out['gained'][agent][i] += g['gained'][s][i]
        for k, v in g['counters'].items():
            if k.startswith('board.node_placed.r'):
                out['nodes_r'][(agent, int(k[-2:]))] += v
            if k.startswith(('board.liberated.', 'board.claimed.', 'board.liberated_card')) or k == 'board.node_placed':
                out['ctr'][(agent, k)] += v
        for a in g['actions']:
            if a.get('type') == 'power':
                b = boards[a['seat']]
                name = powers[b][a['power']]
                out['route'][(agent, b, name)] += 1
            elif a.get('type') == 'standard':
                out['std'][(agent, a['act'])] += 1
    return out


def paired(base, new, agent, d=None):
    keys = [k for k in base['wins'] if k[2] == agent and (d is None or k[1] == d) and k in new['wins']]
    diffs = [new['wins'][k] - base['wins'][k] for k in keys]
    n = len(diffs)
    if n == 0:
        return 0, 0, 0, 0
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(n - 1, 1))
    b = sum(base['wins'][k] for k in keys) / n
    return 100 * b, 100 * (b + m), 100 * m, 100 * sd / math.sqrt(n)


def main():
    a = sys.argv[1:]
    md = a[a.index('--md') + 1] if '--md' in a else None
    base = summarize(a[0], load_powers(a[2]))
    new = summarize(a[1], load_powers(a[3]))
    lines = []
    p = lines.append
    p('## Wins (paired, same seeds; base r112, new r113)\n')
    p('| agent | level | r112 | r113 | gap ± SE |')
    p('|---|---|---|---|---|')
    for agent in AGENTS:
        for d in DIFFS + [None]:
            b, n, m, se = paired(base, new, agent, d)
            p(f'| {agent} | {"all" if d is None else f"d{d}"} | {b:.1f} | {n:.1f} | {m:+.1f} ± {se:.1f} |')
    p('\n## Rounds (mean, all games)\n')
    for agent in AGENTS:
        rb, rn = base['rounds'][agent], new['rounds'][agent]
        p(f'- {agent}: r112 {sum(rb) / len(rb):.2f}, r113 {sum(rn) / len(rn):.2f}')
    p('\n## Nodes placed a game, by round (both seats)\n')
    p('| agent | rev | ' + ' | '.join(f'r{r}' for r in range(1, 11)) + ' | all |')
    p('|---|---|' + '---|' * 11)
    for agent in AGENTS:
        for name, s in (('r112', base), ('r113', new)):
            ng = s['games'][agent]
            row = [s['nodes_r'][(agent, r)] / ng for r in range(1, 11)]
            p(f'| {agent} | {name} | ' + ' | '.join(f'{x:.2f}' for x in row) + f' | {s["ctr"][(agent, "board.node_placed")] / ng:.2f} |')
    p('\n## Node routes and other powers a seat-game (r113)\n')
    p('| agent | board | power | r112 | r113 |')
    p('|---|---|---|---|---|')
    keys = sorted({k for k in list(base['route']) + list(new['route'])})
    for agent, b, name in keys:
        sg_b, sg_n = base['seatgames'][(agent, b)] or 1, new['seatgames'][(agent, b)] or 1
        p(f'| {agent} | {b} | {name} | {base["route"][(agent, b, name)] / sg_b:.2f} | {new["route"][(agent, b, name)] / sg_n:.2f} |')
    p('\n## Standard actions a seat-game\n')
    p('| agent | action | r112 | r113 |')
    p('|---|---|---|---|')
    for agent in AGENTS:
        sgb = sum(v for (ag, _), v in base['seatgames'].items() if ag == agent)
        sgn = sum(v for (ag, _), v in new['seatgames'].items() if ag == agent)
        acts = sorted({k[1] for k in list(base['std']) + list(new['std']) if k[0] == agent})
        tb = tn = 0.0
        for act in acts:
            vb, vn = base['std'][(agent, act)] / sgb, new['std'][(agent, act)] / sgn
            if act not in ('move', 'liberate'):
                tb += vb
                tn += vn
            p(f'| {agent} | {act} | {vb:.2f} | {vn:.2f} |')
        p(f'| {agent} | **beyond Dispatch and Liberate** | **{tb:.2f}** | **{tn:.2f}** |')
    p('\n## Liberations and claims a game\n')
    p('| agent | counter | r112 | r113 |')
    p('|---|---|---|---|')
    for agent in AGENTS:
        ks = sorted({k[1] for k in list(base['ctr']) + list(new['ctr']) if k[0] == agent and k[1] != 'board.node_placed'})
        for k in ks:
            p(f'| {agent} | {k} | {base["ctr"][(agent, k)] / base["games"][agent]:.2f} | {new["ctr"][(agent, k)] / new["games"][agent]:.2f} |')
    p('\n## Resource use (spent / gained, a seat-game)\n')
    p('| agent | resource | r112 spent | r112 share | r113 spent | r113 share |')
    p('|---|---|---|---|---|---|')
    for agent in AGENTS:
        sgb = sum(v for (ag, _), v in base['seatgames'].items() if ag == agent)
        sgn = sum(v for (ag, _), v in new['seatgames'].items() if ag == agent)
        for i, r in enumerate(PEAKS):
            if r == 'card_tokens':
                continue
            sb, gb = base['spent'][agent][i], base['gained'][agent][i]
            sn, gn = new['spent'][agent][i], new['gained'][agent][i]
            p(f'| {agent} | {r} | {sb / sgb:.1f} | {100 * sb / max(gb, 1):.0f}% | {sn / sgn:.1f} | {100 * sn / max(gn, 1):.0f}% |')
    text = '\n'.join(lines) + '\n'
    print(text)
    if md:
        with open(md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main()
