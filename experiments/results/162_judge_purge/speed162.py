"""162: two binaries' cost on the same games, side by side so both see the same machine (a busy box
moves the rates, not the ratio). Each game runs on one thread in each binary at once, game by game.

python experiments/results/162_judge_purge/speed162.py OLD.exe NEW.exe AGENT FIRST GAMES [--set K=V]...
Prints each game's agent ms on both, and the ratio of the totals (OLD / NEW: the speed-up).
"""
import json
import os
import subprocess
import sys


def cmd(exe, agent, seed, sets):
    c = [exe, "run", "configs/baseline.json", "--seed", str(seed), "--agent", agent, "--games", "1",
         "--threads", "1", "--json", "--set", "countries=2", "--set", "adversary.difficulty=2"]
    for kv in sets:
        c += ["--set", kv]
    return c


def main():
    old, new, agent, first, games = sys.argv[1:6]
    old, new = os.path.abspath(old), os.path.abspath(new)
    sets = [a.split("=", 1)[1] if a.startswith("--set=") else a for a in sys.argv[6:] if a != "--set"]
    tot = [0.0, 0.0]
    for seed in range(int(first), int(first) + int(games)):
        procs = [subprocess.Popen(cmd(e, agent, seed, sets), stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                  text=True) for e in (old, new)]
        rows = []
        for p in procs:
            out, _ = p.communicate()
            r = json.loads(out)["results"][0]
            rows.append((r["ms_per_game"], r["decisions_mean"], r["rounds_mean"], r["wins"]))
        same = rows[0][1:] == rows[1][1:]
        tot[0] += rows[0][0]
        tot[1] += rows[1][0]
        print(f"seed {seed}: old {rows[0][0]:.0f} ms, new {rows[1][0]:.0f} ms, "
              f"{rows[0][0] / rows[1][0]:.2f}x, decisions {rows[0][1]:.0f}{'' if same else ' DIFFERENT GAME'}",
              flush=True)
    print(f"{agent}: {games} games, old {tot[0] / 1000:.1f} s, new {tot[1] / 1000:.1f} s, {tot[0] / tot[1]:.2f}x")


if __name__ == "__main__":
    main()
