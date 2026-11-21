"""309: design_check's three table-cost proxies by level for a read, beside the budgets (design_check.json
`table_cost`), so a friction refresh can say which budgets the rules moved.

    python experiments/results/309_rulebook_friction_refresh/table_cost_read.py GAMES.jsonl [GAMES.jsonl ...]
"""
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
import design_check as dc  # noqa: E402


def main():
    cfg = dc.jload('scripts/design_check.json')
    countries = dc.jload('content/map.json')['countries']
    budget = cfg['table_cost']['budget']
    for path in sys.argv[1:]:
        games = list(dc.games_in(path))
        print(f'## {path} ({len(games)} games)')
        print('| Level | Games | Components (budget) | Checks (budget) | Memory (budget) |')
        print('|---|---|---|---|---|')
        for d, gs in sorted(dc.by_level(games).items()):
            t = dc.table_cost(gs, cfg, countries)
            b = {k: budget[k][min(d, len(budget[k]) - 1)] for k in budget}
            print(f"| d{d} | {len(gs)} | {t['components']:.1f} ({b['components']}) | {t['checks']:.1f} ({b['checks']}) "
                  f"| {t['memory']:.2f} ({b['memory']}) |")


if __name__ == '__main__':
    main()
