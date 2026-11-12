"""195: each project's life in a read: how often it is revealed, completes, is drained, and the token moves it makes,
by difficulty, from a read's games.jsonl (r85 counters: `adversary.revealed.<id>`, `adversary.project.<id>`,
`adversary.drained.<id>`, `adversary.project_moves.<id>`, `adversary.project_acts.<id>`). Usage:

    python experiments/results/195_project_overhaul/projects_read.py READ/games.jsonl [--md OUT.md]

Per game, all three pairs pooled. "Completes, share of reveals": completions over reveals (a card revealed twice
counts twice). "Moves a completion": token moves on the map at its resolution, an ongoing project's every round
counted in "moves a round" instead.
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
from screens import cells_of  # noqa: E402


def main():
    path = sys.argv[1]
    md = sys.argv[sys.argv.index('--md') + 1] if '--md' in sys.argv else None
    deck = json.loads((ROOT / 'content' / 'projects.json').read_text(encoding='utf-8'))
    names = {c['id']: c['name'] for c in deck}
    ongoing = {c['id'] for c in deck if c['ongoing']}
    tot = defaultdict(lambda: defaultdict(float))
    games = defaultdict(int)
    with open(path, encoding='utf-8') as f:
        cells = cells_of(json.loads(f.readline())['spec'])
        for line in f:
            g = json.loads(line)
            if 'counters' not in g:
                continue
            d = cells[g['cell']]['difficulty']
            games[d] += 1
            c = g['counters']
            for k, v in c.items():
                for pre in ('adversary.revealed.', 'adversary.project.', 'adversary.drained.',
                            'adversary.project_moves.', 'adversary.project_acts.'):
                    if k.startswith(pre) and '.r' not in k[len(pre):]:
                        tot[(d, k[len(pre):])][pre] += v
    out = []
    for d in sorted(games):
        n = games[d]
        out.append(f'\n### d{d} ({n} games)\n')
        out.append('| Project | revealed a game | completes a game | completes, share of reveals | compute drained a game | '
                   'moves a completion | ongoing rounds a game | moves a round |')
        out.append('|---|---|---|---|---|---|---|---|')
        for pid in names:
            t = tot[(d, pid)]
            rev, done = t['adversary.revealed.'], t['adversary.project.']
            acts, moves = t['adversary.project_acts.'], t['adversary.project_moves.']
            if pid in ongoing:
                # The resolve-time moves are folded into the per-round figure (an ongoing project's Now line is small).
                per_done, per_round = '-', f'{moves / acts:.2f}' if acts else '-'
            else:
                per_done, per_round = (f'{moves / done:.2f}' if done else '-'), '-'
            out.append(f'| {names[pid]} | {rev / n:.2f} | {done / n:.2f} | {done / rev if rev else 0:.2f} | '
                       f'{t["adversary.drained."] / n:.2f} | {per_done} | {acts / n:.2f} | {per_round} |')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
