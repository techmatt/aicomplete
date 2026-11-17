"""254 (253's read253.py, with city energy by route, liberate_energy and owned facilities built): the read's tables. Each SOURCE is LABEL=GAMES.jsonl:SPEC.json:AIS.json (a farmed or experiment output, the spec
that made it, the content/ais.json of its build for power names). A spec's variants are named PAIR or PAIR-ARM (bp, br,
pr); cells are variant-major (variant, level, agent). Every (label, arm) is a row; the first row is the base the paired
gaps compare against (same pair, level, agent and seed).

    python read253.py LABEL=G:S:A [LABEL=G:S:A ...] [--md OUT.md] [--base LABEL/ARM]

Tables: wins by level (paired gap and SE), loss reasons, nodes placed by round per board, nodes by source (base route,
board routes, cards), standard actions a seat-game, Liberate and Claim a seat-game, owned sites made a game, retakes,
the new cards' plays, the share of wins through cards, resource use (spent / gained and whether it binds).
"""
import json
import math
import sys
from collections import defaultdict

RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
BASE_ROUTE = {'Expand'}
BOARD_ROUTES = {'Buy In', 'Annex', 'Accord', 'Outpost', 'Peer Review', 'Field Survey'}
NEW_CARDS = ['hostile_buyout', 'open_research_campus', 'industrial_commons']


def load(src):
    label, rest = src.split('=', 1)
    # Windows paths: split on ':' after the drive letter.
    parts = []
    cur = ''
    for i, ch in enumerate(rest):
        if ch == ':' and not (len(cur) == 1 and cur.isalpha()):
            parts.append(cur)
            cur = ''
        else:
            cur += ch
    parts.append(cur)
    gpath, spath, apath = parts
    spec = json.load(open(spath, encoding='utf-8'))
    powers = {b['id']: [p['name'] for p in b['powers']] for b in json.load(open(apath, encoding='utf-8'))}
    vs, ds, ags = spec['variants'], spec['difficulties'], spec['agents']
    out = []
    with open(gpath, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'cell' not in g or 'won' not in g:
                continue
            v, r = divmod(g['cell'], len(ds) * len(ags))
            d, a = divmod(r, len(ags))
            name = vs[v]['name']
            pair, _, arm = name.partition('-')
            agent = ags[a].split(':')[0]
            out.append(((label, arm or 'base'), pair, ds[d], agent, int(g['seed']), g, powers))
    return out


def summarize(rows):
    S = defaultdict(lambda: {
        'wins': {}, 'reason': defaultdict(int), 'games': 0, 'seatgames': 0, 'sg_board': defaultdict(int),
        'nodes': defaultdict(float), 'src': defaultdict(float), 'std': defaultdict(float), 'ctr': defaultdict(float),
        'plays': defaultdict(float), 'spent': [0] * 8, 'gained': [0] * 8, 'won': 0, 'cardwin': 0, 'cardshare': 0.0,
        'rounds': 0, 'd3loss': defaultdict(int), 'd3games': 0, 'power': defaultdict(float)})
    for key, pair, d, agent, seed, g, powers in rows:
        s = S[(key, agent)]
        s['wins'][(pair, d, seed)] = 1 if g['won'] else 0
        s['games'] += 1
        s['rounds'] += g['rounds']
        if not g['won']:
            s['reason'][g['reason']] += 1
        if d == 3:
            s['d3games'] += 1
            if not g['won']:
                s['d3loss'][g['reason']] += 1
        boards = PAIRS[pair]
        for i, b in enumerate(boards):
            s['seatgames'] += 1
            s['sg_board'][b] += 1
            for k in range(8):
                s['spent'][k] += g['spent'][i][k]
                s['gained'][k] += g['gained'][i][k]
        c = g['counters']
        for k, v in c.items():
            if k.startswith('board.node_placed.s') and '.r' in k:
                seat = int(k.split('.s')[1].split('.')[0])
                s['nodes'][(boards[seat], int(k[-2:]))] += v
            if k.startswith(('board.liberated', 'board.claimed.', 'adversary.retakes', 'board.std.', 'board.city_energy_',
                             'effects.per_site.owned')) or k in (
                    'board.node_placed', 'board.claimed_liberated', 'board.node_lost', 'board.liberate_energy',
                    'board.factories_built.player', 'board.stations_built_free', 'margin.cities_aligned_end'):
                s['ctr'][k] += v
        card_acts = std_acts = 0
        for act in g['actions']:
            t = act.get('type')
            if t == 'power':
                name = powers[boards[act['seat']]][act['power']]
                s['power'][(boards[act['seat']], name)] += 1
                if name in BASE_ROUTE:
                    s['src']['base route'] += 1
                elif name in BOARD_ROUTES:
                    s['src']['board routes'] += 1
            elif t == 'standard':
                s['std'][act['act']] += 1
                if act['act'] == 'node':
                    s['src']['Place a node'] += 1
                if act['act'] not in ('move', 'liberate'):
                    std_acts += 1
            elif t in ('play', 'use'):
                card_acts += 1
                if t == 'play' and act['card'] in NEW_CARDS:
                    s['plays'][act['card']] += 1
        s['src']['all'] += c.get('board.node_placed', 0)
        if g['won']:
            s['won'] += 1
            s['cardshare'] += card_acts / max(card_acts + std_acts, 1)
            if card_acts > std_acts:
                s['cardwin'] += 1
    for s in S.values():
        s['src']['cards and other'] = s['src']['all'] - s['src']['base route'] - s['src']['board routes'] - s['src']['Place a node']
    return S


def paired(base, new, d=None):
    keys = [k for k in base['wins'] if (d is None or k[1] == d) and k in new['wins']]
    n = len(keys)
    if not n:
        return None
    diffs = [new['wins'][k] - base['wins'][k] for k in keys]
    m = sum(diffs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in diffs) / max(n - 1, 1))
    b = sum(base['wins'][k] for k in keys) / n
    return 100 * b, 100 * (b + m), 100 * m, 100 * sd / math.sqrt(n), n


def main():
    args = sys.argv[1:]
    md = args[args.index('--md') + 1] if '--md' in args else None
    base_name = args[args.index('--base') + 1] if '--base' in args else None
    srcs = [a for i, a in enumerate(args) if '=' in a and (i == 0 or args[i - 1] not in ('--md', '--base'))]
    rows = []
    for s in srcs:
        rows += load(s)
    S = summarize(rows)
    seen = []
    for r in rows:
        if r[0] not in seen:
            seen.append(r[0])
    keys = seen
    agents = sorted({a for _, a in S}, key=lambda a: ['strategist', 'greedy', 'oracle'].index(a) if a in ['strategist', 'greedy', 'oracle'] else 9)
    base = next((k for k in keys if '/'.join(k) == base_name), keys[0])
    name = lambda k: '/'.join(k)
    L = []
    p = L.append
    levels = sorted({r[2] for r in rows})
    p(f'## Wins (paired against {name(base)}, same pair, level and seed)\n')
    p('| agent | arm | ' + ' | '.join(f'd{d}' for d in levels) + ' | all | gap ± SE (all) | n |')
    p('|---|---|' + '---|' * (len(levels) + 3))
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            cells = []
            for d in levels:
                ws = [v for kk, v in s['wins'].items() if kk[1] == d]
                cells.append(f'{100 * sum(ws) / len(ws):.1f}' if ws else '-')
            ws = list(s['wins'].values())
            pr = paired(S[(base, a)], s) if (base, a) in S and k != base else None
            gap = f'{pr[2]:+.1f} ± {pr[3]:.1f}' if pr else ''
            p(f'| {a} | {name(k)} | ' + ' | '.join(cells) + f' | {100 * sum(ws) / len(ws):.1f} | {gap} | {len(ws)} |')
    p('\n### Paired gaps by level\n')
    p('| agent | arm | ' + ' | '.join(f'd{d}' for d in levels) + ' |')
    p('|---|---|' + '---|' * len(levels))
    for a in agents:
        for k in keys:
            if k == base or (k, a) not in S or (base, a) not in S:
                continue
            cells = []
            for d in levels:
                pr = paired(S[(base, a)], S[(k, a)], d)
                cells.append(f'{pr[2]:+.1f} ± {pr[3]:.1f}' if pr else '-')
            p(f'| {a} | {name(k)} | ' + ' | '.join(cells) + ' |')
    p('\n## Losses by reason a game; d3 losses (node / deck / other) of d3 games\n')
    p('| agent | arm | rounds | lost: node | deck_empty | other | d3: node | d3: deck | d3: other | d3 games |')
    p('|---|---|---|---|---|---|---|---|---|---|')
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            n = s['games']
            other = sum(v for r, v in s['reason'].items() if r not in ('node', 'deck_empty'))
            o3 = sum(v for r, v in s['d3loss'].items() if r not in ('node', 'deck_empty'))
            p(f'| {a} | {name(k)} | {s["rounds"] / n:.2f} | {s["reason"]["node"] / n:.3f} | {s["reason"]["deck_empty"] / n:.3f} | {other / n:.3f} | '
              f'{s["d3loss"]["node"]} | {s["d3loss"]["deck_empty"]} | {o3} | {s["d3games"]} |')
    p('\n## Nodes placed a seat-game by round, per board\n')
    p('| agent | arm | board | ' + ' | '.join(f'r{r}' for r in range(1, 11)) + ' |')
    p('|---|---|---|' + '---|' * 10)
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            for b in ('builder', 'protector', 'researcher'):
                sg = s['sg_board'][b]
                if not sg:
                    continue
                p(f'| {a} | {name(k)} | {b} | ' + ' | '.join(f'{s["nodes"][(b, r)] / sg:.2f}' for r in range(1, 11)) + ' |')
    p('\n## Nodes by source, a seat-game (share)\n')
    p('| agent | arm | all | base route | board routes | Place a node (r112) | cards and other |')
    p('|---|---|---|---|---|---|---|')
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            sg, al = s['seatgames'], max(s['src']['all'], 1)
            f = lambda x: f'{s["src"][x] / sg:.2f} ({100 * s["src"][x] / al:.0f}%)'
            p(f'| {a} | {name(k)} | {s["src"]["all"] / sg:.2f} | {f("base route")} | {f("board routes")} | {f("Place a node")} | {f("cards and other")} |')
    p('\n## Board powers a seat-game (that board\'s seat-games)\n')
    p('| agent | arm | board | power | uses |')
    p('|---|---|---|---|---|')
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            for (b, nm), v in sorted(s['power'].items()):
                p(f'| {a} | {name(k)} | {b} | {nm} | {v / max(s["sg_board"][b], 1):.2f} |')
    p('\n## Standard actions a seat-game\n')
    acts = sorted({act for s in S.values() for act in s['std']})
    p('| agent | arm | ' + ' | '.join(acts) + ' | beyond Dispatch and Liberate |')
    p('|---|---|' + '---|' * (len(acts) + 1))
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            sg = s['seatgames']
            beyond = sum(v for act, v in s['std'].items() if act not in ('move', 'liberate')) / sg
            p(f'| {a} | {name(k)} | ' + ' | '.join(f'{s["std"][act] / sg:.2f}' for act in acts) + f' | **{beyond:.2f}** |')
    p('\n## Liberations, claims, retakes and neutral sites, a game\n')
    ctrs = sorted({c for s in S.values() for c in s['ctr'] if not c.startswith('board.std.') and c != 'board.node_placed'})
    p('| agent | arm | ' + ' | '.join(c.replace('board.', '').replace('adversary.', 'adv.') for c in ctrs) + ' |')
    p('|---|---|' + '---|' * len(ctrs))
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            p(f'| {a} | {name(k)} | ' + ' | '.join(f'{s["ctr"][c] / s["games"]:.2f}' for c in ctrs) + ' |')
    p('\n## The new cards played a game; wins through cards\n')
    p('| agent | arm | ' + ' | '.join(NEW_CARDS) + ' | won games with more card plays and uses than standard actions | card share of actions in won games |')
    p('|---|---|' + '---|' * (len(NEW_CARDS) + 2))
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            won = max(s['won'], 1)
            p(f'| {a} | {name(k)} | ' + ' | '.join(f'{s["plays"][c] / s["games"]:.3f}' for c in NEW_CARDS)
              + f' | {100 * s["cardwin"] / won:.0f}% | {100 * s["cardshare"] / won:.0f}% |')
    p('\n## Resource use: spent a seat-game (spent / gained)\n')
    p('| agent | arm | ' + ' | '.join(r for r in RES if r != 'card_tokens') + ' |')
    p('|---|---|' + '---|' * 7)
    for a in agents:
        for k in keys:
            if (k, a) not in S:
                continue
            s = S[(k, a)]
            sg = s['seatgames']
            p(f'| {a} | {name(k)} | ' + ' | '.join(
                f'{s["spent"][i] / sg:.1f} ({100 * s["spent"][i] / max(s["gained"][i], 1):.0f}%)' for i, r in enumerate(RES) if r != 'card_tokens') + ' |')
    text = '\n'.join(L) + '\n'
    sys.stdout.reconfigure(encoding='utf-8')
    print(text)
    if md:
        with open(md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main()
