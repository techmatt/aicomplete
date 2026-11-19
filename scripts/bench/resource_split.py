"""Resource use split by arm, board and round: 247's `diag.py` and `arms.py` (experiments/results/247_resource_economy),
promoted in 283 onto `read_lib.games`, so a farmed read, a lever screen and an agent-lever screen all read.

    python scripts/bench/resource_split.py READ[@LABEL] [READ ...] [--resources energy,antimatter,chips] [--levels D,...]
                                           [--agent-arms] [--top 8] [--by-board] [--rounds]

READ as `read_lib.py`'s (a run folder, a games file with its header, or FILE=SPEC); `--agent-arms` as there
(`strategist:w.sink=1` is arm `sink1` of the strategist). `--resources` names any of `screens.PEAKS`: credits, insight,
energy, chips, antimatter, research_cubes, card_tokens, ambassadors (another name is refused). Per arm and agent (every level pooled, or `--levels`), for
each resource: gained and spent a seat-game and spent of gained; the share of games and of won games in which some
seat spent it (design_check's victory-skip reads the won share at d3-d4); held after the last action phase a seat;
`--by-board`: the same per board; `--rounds`: held after actions by round (the `unspent.<seat>.<r>.rNN` counters);
the top uses and sources (`--top`, a seat-game). arms.py's other half, wins, board powers, standard actions and card
plays paired against the base arm on the same seeds, is `read_lib.py` (same READs, `--agent-arms`). Python's
standard library only.
"""
import os
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from read_lib import games, seat_boards  # noqa: E402

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from screens import PEAKS  # noqa: E402

LEFT = ["credits", "chips", "energy", "antimatter", "ambassadors"]
ROUNDS = 30  # the `unspent.*.rNN` counters read, past any round cap


def take(argv, flag, default=None):
    if flag in argv:
        i = argv.index(flag)
        v = argv[i + 1]
        del argv[i:i + 2]
        return v
    return default


def main(argv):
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    want = take(argv, '--resources', 'energy,antimatter,chips').split(',')
    bad = [r for r in want if r not in PEAKS]
    if bad:  # 285: an unknown name crashed with a bare ValueError at PEAKS.index
        sys.exit(f'resource_split: unknown resource {bad[0]!r} in --resources; accepted: {", ".join(PEAKS)}')
    levels = take(argv, '--levels')
    levels = {int(x) for x in levels.split(',')} if levels else None
    top = int(take(argv, '--top', '8'))
    agent_arms = '--agent-arms' in argv
    by_board = '--by-board' in argv
    rounds = '--rounds' in argv
    unknown = [a for a in argv if a.startswith('--') and a not in ('--agent-arms', '--by-board', '--rounds')]
    if unknown:  # 284: a typo (`--by-boards`) was dropped silently
        sys.exit(f'resource_split: unknown flag {unknown[0]}')
    reads = [a for a in argv if not a.startswith('--')]
    if not reads:
        sys.exit(__doc__)
    bare = set()
    if agent_arms:
        for r in reads:
            from read_lib import stream
            spec = next(stream(r.split('@')[0]))['spec']
            bare |= {a.split(':')[0] for a in spec['agents'] if ':' not in a}
    # (arm, agent) -> group -> stats; group is 'all' or a board
    acc = defaultdict(lambda: defaultdict(lambda: defaultdict(float)))
    for r in reads:
        path, _, label = r.partition('@')
        for gm in games(path, label or None, tuple(bare)):
            if levels is not None and gm.level not in levels:
                continue
            g = gm.g
            key = (gm.arm, gm.agent)
            boards = seat_boards(g)
            spent_any = {res: any(row[PEAKS.index(res)] > 0 for row in g['spent']) for res in want}
            a = acc[key]['all']
            a['games'] += 1
            a['won'] += 1 if g['won'] else 0
            for res in want:
                a[f'any.{res}'] += spent_any[res]
                if g['won']:
                    a[f'wonany.{res}'] += spent_any[res]
            for s, b in enumerate(boards):
                groups = ['all'] + ([b] if by_board else [])
                for grp in groups:
                    t = acc[key][grp]
                    t['seats'] += 1
                    for res in want:
                        i = PEAKS.index(res)
                        t[f'gain.{res}'] += g['gained'][s][i]
                        t[f'spent.{res}'] += g['spent'][s][i]
                        if res in LEFT:
                            held = g['margin'].get('held_after_actions')  # older reads lack it
                            if held:
                                t[f'held.{res}'] += held[s][LEFT.index(res)]
                        for k, v in g['uses'][s].items():
                            t[f'use.{res}.{k}'] += v[i]
                        for k, v in g['sources'][s].items():
                            t[f'src.{res}.{k}'] += v[i]
                        if rounds:
                            for rd in range(1, ROUNDS + 1):
                                c = g['counters'].get(f'unspent.{s}.{res}.r{rd:02d}')
                                if c is not None:
                                    t[f'rd.{res}.{rd}'] += c
                                    t[f'rdn.{res}.{rd}'] += 1
    for (arm, agent), groups in sorted(acc.items()):
        a = groups['all']
        print(f"\n## {arm} / {agent}: {int(a['games'])} games, won {a['won'] / a['games']:.0%}")
        for grp in ['all'] + sorted(k for k in groups if k != 'all'):
            t = groups[grp]
            n = t['seats']
            if not n:
                continue
            if grp != 'all':
                print(f"### {grp} ({int(n)} seat-games)")
            for res in want:
                gain, spent = t[f'gain.{res}'] / n, t[f'spent.{res}'] / n
                line = f"- {res}: gained {gain:.2f} spent {spent:.2f} a seat-game ({spent / gain if gain else 0:.0%} of gained)"
                if res in LEFT:
                    line += f", held after the last action phase {t[f'held.{res}'] / n:.2f}"
                if grp == 'all':
                    won = a['won']
                    line += (f"; spent in {a[f'any.{res}'] / a['games']:.0%} of games, "
                             f"{a[f'wonany.{res}'] / won if won else 0:.0%} of won games")
                print(line)
                if rounds:
                    row = [f"r{rd}:{t[f'rd.{res}.{rd}'] / t[f'rdn.{res}.{rd}']:.1f}"
                           for rd in range(1, ROUNDS + 1) if t[f'rdn.{res}.{rd}']]
                    print('  held after actions by round:', ' '.join(row))
                for kind, word in (('use', 'uses'), ('src', 'sources')):
                    items = sorted(((v / n, k.split('.', 2)[2]) for k, v in t.items()
                                    if k.startswith(f'{kind}.{res}.') and v), reverse=True)[:top]
                    if items:
                        print(f'  {word}: ' + ', '.join(f'{k} {v:.2f}' for v, k in items))


if __name__ == '__main__':
    main(sys.argv[1:])
