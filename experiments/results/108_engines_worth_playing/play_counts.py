"""108: offered / kept / played a game per seat pair (the three pairs pooled) for the cards 107 flagged and r43's new
ones, r42 against r43 on the read's seeds. usage: python play_counts.py R42_DIR R43_DIR"""
import csv
import io
import sys

IDS = ['citation_engine', 'foundation_model_campus', 'orbital_mass_driver', 'dyson_swarm_tranche',
       'recursive_self_improvement', 'hyperscale_botnet', 'positron_storage_ring', 'dispatch_control_room',
       'smart_grid_backbone', 'payroll_automation', 'retained_earnings', 'vendor_financing', 'materials_hedge_desk',
       'liquidity_event', 'collateralized_grid', 'conglomerate_headquarters', 'patent_library', 'parts_exchange']
AGENTS = {'strategist': 'engine:strategy=1', 'greedy': 'greedy'}


def read(d):
    out, games = {}, {}
    for r in csv.DictReader(open(f'{d}/cards.csv', encoding='utf-8')):
        o = out.setdefault((r['agent'], r['card']), [0, 0, 0])
        for i, k in enumerate(('offered', 'kept', 'played')):
            o[i] += int(r[k])
    return out


def main():
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    a, b = read(sys.argv[1]), read(sys.argv[2])
    n = 144  # 48 seeds x 3 pairs
    print('| card | strategist r42 → r43 (offered / kept / played) | greedy r42 → r43 |')
    print('|---|---|---|')
    for i in IDS:
        cols = []
        for ag in AGENTS.values():
            x = [v / n for v in a.get((ag, i), [0, 0, 0])]
            y = [v / n for v in b.get((ag, i), [0, 0, 0])]
            cols.append(' / '.join(f'{v:.2f}' for v in x) + ' → ' + ' / '.join(f'{v:.2f}' for v in y))
        print(f'| {i} | {cols[0]} | {cols[1]} |')


if __name__ == '__main__':
    main()
