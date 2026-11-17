"""257's Use tables, per agent spec of a read (the arms are agents: `strategist:w.sink=0` is the shipped judge).

    python experiments/results/257_judge_action_buildings/use257.py FILE=SPEC.json [--all] [--md OUT]

Tables, a game unless named: each Action building's keeps, builds (plays), Action uses, the round it was built and
the rounds it stood (from its play to the game's end, both counted), and uses per round stood; each resource's
spent and spent / gained a seat-game (binds at 80%, 254's reading). `--all`: every building with an Action, not
just the seven this item names. A round starts at the first `tier` action after any non-draft action.
"""
import argparse
import json
import os
import sys
from collections import defaultdict

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import PEAKS, cells_of, stream  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
NAMED = ['mesh_repeater_network', 'research_reactor', 'survey_drone_fleet', 'orbital_relay_launcher',
         'positron_storage_ring', 'wafer_fab_line', 'antiproton_foundry']
RES = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'ambassadors']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('read')
    ap.add_argument('--all', action='store_true')
    ap.add_argument('--md')
    a = ap.parse_args()
    defs = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}
    cards = NAMED
    if a.all:
        cards = sorted(i for i, c in defs.items() if c.get('kind') == 'building' and c.get('ability')
                       and not c['ability'].get('once'))
    lines = stream(a.read)
    cells = cells_of(next(lines)['spec'])
    S = defaultdict(lambda: {'games': 0, 'sg': 0, 'c': defaultdict(lambda: defaultdict(float)),
                             'spent': defaultdict(float), 'gained': defaultdict(float)})
    for g in lines:
        if 'cell' not in g or 'won' not in g:
            continue
        s = S[cells[g['cell']]['agent']]
        s['games'] += 1
        seats = len(g['spent'])
        s['sg'] += seats
        rnd, prev_draft = 0, False
        built, uses = {}, defaultdict(int)
        for act in g['actions']:
            t = act['type']
            if t == 'tier' and not prev_draft:
                rnd += 1
            prev_draft = t in ('tier', 'draft', 'keep') or (t == 'pass' and prev_draft)
            if t == 'play' and act.get('card') in cards:
                built[(act['seat'], act['card'])] = rnd
            elif t == 'use' and act.get('card') in cards:
                uses[act['card']] += 1
        for (_, cid), r in built.items():
            s['c'][cid]['built_at'] += r
            s['c'][cid]['stood'] += g['rounds'] - r + 1
            s['c'][cid]['instances'] += 1
        for cid in cards:
            c = g['cards'].get(cid) or {}
            s['c'][cid]['kept'] += c.get('kept', 0)
            s['c'][cid]['played'] += c.get('played', 0)
            s['c'][cid]['uses'] += uses[cid]
        for i in range(seats):
            for j, r in enumerate(PEAKS):
                s['spent'][r] += g['spent'][i][j]
                s['gained'][r] += g['gained'][i][j]
    out = []
    p = out.append
    for agent in sorted(S):
        s = S[agent]
        n = max(s['games'], 1)
        p(f'### {agent} ({s["games"]} games)\n')
        p('| building | kept | built | uses | built in round | rounds stood | uses a round stood |')
        p('|---|---|---|---|---|---|---|')
        for cid in cards:
            c = s['c'][cid]
            k = max(c['instances'], 1)
            p(f'| {cid} | {c["kept"] / n:.2f} | {c["played"] / n:.2f} | {c["uses"] / n:.2f} | {c["built_at"] / k:.1f} | '
              f'{c["stood"] / k:.1f} | {c["uses"] / max(c["stood"], 1):.2f} |')
        p('')
    p('### Resources: spent a seat-game (spent / gained)\n')
    p('| agent | ' + ' | '.join(RES) + ' |')
    p('|---|' + '---|' * len(RES))
    for agent in sorted(S):
        s = S[agent]
        sg = max(s['sg'], 1)
        p(f'| {agent} | ' + ' | '.join(
            f'{s["spent"][r] / sg:.1f} ({100 * s["spent"][r] / max(s["gained"][r], 1e-9):.0f}%)' for r in RES) + ' |')
    text = '\n'.join(out) + '\n'
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)


if __name__ == '__main__':
    main()
