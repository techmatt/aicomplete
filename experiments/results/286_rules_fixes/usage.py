"""286 (D130): for each threat 286 changed, how often it is dealt, left uncountered and takes a site by its own
execution (its lines and its anchor agent: `threats.corrupted.id.*`), per 100 games, before and after.

python usage.py OLD.jsonl NEW.jsonl [--by-level]
"""
import json
import sys
from collections import defaultdict

IDS = ['agitators', 'insider_access', 'embargo', 'insurrection', 'trade_blockade', 'garrison_state', 'sleeper_cells',
       'iron_curtain', 'manhunt']
NAMES = {'sleeper_cells': 'Rootkit (Sleeper Cells)'}


def side(path):
    games = defaultdict(int)
    tot = defaultdict(lambda: defaultdict(int))
    with open(path, encoding='utf-8') as f:
        for line in f:
            g = json.loads(line)
            if 'format' in g:
                continue
            d = g['difficulty']
            for lvl in (d, 'all'):
                games[lvl] += 1
            c = g['counters']
            for t in IDS:
                for k, key in (('dealt', f'threats.dealt.id.{t}'), ('unc', f'threats.uncountered.id.{t}'),
                               ('corr', f'threats.corrupted.id.{t}')):
                    v = c.get(key, 0)
                    tot[(t, d)][k] += v
                    tot[(t, 'all')][k] += v
    return games, tot


def main():
    sys.stdout.reconfigure(encoding='utf-8')
    old, new = side(sys.argv[1]), side(sys.argv[2])
    levels = ['all'] + (list(range(5)) if '--by-level' in sys.argv else [])
    print('| threat | level | dealt / 100 games old → new | uncountered | takes a site | takes a site, of uncountered |')
    print('|---|---|---|---|---|---|')
    for t in IDS:
        for lvl in levels:
            cells = []
            for k in ('dealt', 'unc', 'corr'):
                o = 100 * old[1][(t, lvl)][k] / max(old[0][lvl], 1)
                n = 100 * new[1][(t, lvl)][k] / max(new[0][lvl], 1)
                cells.append(f'{o:.1f} → {n:.1f}')

            def share(s):
                u = s[1][(t, lvl)]['unc']
                return f"{100 * s[1][(t, lvl)]['corr'] / u:.0f}%" if u else '-'
            cells.append(f'{share(old)} → {share(new)}')
            print(f"| {NAMES.get(t, t.replace('_', ' ').title())} | {lvl} | " + ' | '.join(cells) + ' |')
    print(f"\ngames: old {old[0]['all']}, new {new[0]['all']}")


if __name__ == '__main__':
    main()
