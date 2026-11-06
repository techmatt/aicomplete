"""The loop smoke (121): a 16-seed run read for loops before any full read.

    python scripts/cards/loop_smoke.py [DIR ...] [--run] [--spec experiments/use.json]
        [--games 16] [--seed S] [--max-plays 12] [--threads 6]

With `--run`, or with no DIR, it first runs `aic-sim experiment SPEC --games N [--seed S]
--threads T --out out/loop-smoke` (the release binary) and reads that. For each DIR's
`games.jsonl`, per cell and overall: games, timeouts, the end-reason histogram and the five
games with the most plays of one card (plays, card, cell, seed; a card's plays summed over the
seats). Exits 1 if any game timed out or one card's plays in a game reach `--max-plays`: a
loop shows as a card played dozens of times, or a game that never ends. Seconds once run.
"""
import argparse
import collections
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import cell_labels, stream  # noqa: E402

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
SIM = os.path.join(ROOT, 'target', 'release', 'aic-sim' + ('.exe' if os.name == 'nt' else ''))
OUT = os.path.join(ROOT, 'out', 'loop-smoke')


def most_played(g):
    """(plays, card) of the card played most in game g."""
    best = (0, '')
    for c, row in g.get('cards', {}).items():
        best = max(best, (row.get('played', 0), c))
    return best


def summary(name, games):
    reasons = collections.Counter(g['reason'] for g in games)
    timeouts = reasons.get('timeout', 0)
    mix = ', '.join(f'{r} {n}' for r, n in reasons.most_common())
    print(f'| {name} | {len(games)} | {timeouts} | {mix} |')
    return timeouts


def read(d, a):
    """Prints DIR's tables; returns (timeouts, most plays of one card in a game)."""
    lines = stream(d)
    head = next(lines)
    labels = cell_labels(head['spec'])
    games = collections.defaultdict(list)
    for g in lines:
        g.pop('actions', None)
        games[g['cell']].append(g)
    allg = [g for c in sorted(games) for g in games[c]]
    print(f"\n## {d} (r{head.get('rules_revision', '?')}, {len(allg)} games)\n")
    if not allg:
        print('no games')
        return 0, 0
    print('| cell | games | timeouts | ends |')
    print('|---|---|---|---|')
    for c in sorted(games):
        summary(labels[c] if c < len(labels) else f'cell {c}', games[c])
    timeouts = summary('all', allg)
    rows = sorted(((*most_played(g), labels[g['cell']] if g['cell'] < len(labels) else f"cell {g['cell']}", g['seed'])
                   for g in allg), key=lambda r: (-r[0], r[1], r[2], str(r[3])))
    print(f'\nMost plays of one card in a game (flag at {a.max_plays}):\n')
    print('| plays | card | cell | seed |')
    print('|---|---|---|---|')
    for plays, card, cell, seed in rows[:5]:
        print(f'| {plays} | {card} | {cell} | {seed} |')
    return timeouts, rows[0][0]


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('dirs', nargs='*')
    ap.add_argument('--run', action='store_true', help='run the smoke first (the default with no DIR)')
    ap.add_argument('--spec', default=os.path.join('experiments', 'use.json'))
    ap.add_argument('--games', type=int, default=16)
    ap.add_argument('--seed', type=int)
    ap.add_argument('--max-plays', type=int, default=12, help='fail when one card is played this often in a game')
    ap.add_argument('--threads', type=int, default=6)
    a = ap.parse_args()
    dirs = list(a.dirs)
    if a.run or not dirs:
        if not os.path.exists(SIM):
            sys.exit(f'no binary at {SIM}: build release first')
        cmd = [SIM, 'experiment', a.spec, '--games', str(a.games)]
        if a.seed is not None:
            cmd += ['--seed', str(a.seed)]
        cmd += ['--threads', str(a.threads), '--out', OUT]
        print(' '.join(cmd), flush=True)
        r = subprocess.run(cmd, cwd=ROOT)
        if r.returncode != 0:
            sys.exit(f'aic-sim experiment failed (exit {r.returncode})')
        dirs.append(OUT)
    bad = []
    for d in dirs:
        timeouts, most = read(d, a)
        if timeouts:
            bad.append(f'{d}: {timeouts} timeouts')
        if most >= a.max_plays:
            bad.append(f'{d}: a card played {most} times in a game (flag at {a.max_plays})')
    if bad:
        print('\nFAIL: ' + '; '.join(bad))
        sys.exit(1)
    print('\nno timeouts, no card at the play flag')


if __name__ == '__main__':
    main()
