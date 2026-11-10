"""166: the seeds where two agents of one run differ in outcome, with each game's move counters.

    python flips166.py DIR AGENT_A AGENT_B

AGENT_* are agent names as the spec lists them (cell order: variant, difficulty, agent); lines are paired by (variant, seed).
"""
import json
import pathlib
import sys

KEYS = ['effects.pushed', 'effects.gathered', 'effects.struck', 'effects.moved_canceled', 'board.liberate_standing']


def main():
    d, a, b = sys.argv[1], sys.argv[2], sys.argv[3]
    games = {}
    header = None
    for line in open(pathlib.Path(d) / 'games.jsonl', encoding='utf-8'):
        g = json.loads(line)
        if 'seed' not in g:
            header = g
            continue
        spec = header['spec']
        na, nd = len(spec['agents']), len(spec['difficulties'])
        agent = spec['agents'][g['cell'] % na]
        variant = spec['variants'][g['cell'] // (na * nd)]['name'] + f"/d{spec['difficulties'][g['cell'] // na % nd]}"
        cell = {'variant': variant}
        side = 'a' if agent == a else 'b' if agent == b else None
        if side is None:
            continue
        games.setdefault((cell.get('variant', ''), g['seed']), {})[side] = g
    for key, p in sorted(games.items()):
        if 'a' in p and 'b' in p and p['a']['won'] != p['b']['won']:
            ca = {k: p['a']['counters'].get(k, 0) for k in KEYS}
            cb = {k: p['b']['counters'].get(k, 0) for k in KEYS}
            print(key, 'A won' if p['a']['won'] else 'B won', 'A', ca, 'B', cb)


if __name__ == '__main__':
    main()
