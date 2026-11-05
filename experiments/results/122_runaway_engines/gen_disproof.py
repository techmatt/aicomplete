"""122's disproof spec: each named engine switched off by overlays, on the dev seeds, beside the base.

    python experiments/results/122_runaway_engines/gen_disproof.py [--out experiments/screen-122-disproof.json]

Variants (each crossed with the three pairs, as the standing read's `bp`, `br`, `pr`):
- `base`: r48 as printed.
- `vm_omd`: Vertical Monopoly's and Orbital Mass Driver's credit income off (121's named Researcher engine).
- `prog_cr`: every card's credit income and production (top-level ops in effects or passives) off.
- `embassy`: Orbital Embassy's ambassador trigger off.
- `prog_en`: every card's energy income off.
- `chip_inc`: every card's chip income off and `board.factory_chips` 0.
"""
import argparse
import json
import os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


INCOME = ('income', 'income_per_tag', 'production', 'production_per_tag')


def income_paths(cards, resource, only=None):
    """Overlays dropping each top-level income or production op on `resource` from a card's effects
    and passives (an overlay can't set an amount to 0: content refuses it)."""
    out = []
    for c in cards:
        if only and c['id'] not in only:
            continue
        for field in ('effects', 'passives'):
            ops = c.get(field) or []
            kept = [o for o in ops if not (o.get('op') in INCOME and o.get('resource') == resource)]
            if len(kept) != len(ops):
                out.append(f'cards.{c["id"]}.{field}={json.dumps(kept, separators=(",", ":"))}')
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=os.path.join(ROOT, 'experiments', 'screen-122-disproof.json'))
    a = ap.parse_args()
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cards = json.load(f)
    cards = cards if isinstance(cards, list) else cards['cards']
    levers = {
        'base': [],
        'vm_omd': income_paths(cards, 'credits', {'vertical_monopoly', 'orbital_mass_driver'}),
        'prog_cr': income_paths(cards, 'credits'),
        'embassy': ['cards.orbital_embassy.passives=[]'],
        'prog_en': income_paths(cards, 'energy'),
        'chip_inc': income_paths(cards, 'chips') + ['board.factory_chips=0'],
    }
    variants = []
    for lever, sets in levers.items():
        for pair, pset in PAIRS:
            variants.append({'name': f'{pair}-{lever}', 'set': pset + sets})
    spec = {
        'name': 'screen-122-disproof',
        'note': '122: each named engine off by overlays beside the base, the three pairs at d2, the strategist, '
                '48 a cell on dev seeds 34708-34755 (gen_disproof.py writes it).',
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': [2],
        'agents': ['strategist'],
        'games': 48,
        'first_seed': 34708,
        'records': 0,
    }
    with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')
    for k, v in levers.items():
        print(k, len(v), 'overlays')


if __name__ == '__main__':
    main()
