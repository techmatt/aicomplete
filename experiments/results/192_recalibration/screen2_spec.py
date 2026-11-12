"""192: the package screen's spec (dev seeds 36800-36831, as screen 1, whose `base` cells pair with these).

    python experiments/results/192_recalibration/screen2_spec.py > experiments/farm/f192-screen2.json

Packages: `p16` Vigil's cap 2, face-down land corruption not dripping, compute income 0/4/16/24/24, agent income
0/2/2/2/3; `p18` the same with d2's compute at 18; `p16drip` p16 with the drip kept (is the drip lever worth a rule?).
"""
import json
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
VIGIL = ['board.vigil_cap=2']
DRIP = ['adversary.blight_node_discord=0']
AMB = ['adversary.ambassador_income=0,2,2,2,3']


def comp(d2):
    return [f'adversary.compute_income=0,4,{d2},24,24']


CELLS = [('p16', VIGIL + DRIP + AMB + comp(16)), ('p18', VIGIL + DRIP + AMB + comp(18)),
         ('p16drip', VIGIL + AMB + comp(16))]
if len(sys.argv) > 1 and sys.argv[1] == '3':
    # screen 3 (192): d0's compute back at 1 (0 cost d0 about 7), d2 at 17, d3's agents back at 3; the drip kept or
    # not; Vigil's cap 3; d0's compute 2.
    AMB3 = ['adversary.ambassador_income=0,2,2,3,3']

    def c3(d0):
        return [f'adversary.compute_income={d0},4,17,24,24']
    CELLS = [('q', VIGIL + DRIP + AMB3 + c3(1)), ('qdrip', VIGIL + AMB3 + c3(1)),
             ('qv3', ['board.vigil_cap=3'] + DRIP + AMB3 + c3(1)), ('qc2', VIGIL + DRIP + AMB3 + c3(2))]
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
variants = [{'name': f'{p}_{c}', 'set': ps + cs} for c, cs in CELLS for p, ps in PAIRS]
print(json.dumps({'name': 'f192-screen2', 'note': '192: the package screen (dev seeds): Vigil 2, the drip, the dials.',
                  'configs': ['configs/baseline.json'], 'variants': variants, 'difficulties': [0, 1, 2, 3, 4],
                  'agents': ['strategist'], 'games': 32, 'records': 0}, indent=2))
