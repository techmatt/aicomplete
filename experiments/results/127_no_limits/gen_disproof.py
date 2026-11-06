"""127's disproof spec: each named engine switched off by overlays, on the dev seeds, beside the base (r51, no limits).

    python experiments/results/127_no_limits/gen_disproof.py [--out experiments/screen-127-disproof.json]

Levers (each crossed with the three pairs):
- `base`: r51 as printed.
- `pertag`: every card's credit `production_per_tag` off.
- `flat`: every card's flat credit `production` off.
- `drawprog`: the `draw` on every program and event off (buildings keep theirs).
- `drawall`: every card's `draw` off.
- `embassy`: Orbital Embassy's ambassador trigger off.
"""
import argparse
import json
import os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
PAIRS = [('bp', []), ('br', ['ai.boards=0,2']), ('pr', ['ai.boards=1,2'])]


def drop(cards, keep, kinds=None, fields=('effects', 'passives')):
    """Overlays rewriting each card's top-level op list without the ops `keep` refuses."""
    out = []
    for c in cards:
        if kinds and c.get('kind') not in kinds:
            continue
        for field in fields:
            ops = c.get(field) or []
            kept = [o for o in ops if keep(o)]
            if len(kept) != len(ops):
                out.append(f'cards.{c["id"]}.{field}={json.dumps(kept, separators=(",", ":"))}')
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=os.path.join(ROOT, 'experiments', 'screen-127-disproof.json'))
    ap.add_argument('--games', type=int, default=96)
    a = ap.parse_args()
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cards = json.load(f)
    cards = cards if isinstance(cards, list) else cards['cards']
    levers = {
        'base': [],
        'pertag': drop(cards, lambda o: not (o.get('op') == 'production_per_tag' and o.get('resource') == 'credits')),
        'flat': drop(cards, lambda o: not (o.get('op') == 'production' and o.get('resource') == 'credits')),
        'drawprog': drop(cards, lambda o: o.get('op') != 'draw', kinds=('program', 'event')),
        'drawall': drop(cards, lambda o: o.get('op') != 'draw'),
        'embassy': ['cards.orbital_embassy.passives=[]'],
    }
    variants = []
    for lever, sets in levers.items():
        for pair, pset in PAIRS:
            variants.append({'name': pair if lever == 'base' else f'{pair}-{lever}', 'set': pset + sets})
    spec = {
        'name': 'screen-127-disproof',
        'note': '127: each named engine off by overlays beside the base (r51, no limits), the three pairs at d2, the '
                'strategist, on dev seeds 35392-35487 (gen_disproof.py writes it).',
        'configs': ['configs/baseline.json'],
        'variants': variants,
        'difficulties': [2],
        'agents': ['strategist'],
        'games': a.games,
        'first_seed': 35392,
        'records': 0,
    }
    with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(spec, f, indent=2)
        f.write('\n')
    for k, v in levers.items():
        print(k, len(v), 'overlays')


if __name__ == '__main__':
    main()
