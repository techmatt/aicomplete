"""199: screen specs (dev seeds 36800-36831, three pairs, d0-d3).

    python experiments/results/199_rules_insight_chips/screen_spec.py 1 > out/s1.json

Every cell runs r87's rulings (Focus back at 2 credits, a factory claim 2 ambassadors, the corps' chip converters,
D88); `base` is the prompt's stated numbers with r86's card-play track, the other cells cut insight income:
- `t1`: each board's card-play track grows at its own rate, every insight total lower (Builder fastest);
- `t2`: deeper and with fewer steps (one insight space a board prints none);
- `t1k`: `t1` with the keep upgrade gone (a tier-2 keep stays 2: `cards.keep_tier2_upgraded=2`).
The old side (r86, `out/r86`, the same stats-only counters patched in) plays `read.json`'s three pairs on the same
seeds.
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def track(board, totals):
    """Overlay lines setting a board's card-play track insight totals by space index (None clears a space's)."""
    return [f'ais.{board}.play_track.{i}.insight={"null" if v is None else v}' for i, v in totals.items()]


# r86: Builder / Protector spaces 0, 2, 4, 5, 6 print 7, 9, 11, 12, 13; the Researcher's 0, 1, 3, 4, 6 print 7, 9, 10, 12, 13.
T1 = (track('builder', {0: 5, 2: 7, 4: 8, 5: 9, 6: 10}) + track('protector', {0: 5, 2: 6, 4: 7, 5: 8, 6: 9})
      + track('researcher', {0: 4, 1: 5, 3: 6, 4: 7, 6: 8}))
T2 = (track('builder', {0: 4, 2: 6, 4: 7, 5: None, 6: 8}) + track('protector', {0: 4, 2: 5, 4: 6, 5: None, 6: 7})
      + track('researcher', {0: 4, 1: 5, 3: None, 4: 6, 6: 7}))
NOUP = ['cards.keep_tier2_upgraded=2']
# Matt's fallback: a tier-2 keep at 3 (2 with the keep upgrade).
K3 = ['cards.keep_tier2=3', 'cards.keep_tier2_upgraded=2']
CELLS = {
    '1': [('base', []), ('t1', T1), ('t2', T2), ('t1k', T1 + NOUP)],
    # screen 2: the fallback, alone and with t1; `2f` runs base with the eval's claimed factory at 10 (agent spec).
    '2': [('base', []), ('t1', T1), ('k3', K3), ('t1k3', T1 + K3)],
    '2f': [('base', [])],
    # screen 3: the factorial of t1 and the eval's claimed factory (10, and 16 for its slope).
    '3f': [('base', []), ('t1', T1)],
    '3g': [('base', []), ('t1', T1)],
}
# d3 calibration (the fresh read put d3 at 53.5, over its 40-50 band): on the read's spent seeds 44479-44622, d3 alone.
D3 = {'d3': [('base', [])] + [(f'c{n}', [f'adversary.compute_income=1,4,17,{n},24']) for n in (20, 22, 24)]
      + [('a3', ['adversary.ambassador_income=0,2,2,3,3'])]}
CELLS.update(D3)
# screen 4 (on r87's content and eval, so `base` is r87): the keep plan's horizon (`plan_rounds` 4 -> 2, 3), paired with 3f's `t1`. (An eval-side spending plan, `plan_over`, changed no game: the strategist keeps by `keep_plan.rs`, not the eval; removed.)
CELLS.update({'4a': [('base', [])], '4b': [('base', [])]})
# screen 5 (on r87's content, t1, and eval): early-heavy, late-flat tracks, so tier 2 can be kept early. The fresh read
# put r87's tier-2 keeps later than r86's (rounds 2-5).
T3 = (track('builder', {0: 6, 2: 7, 4: 8, 5: 9, 6: 10}) + track('protector', {0: 6, 2: 7, 4: 8, 5: None, 6: 9})
      + track('researcher', {0: 5, 1: 6, 3: None, 4: 7, 6: 8}))
T4 = (track('builder', {0: 7, 2: 8, 4: None, 5: 9, 6: 10}) + track('protector', {0: 7, 2: 8, 4: None, 5: None, 6: 9})
      + track('researcher', {0: 6, 1: None, 3: 7, 4: None, 6: 8}))
CELLS['5'] = [('base', []), ('t3', T3), ('t4', T4)]
# screen 6 (r87 final): the keep target's next lever, a bigger draft hand (not adopted: a rules change past the prompt).
CELLS['6'] = [('base', []), ('deal5', ['cards.deal=5']), ('t4deal5', T4 + ['cards.deal=5'])]
AGENTS = {'2f': 'strategist:w.owned_factory=10', '3f': 'strategist:w.owned_factory=10',
          '3g': 'strategist:w.owned_factory=16', '4a': 'strategist:plan_rounds=2', '4b': 'strategist:plan_rounds=3'}


def spec(name, cells, agent='strategist', games=32, diffs=(0, 1, 2, 3)):
    variants = []
    for c, cs in cells:
        for p, ps in PAIRS:
            variants.append({'name': p if c == 'base' else f'{p}_{c}', 'set': ps + cs})
    return {'name': name, 'note': '199 screen (dev seeds 36800-36831)', 'configs': ['configs/baseline.json'],
            'variants': variants, 'difficulties': list(diffs), 'agents': [agent], 'games': games, 'records': 0}


if __name__ == '__main__':
    which = sys.argv[1] if len(sys.argv) > 1 else '1'
    agent = sys.argv[2] if len(sys.argv) > 2 else AGENTS.get(which, 'strategist')
    diffs = (3,) if which == 'd3' else (0, 1, 2, 3)
    s = spec(f's199-{which}', CELLS[which], agent, diffs=diffs)
    if which == 'd3':
        s['note'] = '199 d3 calibration (the fresh read spent seeds 44479-44622, d3 only)'
    print(json.dumps(s, indent=2))
