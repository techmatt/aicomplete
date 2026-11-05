"""126's read: how often an auto-counter fires in play, and the paired screen of the entry priced from it.

    python experiments/results/126_auto_counter/read126.py DIR[=TAG]... [--base LABEL] [--measure]

Each DIR is an `aic-sim experiment` output (`games.jsonl`). A label is `TAG/AGENT` (`AGENT` with no tag).

- `--measure` reads the 126 measure counters (`m126.*`, from a build with `measures.patch` applied, play
  unchanged): per agent and per holder's board, the auto-counter cards' plays and keeps a game, holder-rounds
  (the play's round to the game's last, both counted), fires (`m126.fires.seat<s>`, a countered threat its
  holder could pay toward), fires a holder-round with its SE (a ratio over holder-seats), the cut a fire by
  resource, its worth in credits at `configs/agents/values.json`'s resource rates, and the entry that worth
  implies (credits a printed point a round: fires a round x credits a fire / the printed amount).
- Always: win %, the paired win gap against `--base` (default the first label) with its SE, the strategist's lead
  over greedy within each tag (paired by pair and seed), and plays (kept, offered) a game of the auto-counter
  cards and `threats.auto_countered` a game.

A play's round is 1 + the rounds' drafts opened before it: a `tier` action whose last action before it (`choose`
aside) is a `pass`.
"""
import argparse
import collections
import json
import math
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..')
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cell_labels, content, stream  # noqa: E402

BOARD = {'b': 'Builder', 'p': 'Protector', 'r': 'Researcher'}
RES = json.load(open(os.path.join(ROOT, 'configs', 'agents', 'values.json'), encoding='utf-8'))['resources']


def auto_cards():
    """{card id: printed amount} of every card with an auto_counter passive."""
    out = {}
    for c in content('cards'):
        for o in c.get('passives', []):
            if o['op'] == 'auto_counter':
                out[c['id']] = o['amount']
    return out


def mean(xs):
    return sum(xs) / len(xs) if xs else float('nan')


def se(xs):
    n = len(xs)
    if n < 2:
        return 0.0
    m = mean(xs)
    return math.sqrt(sum((x - m) ** 2 for x in xs) / (n - 1) / n)


def play_rounds(g, cards):
    """[(seat, card, round played)] for the auto-counter cards played."""
    out = []
    rnd, last = 1, None
    for a in g['actions']:
        t = a['type']
        if t == 'choose':
            continue
        if t == 'tier' and last == 'pass':
            rnd += 1
        if t == 'play' and a.get('card') in cards:
            out.append((a['seat'], a['card'], rnd))
        last = t
    return out


def load(dirs):
    games = collections.defaultdict(dict)  # label -> (pair, seed) -> g
    order = []
    for arg in dirs:
        d, _, tag = arg.partition('=')
        lines = stream(d)
        labels = cell_labels(next(lines)['spec'])
        for g in lines:
            pair, agent = labels[g['cell']].split('/')[-2:]
            label = f'{tag}/{agent}' if tag else agent
            if label not in games:
                order.append(label)
            games[label][(pair, int(g['seed']))] = g
    return games, order


def measure(games, order, cards):
    print('## Fires in play (the 126 measure)\n')
    print('Holder-rounds: from the play\'s round to the game\'s last, both counted. A fire: a threat of the card\'s '
          'type countered while its holder could pay toward it. Entry: credits a printed point a round '
          '(fires a round x credits a fire / the printed amount); the table\'s is 2.5 (0.5 fires x 5).\n')
    print('| cell | board | holders | plays a game | holder-rounds a holder | fires a game held | '
          'fires a holder-round ± SE | cut a fire (cr / en / ins) | credits a fire | entry |')
    print('|---|---|---|---|---|---|---|---|---|---|')
    for lab in order:
        rows = collections.defaultdict(list)  # board -> [(fires, rounds, cut dict)]
        seats_by_board = collections.Counter()
        cut_tot = collections.defaultdict(lambda: collections.Counter())
        fires_tot = collections.Counter()
        amount = {}
        for (pair, _), g in games[lab].items():
            for s, ch in enumerate(pair[:2]):
                seats_by_board[BOARD[ch]] += 1
            held = {}
            for s, c, rnd in play_rounds(g, cards):
                held.setdefault(s, (c, rnd))
            if not held:
                continue
            fires_all = g['counters'].get('m126.fires', 0)
            cut = {r: g['counters'].get(f'm126.cut.{r}', 0) for r in ('credits', 'energy', 'insight')}
            for s, (c, rnd) in held.items():
                b = BOARD[pair[s]]
                f = g['counters'].get(f'm126.fires.seat{s}', 0)
                rows[b].append((f, g['rounds'] - rnd + 1))
                amount[b] = cards[c]
                # the game's cut, shared by the holders' fires
                share = f / fires_all if fires_all else 0
                for r, v in cut.items():
                    cut_tot[b][r] += v * share
                fires_tot[b] += f
        boards = [b for b in ('Builder', 'Protector', 'Researcher') if rows[b]]
        for b in boards + ['all']:
            rs = rows[b] if b != 'all' else [x for bb in boards for x in rows[bb]]
            if not rs:
                continue
            ft = sum(x[0] for x in rs)
            rt = sum(x[1] for x in rs)
            ratio = ft / rt if rt else float('nan')
            n = len(rs)
            rse = math.sqrt(sum((x[0] - ratio * x[1]) ** 2 for x in rs) / (n * (n - 1))) / mean([x[1] for x in rs]) \
                if n > 1 else 0
            ct = collections.Counter()
            fsum = 0
            for bb in (boards if b == 'all' else [b]):
                ct.update(cut_tot[bb])
                fsum += fires_tot[bb]
            per = {r: ct[r] / fsum if fsum else 0 for r in ('credits', 'energy', 'insight')}
            worth = sum(per[r] * RES[r] for r in per)
            amt = amount[b] if b != 'all' else amount[boards[0]]
            entry = ratio * worth / amt
            denom = seats_by_board[b] if b != 'all' else sum(seats_by_board.values())
            print(f'| {lab} | {b} | {n} | {n / denom:.3f} | {rt / n:.2f} | {ft / n:.2f} | {ratio:.3f} ± {rse:.3f} | '
                  f'{per["credits"]:.2f} / {per["energy"]:.2f} / {per["insight"]:.2f} | {worth:.2f} | {entry:.2f} |')
    print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--base')
    ap.add_argument('--measure', action='store_true')
    a = ap.parse_args()
    games, order = load(a.dirs)
    cards = auto_cards()
    if a.measure:
        measure(games, order, cards)
    base = a.base or order[0]

    print('## Win\n')
    print('| cell | games | win % | rounds | win gap vs base ± SE | bp / br / pr win % |')
    print('|---|---|---|---|---|---|')
    for lab in order:
        gs = games[lab]
        win = 100 * mean([g['won'] for g in gs.values()])
        rounds = mean([g['rounds'] for g in gs.values()])
        common = [k for k in gs if k in games[base]]
        diffs = [100 * (gs[k]['won'] - games[base][k]['won']) for k in common]
        gap = f'{mean(diffs):+.1f} ± {se(diffs):.1f}' if lab != base and common else '-'
        per = []
        for p in ('bp', 'br', 'pr'):
            xs = [g['won'] for (pp, _), g in gs.items() if pp == p]
            per.append(f'{100 * mean(xs):.0f}' if xs else '-')
        print(f'| {lab} | {len(gs)} | {win:.1f} | {rounds:.2f} | {gap} | {" / ".join(per)} |')

    print('\n## The strategist\'s lead over greedy (paired by pair and seed)\n')
    print('| tag | lead ± SE |')
    print('|---|---|')
    tags = []
    for lab in order:
        t = lab.rsplit('/', 1)[0] if '/' in lab else ''
        if t not in tags:
            tags.append(t)
    for t in tags:
        s, gr = (f'{t}/strategist', f'{t}/greedy') if t else ('strategist', 'greedy')
        if s in games and gr in games:
            common = [k for k in games[s] if k in games[gr]]
            d = [100 * (games[s][k]['won'] - games[gr][k]['won']) for k in common]
            print(f'| {t or "-"} | {mean(d):+.1f} ± {se(d):.1f} |')
    if len([t for t in tags]) == 2:
        # the change in lead, paired across tags
        t0, t1 = tags
        k0 = (f'{t0}/strategist', f'{t0}/greedy')
        k1 = (f'{t1}/strategist', f'{t1}/greedy')
        if all(k in games for k in k0 + k1):
            common = [k for k in games[k0[0]] if all(k in games[x] for x in k0 + k1)]
            d = [100 * ((games[k1[0]][k]['won'] - games[k1[1]][k]['won']) - (games[k0[0]][k]['won'] - games[k0[1]][k]['won']))
                 for k in common]
            print(f'| {t1} - {t0} | {mean(d):+.1f} ± {se(d):.1f} |')

    print('\n## The auto-counter cards a game: played (kept, offered), and `threats.auto_countered`\n')
    print('| cell | ' + ' | '.join(cards) + ' | auto_countered |')
    print('|---|' + '---|' * (len(cards) + 1))
    for lab in order:
        gs = list(games[lab].values())
        cells = []
        for c in cards:
            pl = mean([g['cards'].get(c, {}).get('played', 0) for g in gs])
            kp = mean([g['cards'].get(c, {}).get('kept', 0) for g in gs])
            of = mean([g['cards'].get(c, {}).get('offered', 0) for g in gs])
            cells.append(f'{pl:.3f} ({kp:.2f}, {of:.2f})')
        ac = mean([g['counters'].get('threats.auto_countered', 0) for g in gs])
        print(f'| {lab} | ' + ' | '.join(cells) + f' | {ac:.2f} |')


if __name__ == '__main__':
    main()
