"""192: the recalibration screen's spec (dev seeds 36800-36831, strategist, d0-d4, three pairs).

    python experiments/results/192_recalibration/screen_spec.py > experiments/farm/f192-screen.json

Cells: `base` (the prompt's stated numbers: r83 as 191 left it), each asked rule lever alone (Vigil's cap 2, the
node hexes' face-down land corruption not dripping, the Protector's Muster skipping players' factories), the three
together, and the three with the difficulty dials (agent income, compute income, both).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
VIGIL = ['board.vigil_cap=2']
DRIP = ['adversary.blight_node_discord=0']
LAND = ['adversary.wall_landing=1']
ALL3 = VIGIL + DRIP + LAND
AMB = ['adversary.ambassador_income=0,2,2,2,3']
COMP = ['adversary.compute_income=1,2,14,16,24']
CELLS = [('base', []), ('vigil2', VIGIL), ('nodrip', DRIP), ('landing', LAND), ('all3', ALL3),
         ('all3amb', ALL3 + AMB), ('all3comp', ALL3 + COMP), ('all3both', ALL3 + AMB + COMP)]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = [{'name': p if c == 'base' else f'{p}_{c}', 'set': ps + cs} for c, cs in CELLS for p, ps in PAIRS]
print(json.dumps({'name': 'f192-screen', 'note': '192: the recalibration screen (dev seeds): each asked rule lever, '
                  'the three together, and with the difficulty dials; strategist d0-d4.',
                  'configs': ['configs/baseline.json'], 'variants': variants, 'difficulties': [0, 1, 2, 3, 4],
                  'agents': ['strategist'], 'games': 32, 'records': 0}, indent=2))
