"""196: per lever (a screen's cell suffix) and difficulty, the measures this item steers by, paired gaps to base.

    python cells.py OUT_DIR[,OUT_DIR...] [--by-d] [--base NAME]

A variant `PAIR_LEVER` is lever LEVER (no suffix: `base`). Several dirs are pooled (a lever named in two dirs
pools). Columns, per seat-round or per game:
- win %, paired gap to base on the same pair, difficulty and seed, and its SE;
- bound: the share of seat-rounds a keep, or a play short of insight alone, was held for want of insight
  (`cards.insight_bound.<s>.rNN` > 0, 196's counter; seat-rounds from `unspent.<s>.credits.rNN`, and only those
  count); short: any insight shortfall, `bound` included (`cards.insight_short`). The keep half counts any drafted
  card left unkept while the seat held less than its keep, whether or not the seat wanted it;
- out: the share of rounds every seat's insight ended at 0 (189's proxy, `economy.insight_unspent.rNN`);
- unsp: insight unspent at round end, a game (all seats);
- t2: tier-2 chosen in hand-building, a game; kept: cards kept a game; played: cards played a game;
- focus: Focus uses a Builder game (`ai.power.builder.6`); pat: insight carried a Researcher game;
- fclaim: neutral factory claims a game (`board.claimed.factory`); chip0: share of seat-rounds holding 0 chips;
- clear: land corruption cleared a game; tips: Unity tips a game; rounds; cap: share of losses at the round cap.
"""
import json
import math
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'scripts'))
from screens import cells_of  # noqa: E402

BOUND = re.compile(r'^cards\.insight_bound\.(\d+)\.r(\d+)$')
SHORT = re.compile(r'^cards\.insight_short\.(\d+)\.r(\d+)$')
UNSP = re.compile(r'^unspent\.(\d+)\.([a-z_]+)\.r(\d+)$')


def lever_of(variant):
    return variant.split('_', 1)[1] if '_' in variant else 'base'


def has_board(c, board):
    return any(k.startswith(f'ai.power.{board}.') for k in c)


def load(dirs):
    rows = []  # (lever, pair, d, agent, seed, game)
    for d in dirs:
        # a farmed read is the file itself, with no header: give its spec as FILE=SPEC
        d, _, spec_path = d.partition('=')
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
                cell = cells[g['cell']]
                v = cell['variant']
                rows.append((lever_of(v), v.split('_', 1)[0], cell['difficulty'], cell['agent'], g['seed'], g))
    return rows


def measures(g):
    c = g.get('counters', {})
    m = defaultdict(float)
    bound = set()
    chip0 = chipn = 0
    short = set()
    rounds = set()  # the seat-rounds with an action phase that ended (`unspent.<s>.credits.rNN`)
    for k, v in c.items():
        b = BOUND.match(k)
        if b and v > 0:
            bound.add((b.group(1), int(b.group(2))))
        b = SHORT.match(k)
        if b and v > 0:
            short.add((b.group(1), int(b.group(2))))
        u = UNSP.match(k)
        if u:
            if u.group(2) == 'credits':
                rounds.add((u.group(1), int(u.group(3))))
            elif u.group(2) == 'chips':
                chipn += 1
                chip0 += v == 0
    rr = [v for k, v in c.items() if k.startswith('economy.insight_unspent.r')]
    m['out_n'] = sum(v == 0 for v in rr)
    m['out_r'] = len(rr)
    # a keep-phase mark in a round the game ended before its action phase closed has no seat-round to count in
    m['bound_n'] = len(bound & rounds)
    m['short_n'] = len(short & rounds)
    m['sr'] = len(rounds)
    m['chip0_n'] = chip0
    m['chip_sr'] = chipn
    m['unsp'] = sum(v for k, v in c.items() if k.startswith('economy.insight_unspent.r'))
    m['t2'] = sum(k.get('tier2', 0) for k in g.get('keeps') or [])
    m['kept'] = sum(k.get('kept', 0) for k in g.get('keeps') or [])
    m['played'] = sum(v for k, v in c.items() if re.match(r'^cards\.played\.r\d+$', k))
    m['fclaim'] = c.get('board.claimed.factory', 0)
    m['clear'] = c.get('board.land_cleared', 0)
    m['tips'] = (g.get('margin') or {}).get('unity_tips', 0)
    m['rounds'] = g.get('rounds', 0)
    m['builder'] = has_board(c, 'builder')
    m['focus'] = c.get('ai.power.builder.6', 0)
    m['researcher'] = has_board(c, 'researcher')
    m['pat'] = c.get('economy.insight_carried', 0)
    m['cap'] = (not g['won']) and g.get('reason') == 'deck_empty'
    return m


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    by_d = '--by-d' in sys.argv
    base = 'base'
    if '--base' in sys.argv:
        base = sys.argv[sys.argv.index('--base') + 1]
        args = [a for a in args if a != base]
    rows = load(args[0].split(','))
    groups = defaultdict(list)
    index = {}
    for lever, pair, d, agent, seed, g in rows:
        if '--by-pair' in sys.argv:  # a read: each board pair as its own row (the Researcher sits in br and pr)
            lever = pair if lever == 'base' else f'{pair}_{lever}'
        key = (lever, agent, d if by_d else 'all')
        groups[key].append((pair, d, seed, g))
        index[(lever, agent, pair, d, seed)] = g
    print('| lever | agent | d | games | win % | gap ± SE | bound | short | out | unsp | t2 | kept | played | focus |'
          ' pat | fclaim | chip0 | clear | tips | rounds | cap/loss |')
    print('|' + '---|' * 21)
    for key in sorted(groups, key=lambda k: (k[1], str(k[2]), k[0] != base, k[0])):
        lever, agent, d = key
        gs = groups[key]
        n = len(gs)
        wins = sum(g['won'] for _, _, _, g in gs)
        diffs = []
        for pair, dd, seed, g in gs:
            b = index.get((base, agent, pair, dd, seed))
            if b is not None:
                diffs.append(int(g['won']) - int(b['won']))
        if diffs and lever != base:
            mu = sum(diffs) / len(diffs)
            sd = math.sqrt(sum((x - mu) ** 2 for x in diffs) / max(1, len(diffs) - 1)) / math.sqrt(len(diffs))
            gap = f'{100 * mu:+.1f} ± {100 * sd:.1f}'
        else:
            gap = '-'
        ms = [measures(g) for _, _, _, g in gs]
        tot = defaultdict(float)
        for m in ms:
            for k, v in m.items():
                tot[k] += v
        nb = max(1, tot['builder'])
        nr = max(1, tot['researcher'])
        losses = n - wins
        print(f"| {lever} | {agent.split(':')[0] + (':' + agent.split(':', 1)[1] if ':' in agent else '')} | {d} | {n} |"
              f" {100 * wins / n:.1f} | {gap} |"
              f" {tot['bound_n'] / max(1, tot['sr']):.1%} | {tot['short_n'] / max(1, tot['sr']):.1%} | {tot['out_n'] / max(1, tot['out_r']):.1%} |"
              f" {tot['unsp'] / n:.1f} | {tot['t2'] / n:.1f} | {tot['kept'] / n:.1f} | {tot['played'] / n:.1f} |"
              f" {tot['focus'] / nb:.2f} | {tot['pat'] / nr:.1f} | {tot['fclaim'] / n:.2f} |"
              f" {tot['chip0_n'] / max(1, tot['chip_sr']):.0%} | {tot['clear'] / n:.2f} | {tot['tips'] / n:.2f} |"
              f" {tot['rounds'] / n:.2f} | {tot['cap'] / max(1, losses):.0%} |")


if __name__ == '__main__':
    main()
