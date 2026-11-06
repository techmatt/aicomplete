"""131's read: agents' value-table arms (`t.<key>=v` agent cells) against a base on the same seeds.

    python experiments/results/131_agents_table/read131.py DIR[=TAG]... [--base TAG/AGENT] [--cards a,b] [--same]

Each DIR is an `aic-sim experiment` output. An arm is `TAG AGENT` (the agent as the spec names it, `strategist`
for `engine:strategy=1`, with its `t.` settings). Per arm: games, win %, the gap against its base agent (the arm's
agent with no `t.` settings, in the --base tag, default the first DIR's tag) paired on pair and seed, rounds, the
strategist's lead over greedy at the same settings (paired), and keep / play per game of the chip cards and
Autonomous Defense Grid (`cards` counts summed over seats). `--same` checks each arm's games against the base
game by game (won, rounds, decisions and the action list): the same games, or the first that differs.
"""
import argparse
import collections
import hashlib
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, gap, stream  # noqa: E402

CARDS = json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))
CHIP = [c['id'] for c in CARDS if 'chips' in json.dumps(c)]
WATCH = CHIP + ['autonomous_defense_grid', 'market_stabilization_board']


def agent_name(a):
    return 'strategist' + a[len('engine:strategy=1'):] if a.startswith('engine:strategy=1') else a


def split(agent):
    """(base agent, its `t.` settings as text)."""
    kind, _, rest = agent.partition(':')
    keep = [p for p in rest.split(',') if p and not p.startswith('t.')]
    t = [p for p in rest.split(',') if p.startswith('t.')]
    base = kind + (':' + ','.join(keep) if keep else '')
    return base, ','.join(t)


def load(arg):
    d, _, tag = arg.partition('=')
    tag = tag or os.path.basename(d.rstrip('/\\'))
    lines = stream(d)
    labels = cell_labels(next(lines)['spec'])
    for g in lines:
        variant, agent = labels[g['cell']].split('/')[-2:]
        yield tag, variant, agent_name(agent), g


def sig(g):
    return (g['won'], g['rounds'], g['decisions'], hashlib.sha1(json.dumps(g['actions']).encode()).hexdigest())


def main():
    p = argparse.ArgumentParser()
    p.add_argument('dirs', nargs='+')
    p.add_argument('--base', help='TAG holding the base agents (default: the first DIR\'s)')
    p.add_argument('--cards', help='comma list of cards to show (default: chip cards and the auto-counters)')
    p.add_argument('--same', action='store_true')
    a = p.parse_args()
    games = collections.defaultdict(dict)  # (tag, agent) -> (variant, seed) -> game
    tags = []
    for arg in a.dirs:
        for tag, variant, agent, g in load(arg):
            if tag not in tags:
                tags.append(tag)
            games[(tag, agent)][(variant, g['seed'])] = g
    base_tag = a.base or tags[0]
    cards = a.cards.split(',') if a.cards else WATCH
    arms = sorted(games, key=lambda k: (tags.index(k[0]), split(k[1])[0] != 'strategist', k[1]))
    print('| arm | games | win % | vs base ± SE | bp / br / pr | rounds | lead over greedy ± SE |')
    print('|---|---|---|---|---|---|---|')
    for tag, agent in arms:
        gs = games[(tag, agent)]
        n = len(gs)
        win = 100 * sum(g['won'] for g in gs.values()) / n
        base, t = split(agent)
        b = games.get((base_tag, base), {})
        pairs = [gs[k]['won'] - b[k]['won'] for k in gs if k in b]
        d, se, _ = gap(pairs)
        by = []
        for v in ('bp', 'br', 'pr'):
            vs = [g['won'] for (vv, _), g in gs.items() if vv == v]
            by.append(f'{100 * sum(vs) / len(vs):.0f}' if vs else '-')
        rounds = sum(g['rounds'] for g in gs.values()) / n
        lead = ''
        if base == 'strategist':
            other = ('greedy:' + t) if t else 'greedy'
            og = games.get((tag, other))
            if og:
                lp = [gs[k]['won'] - og[k]['won'] for k in gs if k in og]
                ld, lse, _ = gap(lp)
                lead = f'{ld:+.1f} ± {lse:.1f}'
        print(f'| {tag} {agent} | {n} | {win:.1f} | {d:+.1f} ± {se:.1f} | {" / ".join(by)} | {rounds:.2f} | {lead} |')
    print()
    print('Keep / play per game (summed over seats):')
    print()
    print('| arm | ' + ' | '.join(cards) + ' |')
    print('|---|' + '---|' * len(cards))
    for tag, agent in arms:
        gs = games[(tag, agent)].values()
        n = len(gs)
        cells = []
        for c in cards:
            k = sum(g['cards'].get(c, {}).get('kept', 0) for g in gs) / n
            pl = sum(g['cards'].get(c, {}).get('played', 0) for g in gs) / n
            cells.append(f'{k:.2f} / {pl:.2f}')
        print(f'| {tag} {agent} | ' + ' | '.join(cells) + ' |')
    if a.same:
        print()
        for tag, agent in arms:
            base, t = split(agent)
            if (tag, agent) == (base_tag, base):
                continue
            b = games.get((base_tag, base))
            if not b:
                continue
            gs = games[(tag, agent)]
            common = sorted(k for k in gs if k in b)
            diff = [k for k in common if sig(gs[k]) != sig(b[k])]
            verdict = 'same games' if not diff else f'{len(diff)} differ, first {diff[0]}'
            print(f'- {tag} {agent} against {base_tag} {base}: {len(common)} games, {verdict}')


if __name__ == '__main__':
    main()
