"""263: the adversary's spread and game length, by level (and arm). Per group: win %, rounds, deck-empty losses
(and what still blocked the purge at the cap), cities the adversary corrupted in play a game (setup's are placed,
not corrupted) with retakes of liberated cities apart, the share of games with at least one new city, and by
round (new cities a game corrupted in round N, and the share of games with one by the end of round N).

    python scripts/cards/spread_read.py FILE=SPEC[@TAG] [FILE=SPEC[@TAG] ...] [--rounds] [--pool-agents] [--legacy]

FILE is a farmed games JSONL (a read folder too; a header line or a half-written last line is skipped) and SPEC its spec (`experiments/farm/<name>.json` or a path); TAG names
its rows (two revisions of one spec, say). The arm is
the variant's name after its pair prefix (`bp-sp` or `bp_sp` -> `sp`; a bare pair name is `base`), so a base spec and a lever
spec read side by side. `map.cities_corrupted` / `map.cities_retaken` are by-round from r124 (263); an older
file falls back to `adversary.corruptions.city` less `adversary.retakes` (every site kind's: a floor on new cities)
and prints no by-round table. From r130 no game tallies retakes (a retaken city counts in `map.cities_corrupted`):
the retakes column prints `-`.
"""
import argparse
import re
import json
import math
import os
import sys
from collections import defaultdict

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
sys.path.insert(0, os.path.join(ROOT, 'scripts'))
from screens import games_in, source  # noqa: E402


def spec_of(s):
    name = s + ('' if s.endswith('.json') else '.json')
    p = s if os.path.exists(s) else os.path.join(ROOT, 'experiments', 'farm', name)
    if not os.path.exists(p):  # 284: a finished item's spec is archived
        p = os.path.join(ROOT, 'experiments', 'archive', 'farm', name)
    with open(p, encoding='utf-8') as f:
        return json.load(f)


def label(spec, cell):
    nd, na = len(spec['difficulties']), len(spec['agents'])
    per_config = len(spec['variants']) * nd * na
    v = (cell % per_config) // (nd * na)
    a = cell % na
    name = spec['variants'][v]['name']
    # 285: `PAIR_LEVER` too (s275-grow's `bp_cap`), which read as base, pooled with it.
    parts = re.split(r'[_-]', name, maxsplit=1)
    arm = parts[1] if len(parts) > 1 else 'base'
    agent = spec['agents'][a].split(':')[0]
    return arm, agent


def retakes_tallied(gs):
    """285: whether any game in the group tallies retakes (none does from r130: the column was a silent 0)."""
    return any(k.startswith('map.cities_retaken') or k == 'adversary.retakes' for g in gs for k in g['counters'])


def by_round(c, key):
    out = {}
    for k, v in c.items():
        if k.startswith(key + '.r'):
            out[int(k.rsplit('.r', 1)[1])] = v
    return out


def main():
    # LF tables when redirected on Windows (a results file is committed).
    sys.stdout.reconfigure(newline='\n')
    ap = argparse.ArgumentParser()
    ap.add_argument('files', nargs='+')
    ap.add_argument('--rounds', action='store_true', help='print the by-round table')
    ap.add_argument('--pool-agents', action='store_true', help='pool judge and greedy')
    ap.add_argument('--legacy', action='store_true',
                    help="read every file by the pre-r124 counters (a floor on new cities), so a revision without "
                         "the by-round tally compares like with like")
    a = ap.parse_args()
    groups = defaultdict(list)
    for fs in a.files:
        path, sp = fs.split('=', 1)
        sp, _, tag = sp.partition('@')
        spec = spec_of(sp)
        for g in games_in(source(path)[0]):
            if 'cell' not in g:
                continue
            arm, agent = label(spec, g['cell'])
            if tag:
                arm = f'{tag} {arm}' if arm != 'base' else tag
            if a.pool_agents:
                agent = 'both'
            groups[(arm, agent, g['difficulty'])].append(g)
    print('| arm | agent | d | games | win % | ±SE | rounds | won in | deck-empty | botnets at cap | new cities a game | games with one | retakes | freed |')
    print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
    tables = {}
    for key in sorted(groups):
        gs = groups[key]
        n = len(gs)
        win = sum(g['won'] for g in gs) / n
        se = math.sqrt(win * (1 - win) / n) * 100
        rounds = sum(g['rounds'] for g in gs) / n
        won = [g for g in gs if g['won']]
        won_r = sum(g['rounds'] for g in won) / max(1, len(won))
        de = [g for g in gs if g['reason'] == 'deck_empty']
        bot = sum(g['margin'].get('botnets_end', 0) for g in de) / max(1, len(de))
        new, ret, any_new = 0.0, 0.0, 0
        per = defaultdict(float)
        first = defaultdict(int)
        have_rounds = False
        for g in gs:
            c = g['counters']
            br = by_round(c, 'map.cities_corrupted')
            if a.legacy:
                br = {}
            if br or any(k.startswith('map.cities_retaken') for k in c) and not a.legacy:
                have_rounds = True
                nn = sum(br.values())
                rr = sum(by_round(c, 'map.cities_retaken').values())
            else:
                # Before r124 retakes were tallied for every site kind: counted against cities, an upper bound.
                rr = min(c.get('adversary.retakes', 0), c.get('adversary.corruptions.city', 0))
                nn = c.get('adversary.corruptions.city', 0) - rr
            new += nn
            ret += rr
            any_new += nn > 0
            got = None
            for r, v in sorted(br.items()):
                per[r] += v
                if v and got is None:
                    got = r
            if got is not None:
                first[got] += 1
        tables[key] = (per, first, n, have_rounds)
        print(f'| {key[0]} | {key[1]} | d{key[2]} | {n} | {win * 100:.1f} | {se:.1f} | {rounds:.2f} | {won_r:.2f} | '
              f'{len(de)} ({len(de) / n * 100:.0f}%) | {bot:.1f} | {new / n:.2f} | {any_new / n * 100:.0f}% | '
              f"{f'{ret / n:.2f}' if retakes_tallied(gs) else '-'} | "
              f"{sum(g['counters'].get('board.cities_freed', 0) for g in gs) / n:.2f} |")
    if a.rounds:
        print()
        print('New cities a game corrupted in round N (and the share of games with one by the end of round N):')
        print()
        print('| arm | agent | d | ' + ' | '.join(f'r{r}' for r in range(1, 14)) + ' |')
        print('|---|---|---|' + '---|' * 13)
        for key, (per, first, n, have) in sorted(tables.items()):
            if not have:
                continue
            cells, cum = [], 0
            for r in range(1, 14):
                cum += first.get(r, 0)
                cells.append(f'{per.get(r, 0) / n:.2f} ({cum / n * 100:.0f}%)')
            print(f'| {key[0]} | {key[1]} | d{key[2]} | ' + ' | '.join(cells) + ' |')


if __name__ == '__main__':
    main()
