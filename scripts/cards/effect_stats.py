"""Per-effect statistics for the Unity and Discord decks (167).

    python scripts/cards/effect_stats.py READ [--screen DIR] [--flag 5] [--out FILE.md]

READ is a read (a farmed or native `games.jsonl`) on the current build. For each Unity and Discord card it gives,
per level: the games where it resolved, resolutions a game by tier, and what it did a resolution, from the engine's
`fx.<id>.*` tallies (alignment.rs: the board and the team measured around the card: agents, troops, botnets, markers,
corrupted sites and threats on the table; credits, energy, insight and ambassadors the players hold; free keeps,
income, Unity swayed and pulled, Discord swayed). A prohibition's `prohibit.<step>.<where>.*` tallies are mapped back
to its card and tier: the games it fired in (its condition covered a site the step acted on) among the games its card
resolved, and the arrivals or pulls it kept off a firing game.

`--screen DIR`: an on/off screen (167's experiments/archive/farm/onoff167.json): variants `PAIR` (base) and `PAIR_<card id>` (that
card's three tiers blanked, so the deck order and the Discord clock stand). Each card's paired win effect, on minus
off in points, per level: over every paired game, and over the games where the card resolved in the base (before the
draw both sides play the same game). A card whose effect over its games exceeds `--flag` points at any level is
flagged.
"""
import argparse
import json
import math
import os
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of, stream  # noqa: E402

MEASURES = ['agents', 'troops', 'botnets', 'markers', 'corrupted', 'threats', 'credits', 'energy', 'insight',
            'ambassadors', 'free_keeps', 'income_credits', 'income_insight', 'unity', 'unity_pulled', 'discord']


def deck():
    with open(os.path.join(ROOT, 'content', 'alignment.json'), encoding='utf-8') as f:
        cards = json.load(f)
    bans = {}
    for c in cards:
        for t, tier in enumerate(c['tiers'], 1):
            for e in tier['effects']:
                if e['op'] == 'prohibit':
                    # 182: every (card, tier) printing it: the engine's tally carries no tier, so two tiers (or
                    # cards) sharing a (step, where) share one row, labeled with all of them.
                    bans.setdefault((e['step'], e['where']), []).append((c['id'], t))
    return cards, bans


def tiers_label(bans, k, cid):
    """`2-3`: the tiers of card CID that print prohibition K (one engine tally)."""
    return '-'.join(str(t) for c, t in bans[k] if c == cid)


def games(d):
    it = stream(d)
    spec = next(it)['spec']
    cells = cells_of(spec)
    for g in it:
        c = cells[g['cell']]
        yield c['variant'], c['difficulty'], g


def did(g, cid):
    k = g.get('counters', {})
    return {t: k.get(f'fx.{cid}.t{t}', 0) for t in (1, 2, 3)}


def read_table(read, cards, bans):
    by = defaultdict(lambda: defaultdict(list))  # level -> card -> [game]
    n = defaultdict(int)
    for _, lvl, g in games(read):
        n[lvl] += 1
        for c in cards:
            if sum(did(g, c['id']).values()):
                by[lvl][c['id']].append(g)
    levels = sorted(n)
    rows = []
    for c in cards:
        cid = c['id']
        row = {'id': cid, 'name': c['name'], 'side': c['side'], 'levels': {}}
        for lvl in levels:
            gs = by[lvl][cid]
            res = {t: sum(did(g, cid)[t] for g in gs) for t in (1, 2, 3)}
            total = sum(res.values())
            what = {m: sum(g['counters'].get(f'fx.{cid}.{m}', 0) for g in gs) / total for m in MEASURES} if total else {}
            ban = None
            mine = [k for k, v in bans.items() if any(c == cid for c, _ in v)]
            if mine:
                fired = [g for g in gs if any(g['counters'].get(f'prohibit.{s}.{w}.fired', 0) for s, w in mine)]
                blocked = sum(g['counters'].get(f'prohibit.{s}.{w}.blocked', 0) for g in fired for s, w in mine)
                ban = {'fired': len(fired), 'blocked': blocked,
                       'by_tier': {tiers_label(bans, k, cid): (sum(1 for g in gs if g['counters'].get(f'prohibit.{k[0]}.{k[1]}.made', 0)),
                                                sum(1 for g in gs if g['counters'].get(f'prohibit.{k[0]}.{k[1]}.fired', 0)),
                                                sum(g['counters'].get(f'prohibit.{k[0]}.{k[1]}.blocked', 0) for g in gs))
                                   for k in mine}}
            row['levels'][lvl] = {'games': len(gs), 'of': n[lvl], 'res': res, 'what': what, 'ban': ban}
        rows.append(row)
    return levels, rows


def screen_table(d, cards):
    won = defaultdict(dict)  # (pair, lever, level) -> seed -> (won, game)
    for var, lvl, g in games(d):
        pair, _, lever = var.partition('_')
        won[(pair, lever or 'base', lvl)][g['seed']] = g
    out = {}
    levels = sorted({k[2] for k in won})
    pairs = sorted({k[0] for k in won})
    for c in cards:
        cid = c['id']
        per = {}
        for lvl in levels:
            all_d, its_d = [], []
            for p in pairs:
                base, off = won.get((p, 'base', lvl), {}), won.get((p, cid, lvl), {})
                for s in base:
                    if s not in off:
                        continue
                    dlt = 100.0 * (int(base[s]['won']) - int(off[s]['won']))
                    all_d.append(dlt)
                    if sum(did(base[s], cid).values()):
                        its_d.append(dlt)
            per[lvl] = (stats(all_d), stats(its_d))
        out[cid] = per
    return levels, out


def stats(xs):
    if not xs:
        return (0, 0.0, 0.0)
    m = sum(xs) / len(xs)
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / max(1, len(xs) - 1)) if len(xs) > 1 else 0.0
    return (len(xs), m, sd / math.sqrt(len(xs)))


def fmt_what(w):
    keep = [(m, v) for m, v in w.items() if abs(v) >= 0.05]
    return ', '.join(f'{m} {v:+.1f}' for m, v in keep) or '-'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('read')
    ap.add_argument('--screen')
    ap.add_argument('--flag', type=float, default=5.0)
    ap.add_argument('--out')
    a = ap.parse_args()
    cards, bans = deck()
    levels, rows = read_table(a.read, cards, bans)
    lines = ['## What each effect did (the read)', '',
             'Per level: games it resolved in / games; resolutions a game by tier (I/II/III); what one resolution did '
             '(deltas of the board and the team, summed over seats).', '']
    lines.append('| Card | Side | ' + ' | '.join(f'd{l} games, I/II/III a game' for l in levels) + ' | Did a resolution (all levels) |')
    lines.append('|---|---|' + '---|' * len(levels) + '---|')
    for r in rows:
        cells = []
        tot = defaultdict(float)
        nres = 0
        for l in levels:
            x = r['levels'][l]
            per = '/'.join(f"{x['res'][t] / x['of']:.2f}" for t in (1, 2, 3))
            cells.append(f"{x['games']}/{x['of']}, {per}")
            n = sum(x['res'].values())
            nres += n
            for m, v in x['what'].items():
                tot[m] += v * n
        what = {m: v / nres for m, v in tot.items()} if nres else {}
        lines.append(f"| {r['name']} | {r['side']} | " + ' | '.join(cells) + f' | {fmt_what(what)} |')
    lines += ['', '## Prohibitions', '',
              'Games its card resolved in; of those, games it fired in (its condition covered a site its step acted on) '
              'and arrivals or pulls it kept off a firing game; by tier: made / fired / blocked.', '']
    lines.append('| Card | Level | Resolved | Fired | Fired % | Blocked a firing game | By tier (made/fired/blocked) |')
    lines.append('|---|---|---|---|---|---|---|')
    for r in rows:
        tot = [0, 0, 0]
        tiers_all = defaultdict(lambda: [0, 0, 0])
        for l in levels:
            x = r['levels'][l]
            b = x['ban']
            if b is None:
                break
            tot[0] += x['games']
            tot[1] += b['fired']
            tot[2] += b['blocked']
            for t, v in b['by_tier'].items():
                for i in range(3):
                    tiers_all[t][i] += v[i]
            tiers = '; '.join(f'{t}: {m}/{f}/{k}' for t, (m, f, k) in sorted(b['by_tier'].items()))
            pct = 100.0 * b['fired'] / x['games'] if x['games'] else 0.0
            per = b['blocked'] / b['fired'] if b['fired'] else 0.0
            lines.append(f"| {r['name']} | d{l} | {x['games']} | {b['fired']} | {pct:.0f}% | {per:.1f} | {tiers} |")
        if r['levels'][levels[0]]['ban'] is not None:
            pct = 100.0 * tot[1] / tot[0] if tot[0] else 0.0
            per = tot[2] / tot[1] if tot[1] else 0.0
            # A tier's fire rate: of the games its tier was made in, those it fired in (two tiers sharing a condition
            # share a tally: the later tier's row holds both).
            tiers = '; '.join(f'{t}: {m}/{f}/{k} ({100.0 * f / m if m else 0:.0f}%, {k / f if f else 0:.1f} a fire)'
                              for t, (m, f, k) in sorted(tiers_all.items()))
            lines.append(f"| {r['name']} | all | {tot[0]} | {tot[1]} | **{pct:.0f}%** | **{per:.1f}** | {tiers} |")
    flagged = []
    if a.screen:
        slevels, eff = screen_table(a.screen, cards)
        lines += ['', '## Win effect, on minus off (the on/off screen)', '',
                  f'Points, paired: over every game (n), and over the games the card resolved in (n, ± SE). '
                  f'Flagged: beyond ±{a.flag:g} over its games at any level.', '']
        lines.append('| Card | ' + ' | '.join(f'd{l} all | d{l} its games' for l in slevels) + ' | Flag |')
        lines.append('|---|' + '---|---|' * len(slevels) + '---|')
        for c in cards:
            e = eff[c['id']]
            cells = []
            flag = []
            for l in slevels:
                (na, ma, _), (ni, mi, si) = e[l]
                cells.append(f'{ma:+.1f} ({na})')
                cells.append(f'{mi:+.1f} ± {si:.1f} ({ni})')
                if ni and abs(mi) > a.flag:
                    flag.append(f'd{l}')
            if flag:
                flagged.append((c['name'], flag))
            lines.append(f"| {c['name']} | " + ' | '.join(cells) + f" | {'**' + ', '.join(flag) + '**' if flag else ''} |")
    text = '\n'.join(lines) + '\n'
    if a.out:
        with open(a.out, 'w', encoding='utf-8', newline='\n') as f:
            f.write(text)
    print(text)
    if flagged:
        print('Flagged:', '; '.join(f'{n} ({", ".join(l)})' for n, l in flagged))


if __name__ == '__main__':
    main()
