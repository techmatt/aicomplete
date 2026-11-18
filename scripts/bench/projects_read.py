"""Each project's life in a read, by difficulty (195's reader, 233's form, promoted in 237). From a run's folder, a
games file with its header (a farmed read, `shards.sh` output) or FILE=SPEC (a headerless farmed JSONL and its spec;
`screens.stream`). r85 counters: `adversary.revealed.<id>`, `adversary.project.<id>`, `adversary.drained.<id>`,
`adversary.project_moves.<id>`.

    python scripts/bench/projects_read.py DIR|GAMES.jsonl|FILE=SPEC [--md OUT.md]

Per game, all three pairs and both agents pooled. "Completes, share of reveals": completions over reveals (a card
revealed twice counts twice). "Moves a completion": token moves on the map at its resolution. Small / big: one
effect or more (content/projects.json).
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts'))
from screens import ROUND, cells_of, stream  # noqa: E402

PRE = ('adversary.revealed.', 'adversary.project.', 'adversary.drained.', 'adversary.project_moves.')


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    path = sys.argv[1]
    md = sys.argv[sys.argv.index('--md') + 1] if '--md' in sys.argv else None
    deck = json.loads((ROOT / 'content' / 'projects.json').read_text(encoding='utf-8'))
    size = {c['id']: 'big' if len(c['effects']) > 1 else 'small' for c in deck}
    names = {c['id']: c['name'] for c in deck}
    tot = defaultdict(lambda: defaultdict(float))
    games = defaultdict(int)
    lines = stream(path)
    cells = cells_of(next(lines)['spec'])
    for g in lines:
        if 'counters' not in g:
            continue
        d = cells[g['cell']]['difficulty']
        games[d] += 1
        for k, v in g['counters'].items():
            for pre in PRE:
                if k.startswith(pre) and not ROUND.search(k):
                    tot[(d, k[len(pre):])][pre] += v
    out = []
    for d in sorted(games):
        n = games[d]
        out.append(f'\n### d{d} ({n} games)\n')
        out.append('| Project | size | revealed a game | completes a game | completes, share of reveals | '
                   'compute drained a game | moves a completion |')
        out.append('|---|---|---|---|---|---|---|')
        for pid in names:
            t = tot[(d, pid)]
            rev, done, moves = t['adversary.revealed.'], t['adversary.project.'], t['adversary.project_moves.']
            per_done = f'{moves / done:.2f}' if done else '-'
            out.append(f'| {names[pid]} | {size[pid]} | {rev / n:.2f} | {done / n:.2f} | {done / rev if rev else 0:.2f} | '
                       f'{t["adversary.drained."] / n:.2f} | {per_done} |')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
