"""287: antimatter per card and agent from reads.

usage: python am_cards.py READ[@LABEL]... [--cards id,id,...]

Per read, arm and agent: antimatter gained and spent a seat-game (share spent), its top spends, the share of d3-d4
won games that spent any (and the share of their antimatter spent), and for each antimatter card (default: every card
whose play, pay or use spent it) its keep rate (games kept / games) and plays a game.
"""
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..', 'scripts', 'bench'))
from read_lib import games  # noqa: E402
from screens import PEAKS  # noqa: E402

AM = PEAKS.index('antimatter')


def new():
    return {'n': 0, 'seats': 0, 'gain': 0, 'spent': 0, 'src': defaultdict(float), 'won34': 0, 'won34_spent': 0,
            'won34_gain': 0, 'won34_am': 0, 'kept': defaultdict(int), 'played': defaultdict(int),
            'lv': defaultdict(lambda: [0, 0, 0, 0, 0, 0])}  # level -> seats, gained, spent, wins, wins' gained, spent


def main():
    argv = sys.argv[1:]
    cards = None
    if '--cards' in argv:
        i = argv.index('--cards')
        cards = argv[i + 1].split(',')
        argv = argv[:i] + argv[i + 2:]
    for arg in argv:
        read, _, label = arg.partition('@')
        st = defaultdict(new)
        for G in games(read, label or None):
            g = G.g
            s = st[(G.arm, G.agent)]
            s['n'] += 1
            s['seats'] += len(g['gained'])
            gain = sum(x[AM] for x in g['gained'])
            spent = sum(x[AM] for x in g['spent'])
            s['gain'] += gain
            s['spent'] += spent
            lv = s['lv'][G.level]
            lv[0] += len(g['gained'])
            lv[1] += gain
            lv[2] += spent
            if g['won']:
                lv[3] += 1
                lv[4] += gain
                lv[5] += spent
            for seat in g['uses']:
                for k, v in seat.items():
                    if v[AM]:
                        s['src'][k.split(':', 1)[-1]] += v[AM]
            if g['won'] and G.level >= 3:
                s['won34'] += 1
                s['won34_spent'] += spent > 0
                s['won34_gain'] += gain
                s['won34_am'] += spent
            for cid, c in g['cards'].items():
                s['kept'][cid] += c['kept'] > 0
                s['played'][cid] += c['played']
        print(f'== {arg}')
        for (arm, ag), s in sorted(st.items()):
            n, seats = s['n'], s['seats']
            print(f'{arm} {ag}: {n} games; antimatter a seat-game gained {s["gain"]/seats:.2f} spent '
                  f'{s["spent"]/seats:.2f} ({100*s["spent"]/max(s["gain"],1):.0f}%); d3-d4 wins: {s["won34"]}, '
                  f'{100*s["won34_spent"]/max(s["won34"],1):.0f}% spent any, '
                  f'{100*s["won34_am"]/max(s["won34_gain"],1):.0f}% of their antimatter spent')
            print('  by level (gained > spent a seat-game, share; in wins): ' + '; '.join(
                f'd{d} {v[1]/v[0]:.2f} > {v[2]/v[0]:.2f} {100*v[2]/max(v[1],1):.0f}%, wins {100*v[5]/max(v[4],1):.0f}%'
                for d, v in sorted(s['lv'].items())))
            top = sorted(s['src'].items(), key=lambda kv: -kv[1])[:10]
            print('  spends a seat-game: ' + ', '.join(f'{k} {v/seats:.2f}' for k, v in top))
            ids = cards or sorted({k.split(':')[-1] for k in s['src']})
            for cid in ids:
                print(f'  {cid:30s} kept {100*s["kept"][cid]/n:5.1f}% of games, plays {s["played"][cid]/n:.2f} a game')


if __name__ == '__main__':
    main()
