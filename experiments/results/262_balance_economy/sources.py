"""262: where a resource comes from and goes, per seat a game, pooled (design_check's `tally`): the supply-trim
candidates for research cubes and antimatter.

    python experiments/results/262_balance_economy/sources.py READ [--resource antimatter] [--top 15] [--agent SUBSTR]

READ: a read's folder (games.jsonl) or a farmed out file as FILE=SPEC. `--agent`: only cells whose agent contains it.
"""
import argparse
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'scripts'))
import design_check  # noqa: E402
import screens  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('read')
    ap.add_argument('--resource', action='append')
    ap.add_argument('--top', type=int, default=15)
    ap.add_argument('--agent')
    a = ap.parse_args()
    cells, by_cell = screens.load(a.read)
    games = [g for cid, gs in by_cell.items() for g in gs
             if not a.agent or a.agent in str(cells[cid].get('agent', ''))]
    src, use = design_check.tally(games, 'sources'), design_check.tally(games, 'uses')
    for r in a.resource or ['research_cubes', 'antimatter']:
        tin, tout = sum(src[r].values()), sum(use[r].values())
        print(f'## {r}: {tin:.2f} gained, {tout:.2f} spent a seat-game ({tout / tin:.0%} spent; {len(games)} games)\n')
        for name, t, tot in (('sources', src[r], tin), ('uses', use[r], tout)):
            print(f'| {name} | a seat-game | share |\n|---|---|---|')
            for s, v in sorted(t.items(), key=lambda x: -x[1])[:a.top]:
                print(f'| {s} | {v:.2f} | {v / tot:.0%} |')
            print()


if __name__ == '__main__':
    main()
