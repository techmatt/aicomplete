"""128's read: wins and rounds (paired against a base on pair and seed), the strategist's lead over greedy, keep and
play rates per board of a card list, Action uses per build, tags at play of the per-tag cards, energy held after the
last action phase and energy flags per board, credits gained a seat-game (mean, p99, max).

    python experiments/results/128_draft_and_price/read128.py DIR[=TAG]... [--base TAG] [--cards a,b] [--tags]

Each DIR is an `aic-sim experiment` output (`games.jsonl`). A row is `TAG VARIANT/AGENT`; the board pair is the
variant's first two letters (`bp`, `br-lever`). Seats map to boards by the pair's letters. A seat's keeps, plays and
Action uses are read from the action stream; "tags at play" counts the tags on the non-event cards the seat played
before it, plus the card's own (the engine counts cards in play, a program counting itself).
"""
import argparse
import collections
import json
import os
import statistics
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, gap, stream  # noqa: E402

BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
PERTAG = ['economies_of_scale', 'ransomware_syndicate', 'research_royalties', 'defense_appropriations',
          'capacity_market', 'peace_dividend', 'antitrust_action', 'homeland_security_act', 'continental_supergrid']
SINKS = ['particle_accelerator', 'demand_response_grid', 'citywide_microgrid']
DRAWS = ['breach_and_clear', 'force_protection_order', 'predictive_modeling', 'recruit_operatives', 'field_sensor_grid']
DEFAULT = PERTAG + ['orbital_embassy'] + DRAWS + SINKS
CARDS = {c['id']: c for c in json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))}


def per_tag_tag(c):
    """The tag a per-tag card counts, or None."""
    for o in c.get('effects', []) + c.get('passives', []):
        if o.get('op', '').endswith('per_tag'):
            return o.get('tag')
    return None


def agent_name(a):
    """`strategist` for `engine:strategy=1`, with any further settings after it (`strategist,w.tag_hand=0`)."""
    return 'strategist' + a[len('engine:strategy=1'):] if a.startswith('engine:strategy=1') else a


def load(arg):
    d, _, tag = arg.partition('=')
    tag = tag or os.path.basename(d.rstrip('/\\'))
    lines = stream(d)
    labels = cell_labels(next(lines)['spec'])
    for g in lines:
        parts = labels[g['cell']].split('/')
        variant, agent = parts[-2], agent_name(parts[-1])
        pair, _, lever = variant.partition('-')
        yield f'{tag} {lever}'.strip() + f'/{agent}', pair, g


def seat_stream(g, nseats):
    """Per seat: Counter of (kind, card), and the tag counts at each play of a per-tag card."""
    acts = [collections.Counter() for _ in range(nseats)]
    tags = [collections.Counter() for _ in range(nseats)]
    at_play = [[] for _ in range(nseats)]
    for a in g['actions']:
        t = a['type']
        if t not in ('draft', 'keep', 'play', 'use'):
            continue
        s, card = a['seat'], a['card']
        acts[s][(t, card)] += 1
        if t == 'play':
            c = CARDS.get(card, {})
            if c.get('kind') != 'event':
                tags[s].update(c.get('tags', []))
            pt = per_tag_tag(c)
            if pt:
                at_play[s].append((card, tags[s][pt]))
    return acts, at_play


def pct(xs, q):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))] if xs else 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--cards', default='')
    ap.add_argument('--tags', action='store_true', help='tags at play of the per-tag cards')
    a = ap.parse_args()
    cards = a.cards.split(',') if a.cards else DEFAULT
    games = collections.defaultdict(list)          # label -> [(pair, g)]
    seats = collections.defaultdict(list)          # (label, board) -> [seat record]
    for d in a.dirs:
        for label, pair, g in load(d):
            games[label].append((pair, g))
            n = len(g['peaks'])
            acts, at_play = seat_stream(g, n)
            for s in range(n):
                held = (g['margin'].get('held_after_actions') or [[0] * 5] * n)[s]
                rise = g['peaks'][s][2] - (g.get('start') or [[0] * 8] * n)[s][2]
                inc = g['gained'][s][2] / max(g['rounds'], 1)
                seats[(label, BOARD[pair[s]])].append(dict(
                    acts=acts[s], at_play=at_play[s], held_e=held[2], credits=g['gained'][s][0],
                    eflag=inc > 0 and rise > 3 * inc and rise > 10))
    print('## Wins and rounds\n')
    print('| run | games | win % | bp / br / pr | rounds | gap vs base ± SE |')
    print('|---|---|---|---|---|---|')
    for label in sorted(games):
        gs = games[label]
        w = 100 * sum(g['won'] for _, g in gs) / len(gs)
        by = {p: [g['won'] for q, g in gs if q == p] for p in ('bp', 'br', 'pr')}
        bys = ' / '.join(f'{100 * sum(v) / len(v):.0f}' if v else '-' for v in by.values())
        r = statistics.mean(g['rounds'] for _, g in gs)
        gp = ''
        if a.base:
            # pair with the base's same agent, or its plain one (`strategist` for `strategist,w.x=0`)
            agent = label.split('/')[-1]
            base_label = next((b for b in (a.base + '/' + agent, a.base + '/' + agent.split(',')[0])
                               if b in games), None)
            if base_label and base_label != label:
                bk = {(p, g['seed']): g['won'] for p, g in games[base_label]}
                m, se, n = gap([g['won'] - bk[(p, g['seed'])] for p, g in gs if (p, g['seed']) in bk])
                gp = f'{m:+.1f} ± {se:.1f} ({n})'
        print(f'| {label} | {len(gs)} | {w:.1f} | {bys} | {r:.2f} | {gp} |')
    # strategist lead over greedy, paired on (pair, seed) within a tag
    print('\n## Strategist lead over greedy (paired on pair and seed)\n')
    tags = sorted({l.split('/')[0] for l in games})
    for t in tags:
        st, gr = games.get(t + '/strategist'), games.get(t + '/greedy')
        if st and gr:
            gk = {(p, g['seed']): g['won'] for p, g in gr}
            m, se, n = gap([g['won'] - gk[(p, g['seed'])] for p, g in st if (p, g['seed']) in gk])
            print(f'- {t}: {m:+.1f} ± {se:.1f} ({n} pairs)')
    print('\n## Energy and credits per board (seat-games)\n')
    print('| run | board | seats | energy held after last actions | energy flags | credits gained mean / p99 / max |')
    print('|---|---|---|---|---|---|')
    for (label, board), rs in sorted(seats.items()):
        cr = [r['credits'] for r in rs]
        print(f"| {label} | {board} | {len(rs)} | {statistics.mean(r['held_e'] for r in rs):.2f} | "
              f"{sum(r['eflag'] for r in rs)} | {statistics.mean(cr):.0f} / {pct(cr, 0.99)} / {max(cr)} |")
    print('\n## Cards per board: drafted / kept / played / Action uses, a seat-game; uses per build\n')
    print('| run | board | ' + ' | '.join(cards) + ' |')
    print('|---|---|' + '---|' * len(cards))
    for (label, board), rs in sorted(seats.items()):
        n = len(rs)
        cells = []
        for c in cards:
            dr = sum(r['acts'][('draft', c)] for r in rs) / n
            kp = sum(r['acts'][('keep', c)] for r in rs) / n
            pl = sum(r['acts'][('play', c)] for r in rs) / n
            us = sum(r['acts'][('use', c)] for r in rs) / n
            cell = f'{dr:.2f}/{kp:.2f}/{pl:.2f}'
            if CARDS.get(c, {}).get('ability'):
                cell += f'/{us:.2f} ({us / pl:.1f})' if pl else '/0'
            cells.append(cell)
        print(f'| {label} | {board} | ' + ' | '.join(cells) + ' |')
    print('\n## Cards, all boards: kept / played a game (both seats),\n')
    labels = sorted(games)
    print('| card | ' + ' | '.join(labels) + ' |')
    print('|---|' + '---|' * len(labels))
    for c in cards:
        cells = []
        for label in labels:
            rs = [r for (l, _), v in seats.items() if l == label for r in v]
            ng = len(games[label])
            kp = sum(r['acts'][('keep', c)] for r in rs) / ng
            pl = sum(r['acts'][('play', c)] for r in rs) / ng
            us = sum(r['acts'][('use', c)] for r in rs) / ng
            cells.append(f'{kp:.2f} / {pl:.2f}' + (f' / {us:.2f} uses' if CARDS.get(c, {}).get('ability') else ''))
        print(f'| {c} | ' + ' | '.join(cells) + ' |')
    if a.tags:
        print('\n## Tags of the counted tag at play (median / p90 / n), per-tag cards, all boards\n')
        print('| run | ' + ' | '.join(PERTAG) + ' |')
        print('|---|' + '---|' * len(PERTAG))
        for label in sorted(games):
            at = collections.defaultdict(list)
            for (l, _), rs in seats.items():
                if l == label:
                    for r in rs:
                        for c, k in r['at_play']:
                            at[c].append(k)
            print(f'| {label} | ' + ' | '.join(
                f'{statistics.median(at[c]):.0f} / {pct(at[c], 0.9)} / {len(at[c])}' if at[c] else '-'
                for c in PERTAG) + ' |')


if __name__ == '__main__':
    main()
