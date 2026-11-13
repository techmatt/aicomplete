"""212's reader: per lever (a spec variant's `<pair>_<lever>` suffix, pairs pooled) the win rate by level, paired
against base on the same seeds, each node power's and each Protector tag power's use, the riders' hit rate,
land lifts, and where node placements land by board.

    python experiments/results/212_tags_node_powers/read212.py DIR [DIR_OLD]

DIR: an experiment folder (or games.jsonl). DIR_OLD (optional): the old side's run of the same spec's base cells on
the same seeds (r92), read as lever `r92` and paired with base.
"""
import json
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
POWERS = [('builder', 'Buy In'), ('protector', 'Accord'), ('researcher', 'Peer Review'), ('protector', 'Stand Down'),
          ('protector', 'Recovery'), ('protector', 'Debrief'), ('protector', 'Interdict'), ('protector', 'Shelter')]
KINDS = ('factory', 'city', 'station', 'none', 'own_factory', 'own_city', 'own_station')


def load(path, boards_path):
    p = path if path.endswith('.jsonl') else os.path.join(path, 'games.jsonl')
    with open(boards_path, encoding='utf-8') as f:
        boards = {b['id']: [x['name'] for x in b['powers']] for b in json.load(f)}
    head, games = None, []
    with open(p, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                head = g
            else:
                games.append(g)
    spec = head['spec'] if isinstance(head['spec'], dict) else json.loads(head['spec'])
    names = [v['name'] for v in spec['variants']]
    per = len(spec['difficulties']) * len(spec['agents'])
    for g in games:
        v = names[(g['cell'] // per) % len(names)]
        g['pair'], _, lever = v.partition('_')
        g['lever'] = lever or 'base'
    return games, boards


def tally(games, boards):
    out = defaultdict(lambda: defaultdict(float))
    for g in games:
        a = out[g['lever']]
        sb = [p.split(':', 1)[0] for p in g['build'].split('|')]
        a['games'] += 1
        a['rounds'] += g['rounds']
        for s, b in enumerate(sb):
            a[f'seats:{b}'] += 1
            a[f'sr:{b}'] += g['rounds']
            for k in ('n',) + KINDS:
                a[f'at:{b}:{k}'] += g['counters'].get(f'board.node_at.{s}.{k}', 0)
            spread = g['margin'].get('node_spread', [])
            if s < len(spread):
                for i, k in enumerate(('nodes', 'countries', 'groups', 'far')):
                    a[f'sp:{b}:{k}'] += spread[s][i]
        for act in g['actions']:
            if act.get('type') == 'power':
                b = sb[act['seat']]
                a[f'pw:{b}:{boards[b][act["power"]]}'] += 1
            elif act.get('type') == 'standard' and act.get('act') == 'lift':
                a['lift'] += 1
        for k in ('effects.if_node_next_met', 'effects.ambassador_to_spot', 'effects.land_lifted'):
            a[k] += g['counters'].get(k, 0)
    return out


def wins(games):
    by = defaultdict(dict)
    for g in games:
        by[g['lever']][(g['pair'], g['difficulty'], g['seed'])] = 1.0 if g['won'] else 0.0
    return by


def paired(by, lever, d):
    base, other = by['base'], by[lever]
    xs = [other[k] - base[k] for k in other if k in base and k[1] == d]
    if not xs:
        return None
    m = sum(xs) / len(xs)
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / max(1, len(xs) - 1))
    return 100 * m, 100 * sd / math.sqrt(len(xs)), len(xs)


def main():
    new, boards = load(sys.argv[1], os.path.join(ROOT, 'content', 'ais.json'))
    games = new
    if len(sys.argv) > 2:
        old, oboards = load(sys.argv[2], os.path.join(ROOT, 'out', 'r92src', 'content', 'ais.json'))
        for g in old:
            g['lever'] = 'r92' if g['lever'] == 'base' else 'r92_' + g['lever']
        t_old = tally(old, oboards)
        games = new + old
    else:
        t_old = {}
    t = tally(new, boards)
    t.update(t_old)
    by = wins(games)
    levers = sorted(t, key=lambda x: (x != 'base', x))
    diffs = sorted({g['difficulty'] for g in games})
    print('## Win rate by level, paired against base (same pair, level and seed)\n')
    print('| lever | ' + ' | '.join(f'd{d}' for d in diffs) + ' | rounds |')
    print('|---|' + '---|' * (len(diffs) + 1))
    for lv in levers:
        cells = []
        for d in diffs:
            ks = [k for k in by[lv] if k[1] == d]
            w = 100 * sum(by[lv][k] for k in ks) / max(1, len(ks))
            p = paired(by, lv, d) if lv != 'base' else None
            cells.append(f'{w:.1f}' + (f' ({p[0]:+.1f} ± {p[1]:.1f})' if p else ''))
        print(f'| {lv} | ' + ' | '.join(cells) + f' | {t[lv]["rounds"] / t[lv]["games"]:.2f} |')
    print('\n## Power use (node powers a seat-round; the others a seat-game) and riders\n')
    print('| lever | ' + ' | '.join(f'{p}' for _, p in POWERS) + ' | riders met / node-power uses | ambassadors to a city a game | lifts a game (action, card) |')
    print('|---|' + '---|' * (len(POWERS) + 3))
    for lv in levers:
        a = t[lv]
        cells = []
        uses = 0
        for i, (b, p) in enumerate(POWERS):
            n = a[f'pw:{b}:{p}']
            if i < 3:
                uses += n
                cells.append(f'{n / a[f"sr:{b}"]:.2f}' if a[f'sr:{b}'] else '-')
            else:
                cells.append(f'{n / a[f"seats:{b}"]:.2f}' if a[f'seats:{b}'] else '-')
        met = a['effects.if_node_next_met']
        rider = f'{met / uses:.0%}' if uses and lv in ('base',) or (uses and not lv.startswith('r92') and 'flat' not in lv) else '-'
        print(f'| {lv} | ' + ' | '.join(cells) + f' | {rider} | {a["effects.ambassador_to_spot"] / a["games"]:.2f} | '
              f'{a["lift"] / a["games"]:.3f}, {a["effects.land_lifted"] / a["games"]:.3f} |')
    print('\n## Where placements land, by board (share of placements on or next to; spread at the end)\n')
    print('| lever | board | placements a seat-game | ' + ' | '.join(k.replace('_', ' ') for k in KINDS) + ' | nodes | countries | groups | farthest |')
    print('|---|---|' + '---|' * (len(KINDS) + 5))
    for lv in levers:
        a = t[lv]
        for b in ('builder', 'protector', 'researcher'):
            sg, n = a[f'seats:{b}'], a[f'at:{b}:n']
            if not sg:
                continue
            print(f'| {lv} | {b} | {n / sg:.2f} | ' + ' | '.join(f'{100 * a[f"at:{b}:{k}"] / max(1, n):.0f}%' for k in KINDS)
                  + ' | ' + ' | '.join(f'{a[f"sp:{b}:{k}"] / sg:.2f}' for k in ('nodes', 'countries', 'groups', 'far')) + ' |')


if __name__ == '__main__':
    main()
