"""Mean counters a game, side by side for two runs, largest relative change first.

    python experiments/results/137_sites_and_war/diff_counters.py NEW_DIR OLD_DIR [--agent A] [--diff D] [--prefix P] [--top N]

Reads each DIR's games.jsonl (the first line is the header); a game's counter missing counts 0. `--agent` keeps
games whose agent label starts with A (default: the first agent of each run).
"""
import argparse
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / '135_adversary_model'))
import read135  # noqa: E402


def load(d, agent, diff):
    _, games = read135.load(d)
    agents = sorted({a for (_, _, a) in games}, key=len)
    want = agent if agent is not None else agents[0]
    return [g for (df, _, a), gs in games.items() for g in gs if a == want and (diff is None or df == diff)]


def means(games):
    acc = defaultdict(float)
    for g in games:
        for k, v in g['counters'].items():
            acc[k] += v
    return {k: v / max(1, len(games)) for k, v in acc.items()}


def main():
    a = argparse.ArgumentParser()
    a.add_argument('new')
    a.add_argument('old')
    a.add_argument('--agent')
    a.add_argument('--old-agent')
    a.add_argument('--diff', type=int)
    a.add_argument('--prefix', default='')
    a.add_argument('--top', type=int, default=60)
    o = a.parse_args()
    n = load(o.new, o.agent, o.diff)
    m = load(o.old, o.old_agent, o.diff)
    mn, mo = means(n), means(m)
    keys = [k for k in set(mn) | set(mo) if k.startswith(o.prefix) and '.r0' not in k and '.r1' not in k]
    keys.sort(key=lambda k: -abs(mn.get(k, 0) - mo.get(k, 0)) / (abs(mo.get(k, 0)) + 1))
    print(f'{len(n)} new games, {len(m)} old games')
    for k in keys[:o.top]:
        print(f'{k:55s} {mn.get(k, 0):9.2f} {mo.get(k, 0):9.2f}')


if __name__ == '__main__':
    main()
