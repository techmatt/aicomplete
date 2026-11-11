"""174's timing specs, written into out/ from the standard read's set a (read_specs.py read base 36800 --tag 174).

    python experiments/results/174_speed/specs174.py

out/t174-strat.json: the strategist, set a (48 seeds 36800-36847, three pairs, d0-d3, 576 games).
out/t174-greedy.json: greedy (flat terms) on the first 8 seeds of set a (96 games): a flat-weight agent costs
more a game, so it is capped as experiments.md says.
"""
import json
import os

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".."))


def main():
    a = json.load(open(os.path.join(TOP, "out", "read-174-base-a.json"), encoding="utf-8"))
    rows = [("t174-strat", "strategist", 48), ("t174-greedy", "greedy", 8), ("s174-greedy-a", "greedy", 48)]
    # The discard cap's speed rows (`discards` 1 and 0), on the same games as the base rows.
    for k in (1, 0):
        rows += [(f"t174-strat-d{k}", f"strategist:discards={k}", 48), (f"t174-greedy-d{k}", f"greedy:discards={k}", 8)]
    for name, agent, games in rows:
        s = dict(a)
        s["name"] = name
        s["agents"] = [agent]
        s["games"] = games
        s["note"] = f"174 timing: {agent}, seeds 36800-{36800 + games - 1}, the three pairs, d0-d3."
        p = os.path.join(TOP, "out", name + ".json")
        with open(p, "w", encoding="utf-8", newline="\n") as f:
            json.dump(s, f, indent=2)
        print(p)
    # One game each for the `profile` feature's split (it prints from `experiment` only): set a's slowest
    # strategist game (pr d1 36804), a typical d2 game, and greedy's slowest (pr d2 36802).
    games = [("p174-slow", "strategist", "pr", 1, 36804),
             ("p174-typ", "strategist", "bp", 2, 36800),
             ("p174-gslow", "greedy", "pr", 2, 36802)]
    # The same games with the discard cap at 1 (174's adopted default, named for an older binary).
    games += [(n + "-d1", a + ":discards=1", v, d, s) for n, a, v, d, s in games]
    for name, agent, var, d, seed in games:
        s = dict(a)
        s["name"] = name
        s["agents"] = [agent]
        s["games"] = 1
        s["first_seed"] = seed
        s["difficulties"] = [d]
        s["variants"] = [v for v in a["variants"] if v["name"] == var]
        s["note"] = f"174 profile: {agent}, {var} d{d} seed {seed}."
        p = os.path.join(TOP, "out", name + ".json")
        with open(p, "w", encoding="utf-8", newline="\n") as f:
            json.dump(s, f, indent=2)
        print(p)


if __name__ == "__main__":
    main()
