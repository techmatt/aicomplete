"""214: the item's specs, cells as `<pair>_<lever>` variants (the strategist unless named).

    python experiments/results/214_tier2_pricing_insight/spec.py NAME > experiments/farm/NAME.json

Levers (199's track shapes on the r94 card-play track, which is 199's t1 space for space):
- `k3`: a tier-2 keep 3 insight (2 once the keep upgrade is uncovered);
- `t4`: 199's t4, early-heavy and late-flat (start 7: Builder 7, 8, 9, 10; Protector 7, 8, 9; Researcher 6, 7, 8);
- `t4k3`: both.
NAME: s214-smoke (16 dev seeds from 36800, d2, all four cells), s214-a (screen 1: spent fresh seeds 46240-46431, the
factorial, d0-d3, 192 seeds a pair: 576 games a cell and level), f214 (the fresh read) and f214g (its greedy line).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def track(board, totals):
    return [f'ais.{board}.play_track.{i}.insight={"null" if v is None else v}' for i, v in totals.items()]


T4 = (track('builder', {0: 7, 2: 8, 4: None, 5: 9, 6: 10}) + track('protector', {0: 7, 2: 8, 4: None, 5: None, 6: 9})
      + track('researcher', {0: 6, 1: None, 3: 7, 4: None, 6: 8}))
K3 = ['cards.keep_tier2=3', 'cards.keep_tier2_upgraded=2']
HERE = __file__.replace('\\', '/').rsplit('/', 1)[0]
PRICES = [ln.strip() for ln in open(f'{HERE}/overlay.txt', encoding='utf-8') if ln.strip() and not ln.startswith('#')]
LEVERS = {'base': [], 'k3': K3, 't4': T4, 't4k3': T4 + K3, 't4k3p': T4 + K3 + PRICES, 'p': PRICES}


def spec(name, levers, diffs, games, note, agent='strategist'):
    variants = []
    for lever in levers:
        for p, s in PAIRS:
            variants.append({'name': f'{p}_{lever}' if lever != 'base' else p, 'set': s + LEVERS[lever]})
    return {'name': name, 'note': note, 'configs': ['configs/baseline.json'], 'variants': variants,
            'difficulties': diffs, 'agents': [agent], 'games': games, 'records': 0}


name = sys.argv[1]
FOUR = ['base', 'k3', 't4', 't4k3']
if name == 's214-smoke':
    out = spec(name, FOUR, [2], 16, '214: engine smoke, dev seeds 36800')
elif name == 's214-a':
    out = spec(name, FOUR, [0, 1, 2, 3], 192, '214 screen 1: keep 3 x track t4, spent fresh seeds 46240-46431')
elif name == 's214-b':
    # screen 2: the anchor prices (overlay.txt) on t4k3, paired with s214-a's cells on the same seeds
    out = spec(name, ['t4k3p'], [0, 1, 2, 3], 192, '214 screen 2: t4k3 with the anchor prices, spent fresh 46240-46431')
elif name == 's214-c':
    # after the fresh read (greedy led the strategist by 15 at d2): on the final build and the spent seeds of screens
    # 1-2, r95 base and r94 put back by overlay (`r94`), strategist and greedy
    R94 = (['cards.keep_tier2=2', 'cards.keep_tier2_upgraded=1']
           + track('builder', {0: 5, 2: 7, 4: 8, 5: 9, 6: 10}) + track('protector', {0: 5, 2: 6, 4: 7, 5: 8, 6: 9})
           + track('researcher', {0: 4, 1: 5, 3: 6, 4: 7, 6: 8})
           + [f'cards.{c}.cost={v}' for c, v in (('mass_defection', 9), ('embassy_of_exiles', 14),
              ('penning_trap_array', 10), ('antiproton_foundry', 10), ('positron_storage_ring', 9),
              ('forensic_audit', 27), ('kettle_operation', 18), ('surgical_strike', 14), ('decapitation_strike', 4),
              ('singularity_exploit', 8), ('kinetic_interdiction', 8), ('martial_cordon', 18), ('nuclear_option', 23),
              ('annihilation_lance', 2))] + ['cards.annihilation_lance.antimatter=2'])
    LEVERS['r94'] = R94
    agent = sys.argv[2] if len(sys.argv) > 2 else 'strategist'
    out = spec(name, ['base', 'r94'], [0, 1, 2, 3], 192, f'214 screen 3: r95 and r94 by overlay, {agent}', agent)
elif name.startswith('f214'):
    levers = sys.argv[2].split(',') if len(sys.argv) > 2 else ['base']
    agent = 'greedy' if name.endswith('g') else 'strategist'
    out = spec(name, levers, [0, 1, 2, 3, 4], 96, f'214: the fresh read ({agent})', agent)
else:
    sys.exit(f'unknown spec {name}')
print(json.dumps(out, indent=2))
