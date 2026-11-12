"""199: the keep plan, insight by board and chips, per lever (a screen's cell suffix), from games.jsonl.

    python plan_read.py OUT[=SPEC][,OUT[=SPEC]...] [--base NAME] [--section insight|keep|timing|chips|all] [--by-d]

OUT is a run folder (games.jsonl with its header) or a farmed file given its spec as FILE=SPEC; several pool. A
variant `PAIR_LEVER` is lever LEVER (no suffix: `base`). Seats map to boards by the pair (bp: Builder, Protector;
br: Builder, Researcher; pr: Protector, Researcher). Seat-rounds are those whose action phase ended
(`unspent.<s>.credits.rNN`). Counters (199, stats only, `crates/aic-core/src/modules/cards/`):
- insight: `cards.seat_insight_kept.<s>.rNN` and `cards.seat_insight_played.<s>.rNN` (insight to keeps and plays),
  `unspent.<s>.insight.rNN` (held as the action phase ends: the Researcher's includes its Patience bank), bound
  (`cards.insight_bound.<s>.rNN` > 0, 196), Focus uses a Builder game (`ai.power.builder.focus`);
- keep plan: a drafted keep sets `cards.keep_at.<s>.<id>` to its round, set to 0 when it is played; one above 0 at the
  end is a dead keep, "held back" if `cards.keep_ok.<s>.<id>` (it was playable at the end of some action phase),
  else "never had" (never had the credits, energy, chips or tags). Dead keeps from the game's last round are counted
  apart (no round to play them in). A late keep is played 3+ rounds after its keep (`cards.keep_plan.late`,
  `cards.late_play.<id>`), valued at the effect ledger's `reasoned` and `printed` (the table, `values.json`);
- passed: a drafted card unkept short of insight (`limit.keep_short.insight`), keepable but not playable by next
  round (`limit.pass.unaffordable`: tags unmet, or credits over held + a round's credit income), or chosen
  (`limit.pass.chosen`), per seat a game;
- timing: tier-2 (and tier-1) keeps and plays by round, per game (`cards.kept_t2.rNN`, `cards.played_t2.rNN`);
- chips: neutral factory claims a game (`board.claimed.factory`), chips held at the action phase's end a seat-round,
  chips spent a game (`board.chips_spent`) and what they bought (stations, Procure, Mint's make, the converters).
"""
import csv
import json
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import cells_of  # noqa: E402

BOARDS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
SEATK = re.compile(r'^(cards\.seat_insight_kept|cards\.seat_insight_played|cards\.insight_bound)\.(\d+)\.r(\d+)$')
UNSP = re.compile(r'^unspent\.(\d+)\.([a-z_]+)\.r(\d+)$')
INS_UNSP = re.compile(r'^economy\.seat_insight_unspent\.(\d+)\.r(\d+)$')
KEEP_AT = re.compile(r'^cards\.keep_at\.(\d+)\.(.+)$')
BYR = re.compile(r'^cards\.(kept_t1|kept_t2|played_t1|played_t2)\.r(\d+)$')


def ledger():
    out = {}
    with open(os.path.join(ROOT, 'experiments', 'results', 'effect-ledger', 'cards.csv'), encoding='utf-8') as f:
        for r in csv.DictReader(f):
            out[r['card']] = (float(r['reasoned']), float(r['printed']), int(r['tier']))
    return out


def load(dirs):
    rows = []
    for d in dirs:
        # FILE=SPEC@TAG: TAG renames the file's levers (base -> TAG, x -> TAG+x), to pool an agent spec's run beside
        # the rules screen it pairs with
        d, _, spec_path = d.partition('=')
        d, _, tag0 = d.partition('@')
        spec_path, _, tag = spec_path.partition('@')
        tag = tag or tag0
        path = d if os.path.isfile(d) else os.path.join(d, 'games.jsonl')
        with open(path, encoding='utf-8') as f:
            if spec_path:
                with open(spec_path, encoding='utf-8') as s:
                    cells = cells_of(json.load(s))
            else:
                cells = cells_of(json.loads(f.readline())['spec'])
            for line in f:
                try:
                    g = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if 'cell' not in g:
                    continue
                cell = cells[g['cell']]
                v = cell['variant']
                lever = v.split('_', 1)[1] if '_' in v else 'base'
                if tag:
                    lever = tag if lever == 'base' else tag + lever
                rows.append((lever, v.split('_', 1)[0], cell['difficulty'], g))
    return rows


def game_measures(pair, g, led):
    c = g.get('counters', {})
    m = defaultdict(float)
    boards = BOARDS[pair]
    sr = set()
    for k, v in c.items():
        u = UNSP.match(k)
        if u and u.group(2) == 'credits':
            sr.add((int(u.group(1)), int(u.group(3))))
    for s, r in sr:
        b = boards[s % 2]
        m[f'sr.{b}'] += 1
        m['sr'] += 1
    for k, v in c.items():
        x = SEATK.match(k)
        if x:
            s, r = int(x.group(2)), int(x.group(3))
            if (s, r) not in sr:
                continue
            b = boards[s % 2]
            what = {'cards.seat_insight_kept': 'keep', 'cards.seat_insight_played': 'play',
                    'cards.insight_bound': 'bound'}[x.group(1)]
            m[f'{what}.{b}'] += (v > 0) if what == 'bound' else v
            continue
        u = INS_UNSP.match(k)
        if u:
            s, r = int(u.group(1)), int(u.group(2))
            if (s, r) in sr:
                m[f'unsp.{boards[s % 2]}'] += v
            continue
        u = UNSP.match(k)
        if u:
            s, res, r = int(u.group(1)), u.group(2), int(u.group(3))
            if (s, r) not in sr:
                continue
            if res == 'chips':
                m['chips_held'] += v
                m['chip0'] += v == 0
            continue
        x = BYR.match(k)
        if x:
            m[f'{x.group(1)}.r{int(x.group(2))}'] += v
            m[x.group(1)] += v
            continue
        x = KEEP_AT.match(k)
        if x and v > 0:
            s, card = int(x.group(1)), x.group(2)
            b = boards[s % 2]
            if v >= g.get('rounds', 0):
                m['dead_last'] += 1
            elif c.get(f'cards.keep_ok.{s}.{card}', 0) > 0:
                m['dead_held'] += 1
                m[f'dead_held.{b}'] += 1
            else:
                m['dead_never'] += 1
                m[f'dead_never.{b}'] += 1
            continue
        if k.startswith('cards.late_play.'):
            card = k[len('cards.late_play.'):]
            rv, pv, _ = led.get(card, (0.0, 0.0, 0))
            m['late_reasoned'] += rv * v
            m['late_printed'] += pv * v
    # kept cards that left the hand unplayed (sold, discarded): dead keeps counted when they left
    m['dead_never'] += c.get('cards.keep_plan.left_never', 0)
    m['dead_held'] += c.get('cards.keep_plan.left_held', 0)
    m['played_kept'] = c.get('cards.keep_plan.played', 0)
    m['late'] = c.get('cards.keep_plan.late', 0)
    m['wait'] = c.get('cards.keep_plan.wait', 0)
    m['short_ins'] = c.get('limit.keep_short.insight', 0)
    m['pass_unaff'] = c.get('limit.pass.unaffordable', 0)
    m['pass_chosen'] = c.get('limit.pass.chosen', 0)
    m['focus'] = c.get('ai.power.builder.focus', 0)
    m['pat'] = c.get('economy.insight_carried', 0)
    m['synth'] = c.get('ai.power.researcher.synthesis', 0)
    m['builder'] = 'builder' in boards
    m['researcher'] = 'researcher' in boards
    m['seats'] = 2
    m['claims'] = c.get('board.claimed.factory', 0)
    # chips spent and by what: the game's `spent` and `uses` rows (per seat; PEAKS order, chips 4th).
    m['chips_spent'] = sum(row[3] for row in g.get('spent') or [])
    for u in g.get('uses') or []:
        for src, row in u.items():
            if len(row) > 3 and row[3]:
                m[f'use.{src}'] += row[3]
    for s, row in enumerate(g.get('spent') or []):
        m[f'ins_spent.{boards[s % 2]}'] += row[1]
    m['chips_conv'] = c.get('board.chips_converted', 0)
    for k, v in c.items():
        if k.startswith('board.chip_converter.') and not k.startswith('board.chip_converter_idle'):
            m[f'cc.{k.rsplit(".", 1)[1]}'] += v
    m['chip_idle'] = c.get('board.chip_converter_idle', 0)
    m['stations'] = c.get('board.std.station', 0)
    m['procure'] = c.get('ai.power.builder.procure', 0)
    m['mint'] = c.get('ai.power.builder.mint', 0)
    m['scrap'] = c.get('ai.power.builder.scrap', 0)
    m['done'] = sum(1 for k in c if k.startswith('board.project_done') or k.startswith('board.completed'))
    m['won'] = g['won']
    m['rounds'] = g.get('rounds', 0)
    return m


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    base = sys.argv[sys.argv.index('--base') + 1] if '--base' in sys.argv else None
    if base:
        args = [a for a in args if a != base]
    sec = sys.argv[sys.argv.index('--section') + 1] if '--section' in sys.argv else 'all'
    if sec in args:
        args.remove(sec)
    by_d = '--by-d' in sys.argv
    led = ledger()
    groups = defaultdict(lambda: defaultdict(float))
    won = {}
    members = defaultdict(list)
    for lever, pair, d, g in load(args[0].split(',')):
        key = (lever, d if by_d else 'all')
        t = groups[key]
        t['games'] += 1
        won[(lever, pair, d, g['seed'])] = int(g['won'])
        members[key].append((pair, d, g['seed']))
        for k, v in game_measures(pair, g, led).items():
            t[k] += v
    ref = base or 'base'
    gaps = {}
    for key, ms in members.items():
        diffs = [won[(key[0], p, d, s)] - won[(ref, p, d, s)] for p, d, s in ms if (ref, p, d, s) in won]
        if key[0] != ref and len(diffs) > 1:
            mu = sum(diffs) / len(diffs)
            se = (sum((x - mu) ** 2 for x in diffs) / (len(diffs) - 1)) ** 0.5 / len(diffs) ** 0.5
            gaps[key] = f'{100 * mu:+.1f} ± {100 * se:.1f}'
    keys = sorted(groups, key=lambda k: (str(k[1]), k[0] != 'base', k[0]))
    if sec in ('insight', 'all'):
        print('### Insight a seat-round, by board (to keeps / to plays / unspent at the action phase\'s end / bound)\n')
        print('| lever | d | games | win % | gap ± SE | Builder keep / play / unsp / bound | Protector | Researcher |'
              ' Focus a Builder game | carried (Patience) / Synthesis a Researcher game |')
        print('|---|---|---|---|---|---|---|---|---|---|')
        for key in keys:
            t = groups[key]
            cols = []
            for b in ('builder', 'protector', 'researcher'):
                n = max(1, t[f'sr.{b}'])
                cols.append(f"{t[f'keep.{b}'] / n:.2f} / {t[f'play.{b}'] / n:.2f} / {t[f'unsp.{b}'] / n:.1f} / {t[f'bound.{b}'] / n:.0%}")
            nb = max(1, t['builder'])
            nr = max(1, t['researcher'])
            print(f"| {key[0]} | {key[1]} | {int(t['games'])} | {100 * t['won'] / t['games']:.1f} | {gaps.get(key, '-')} |"
                  f" {' | '.join(cols)} |"
                  f" {t['focus'] / nb:.2f} | {t['pat'] / nr:.1f} / {t['synth'] / nr:.2f} |")
        print()
    if sec in ('keep', 'all'):
        print('### The keep plan (per game, both seats)\n')
        print('| lever | d | kept | played from keeps | dead: never had | dead: held back | dead in the last round |'
              ' never-had share | late plays (share of kept plays) | late value: reasoned / printed | mean wait |'
              ' passed: short of insight / unaffordable / chosen (a seat-game) |')
        print('|' + '---|' * 12)
        for key in keys:
            t = groups[key]
            n = t['games']
            kept = t['kept_t1'] + t['kept_t2']
            judged = max(1, kept - t['dead_last'])
            late = max(1, t['late'])
            ns = max(1, n * 2)
            print(f"| {key[0]} | {key[1]} | {kept / n:.1f} | {t['played_kept'] / n:.1f} | {t['dead_never'] / n:.2f} |"
                  f" {t['dead_held'] / n:.2f} | {t['dead_last'] / n:.2f} | {t['dead_never'] / judged:.0%} |"
                  f" {t['late'] / n:.2f} ({t['late'] / max(1, t['played_kept']):.0%}) |"
                  f" {t['late_reasoned'] / late:.1f} / {t['late_printed'] / late:.1f} |"
                  f" {t['wait'] / max(1, t['played_kept']):.2f} |"
                  f" {t['short_ins'] / ns:.2f} / {t['pass_unaff'] / ns:.2f} / {t['pass_chosen'] / ns:.2f} |")
        print()
    if sec in ('timing', 'all'):
        print('### Tier-2 keeps and plays by round (per game, both seats; tier 1 in brackets)\n')
        print('| lever | d | ' + ' | '.join(f'r{r}' for r in range(1, 11)) + ' |')
        print('|' + '---|' * 12)
        for key in keys:
            t = groups[key]
            n = t['games']
            cells = []
            for r in range(1, 11):
                cells.append(f"{t[f'kept_t2.r{r}'] / n:.2f} / {t[f'played_t2.r{r}'] / n:.2f}"
                             f" ({t[f'kept_t1.r{r}'] / n:.1f} / {t[f'played_t1.r{r}'] / n:.1f})")
            print(f"| {key[0]} | {key[1]} | " + ' | '.join(cells) + ' |')
        print()
    if sec in ('chips', 'all'):
        print('### Claims and chips (per game, both seats; held a seat-round)\n')
        print('| lever | d | factory claims | chips held | seat-rounds at 0 | Mint | Scrap | chips spent |'
              ' spent on (chips a game) | chip converter uses (by country) | idle (a seat holding chips could) |')
        print('|' + '---|' * 11)
        for key in keys:
            t = groups[key]
            n = t['games']
            sr = max(1, t['sr'])
            cc = ', '.join(f"{k[3:]}: {v / n:.2f}" for k, v in sorted(t.items()) if k.startswith('cc.')) or '-'
            uses = sorted(((v, k[4:]) for k, v in t.items() if k.startswith('use.')), reverse=True)
            us = ', '.join(f'{src} {v / n:.2f}' for v, src in uses[:6]) or '-'
            print(f"| {key[0]} | {key[1]} | {t['claims'] / n:.2f} | {t['chips_held'] / sr:.2f} | {t['chip0'] / sr:.0%} |"
                  f" {t['mint'] / n:.2f} | {t['scrap'] / n:.2f} | {t['chips_spent'] / n:.2f} | {us} | {cc} |"
                  f" {t['chip_idle'] / n:.2f} |")
        print()
    if sec in ('spent', 'all'):
        print('### Insight spent a seat-round by board (the game rows: keeps, plays, powers, counters, standard)\n')
        print('| lever | d | Builder | Protector | Researcher |')
        print('|---|---|---|---|---|')
        for key in keys:
            t = groups[key]
            print(f"| {key[0]} | {key[1]} | " + ' | '.join(
                f"{t[f'ins_spent.{b}'] / max(1, t[f'sr.{b}']):.2f}" for b in ('builder', 'protector', 'researcher')) + ' |')
        print()


if __name__ == '__main__':
    main()
