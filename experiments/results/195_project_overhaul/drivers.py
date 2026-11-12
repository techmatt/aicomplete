"""195: which projects go with losses: per difficulty, each project's completions a game and the win rate of the games
where it completed against those where it was revealed and didn't, for a read (any revision: `adversary.project.<id>`
and `adversary.revealed.<id>` are tallied since 151). An association, not a cause: a long game reveals more. Usage:

    python experiments/results/195_project_overhaul/drivers.py READ/games.jsonl [--md OUT.md]
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
    done = defaultdict(lambda: [0, 0])  # (d, id) -> [games completed, wins among them]
    miss = defaultdict(lambda: [0, 0])  # revealed, never completed
    comp = defaultdict(float)
    games = defaultdict(int)
    ids = set()
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
                if k.startswith('adversary.revealed.') and '.r' not in k[19:]:
                    pid = k[19:]
                    ids.add(pid)
                    n = c.get('adversary.project.' + pid, 0)
                    comp[(d, pid)] += n
                    bucket = done if n > 0 else miss
                    bucket[(d, pid)][0] += 1
                    bucket[(d, pid)][1] += int(g['won'])
    out = []
    for d in sorted(games):
        out.append(f'\n### d{d} ({games[d]} games)\n')
        out.append('| Project | completes a game | win % when it completed | win % when revealed, not completed | gap |')
        out.append('|---|---|---|---|---|')
        rows = []
        for pid in sorted(ids):
            a, b = done[(d, pid)], miss[(d, pid)]
            wa = 100 * a[1] / a[0] if a[0] else float('nan')
            wb = 100 * b[1] / b[0] if b[0] else float('nan')
            rows.append((wa - wb if a[0] and b[0] else 0, pid, comp[(d, pid)] / games[d], wa, wb, a[0], b[0]))
        for gap, pid, per, wa, wb, na, nb in sorted(rows):
            out.append(f'| {pid} | {per:.2f} | {wa:.0f} ({na}) | {wb:.0f} ({nb}) | {gap:+.0f} |')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
