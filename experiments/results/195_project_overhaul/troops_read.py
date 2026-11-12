"""195: adversary troops on the map by round and difficulty, and where a game's troops come from, new read against old
(`map.troops` per round; sources: factory output `board.troops_built`, threats `threats.troops`, the event and
Protector Musters, and r85's project troops `adversary.factory_troops` and `adversary.botnet_troops`). Usage:

    python experiments/results/195_project_overhaul/troops_read.py NEW/games.jsonl OLD/games.jsonl [--md OUT.md]

On the map: the mean over games still running at that round (production's measure, before the march).
`adversary.project_troops` counts every team-slot landing, so the Musters are it less the r85 project troops.
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
from screens import cells_of  # noqa: E402

ROUNDS = range(1, 11)


def read(path):
    on = defaultdict(float)
    alive = defaultdict(int)
    src = defaultdict(float)
    games = defaultdict(int)
    rev = None
    with open(path, encoding='utf-8') as f:
        head = json.loads(f.readline())
        rev = head.get('rules_revision')
        cells = cells_of(head['spec'])
        for line in f:
            g = json.loads(line)
            if 'counters' not in g:
                continue
            d = cells[g['cell']]['difficulty']
            games[d] += 1
            c = g['counters']
            for r in ROUNDS:
                if r <= g['rounds']:
                    alive[(d, r)] += 1
                    on[(d, r)] += c.get(f'map.troops.r{r:02d}', 0)
            proj = c.get('adversary.factory_troops', 0) + c.get('adversary.botnet_troops', 0)
            src[(d, 'factory output')] += c.get('board.troops_built', 0)
            src[(d, 'threats')] += c.get('threats.troops', 0)
            src[(d, 'Musters (events, Protector)')] += c.get('adversary.project_troops', 0) - proj
            src[(d, 'projects: factory troops')] += c.get('adversary.factory_troops', 0)
            src[(d, 'projects: botnet troops')] += c.get('adversary.botnet_troops', 0)
    return rev, games, on, alive, src


def main():
    new, old = sys.argv[1], sys.argv[2]
    md = sys.argv[sys.argv.index('--md') + 1] if '--md' in sys.argv else None
    sides = [read(new), read(old)]
    out = ['Adversary troops on the map, mean over games still running at that round.\n']
    out.append('| d | side | ' + ' | '.join(f'r{r}' for r in ROUNDS) + ' |')
    out.append('|---|---|' + '---|' * len(ROUNDS))
    for d in sorted(sides[0][1]):
        for rev, games, on, alive, _ in sides:
            cells = [f'{on[(d, r)] / alive[(d, r)]:.1f}' if alive[(d, r)] else '-' for r in ROUNDS]
            out.append(f'| d{d} | r{rev} | ' + ' | '.join(cells) + ' |')
    out.append('\nTroops made a game, by source.\n')
    kinds = ['factory output', 'threats', 'Musters (events, Protector)', 'projects: factory troops',
             'projects: botnet troops']
    out.append('| d | side | ' + ' | '.join(kinds) + ' |')
    out.append('|---|---|' + '---|' * len(kinds))
    for d in sorted(sides[0][1]):
        for rev, games, _, _, src in sides:
            out.append(f'| d{d} | r{rev} | ' + ' | '.join(f'{src[(d, k)] / games[d]:.2f}' for k in kinds) + ' |')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
