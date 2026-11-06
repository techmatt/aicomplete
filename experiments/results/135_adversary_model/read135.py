"""135's read: the new adversary model against r53 on the same seeds.

    python experiments/results/135_adversary_model/read135.py DIR [DIR ...] [--md OUT.md]

Per cell (difficulty, board pair, agent) of each DIR's games.jsonl: win %, one SE, rounds, how the games
ended; how the losses happened (end reason, nodes lost by cause); threats dealt, countered and uncountered
per game and per round; the counter kinds used per game; each resource's peak holding per seat, by board.
"""
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

PEAKS = ['credits', 'insight', 'energy', 'chips', 'antimatter', 'research_cubes', 'card_tokens', 'ambassadors']
PAIRS = {'bp': ('builder', 'protector'), 'br': ('builder', 'researcher'), 'pr': ('protector', 'researcher')}
KINDS = ['credits', 'energy', 'insight', 'chips', 'ambassadors', 'research_cubes', 'tags', 'nodes', 'card', 'standing']


def load(d):
    lines = (Path(d) / 'games.jsonl').read_text(encoding='utf-8').splitlines()
    head = json.loads(lines[0])
    spec = head['spec']
    # The order of `cells` in crates/aic-sim/src/experiment.rs: config, variant, difficulty, agent.
    cells = []
    for _ in spec['configs']:
        for v in spec['variants']:
            for diff in spec.get('difficulties', [0]):
                for a in spec['agents']:
                    cells.append((diff, v['name'], a))
    games = defaultdict(list)
    for ln in lines[1:]:
        g = json.loads(ln)
        games[cells[g['cell']]].append(g)
    return head, games


def se(p, n):
    return 100 * math.sqrt(p * (1 - p) / n) if n else 0.0


def main():
    args = sys.argv[1:]
    md = None
    if '--md' in args:
        i = args.index('--md')
        md = args[i + 1]
        args = args[:i] + args[i + 2:]
    out = []
    p = out.append
    for d in args:
        head, games = load(d)
        p(f'## {d} (rules r{head.get("rules_revision")})')
        p('')
        p('| Difficulty | Pair | Agent | Games | Win % | Rounds | Ends |')
        p('|---|---|---|---|---|---|---|')
        by_diff = defaultdict(list)
        for (diff, var, agent), gs in games.items():
            n = len(gs)
            w = sum(g['won'] for g in gs) / n
            r = sum(g['rounds'] for g in gs) / n
            ends = Counter(g['reason'] for g in gs)
            by_diff[(diff, agent)].extend(gs)
            p(f'| d{diff} | {var} | {agent} | {n} | {100 * w:.1f} ±{se(w, n):.1f} | {r:.2f} | '
              + ', '.join(f'{k} {v}' for k, v in ends.most_common()) + ' |')
        p('')
        p('Pooled over the pairs:')
        p('')
        p('| Difficulty | Agent | Games | Win % | Rounds | Losses by end | Nodes lost a game, by cause |')
        p('|---|---|---|---|---|---|---|')
        for (diff, agent), gs in sorted(by_diff.items()):
            n = len(gs)
            w = sum(g['won'] for g in gs) / n
            r = sum(g['rounds'] for g in gs) / n
            losses = Counter(g['reason'] for g in gs if not g['won'])
            lost = Counter()
            for g in gs:
                for k, v in g['counters'].items():
                    if k.startswith('board.node_lost.') and not k[len('board.node_lost.'):].startswith('r'):
                        lost[k[len('board.node_lost.'):]] += v
            p(f'| d{diff} | {agent} | {n} | {100 * w:.1f} ±{se(w, n):.1f} | {r:.2f} | '
              + ', '.join(f'{k} {v}' for k, v in losses.most_common()) + ' | '
              + ', '.join(f'{k} {v / n:.2f}' for k, v in lost.most_common(6)) + ' |')
        p('')
        # Threats.
        p('Threats a game (dealt / countered / uncountered executed), and the counter kinds used a game:')
        p('')
        p('| Difficulty | Agent | Dealt | Countered | Uncountered | ' + ' | '.join(KINDS) + ' |')
        p('|---|---|---|---|---|' + '---|' * len(KINDS))
        for (diff, agent), gs in sorted(by_diff.items()):
            n = len(gs)
            c = lambda k: sum(g['counters'].get(k, 0) for g in gs) / n  # noqa: E731
            p(f'| d{diff} | {agent} | {c("threats.dealt"):.1f} | {c("threats.countered"):.1f} | '
              f'{c("threats.executed"):.1f} | '
              + ' | '.join(f'{c("threats.countered_by." + k):.2f}' for k in KINDS) + ' |')
        p('')
        rounds = sorted({int(k[-2:]) for gs in by_diff.values() for g in gs for k in g['counters']
                         if k.startswith('threats.countered.r')})
        if rounds:
            p('Threats countered / uncountered per round (mean over the games that reached it):')
            p('')
            p('| Difficulty | Agent | ' + ' | '.join(f'r{r}' for r in rounds) + ' |')
            p('|---|---|' + '---|' * len(rounds))
            for (diff, agent), gs in sorted(by_diff.items()):
                cells = []
                for r in rounds:
                    reach = [g for g in gs if g['rounds'] >= r]
                    if not reach:
                        cells.append('')
                        continue
                    cn = sum(g['counters'].get(f'threats.countered.r{r:02}', 0) for g in reach) / len(reach)
                    un = sum(g['counters'].get(f'threats.uncountered.r{r:02}', 0) for g in reach) / len(reach)
                    cells.append(f'{cn:.1f}/{un:.1f}')
                p(f'| d{diff} | {agent} | ' + ' | '.join(cells) + ' |')
            p('')
        # The 135 ops' uses a game (`effects.<op>` tallies), pooled over the cells.
        ops = sorted({k for gs in by_diff.values() for g in gs for k in g['counters'] if k.startswith('effects.')
                      and not k[-3:-2] == 'r' and k.count('.') == 1})
        if ops:
            p('The new ops: resolutions a game, by difficulty and agent:')
            p('')
            p('| Op | ' + ' | '.join(f'd{d} {a}' for d, a in sorted(by_diff)) + ' |')
            p('|---|' + '---|' * len(by_diff))
            for k in ops:
                p(f'| `{k[len("effects."):]}` | ' + ' | '.join(
                    f'{sum(g["counters"].get(k, 0) for g in gs) / len(gs):.2f}' for _, gs in sorted(by_diff.items())) + ' |')
            p('')
        # Peaks by board.
        p("Each resource's peak holding a game, by seat's board (mean):")
        p('')
        p('| Difficulty | Agent | Board | ' + ' | '.join(PEAKS) + ' |')
        p('|---|---|---|' + '---|' * len(PEAKS))
        acc = defaultdict(lambda: [0.0] * len(PEAKS))
        cnt = Counter()
        for (diff, var, agent), gs in games.items():
            for g in gs:
                for s, row in enumerate(g.get('peaks') or []):
                    b = PAIRS[var][s] if var in PAIRS and s < 2 else f'seat{s}'
                    key = (diff, agent, b)
                    cnt[key] += 1
                    for i, v in enumerate(row[:len(PEAKS)]):
                        acc[key][i] += v
        for key in sorted(acc):
            diff, agent, b = key
            p(f'| d{diff} | {agent} | {b} | ' + ' | '.join(f'{v / cnt[key]:.1f}' for v in acc[key]) + ' |')
        p('')
    text = '\n'.join(out) + '\n'
    print(text)
    if md:
        Path(md).write_text(text, encoding='utf-8', newline='\n')


if __name__ == '__main__':
    main()
