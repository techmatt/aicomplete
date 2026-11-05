"""122's cap screen: the holding limits on and off, crossed with the three pairs, on the dev seeds.

    python experiments/results/122_runaway_engines/gen_caps.py [--out experiments/archive/screen-122-caps.json]
        [--first-seed 34708] [--name screen-122-caps]

Levers (r48's baseline holds 9999 credits, 999 of the rest: no limit):
- `base`: no limits (r48's play).
- `caps`: credits 150, energy, chips and ambassadors 10 each (the candidate).
- `caps_nocr`: the candidate without the credit limit (the factorial's other half).
- `caps_cr120`: the candidate with credits 120.
- `caps_pat10`: the candidate and Patience carrying at most 10 (Matt's diagnostic, message 5117: never ships).

It ran on the screen build (out/caps), which had `economy.patience_carry`; r49 dropped that parameter (Patience stays
uncapped), so the final binary refuses this spec: its record is caps-dev.md.
"""
import argparse
import json
import os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]
OFF = ['economy.hold_credits=9999', 'economy.hold_energy=999', 'economy.hold_chips=999',
       'economy.hold_ambassadors=999', 'economy.patience_carry=999']
CAPS = ['economy.hold_credits=150', 'economy.hold_energy=10', 'economy.hold_chips=10',
        'economy.hold_ambassadors=10', 'economy.patience_carry=999']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=os.path.join(ROOT, 'experiments', 'archive', 'screen-122-caps.json'))
    ap.add_argument('--first-seed', type=int, default=34708)
    ap.add_argument('--name', default='screen-122-caps')
    a = ap.parse_args()
    levers = {
        'base': OFF,
        'caps': CAPS,
        'caps_nocr': ['economy.hold_credits=9999'] + CAPS[1:],
        'caps_cr120': ['economy.hold_credits=120'] + CAPS[1:],
        'caps_pat10': CAPS[:-1] + ['economy.patience_carry=10'],
    }
    variants = [{'name': f'{p}-{k}', 'set': ps + v} for k, v in levers.items() for p, ps in PAIRS]
    spec = {
        'name': a.name,
        'note': '122: the holding limits on and off (gen_caps.py), the three pairs at d2, the strategist and greedy, '
                f'48 a cell from seed {a.first_seed}.',
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': [2],
        'agents': ['strategist', 'greedy'],
        'games': 48,
        'first_seed': a.first_seed,
        'records': 0,
    }
    with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')


if __name__ == '__main__':
    main()
