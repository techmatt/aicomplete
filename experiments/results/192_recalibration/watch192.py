"""192: the watch-list rows no earlier script reads, new against old, per game: Focus (the Builder's 5 credits for 1
insight) uses a Builder game, Vigil's insight, adversary cubes on megaprojects (landed by stations, by ops, paid at
completion), land corruption's drip split face up / face down, and the Protector's Muster landings.

    python watch192.py NEW_DIR OLD_DIR > watch192.md

The other rows come from the earlier items' readers on the same runs: 189's `watch.py` (blocked Rally shares, the
opening's tier-2s, Patience), 189's `resource_use.py` (insight binding), 191's `plays.py` (plays by insight cost, the
node cards), 190's `map_read.py` (growth and neutral claims).
"""
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROWS = [
    ('Focus uses a Builder game', 'ai.power.builder.6', 'builder'),
    ('Vigil insight a Protector game', 'board.vigil', 'protector'),
    ('Vigil firings capped a Protector game', 'board.vigil_capped', 'protector'),
    ('adversary cubes landed by stations', 'board.adversary_cubes', None),
    ('adversary cubes landed by ops', 'board.adversary_cubes_op', None),
    ('adversary cubes paid at completion', 'board.cubes_paid.adversary', None),
    ('Discord from land corruption', 'alignment.discord_from.blight', None),
    ('Protector Muster troops landed', 'adversary.wall_troops', None),
    ('land corruption cleared', 'board.land_cleared', None),
]


def load(d):
    out = []
    with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
        header = json.loads(f.readline())
        cells = header.get('cells') or []
        for line in f:
            g = json.loads(line)
            if 'counters' not in g:
                continue
            cell = cells[g['cell']] if isinstance(g.get('cell'), int) and g['cell'] < len(cells) else {}
            g['_cell'] = json.dumps(cell)
            out.append(g)
    return out


def has_board(g, board):
    # a board's seat uses a power almost every game (Expand); 2,160 of 2,160 at r83
    return any(k.startswith(f'ai.power.{board}.') for k in g['counters'])


def main():
    new, old = load(sys.argv[1]), load(sys.argv[2])
    print(f'New ({len(new)} games), old ({len(old)}). Per game; a board row counts only games with that board.\n')
    print('| measure | new | old |')
    print('|---|---|---|')
    for name, key, board in ROWS:
        vals = []
        for games in (new, old):
            gs = [g for g in games if board is None or has_board(g, board)]
            vals.append(sum(g['counters'].get(key, 0) for g in gs) / max(1, len(gs)))
        print(f'| {name} | {vals[0]:.2f} | {vals[1]:.2f} |')


if __name__ == '__main__':
    main()
