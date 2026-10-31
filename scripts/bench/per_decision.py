"""Cost per decision, for telling longer games from a slower agent (040).

    python scripts/bench/per_decision.py OLD_ROOT NEW_ROOT [--agent planner] [--games 6]
        [--old-agent SPEC] [--first 1000] [--countries 2] [--pairs 2]

Each ROOT is a checkout with `aic-sim.exe` built at ROOT/aic-sim.exe (and configs/). Plays
the same seeds on both, one thread, in turn so a busy machine hits both alike; the best
of `--pairs` passes. Prints each side's rounds, decisions and ms per game and per decision,
and the ratio. Same rules: the same games, so any ratio is the code. Other rules: other
games on the same seeds, so read ms/decision, and for an agent that plays forward, ms per
decision per round left.
"""
import json
import os
import subprocess
import sys


def one(root, agent, games, first, countries):
    cmd = [os.path.abspath(os.path.join(root, 'aic-sim.exe')), 'run', 'configs/baseline.json',
           '--seed', str(first), '--agent', agent, '--games', str(games), '--threads', '1',
           '--json', '--set', f'countries={countries}', '--set', 'adversary.difficulty=2']
    run = subprocess.run(cmd, cwd=root, capture_output=True, text=True)
    if run.returncode != 0:
        sys.exit(f"{cmd[0]} failed in {root} (its own configs/baseline.json is what it plays): {run.stderr.strip()}")
    out = run.stdout
    r = json.loads(out)['results'][0]
    return {
        'rules': json.loads(out)['rules_revision'],
        'ms': r['ms_per_game'],
        'rounds': r['rounds_mean'],
        'decisions': r['decisions_mean'],
        'wins': r['wins'],
        'options': r['options_per_decision'],
    }


def main():
    argv = sys.argv[1:]

    def opt(name, default):
        return type(default)(argv[argv.index(name) + 1]) if name in argv else default
    old, new = argv[0], argv[1]
    agent, games, first = opt('--agent', 'planner'), opt('--games', 6), opt('--first', 1000)
    old_agent = opt('--old-agent', agent)
    countries, pairs = opt('--countries', 2), opt('--pairs', 2)
    best = {}
    for _ in range(pairs):
        for side, root in (('old', old), ('new', new)):
            r = one(root, old_agent if side == 'old' else agent, games, first, countries)
            if side not in best or r['ms'] < best[side]['ms']:
                best[side] = r
    a, b = best['old'], best['new']
    print(f'{agent}, c{countries}, d2, {games} games from seed {first}, one thread, best of {pairs}')
    print('| | rules | wins | rounds | decisions | options/decision | ms/game | ms/decision |')
    print('|---|---|---|---|---|---|---|---|')
    for name, r in (('old', a), ('new', b)):
        print(f"| {name} | r{r['rules']} | {r['wins']} | {r['rounds']:.2f} | {r['decisions']:.0f} "
              f"| {r['options']:.2f} | {r['ms']:.0f} | {r['ms'] / r['decisions']:.1f} |")
    ratio = lambda f: f(b) / f(a)
    print(f"| new / old | | | {ratio(lambda r: r['rounds']):.2f} | {ratio(lambda r: r['decisions']):.2f} "
          f"| {ratio(lambda r: r['options']):.2f} | {ratio(lambda r: r['ms']):.2f} "
          f"| {ratio(lambda r: r['ms'] / r['decisions']):.2f} |")


if __name__ == '__main__':
    main()
