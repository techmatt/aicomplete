"""189: each resource's use a game, new against old, and how often it ran out (the limit proxy).

    python resource_use.py NEW_DIR OLD_DIR > resource_use.md

Per seat and game, from games.jsonl: what was spent (`spent`, PEAKS order) and by what (`uses`: keeps, plays,
powers, standard actions, counters). "Ran out" is the share of seat-rounds a resource ended the action phase at 0
(`unspent.<seat>.<resource>.rNN`; insight from `economy.insight_unspent.rNN`, summed over seats, so a round counts
as out only when every seat was). No engine counter records a keep or play skipped for want of a resource: the
ran-out share and the insight left after keeps are the proxy. Insight's keep-versus-play split is in its own table.
"""
import json
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
UNSPENT = re.compile(r'^unspent\.(\d+)\.([a-z_]+)\.r(\d+)$')


def kind(src):
    # own:play:x, own:keep, own:power:Expand, own:standard:x, counter..., phase:...
    parts = src.split(':')
    if parts[0] == 'own' and len(parts) > 1:
        return parts[1]
    return parts[0]


def load(d):
    n = seats = 0
    spent = defaultdict(float)
    by_use = defaultdict(float)
    out = defaultdict(lambda: [0, 0])  # resource -> [seat-rounds at 0, seat-rounds]
    ins = defaultdict(float)
    lim = defaultdict(float)
    with open(f'{d}/games.jsonl', encoding='utf-8') as f:
        next(f)
        for line in f:
            g = json.loads(line)
            n += 1
            seats += len(g['spent'])
            for row in g['spent']:
                for i, v in enumerate(row):
                    spent[PEAKS[i]] += v
            for u in g.get('uses') or []:
                for src, row in u.items():
                    for i, v in enumerate(row):
                        if v:
                            by_use[(PEAKS[i], kind(src))] += v
            c = g.get('counters', {})
            for k, v in c.items():
                m = UNSPENT.match(k)
                if m:
                    out[m.group(2)][1] += 1
                    out[m.group(2)][0] += v == 0
                elif k.startswith('economy.insight_unspent.r'):
                    out['insight'][1] += 1
                    out['insight'][0] += v == 0
            ins['insight to keeps'] += c.get('cards.insight_kept', 0)
            ins['insight to plays'] += c.get('cards.insight_played', 0)
            ins['insight left after keeps'] += sum(v for k, v in c.items() if k.startswith('cards.insight_after_keep.r'))
            ins['insight unspent at round end'] += sum(v for k, v in c.items() if k.startswith('economy.insight_unspent.r'))
            ins['insight carried (Patience)'] += c.get('economy.insight_carried', 0)
            ins['insight from the cube converter'] += 0
            for k in g.get('keeps') or []:
                ins['cards drafted (offered)'] += k.get('offered', 0)
                ins['cards kept'] += k.get('kept', 0)
                ins['tier-2 chosen in hand-building'] += k.get('tier2', 0)
            ins['cards played'] += sum(v for k, v in c.items() if re.match(r'^cards\.played\.r\d+$', k))
            # 189's counters (a build with them): why a drafted card went unkept, why a card stayed in hand.
            for k, v in c.items():
                if k.startswith('limit.'):
                    lim[k] += v
            ins['rounds'] += g.get('rounds', 0)
    per_seat = {k: v / seats for k, v in spent.items()}
    uses = {k: v / seats for k, v in by_use.items()}
    ins = {k: v / n for k, v in ins.items()}
    ran_out = {k: (a / b if b else 0.0, b) for k, (a, b) in out.items()}
    lim = {k: v / seats for k, v in lim.items()}
    return n, seats, per_seat, uses, ran_out, ins, lim


def main():
    new = load(sys.argv[1])
    old = load(sys.argv[2])
    print(f'New {sys.argv[1]} ({new[0]} games), old {sys.argv[2]} ({old[0]} games). Per seat a game unless marked.\n')
    print('## Spent a game, and the share of seat-rounds it ran out\n')
    print('| resource | spent new | spent old | ran out new | ran out old |')
    print('|---|---|---|---|---|')
    for r in PEAKS:
        if r == 'card_tokens':
            continue
        rn, ro = new[4].get(r, (0, 0)), old[4].get(r, (0, 0))
        print(f'| {r} | {new[2].get(r, 0):.1f} | {old[2].get(r, 0):.1f} | {rn[0]:.0%} | {ro[0]:.0%} |')
    print('\n## What each resource buys (per seat a game)\n')
    print('| resource | use | new | old |')
    print('|---|---|---|---|')
    keys = sorted(set(new[3]) | set(old[3]))
    for r in PEAKS:
        rows = [k for k in keys if k[0] == r and max(new[3].get(k, 0), old[3].get(k, 0)) >= 0.1]
        rows.sort(key=lambda k: -new[3].get(k, 0))
        for k in rows:
            print(f'| {r} | {k[1]} | {new[3].get(k, 0):.2f} | {old[3].get(k, 0):.2f} |')
    print('\n## Insight: keeps against plays (per game, all seats)\n')
    print('| measure | new | old |')
    print('|---|---|---|')
    for k in sorted(set(new[5]) | set(old[5])):
        if k == 'insight from the cube converter':
            continue
        print(f'| {k} | {new[5].get(k, 0):.2f} | {old[5].get(k, 0):.2f} |')

    if new[6] or old[6]:
        print('\n## The limit: why a drafted card went unkept, why a card stayed in hand (per seat a game)\n')
        print('`limit.keep_short.<r>`: a drafted card not kept while its keep cost more than the seat held (r81:')
        print('`insight_bought` is insight short with the credits to buy it, so not a skip; `credits` short of both).')
        print('`limit.play.short.<r>`: a card left in hand at the end of the action phase lacking that resource')
        print('(one card may count for two); `needs_tags`: its tag requirement unmet; `held`: playable, kept back.\n')
        print('| counter | new | old |')
        print('|---|---|---|')
        for k in sorted(set(new[6]) | set(old[6])):
            print(f'| {k} | {new[6].get(k, 0):.2f} | {old[6].get(k, 0):.2f} |')


main()
