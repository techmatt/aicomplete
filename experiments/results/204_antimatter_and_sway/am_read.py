"""204: the antimatter and Sway measures of a read, by level.

    python experiments/results/204_antimatter_and_sway/am_read.py LABEL=DIR[:LEVER] ...

Per level: games, win %, rounds (all, won), Unity and Discord tips a game, antimatter gained and spent a seat-game,
the desperate action's share of supply, and at d3-d4 the share of won games that spent antimatter. Then antimatter's
sources and uses a seat-game by key (pooled over levels), and by level for the top ones.
"""
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
AM = 4  # experiment.rs PEAKS: credits, insight, energy, chips, antimatter, research_cubes, card_tokens, ambassadors


def load(arg):
    label, _, rest = arg.partition('=')
    d, _, lever = rest.partition(':')
    lever = lever or 'base'
    games = []
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        head = json.loads(next(f))
        names = [v['name'] for v in head['spec']['variants']]
        diffs = head['spec']['difficulties']
        for line in f:
            g = json.loads(line)
            name = names[int(g['cell']) // len(diffs)] if 'difficulty' not in g else names[int(g['cell']) // len(diffs)]
            lev = name.split('_', 1)[1] if '_' in name else 'base'
            if lev == lever:
                games.append(g)
    return label, games


def key(k):
    return k.split(':', 1)[1] if k.startswith('own:') else k


def main():
    for arg in sys.argv[1:]:
        label, games = load(arg)
        print(f'## {label} ({len(games)} games)\n')
        print('| d | games | win % | rounds (won) | Unity tips | Discord tips | antimatter gained / spent a seat-game | '
              'desperate share | won games spending antimatter |')
        print('|---|---|---|---|---|---|---|---|---|')
        by = defaultdict(list)
        for g in games:
            by[g['difficulty']].append(g)
        src_all, use_all, seatgames = defaultdict(float), defaultdict(float), 0
        for d in sorted(by):
            gs = by[d]
            n = len(gs)
            won = [g for g in gs if g['won']]
            seats = sum(len(g['gained']) for g in gs) or 1
            gained = sum(s[AM] for g in gs for s in g['gained']) / seats
            spent = sum(s[AM] for g in gs for s in g['spent']) / seats
            desp = sum(v[AM] for g in gs for s in g['sources'] for k, v in s.items() if key(k) == 'standard:antimatter')
            tot = sum(v[AM] for g in gs for s in g['sources'] for k, v in s.items()) or 1
            spend_won = sum(1 for g in won if sum(s[AM] for s in g['spent']) > 0) / (len(won) or 1)
            ut = sum(g['counters'].get('alignment.tips_unity', 0) for g in gs) / n
            dt = sum(g['counters'].get('alignment.tips_discord', 0) for g in gs) / n
            rw = sum(g['rounds'] for g in won) / (len(won) or 1)
            print(f'| d{d} | {n} | {100 * len(won) / n:.1f} | {sum(g["rounds"] for g in gs) / n:.2f} ({rw:.2f}) | '
                  f'{ut:.2f} | {dt:.2f} | {gained:.2f} / {spent:.2f} | {100 * desp / tot:.0f}% | {100 * spend_won:.0f}% |')
            for g in gs:
                seatgames += len(g['gained'])
                for s in g['sources']:
                    for k, v in s.items():
                        if v[AM]:
                            src_all[key(k)] += v[AM]
                for s in g['uses']:
                    for k, v in s.items():
                        if v[AM]:
                            use_all[key(k)] += v[AM]
        seatgames = seatgames or 1
        for title, tab in (('sources', src_all), ('uses', use_all)):
            tot = sum(tab.values()) or 1
            rows = sorted(tab.items(), key=lambda kv: -kv[1])
            print(f'\nantimatter {title}, a seat-game (share): ' +
                  '; '.join(f'{k} {v / seatgames:.2f} ({100 * v / tot:.0f}%)' for k, v in rows[:12]))
        print()


if __name__ == '__main__':
    main()
