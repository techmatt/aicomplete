"""127: the engine cards in content (draws, credit / energy / chip income and production, ambassador gains and
triggers), with their plays a game and plays per seat-game in the flagged seat-games against the rest.

    python experiments/results/127_no_limits/engine_cards.py DIR [--agent strategist] [--res credits] [--top 40]

A seat's plays are read from the action stream (`play` actions). "Flagged" is a seat-game whose `--res` peak rises
over 3x its income a round and over 10, or (`--gained N`) one that gained over N of it.
"""
import argparse
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, stream  # noqa: E402

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
INCOME = ('income', 'income_per_tag', 'production', 'production_per_tag', 'gain', 'gain_per_tag')


def walk(ops):
    for o in ops or []:
        yield o
        for k in ('then', 'else', 'effects', 'options', 'a', 'b'):
            v = o.get(k)
            if isinstance(v, list):
                for x in v:
                    if isinstance(x, dict):
                        yield from walk([x])
                    elif isinstance(x, list):
                        yield from walk(x)


def engine_ops(c, res):
    out = []
    for field in ('effects', 'passives'):
        for o in walk(c.get(field)):
            op = o.get('op')
            if op == 'draw' or (op in INCOME and o.get('resource') == res) or (
                    res == 'ambassadors' and op in ('gain_ambassadors',)):
                out.append(f'{field[0]}:{op}' + (f':{o.get("tag")}' if o.get('tag') else '') +
                           f'={o.get("amount", o.get("count", o.get("per", "")))}')
    ab = c.get('ability') or {}
    for o in walk(ab.get('effects')):
        op = o.get('op')
        if op == 'draw' or (op in INCOME and o.get('resource') == res) or (res == 'ambassadors' and op == 'gain_ambassadors'):
            out.append(f'a:{op}={o.get("amount", o.get("count", ""))}')
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dir')
    ap.add_argument('--agent', default='strategist')
    ap.add_argument('--res', default='credits')
    ap.add_argument('--gained', type=int, default=0)
    ap.add_argument('--top', type=int, default=40)
    a = ap.parse_args()
    with open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8') as f:
        cards = json.load(f)
    cards = {c['id']: c for c in (cards if isinstance(cards, list) else cards['cards'])}
    i = PEAKS.index(a.res)
    lines = stream(a.dir)
    labels = cell_labels(next(lines)['spec'])
    fl, rest = collections.Counter(), collections.Counter()
    nf = nr = 0
    for g in lines:
        if not labels[g['cell']].endswith('/' + a.agent):
            continue
        plays = [collections.Counter(), collections.Counter()]
        for act in g['actions']:
            if act.get('type') == 'play':
                plays[act['seat']][act['card']] += 1
        for s in range(2):
            if a.gained:
                hit = g['gained'][s][i] > a.gained
            else:
                rise = g['peaks'][s][i] - g['start'][s][i]
                inc = g['gained'][s][i] / max(g['rounds'], 1)
                hit = inc > 0 and rise > 3 * inc and rise > 10
            if hit:
                nf += 1
                fl.update(plays[s])
            else:
                nr += 1
                rest.update(plays[s])
    print(f'{nf} flagged seat-games, {nr} others ({a.agent}, {a.res})\n')
    print('| card | tier | kind | cost | engine ops | plays a flagged seat-game | a other | ratio |')
    print('|---|---|---|---|---|---|---|---|')
    rows = []
    for cid, c in cards.items():
        ops = engine_ops(c, a.res)
        if not ops:
            continue
        f_ = fl[cid] / max(nf, 1)
        r_ = rest[cid] / max(nr, 1)
        rows.append((f_, cid, c, ops, r_))
    rows.sort(reverse=True)
    for f_, cid, c, ops, r_ in rows[:a.top]:
        ratio = f'{f_ / r_:.1f}' if r_ else '-'
        print(f'| {cid} | {c.get("tier")} | {c.get("kind")} | {c.get("cost")} | {" ".join(ops)} | {f_:.2f} | {r_:.3f} | {ratio} |')


if __name__ == '__main__':
    main()
