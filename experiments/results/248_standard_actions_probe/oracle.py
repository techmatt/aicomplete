"""248: read the oracle spot-check (p248-oracle: arms a, b, c x three pairs x d0-d3, strategist and oracle on the same
seeds): wins of each, the seeds the oracle wins and the judge loses, and how the oracle wins without standard actions.

usage: python oracle.py GAMES.jsonl [--md OUT.md]
cell = ((arm*3 + pair)*4 + level)*2 + agent (0 strategist, 1 oracle)."""
import argparse
import json
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ARMS = ['a', 'b', 'c']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('games')
    ap.add_argument('--md', default='')
    a = ap.parse_args()
    g2 = {}
    for line in open(a.games, encoding='utf-8'):
        if not line.startswith('{"cell"'):
            continue
        g = json.loads(line)
        rest, agent = divmod(g['cell'], 2)
        v, lv = divmod(rest, 4)
        arm, pair = divmod(v, 3)
        c = g['counters']
        g2[(ARMS[arm], agent, g['seed'], pair, lv)] = {
            'won': g['won'], 'reason': g['reason'], 'rounds': g['rounds'],
            'lib_card': c.get('board.liberated_card', 0), 'lib_std': c.get('board.std.liberate', 0),
            'freed': c.get('board.cities_freed', 0), 'move': c.get('board.std.move', 0),
            'played': sum(x.get('played', 0) for x in g['cards'].values()),
            'liberators': sorted(k for k, x in g['cards'].items() if x.get('played') and 'liberat' in k or
                                 x.get('played') and k in LIBERATE_CARDS),
        }
    out = []
    p = out.append
    n = len({k[2:] for k in g2})
    p(f"{n} seed-cells an arm and agent (4 seeds a pair, 12 a level)\n")
    p("| arm | agent | d0 | d1 | d2 | d3 | all | oracle wins where the judge lost |")
    p("|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        for agent, name in ((0, 'strategist'), (1, 'oracle')):
            ks = [k for k in g2 if k[0] == arm and k[1] == agent]
            cells = []
            for lv in range(4):
                kl = [k for k in ks if k[4] == lv]
                cells.append(f"{sum(g2[k]['won'] for k in kl)}/{len(kl)}")
            extra = ''
            if agent == 1:
                flips = sum(1 for k in ks if g2[k]['won'] and not g2[(arm, 0) + k[2:]]['won'])
                back = sum(1 for k in ks if not g2[k]['won'] and g2[(arm, 0) + k[2:]]['won'])
                extra = f"{flips} (judge wins it loses: {back})"
            p(f"| {arm} | {name} | " + " | ".join(cells) + f" | {sum(g2[k]['won'] for k in ks)}/{len(ks)} | {extra} |")
    p("\n## How each wins (won games, means)\n")
    p("| arm | agent | rounds | cities freed | by Liberate | by a card | Dispatches | cards played |")
    p("|---|---|---|---|---|---|---|---|")
    for arm in ARMS:
        for agent, name in ((0, 'strategist'), (1, 'oracle')):
            ks = [k for k in g2 if k[0] == arm and k[1] == agent and g2[k]['won']]
            if not ks:
                continue
            m = lambda f: sum(g2[k][f] for k in ks) / len(ks)
            p(f"| {arm} | {name} | {m('rounds'):.2f} | {m('freed'):.2f} | {m('lib_std'):.2f} | {m('lib_card'):.2f} | {m('move'):.1f} | {m('played'):.1f} |")
    p("\n## The oracle's losses\n")
    for arm in ARMS:
        ks = sorted(k for k in g2 if k[0] == arm and k[1] == 1 and not g2[k]['won'])
        p(f"- {arm}: " + (", ".join(f"seed {k[2]} {['bp', 'br', 'pr'][k[3]]} d{k[4]} ({g2[k]['reason']}, round {g2[k]['rounds']})" for k in ks) or "none"))
    text = "\n".join(out)
    print(text)
    if a.md:
        with open(a.md, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text + "\n")


LIBERATE_CARDS = set()
main()
