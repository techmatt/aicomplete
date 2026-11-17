"""256's Use tables, per arm and agent, from read_lib's games (a merged read: `merge.py`).

    python experiments/results/256_item_c_cards/use256.py READ [--md OUT]

Tables: each touched card's keeps and plays a game (and its Action uses); claims a game by route (the Claim action,
a card, the liberate cards' rider) and kind, and the owned stations and factories made (claims, stations built
free, factories built); nodes by source a seat-game (Expand, the board routes, cards and other: what's left of
`board.node_placed`, 253's method); each resource's spend share (binds at 80%, 254's reading).
"""
import argparse
import json
import os
import sys
from collections import defaultdict

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts', 'bench'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from read_lib import games, resources, seat_boards  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
CARDS = ['open_research_campus', 'industrial_commons', 'power_purchase_agreement', 'microgrid_lab_compact',
         'nuclear_option', 'power_to_the_people', 'forensic_audit', 'root_access', 'peacekeeping_mission',
         'popular_uprising', 'citywide_microgrid', 'hostile_buyout', 'relay_mast_rollout', 'mesh_repeater_network',
         'research_reactor', 'survey_drone_fleet', 'orbital_relay_launcher',
         'positron_storage_ring', 'wafer_fab_line', 'antiproton_foundry']
BOARD_ROUTES = {'Buy In', 'Annex', 'Accord', 'Outpost', 'Peer Review', 'Field Survey'}
RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'ambassadors']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('read')
    ap.add_argument('--md')
    a = ap.parse_args()
    powers = {b['id']: [p['name'] for p in b['powers']] for b in json.load(open(os.path.join(ROOT, 'content', 'ais.json'), encoding='utf-8'))}
    S = defaultdict(lambda: {'games': 0, 'sg': 0, 'kept': defaultdict(int), 'played': defaultdict(int),
                             'uses': defaultdict(int), 'ctr': defaultdict(float), 'src': defaultdict(float),
                             'spent': defaultdict(float), 'gained': defaultdict(float)})
    for G in games(a.read):
        g = G.g
        s = S[(G.agent, G.arm)]
        s['games'] += 1
        boards = seat_boards(g)
        s['sg'] += len(boards)
        for cid in CARDS:
            c = g['cards'].get(cid) or {}
            s['kept'][cid] += c.get('kept', 0)
            s['played'][cid] += c.get('played', 0)
        cs = g['counters']
        for k, v in cs.items():
            if k.startswith(('board.claimed_by.', 'board.std.claim', 'board.stations_built_free',
                             'board.factories_built.player', 'board.sites_opened.')) and '.r' not in k[-4:]:
                s['ctr'][k] += v
        s['src']['all'] += cs.get('board.node_placed', 0)
        for act in g['actions']:
            t = act.get('type')
            if t == 'power':
                name = powers[boards[act['seat']]][act['power']]
                if name == 'Expand':
                    s['src']['Expand'] += 1
                elif name in BOARD_ROUTES:
                    s['src']['board routes'] += 1
            elif t == 'use' and act.get('card') in CARDS:
                s['uses'][act['card']] += 1
        for i in range(len(boards)):
            for r, (sp, ga) in resources(g, i).items():
                s['spent'][r] += sp
                s['gained'][r] += ga
    out = []
    p = out.append
    keys = sorted(S)
    p('## Touched cards: kept / played a game (Action uses a game)\n')
    p('| card | ' + ' | '.join(f'{ag} {arm}' for ag, arm in keys) + ' |')
    p('|---|' + '---|' * len(keys))
    for cid in CARDS:
        cells = []
        for k in keys:
            s = S[k]
            n = max(s['games'], 1)
            u = f' ({s["uses"][cid] / n:.2f})' if s['uses'][cid] else ''
            cells.append(f'{s["kept"][cid] / n:.2f} / {s["played"][cid] / n:.2f}{u}')
        p(f'| {cid} | ' + ' | '.join(cells) + ' |')
    p('\n## Claims and owned sites a game\n')
    ck = sorted({c for s in S.values() for c in s['ctr']})
    p('| counter | ' + ' | '.join(f'{ag} {arm}' for ag, arm in keys) + ' |')
    p('|---|' + '---|' * len(keys))
    for c in ck:
        p(f'| {c} | ' + ' | '.join(f'{S[k]["ctr"][c] / max(S[k]["games"], 1):.2f}' for k in keys) + ' |')
    owned = []
    for k in keys:
        s = S[k]
        n = sum(v for c, v in s['ctr'].items() if c.startswith('board.claimed_by.')) + s['ctr']['board.stations_built_free'] \
            + s['ctr']['board.factories_built.player']
        owned.append(f'{n / max(s["games"], 1):.2f}')
    p('| **owned stations and factories made** | ' + ' | '.join(owned) + ' |')
    p('\n## Nodes by source, a seat-game (share)\n')
    p('| agent | arm | all | Expand | board routes | cards and other |')
    p('|---|---|---|---|---|---|')
    for k in keys:
        s = S[k]
        sg, al = max(s['sg'], 1), max(s['src']['all'], 1)
        rest = s['src']['all'] - s['src']['Expand'] - s['src']['board routes']
        f = lambda x: f'{x / sg:.2f} ({100 * x / al:.0f}%)'
        p(f'| {k[0]} | {k[1]} | {s["src"]["all"] / sg:.2f} | {f(s["src"]["Expand"])} | {f(s["src"]["board routes"])} | {f(rest)} |')
    p('\n## Resources: spent a seat-game (spent / gained)\n')
    p('| agent | arm | ' + ' | '.join(RES) + ' |')
    p('|---|---|' + '---|' * len(RES))
    for k in keys:
        s = S[k]
        sg = max(s['sg'], 1)
        p(f'| {k[0]} | {k[1]} | ' + ' | '.join(
            f'{s["spent"][r] / sg:.1f} ({100 * s["spent"][r] / max(s["gained"][r], 1e-9):.0f}%)' for r in RES) + ' |')
    text = '\n'.join(out) + '\n'
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main()
