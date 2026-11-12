"""196: screen specs (dev seeds 36800-36831, strategist, three pairs, d0-d3).

    python experiments/results/196_balance_pass/screen_spec.py 1 > out/s1.json         # screen 1, plain agent
    python experiments/results/196_balance_pass/screen_spec.py 1w > out/s1w.json       # screen 1, w.chips=4

Screen 1 cells (each against base on the same seeds):
- `a`: the card-play track starts at 4 (`ais.*.insight=4`; its later spaces print totals 7, 9, ... and stay);
- `b`: 191's tier-1 play costs back: each tier-1 card whose insight 191 moved goes back to its de6b0aa row's
  `cost` and `insight` (the hand-priced node and D82 cards aside, as 194 listed);
- `ab`: both;
- `claim2`: `board.claim_factory=2`; `w.chips=4` (the eval's held-chip weight) runs as a second spec (one agent a spec);
- `d0x`: one more stage-I card at d0 (`adversary.stage1_extra=1,0,0,0,0`; only d0's games move);
- `drip`: face-down land corruption drips again (`adversary.blight_node_discord=1`): 194's d0 lead (Clears).
"""
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
HAND = {'national_grid_buildout', 'beamed_power_constellation', 'relay_mast_rollout', 'aegis_grid_lockdown',
        'rally_the_allies', 'mass_defection', 'surgical_strike', 'double_agents', 'air_gap'}
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def tier1_back():
    """Overlay lines putting 191's tier-1 play costs (and their credits) back to de6b0aa's rows."""
    raw = subprocess.run(['git', 'show', 'de6b0aa:content/cards.json'], cwd=ROOT, capture_output=True, check=True)
    old = {c['id']: c for c in json.loads(raw.stdout.decode('utf-8'))}
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        new = {c['id']: c for c in json.load(f)}
    sets = []
    for i, c in new.items():
        o = old.get(i)
        if o is None or c['tier'] != 1 or i in HAND or c['insight'] == o['insight']:
            continue
        sets += [f'cards.{i}.cost={o["cost"]}', f'cards.{i}.insight={o["insight"]}']
    return sets


A = ['ais.*.insight=4']
B = tier1_back()


def spec(name, cells, agent='strategist', games=32, diffs=(0, 1, 2, 3)):
    variants = []
    for c, cs in cells:
        for p, ps in PAIRS:
            variants.append({'name': p if c == 'base' else f'{p}_{c}', 'set': ps + cs})
    return {'name': name, 'note': '196 screen (dev seeds 36800-36831)', 'configs': ['configs/baseline.json'],
            'variants': variants, 'difficulties': list(diffs), 'agents': [agent], 'games': games, 'records': 0}


if __name__ == '__main__':
    which = sys.argv[1] if len(sys.argv) > 1 else '1'
    print(f'# tier-1 cards back: {len(B) // 2}', file=sys.stderr)
    if which == '1':
        cells = [('base', []), ('a', A), ('b', B), ('ab', A + B), ('claim2', ['board.claim_factory=2']),
                 ('d0x', ['adversary.stage1_extra=1,0,0,0,0']), ('drip', ['adversary.blight_node_discord=1'])]
        print(json.dumps(spec('s196-1', cells), indent=1))
    elif which == '1w':
        # the eval's claim decision: held chips 2 -> 4; an owned factory 4 -> 12 (about its chip a round at the
        # flat income weight, 3 x chips 2, on top of 4 for the site, against 3 ambassadors at 5 each); both
        cells = [('base', []), ('claim2', ['board.claim_factory=2'])]
        s = spec('s196-1w', cells)
        s['agents'] = ['strategist:w.chips=4', 'strategist:w.owned_factory=12', 'strategist:w.chips=4,w.owned_factory=12']
        print(json.dumps(s, indent=1))
    elif which == '2':
        # screen 2 (the build with w.chips 4 and the insight-alone measure): insight a / b / ab, each with and
        # without Focus at the unit anchors (2 credits for 1 insight: give 2, get 2.5)
        F = ['ais.builder.powers.6.cost=[["credits",2]]']
        cells = [('base', []), ('a', A), ('b', B), ('ab', A + B), ('f', F), ('af', A + F), ('abf', A + B + F)]
        print(json.dumps(spec('s196-2', cells), indent=1))
    elif which == '3':
        # screen 3 (the build with w.chips 4, Focus cut, the `stage_extra` dial): the package and its d0 / d1 dials
        X1 = ['adversary.stage_extra=1,0,0,0,0']
        cells = [('base', []), ('P', A + X1), ('P2', A + ['adversary.stage_extra=2,0,0,0,0']),
                 ('P3', ['ais.*.insight=3'] + X1), ('Pc', A + X1 + ['adversary.compute_income=1,2,19,24,24'])]
        print(json.dumps(spec('s196-3', cells), indent=1))
    elif which == '4':
        # screen 4: the track at 3 with the dials; each level reads only its own entry, so a cell tries one option a
        # level: (event cards added, compute, agents) a level
        def dial(extra, comp, amb):
            return ['ais.*.insight=3', f'adversary.stage_extra={extra}', f'adversary.compute_income={comp}',
                    f'adversary.ambassador_income={amb}']
        cells = [('base', []),
                 ('Q0', dial('2,0,0,0,0', '1,4,19,24,24', '0,2,2,3,3')),
                 ('Qa', dial('2,0,0,0,0', '1,2,19,20,24', '0,2,2,3,3')),
                 ('Qb', dial('3,0,0,0,0', '1,4,19,22,24', '0,1,2,3,3')),
                 ('Qc', dial('3,0,0,0,0', '1,0,17,18,24', '0,2,2,2,3')),
                 ('Qd', dial('2,0,0,0,0', '0,2,21,16,24', '0,1,2,3,3'))]
        print(json.dumps(spec('s196-4', cells), indent=1))
    elif which == 'ac':
        # 192's inert switch, re-read after 195's projects (Backdoor Research lands adversary cubes by an op):
        # does the eval's adversary-cube weight change any game? 16 seeds, the plain build
        s = spec('s196-ac', [('base', [])], games=16)
        s['agents'] = ['strategist', 'strategist:w.adversary_cube=0', 'strategist:w.adversary_cube=1']
        print(json.dumps(s, indent=1))
    elif which == 'b':
        print('\n'.join(B))
