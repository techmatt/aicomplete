"""142: save every node-loss record of the fresh read's d1-d3 games (r59 final), for 136's death_audit.py (139's, seeds moved).

    python experiments/results/142_threat_intensity/death_runs.py BIN OUT [--threads 4] [--diffs 1,2,3]   # BIN's directory holds configs/

Runs `BIN run configs/baseline.json` for each pair (bp, br, pr), difficulty (1, 2, 3) and seed range of the read
(36776-36799 and 36832-36855), saving the node losses into OUT/rec_d<D>_<pair>_<range>. Then:

    python experiments/results/136_alignment_and_win/death_audit.py --bin BIN --out OUT OUT/rec_d2_* > ...
"""
import subprocess
import sys
from pathlib import Path

PAIRS = {"bp": [], "br": ["--set", "ai.boards=0,2"], "pr": ["--set", "ai.boards=1,2"]}
RANGES = [(36776, 24), (36832, 24)]  # the confirming read (Matt, message 6016)


def main():
    bin_, out = str(Path(sys.argv[1]).resolve()), Path(sys.argv[2]).resolve()
    bdir = str(Path(bin_).parent)  # run from the binary's own directory, with its configs (as death_audit.py does)
    threads = sys.argv[sys.argv.index("--threads") + 1] if "--threads" in sys.argv else "4"
    diffs = sys.argv[sys.argv.index("--diffs") + 1] if "--diffs" in sys.argv else "1,2,3"
    out.mkdir(parents=True, exist_ok=True)
    for d in (int(x) for x in diffs.split(",")):
        for pair, sets in PAIRS.items():
            for seed, games in RANGES:
                dest = out / f"rec_d{d}_{pair}_{seed}"
                cmd = [bin_, "run", "configs/baseline.json", "--games", str(games), "--seed", str(seed),
                       "--agent", "strategist", "--set", f"adversary.difficulty={d}", *sets, "--threads", threads,
                       "--save", str(dest), "--save-when", "node", "--save-max", str(games)]
                print(" ".join(cmd), flush=True)
                subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, cwd=bdir)
    print("done", flush=True)


if __name__ == "__main__":
    main()
