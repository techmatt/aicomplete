#!/usr/bin/env python3
"""The boards readout (075): per variant and agent, wins, how the three boards' powers get used
(every power, a game), Patience's carried insight, cubes bought, free stations, strikes a Wall
stopped, troop hits, cards the new set saw, and what seats leave unspent. Reads the `games.jsonl`
of each experiment output.

    python scripts/cards/board_read.py DIR [DIR ...] [--powers] [--cards]

Several DIRs pool by (variant, agent).
"""
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from corp_read import cnt, load  # noqa: E402
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from powers import power_key  # noqa: E402

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')


def cube_cards():
    """The deck's research-cube cards (118: was a hard-coded six, five of them gone or out of the
    deck since 106): a card in the baseline's cards module whose content projects a cube or pays
    or makes research cubes. Each with whether it has an Action (only then a use to read)."""
    with open(os.path.join(ROOT, 'configs', 'baseline.json'), encoding='utf-8') as f:
        deck = next(m['cards'] for m in json.load(f)['modules'] if m.get('module') == 'cards')
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cards = json.load(f)
    cards = cards['cards'] if isinstance(cards, dict) else cards
    by_id = {c['id']: c for c in cards}
    out = []
    for cid in deck:
        c = by_id.get(cid)
        s = json.dumps(c) if c else ''
        if '"project_cube"' in s or '"research_cubes"' in s:
            out.append((cid, 'ability' in c))
    return out


def se(p, n):
    return 100 * (p * (1 - p) / n) ** 0.5 if n else 0.0


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--powers', action='store_true', help='every power, a game')
    ap.add_argument('--cards', action='store_true', help="the research-cube cards' keeps, plays and uses, a game")
    a = ap.parse_args()
    ais = json.load(open(os.path.join(ROOT, 'content', 'ais.json'), encoding='utf-8'))
    cells = load(a.dirs)
    # 198: a power's counter is keyed by its name (it was its index, which 197 found mislabeled here).
    peer = power_key('researcher', 'Peer Review')
    print('| cell | games | win % | purge wins | rounds | insight carried | cubes bought | free stations | Peer Review | wall strikes | troop hits | lost nodes | left: credits / energy (last round) |')
    print('|---|' + '---|' * 12)
    for (variant, agent), gs in sorted(cells.items()):
        n = len(gs)
        per = lambda k: sum(cnt(g, k) for g in gs) / n
        wins = sum(1 for g in gs if g.get('won'))
        purge = sum(1 for g in gs if g.get('won') and g.get('reason') == 'purge')
        p = wins / n if n else 0
        second = per(peer)
        print(f"| {variant} {agent[:40]} | {n} | {100 * p:.1f} ±{se(p, n):.1f} | {purge} "
              f"| {sum(g.get('rounds', 0) for g in gs) / n:.1f} | {per('economy.insight_carried'):.1f} "
              f"| {per('board.cubes_bought'):.2f} | {per('board.stations_built_free'):.2f} | {second:.2f} "
              f"| {per('board.wall_strikes'):.2f} | {per('board.troop_destroyed'):.2f} | {per('board.node_lost'):.1f} "
              f"| {spent(gs, 'credits'):.1f} / {spent(gs, 'energy'):.1f} |")
    if a.powers:
        print()
        for (variant, agent), gs in sorted(cells.items()):
            n = len(gs)
            per = lambda k: sum(cnt(g, k) for g in gs) / n
            parts = []
            for b in ais:
                for p in b['powers']:
                    v = per(power_key(b['id'], p['name']))
                    parts.append(f"{p['name']} {v:.2f}")
            print(f"{variant} {agent[:40]}: " + ', '.join(parts))
    if a.cards:
        cubes = cube_cards()
        print('\nThe research-cube cards in the deck, a game: kept, played, and used (cards with an Action only).\n')
        for (variant, agent), gs in sorted(cells.items()):
            n = len(gs)
            per = lambda k: sum(cnt(g, k) for g in gs) / n
            got = lambda c, k: sum(g['cards'].get(c, {}).get(k, 0) for g in gs) / n
            parts = [f"{c}: kept {got(c, 'kept'):.2f} played {got(c, 'played'):.2f}"
                     + (f" used {per('cards.used.' + c):.2f}" if act else '') for c, act in cubes]
            print(f"{variant} {agent[:40]}: " + '; '.join(parts))


def spent(gs, resource):
    """What a seat holds at the end of its last action phase, on average."""
    at = ('credits', 'chips', 'energy', 'antimatter', 'ambassadors').index(resource)
    held = [seat[at] for g in gs for seat in g['margin'].get('unspent', [])]
    return sum(held) / len(held) if held else 0.0


if __name__ == '__main__':
    main()
