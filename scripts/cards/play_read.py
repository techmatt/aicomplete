"""Cards kept and never played, and cards never kept, per agent (076).

    python scripts/cards/play_read.py DIR [DIR ...] [--min 1.0] [--agents planner,engine]

Each DIR holds an experiment's games.jsonl and results.json (whose `cells` name each game's agent). Per agent, pooled over its cells and DIRs: for each player card,
the share of games that kept it and its plays in all of them: listed when kept in more than `--min` percent of the
games and played in none (the "kept, never played" read); then every draft card no game of that agent kept.
A card counts once a game, whichever seat held it.
"""
import argparse
import collections
import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))
from screens import dealt  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('dirs', nargs='+')
    ap.add_argument('--min', type=float, default=1.0, help='percent of games kept with 0 plays to list')
    ap.add_argument('--agents', default='', help='comma-separated agent specs to read (default: all)')
    a = ap.parse_args()
    content = json.load(open(os.path.join(ROOT, 'content', 'cards.json'), encoding='utf-8'))
    deck = {c['id']: c for c in content}
    # 118: only the runs' decks.
    in_runs = dealt(a.dirs)
    deck = {k: v for k, v in deck.items() if k in in_runs}
    want = set(x for x in a.agents.split(',') if x)
    games = collections.Counter()
    idle = collections.defaultdict(collections.Counter)
    kept = collections.defaultdict(collections.Counter)
    for d in a.dirs:
        cells = {c['cell']: c for c in json.load(open(os.path.join(d, 'results.json'), encoding='utf-8'))['cells']}
        with open(os.path.join(d, 'games.jsonl'), encoding='utf-8') as f:
            f.readline()
            for line in f:
                g = json.loads(line)
                agent = cells[g['cell']]['agent']
                if want and agent not in want:
                    continue
                games[agent] += 1
                for cid, c in g['cards'].items():
                    if cid not in deck:
                        continue
                    if c.get('kept', 0) > 0:
                        kept[agent][cid] += 1
                    idle[agent][cid] += c.get('played', 0)
    for agent in sorted(games):
        n = games[agent]
        print(f'## {agent} ({n} games)\n')
        rows = sorted(
            ((100.0 * k / n, cid) for cid, k in kept[agent].items() if 100.0 * k / n > a.min and idle[agent][cid] == 0),
            reverse=True,
        )
        print(f'Kept in more than {a.min:g}% of games and never played: {len(rows)}\n')
        if rows:
            print('| card | tier | kind | games kept, % |')
            print('|---|---|---|---|')
            for pct, cid in rows:
                c = deck[cid]
                print(f"| {c['name']} | {c['tier']} | {c['kind']} | {pct:.1f} |")
        never = sorted(deck[cid]['name'] for cid in deck if kept[agent][cid] == 0)
        print(f'\nNever kept: {len(never)}' + (': ' + ', '.join(never) if never else '') + '\n')


if __name__ == '__main__':
    main()
